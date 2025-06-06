use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use clap::command;
use itertools::Itertools;
use log::debug;
use log::info;
use log::trace;
use multimap::MultiMap;
use proc_macro2::{Ident, Span};
use std::io::BufRead;
use syn::Fields;
use syn::File;
use syn::Item;
use syn::ItemStruct;
use syn::PathSegment;
use syn::Type;
use syn::visit::{self, Visit};
use syn::visit_mut;
use syn::visit_mut::VisitMut;

const COMMON_TYPES_FILE_PREAMBLE: &str = "#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;";

const COMMON_TYPES_USE_PREAMBLE: &str = "use super::common_types::*;";
const GENERATED_PREAMBLE: &str = "// WARNING! generated file do not edit\n\n";

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_logger::init_with_env().unwrap();
    let Args {
        apis_dir,
        out_dir,
        current_pass_substitute_names,
        previous_pass_derived_type_names,
    } = Args::parse();

    let Ok(_) = fs::exists(out_dir.clone()) else {
        return Err("our dir doesn't exist".into());
    };

    let current_pass_type_name_substitutes =
        if let Some(mapped_names) = current_pass_substitute_names.as_ref() {
            read_type_mappings_from_file(mapped_names.as_path())?
        } else {
            BTreeMap::new()
        };

    let previous_pass_derived_type_names =
        if let Some(mapped_names) = previous_pass_derived_type_names.as_ref() {
            read_type_names_from_file(mapped_names.as_path())?
        } else {
            BTreeSet::new()
        };

    let mut visitors = vec![];

    for dir_entry in fs::read_dir(apis_dir).unwrap() {
        let Ok(dir_entry) = dir_entry else {
            continue;
        };

        if let Ok(name) = dir_entry.file_name().into_string() {
            if name.ends_with(".rs") && name != "mod.rs" {
                info!("Adding file {:?}", dir_entry.path());
                if let Ok(api_file) = fs::read_to_string(dir_entry.path()) {
                    if let Ok(syntaxt_file) = syn::parse_file(&api_file) {
                        let visitor = StructVisitor {
                            name,
                            structs: Vec::new(),
                            derived_type_names: &previous_pass_derived_type_names,
                        };
                        visitors.push((visitor, syntaxt_file));
                    }
                }
            }
        }
    }

    let FindSimilarTypesResult {
        visitors,
        similar_items,
    } = find_similar_types(visitors);

    let items: Vec<_> = similar_items
        .iter_all()
        .filter(|(_k, v)| v.len() > 1)
        .filter_map(|(_k, v)| {
            info!(
                "Potentially similar structs {:#?}",
                v.iter().map(|(i, _)| i.to_string()).collect::<Vec<_>>()
            );
            let mapped_type_names = v.iter().map(|v| v.0.to_string()).collect::<Vec<_>>();

            let type_new_name = create_type_name_substitute(&current_pass_type_name_substitutes, v);

            if let Some((_i, s)) = v.first() {
                let new_struct = create_common_type_struct(s, &type_new_name);

                let mut mapped = BTreeMap::new();
                for mapped_type_name in mapped_type_names {
                    mapped.insert(mapped_type_name, new_struct.ident.to_string().to_owned());
                }

                info!("Mapped types = {:#?}", &mapped);
                if mapped.keys().len() < 2 {
                    None
                } else {
                    Some((mapped, Item::Struct(new_struct)))
                }
            } else {
                None
            }
        })
        .collect();

    let (mapped_types, items): (Vec<BTreeMap<String, String>>, Vec<Item>) =
        items.into_iter().unzip();

    let mut renaming_visitor = StructRenamer {
        changed: false,
        names: mapped_types.into_iter().flatten().collect(),
    };

    if current_pass_substitute_names.is_none() {
        write_type_names_to_file(&renaming_visitor.names)?
    };

    let unparsed_files = prune_replaced_structs(&mut renaming_visitor, visitors);

    recreate_project_files(&out_dir, unparsed_files, items)
}

struct StructVisitor<'ast, 'b> {
    name: String,
    structs: Vec<&'ast ItemStruct>,
    derived_type_names: &'b BTreeSet<String>,
}

struct StructRenamer {
    changed: bool,
    names: BTreeMap<String, String>,
}

fn rewrite_ident(path: &mut PathSegment, names: &BTreeMap<String, String>) -> bool {
    let mut file_changed = false;
    if path.arguments.is_empty() {
        let ident = &path.ident;
        if let Some(new_name) = names.get(&ident.to_string()) {
            path.ident = Ident::new(new_name, Span::call_site());
            file_changed = true;
        }
    } else {
        match path.arguments {
            syn::PathArguments::None => {}
            syn::PathArguments::AngleBracketed(ref mut angle_bracketed_generic_arguments) => {
                for a in angle_bracketed_generic_arguments.args.iter_mut() {
                    if let syn::GenericArgument::Type(Type::Path(path)) = a {
                        for s in path.path.segments.iter_mut() {
                            file_changed |= rewrite_ident(s, names);
                        }
                    }
                }
            }
            syn::PathArguments::Parenthesized(_) => {}
        }
    }
    file_changed
}

fn check_simple_type(
    path: &PathSegment,
    is_simple: &mut bool,
    derived_type_names: &BTreeSet<String>,
) {
    if path.arguments.is_empty() {
        let ident = &path.ident;
        if ident == &Ident::new("String", Span::call_site())
            || ident == &Ident::new("i32", Span::call_site())
            || ident == &Ident::new("i64", Span::call_site())
            || derived_type_names.contains(&ident.to_string())
        {
        } else {
            *is_simple = false;
        }
    } else {
        match &path.arguments {
            syn::PathArguments::None => *is_simple = false,
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                for a in &angle_bracketed_generic_arguments.args {
                    match a {
                        syn::GenericArgument::Type(Type::Path(path)) => {
                            for s in &path.path.segments {
                                check_simple_type(s, is_simple, derived_type_names);
                            }
                        }
                        _ => *is_simple = false,
                    }
                }
            }
            syn::PathArguments::Parenthesized(_) => *is_simple = false,
        }
    }
}

impl<'ast, 'b> Visit<'ast> for StructVisitor<'ast, 'b> {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        debug!("Visiting Struct name == {}", node.ident);
        //debug!("Visiting Struct name == {:#?}", node);
        let mut is_simple_leaf = true;
        node.fields.iter().for_each(|f| match &f.ty {
            Type::Path(path_type) => {
                trace!(
                    "\twith field name = {:?} \n\t\tfield type = {:?}",
                    f.ident, f.ty
                );

                for segment in &path_type.path.segments {
                    check_simple_type(segment, &mut is_simple_leaf, self.derived_type_names);
                }
            }

            _ => {
                is_simple_leaf = false;
            }
        });
        debug!(
            "Visiting Struct name == {} is leaf {is_simple_leaf}",
            node.ident
        );
        if is_simple_leaf {
            self.structs.push(node);
        }
        visit::visit_item_struct(self, node);
    }
}

impl VisitMut for StructRenamer {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        debug!(
            "Visiting and changing fields in struct name == {}",
            node.ident
        );

        node.fields.iter_mut().for_each(|f| {
            let ty = f.ty.clone();
            if let Type::Path(path_type) = &mut f.ty {
                trace!(
                    "\twith field name = {:?} \n\t\tfield type = {:?}",
                    f.ident, ty
                );

                for segment in &mut path_type.path.segments {
                    self.changed |= rewrite_ident(segment, &self.names);
                }
            }
        });

        visit_mut::visit_item_struct_mut(self, node);
    }
}

fn break_into_words(type_name: &str) -> Vec<String> {
    let mut words = vec![];
    let mut current_word = String::new();

    for t in type_name.chars().tuple_windows() {
        let (current, next, next_next) = t;
        if current.is_uppercase() {
            if next.is_uppercase() {
                current_word.push(current);
                if !next_next.is_uppercase() {
                    words.push(current_word);
                    current_word = String::new();
                }
            } else {
                current_word.push(current);
            }
        } else {
            current_word.push(current);
            if next.is_uppercase() {
                words.push(current_word);
                current_word = String::new();
            }
        }
    }
    let len = type_name.len() - 2;
    if len > 0 {
        current_word += &type_name[len..];
        words.push(current_word);
    } else {
        words.push(type_name.to_owned());
    }

    words
}

pub fn common_words(words_sets: &[Vec<String>]) -> Vec<String> {
    let word_sets: Vec<BTreeSet<String>> = words_sets
        .iter()
        .cloned()
        .map(BTreeSet::from_iter)
        .collect();

    let mut intersection = if let Some(first) = word_sets.first() {
        first.clone()
    } else {
        return vec![];
    };

    for word_set in word_sets {
        intersection = intersection.intersection(&word_set).cloned().collect();
    }
    Vec::from_iter(intersection)
}

fn create_type_name_substitute(
    customized_names_from_file: &BTreeMap<String, String>,
    v: &[(Ident, ItemStruct)],
) -> String {
    let words: Vec<Vec<String>> = v
        .iter()
        .map(|v| break_into_words(&v.0.to_string()))
        .collect();

    let common_words = common_words(&words);

    let new_name = common_words.iter().cloned().collect::<String>();

    if let Some(customized_name) = customized_names_from_file.get(&new_name) {
        customized_name.clone()
    } else {
        new_name
    }
}

fn read_type_mappings_from_file(
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

fn read_type_names_from_file(
    mapped_names: &Path,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mapped_names_file = std::fs::File::open(mapped_names)?;
    Ok(io::BufReader::new(mapped_names_file)
        .lines()
        .map_while(Result::ok)
        .collect::<BTreeSet<String>>())
}

fn write_type_names_to_file(
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

fn delete_replaced_structs(file: File, type_names: Vec<String>) -> File {
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
            _ => true,
        })
        .collect();

    File {
        shebang,
        attrs,
        items,
    }
}

struct FindSimilarTypesResult {
    visitors: Vec<(String, File)>,
    similar_items: MultiMap<Fields, (Ident, ItemStruct)>,
}

fn find_similar_types(visitors: Vec<(StructVisitor<'_, '_>, File)>) -> FindSimilarTypesResult {
    let mut similar_items = MultiMap::new();

    let visitors: Vec<_> = visitors
        .into_iter()
        .map(|(mut visitor, file)| {
            visitor.visit_file(&file);
            visitor.structs.into_iter().for_each(|i| {
                similar_items.insert(i.fields.clone(), (i.ident.clone(), (*i).clone()));
            });
            (visitor.name, file)
        })
        .collect();

    FindSimilarTypesResult {
        visitors,
        similar_items,
    }
}

fn prune_replaced_structs(
    renaming_visitor: &mut StructRenamer,
    visitors: Vec<(String, File)>,
) -> Vec<(String, String, bool)> {
    visitors
        .into_iter()
        .map(|(name, mut f)| {
            renaming_visitor.changed = false;
            renaming_visitor.visit_file_mut(&mut f);
            let new_file =
                delete_replaced_structs(f, renaming_visitor.names.keys().cloned().collect());
            (
                name,
                prettyplease::unparse(&new_file),
                renaming_visitor.changed,
            )
        })
        .collect()
}

fn generate_file_preamble(
    changed: bool,
    content: &str,
    output_path: &Path,
    name: &str,
) -> Result<std::fs::File, Box<dyn std::error::Error + Send + Sync>> {
    if changed {
        info!("Writing changed file {name}");
        let mut out_file = std::fs::File::create(output_path.join(name))?;
        if !content.starts_with(GENERATED_PREAMBLE) {
            out_file.write_all(GENERATED_PREAMBLE.as_bytes())?;
        }

        if !content.contains(COMMON_TYPES_USE_PREAMBLE) {
            out_file.write_all(COMMON_TYPES_USE_PREAMBLE.as_bytes())?;
        }
        Ok(out_file)
    } else {
        info!("Writing NOT changed file {name}");
        let mut out_file = std::fs::File::create(output_path.join(name))?;
        if !content.starts_with(GENERATED_PREAMBLE) {
            out_file.write_all(GENERATED_PREAMBLE.as_bytes())?;
        }
        Ok(out_file)
    }
}

fn recreate_project_files(
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

        let mut mod_names = vec!["pub mod common_types;".to_owned()];

        for (name, content, changed) in unparsed_files {
            let mut out_file = generate_file_preamble(changed, &content, output_path, &name)?;
            out_file.write_all(content.as_bytes())?;
            mod_names.push(format!("pub mod {};", &name[..name.len() - 3]));
        }

        for mod_name in mod_names.into_iter().sorted().dedup() {
            mod_file.write_all((mod_name + "\n").as_bytes())?;
        }

        let common_types_file_name = output_path.join("common_types.rs");

        if common_types_file_name.exists() {
            let mut common_out_file = OpenOptions::new()
                .append(true)
                .open(common_types_file_name)?;

            info!("Current position  {}", common_out_file.stream_position()?);

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

fn create_common_type_struct(s: &ItemStruct, type_new_name: &str) -> ItemStruct {
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    apis_dir: String,

    #[arg(long)]
    out_dir: String,

    #[arg(long)]
    current_pass_substitute_names: Option<PathBuf>,

    #[arg(long)]
    previous_pass_derived_type_names: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use crate::break_into_words;

    #[test]
    fn test_word_breaking() {
        let expected_words = [
            "GRPC", "Route", "Rules", "Backend", "Refs", "Filters", "Request", "Mirror", "Backend",
            "Ref",
        ];
        let words = break_into_words("GRPCRouteRulesBackendRefsFiltersRequestMirrorBackendRef");
        assert_eq!(expected_words.to_vec(), words);

        let expected_words = [
            "GRPC", "Route", "Rules", "Backend", "Refs", "Filters", "Request", "HTTPS", "Mirror",
            "Backend", "Ref",
        ];
        let words =
            break_into_words("GRPCRouteRulesBackendRefsFiltersRequestHTTPSMirrorBackendRef");
        assert_eq!(expected_words.to_vec(), words);

        let expected_words = ["f", "RP"];
        let words = break_into_words("fRP");
        assert_eq!(expected_words.to_vec(), words);
    }
}
