use clap::Parser;
use log::info;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use syn::Item;
use type_reducer::*;

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
                        let visitor = StructEnumVisitor {
                            name,
                            structs: Vec::new(),
                            enums: Vec::new(),
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
        similar_structs,
        similar_enums,
    } = find_similar_types(visitors);

    let struct_items: Vec<_> = similar_structs
        .iter_all()
        .filter(|(_k, v)| v.len() > 1)
        .filter_map(|(_k, v)| {
            info!(
                "Potentially similar structs {:#?}",
                v.iter().map(|(i, _)| i.to_string()).collect::<Vec<_>>()
            );
            let mapped_type_names = v.iter().map(|v| v.0.to_string()).collect::<Vec<_>>();

            // let type_new_name =
            //     create_struct_type_name_substitute(&current_pass_type_name_substitutes, v);

            if let Some((i, s)) = v.first() {
                let new_struct = create_common_type_struct(
                    s,
                    &read_substitute(&current_pass_type_name_substitutes, i),
                );

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

    let enum_items: Vec<_> = similar_enums
        .iter_all()
        .filter(|(_k, v)| v.len() > 1)
        .filter_map(|(_k, v)| {
            info!(
                "Potentially similar enums {:#?}",
                v.iter().map(|(i, _)| i.to_string()).collect::<Vec<_>>()
            );
            let mapped_type_names = v.iter().map(|v| v.0.to_string()).collect::<Vec<_>>();

            // let type_new_name =
            //     create_enum_type_name_substitute(&current_pass_type_name_substitutes, v);

            if let Some((i, s)) = v.first() {
                let new_enum = create_common_type_enum(
                    s,
                    &read_substitute(&current_pass_type_name_substitutes, i),
                );

                let mut mapped = BTreeMap::new();
                for mapped_type_name in mapped_type_names {
                    mapped.insert(mapped_type_name, new_enum.ident.to_string().to_owned());
                }

                info!("Mapped types = {:#?}", &mapped);
                if mapped.keys().len() < 2 {
                    None
                } else {
                    Some((mapped, Item::Enum(new_enum)))
                }
            } else {
                None
            }
        })
        .collect();

    let (mapped_types, items): (Vec<BTreeMap<String, String>>, Vec<Item>) =
        struct_items.into_iter().chain(enum_items).unzip();

    let mut renaming_visitor = StructEnumRenamer {
        changed: false,
        names: mapped_types.into_iter().flatten().collect(),
    };

    write_type_names_to_file(&renaming_visitor.names)?;

    let unparsed_files = prune_replaced_structs(&mut renaming_visitor, visitors);

    recreate_project_files(&out_dir, unparsed_files, items)
}
