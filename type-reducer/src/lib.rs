use std::collections::BTreeMap;
use std::collections::BTreeSet;

use itertools::Itertools;
use log::debug;
use log::info;
use multimap::MultiMap;
use proc_macro2::{Ident, Span};
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use syn::Fields;
use syn::File;
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Variant;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use syn::visit::Visit;

use syn::visit_mut::VisitMut;

mod visitors;
pub use visitors::*;

pub const COMMON_TYPES_MOD_NAME: &str = "common";
const COMMON_TYPES_FILE_PREAMBLE: &str = "#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;";

const COMMON_TYPES_USE_PREAMBLE: &str = "use super::common::*;\n";
const GENERATED_PREAMBLE: &str =
    "// WARNING: generated file - manual changes will be overriden\n\n";

pub fn read_substitute(customized_names_from_file: &BTreeMap<String, String>, i: &Ident) -> String {
    if let Some(customized_name) = customized_names_from_file.get(&i.to_string()) {
        customized_name.clone()
    } else {
        i.to_string()
    }
}

pub fn read_type_mappings_from_file(
    mapped_names: &Path,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut mapped_types = BTreeMap::new();
    let mapped_names_file = std::fs::File::open(mapped_names)?;
    for line in io::BufReader::new(mapped_names_file)
        .lines()
        .map_while(Result::ok)
    {
        let mut parts = line.split("->");
        if let (Some(type_name), Some(new_type_name)) = (parts.next(), parts.next()) {
            mapped_types.insert(type_name.to_owned(), new_type_name.to_owned());
        }
    }
    Ok(mapped_types)
}

pub fn read_type_names_from_file(
    mapped_names: &Path,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mapped_names_file = std::fs::File::open(mapped_names)?;
    Ok(io::BufReader::new(mapped_names_file)
        .lines()
        .map_while(Result::ok)
        .collect::<BTreeSet<String>>())
}

pub fn write_type_names_to_file(
    mapped_types: &BTreeMap<String, String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut mapped_names_file = std::fs::File::create("./mapped_names.txt")?;
    for v in mapped_types.values().sorted().dedup() {
        mapped_names_file.write_all(format!("{v}\n").as_bytes())?;
    }

    let mut mapped_names_file = std::fs::File::create("./mapped_types_to_names.txt")?;
    for (k, v) in mapped_types
        .iter()
        .sorted_by(|(_, this), (_, other)| this.cmp(other))
    {
        mapped_names_file.write_all(format!("{k}->{v}\n").as_bytes())?;
    }
    Ok(())
}

pub fn delete_replaced_types(file: File, type_names: Vec<String>) -> File {
    let File {
        shebang,
        attrs,
        items,
    } = file;

    let items = items
        .into_iter()
        .filter(|i| match i {
            // delete top level items with ident that was replaced
            Item::Struct(item) => {
                if type_names.contains(&item.ident.to_string()) {
                    debug!("Deleting {}", item.ident);
                    false
                } else {
                    true
                }
            }
            Item::Enum(item) => {
                if type_names.contains(&item.ident.to_string()) {
                    debug!("Deleting {}", item.ident);
                    false
                } else {
                    true
                }
            }
            _ => true,
        })
        .collect();

    File {
        shebang,
        attrs,
        items,
    }
}

pub struct FindSimilarTypesResult {
    pub visitors: Vec<(String, File)>,
    pub similar_structs: MultiMap<Fields, (Ident, ItemStruct)>,
    pub similar_enums: MultiMap<Punctuated<Variant, Comma>, (Ident, ItemEnum)>,
}

pub fn find_similar_types(
    visitors: Vec<(StructEnumVisitor<'_, '_>, File)>,
) -> FindSimilarTypesResult {
    let mut similar_structs = MultiMap::new();
    let mut similar_enums = MultiMap::new();

    let visitors: Vec<_> = visitors
        .into_iter()
        .map(|(mut visitor, file)| {
            visitor.visit_file(&file);
            visitor.structs.into_iter().for_each(|i| {
                let mut fields = i.fields.clone();

                fields.iter_mut().for_each(|f| {
                    f.attrs = f
                        .attrs
                        .clone()
                        .into_iter()
                        .filter(|a| {
                            a.meta.path().get_ident() != Some(&Ident::new("doc", Span::call_site()))
                        })
                        .collect::<Vec<_>>()
                });

                similar_structs.insert(fields, (i.ident.clone(), (*i).clone()));
            });
            visitor.enums.into_iter().for_each(|i| {
                similar_enums.insert(i.variants.clone(), (i.ident.clone(), (*i).clone()));
            });
            (visitor.name, file)
        })
        .collect();

    FindSimilarTypesResult {
        visitors,
        similar_structs,
        similar_enums,
    }
}

pub fn prune_replaced_types(
    renaming_visitor: &mut StructEnumFieldsRenamer,
    visitors: Vec<(String, File)>,
) -> Vec<(String, String, bool)> {
    visitors
        .into_iter()
        .map(|(name, mut f)| {
            renaming_visitor.changed = false;
            renaming_visitor.visit_file_mut(&mut f);
            let new_file =
                delete_replaced_types(f, renaming_visitor.names.keys().cloned().collect());
            (
                name,
                prettyplease::unparse(&new_file),
                renaming_visitor.changed,
            )
        })
        .collect()
}

pub fn generate_file_preamble(
    changed: bool,
    content: &str,
    output_path: &Path,
    name: &str,
) -> Result<std::fs::File, Box<dyn std::error::Error + Send + Sync>> {
    let output_path = output_path.join(name);

    if changed {
        info!("Writing changed file {}", output_path.display());
        let mut out_file = std::fs::File::create(output_path)?;
        if !content.starts_with(GENERATED_PREAMBLE) {
            out_file.write_all(GENERATED_PREAMBLE.as_bytes())?;
        }

        if !content.contains(COMMON_TYPES_USE_PREAMBLE) {
            out_file.write_all(COMMON_TYPES_USE_PREAMBLE.as_bytes())?;
        }
        Ok(out_file)
    } else {
        info!("Writing NOT changed file {}", output_path.display());
        let mut out_file = std::fs::File::create(output_path)?;
        if !content.starts_with(GENERATED_PREAMBLE) {
            out_file.write_all(GENERATED_PREAMBLE.as_bytes())?;
        }
        Ok(out_file)
    }
}

pub fn recreate_project_files(
    out_dir: &str,
    unparsed_files: Vec<(String, String, bool)>,
    items: Vec<Item>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let common_types = prettyplease::unparse(&File {
        shebang: None,
        attrs: vec![],
        items,
    });

    let output_path = std::path::Path::new(&out_dir);
    if output_path.is_dir() && output_path.exists() {
        info!("Writing changed file mod.rs");
        let mut mod_file = std::fs::File::create(output_path.join("mod.rs"))?;
        mod_file.write_all(GENERATED_PREAMBLE.as_bytes())?;

        let mut mod_names = vec![format!("pub mod {COMMON_TYPES_MOD_NAME};")];

        for (name, content, changed) in unparsed_files {
            let mut out_file = generate_file_preamble(changed, &content, output_path, &name)?;
            out_file.write_all(content.as_bytes())?;
            mod_names.push(format!("pub mod {};", &name[..name.len() - 3]));
        }

        for mod_name in mod_names.into_iter().sorted().dedup() {
            mod_file.write_all((mod_name + "\n").as_bytes())?;
        }

        let common_types_file_name = output_path.join(COMMON_TYPES_MOD_NAME.to_owned() + ".rs");

        if common_types_file_name.exists() {
            let mut common_out_file = OpenOptions::new()
                .append(true)
                .open(common_types_file_name)?;

            common_out_file.write_all("\n\n// Next attempt \n\n".as_bytes())?;
            common_out_file.write_all(common_types.as_bytes())?;
        } else {
            let mut common_out_file = std::fs::File::create(common_types_file_name)?;
            let common_types_file_content =
                COMMON_TYPES_FILE_PREAMBLE.to_owned() + "\n\n\n" + &common_types;
            common_out_file.write_all(common_types_file_content.as_bytes())?;
        }
        Ok(())
    } else {
        Err("Make sure that output path is a folder and tha it exists".into())
    }
}

pub fn create_common_type_struct(s: &ItemStruct, type_new_name: &str) -> ItemStruct {
    let mut new_struct = s.clone();
    new_struct.attrs = s
        .attrs
        .iter()
        .filter(|&a| a.meta.path().get_ident() != Some(&Ident::new("doc", Span::call_site())))
        .cloned()
        .collect();
    new_struct.fields = s.fields.clone();
    new_struct.fields.iter_mut().for_each(|f| {
        f.attrs = f
            .attrs
            .clone()
            .into_iter()
            .filter(|a| a.meta.path().get_ident() != Some(&Ident::new("doc", Span::call_site())))
            .collect::<Vec<_>>()
    });

    new_struct.ident = Ident::new(type_new_name, Span::call_site());
    new_struct
}

pub fn create_common_type_enum(s: &ItemEnum, type_new_name: &str) -> ItemEnum {
    let mut new_enum = s.clone();
    new_enum.ident = Ident::new(type_new_name, Span::call_site());
    new_enum.attrs = s
        .attrs
        .iter()
        .filter(|&a| a.meta.path().get_ident() != Some(&Ident::new("doc", Span::call_site())))
        .cloned()
        .collect();
    new_enum
}
