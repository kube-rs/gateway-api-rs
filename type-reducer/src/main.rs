use clap::Parser;
use clap::Subcommand;
use itertools::Itertools;
use log::debug;
use log::info;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use syn::Item;
use syn::visit_mut::VisitMut;
use type_reducer::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Action {
    Reduce(ReduceArgs),
    Rename(RenameArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parser)]
struct ReduceArgs {
    #[arg(long)]
    current_pass_substitute_names: PathBuf,

    #[arg(long)]
    previous_pass_derived_type_names: PathBuf,

    #[arg(long)]
    ignorable_type_names: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Parser)]
struct RenameArgs {
    #[arg(long)]
    rename_only_substitute_names: PathBuf,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
    #[arg(long)]
    apis_dir: String,

    #[arg(long)]
    out_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_logger::init_with_env().unwrap();
    let Args {
        action,
        apis_dir,
        out_dir,
    } = Args::parse();

    let Ok(_) = fs::exists(out_dir.clone()) else {
        return Err("our dir doesn't exist".into());
    };

    match action {
        Action::Rename(args) => {
            let RenameArgs {
                rename_only_substitute_names,
            } = args;
            let rename_only_substitute_names =
                read_type_mappings_from_file(rename_only_substitute_names.as_path())?;

            let previous_pass_derived_type_names = BTreeSet::new();

            let visitors = create_visitors(&apis_dir, &previous_pass_derived_type_names)?;
            handle_rename_types(rename_only_substitute_names, visitors, &out_dir)
        }

        Action::Reduce(args) => {
            let ReduceArgs {
                current_pass_substitute_names,
                previous_pass_derived_type_names,
                ignorable_type_names,
            } = args;
            let previous_pass_derived_type_names =
                read_type_names_from_file(previous_pass_derived_type_names.as_path())?;

            let current_pass_type_name_substitutes =
                read_type_mappings_from_file(current_pass_substitute_names.as_path())?;

            let ignorable_type_names =
                if let Some(ignorable_type_names) = ignorable_type_names.as_ref() {
                    read_type_mappings_from_file(ignorable_type_names.as_path())?
                } else {
                    BTreeMap::new()
                };

            let visitors = create_visitors(&apis_dir, &previous_pass_derived_type_names)?;
            handle_reduce_types(
                current_pass_type_name_substitutes,
                visitors,
                &out_dir,
                ignorable_type_names,
            )
        }
    }
}

fn handle_reduce_types(
    current_pass_type_name_substitutes: BTreeMap<String, String>,
    visitors: Vec<(StructEnumVisitor<'_, '_>, syn::File)>,
    out_dir: &str,
    ignorable_types: BTreeMap<String, String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

            let mut ignore = false;
            if !ignorable_types.is_empty() {
                for mapped_type in &mapped_type_names {
                    if ignorable_types.contains_key(mapped_type) {
                        debug!("Ignoring type {mapped_type}");
                        ignore = true;
                    }
                }
            }
            if ignore {
                return None;
            }

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

    let mut renaming_visitor = StructEnumFieldsRenamer {
        changed: false,
        names: mapped_types.into_iter().flatten().collect(),
    };

    write_type_names_to_file(&renaming_visitor.names)?;

    let unparsed_files = prune_replaced_structs(&mut renaming_visitor, visitors);

    recreate_project_files(
        out_dir,
        unparsed_files,
        items.into_iter().sorted_by(order_types).collect(),
    )
}

fn handle_rename_types(
    rename_only_substitute_names: BTreeMap<String, String>,
    visitors: Vec<(StructEnumVisitor<'_, '_>, syn::File)>,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !rename_only_substitute_names.is_empty() {
        let mut renaming_visitor = StructEnumNameRenamer {
            changed: false,
            names: rename_only_substitute_names,
        };

        write_type_names_to_file(&renaming_visitor.names)?;

        let files: Vec<_> = visitors
            .into_iter()
            .map(|(visitor, mut f)| {
                renaming_visitor.changed = false;
                renaming_visitor.visit_file_mut(&mut f);

                (renaming_visitor.changed, visitor, f)
            })
            .collect();
        for (changed, visitor, file) in files {
            let changed = if visitor.name == COMMON_TYPES_MOD_NAME.to_owned() + ".rs" {
                false
            } else {
                changed
            };
            let path = PathBuf::from(&visitor.name);
            info!("Renaming types in files {}", path.display());
            let content = &prettyplease::unparse(&file);
            let mut file = generate_file_preamble(
                changed,
                content,
                std::path::Path::new(&out_dir),
                &visitor.name,
            )?;
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn create_visitors<'a>(
    apis_dir: &'a str,
    previous_pass_derived_type_names: &'a BTreeSet<String>,
) -> Result<Vec<(StructEnumVisitor<'a, 'a>, syn::File)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut visitors = vec![];

    for dir_entry in fs::read_dir(apis_dir)? {
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
                            derived_type_names: previous_pass_derived_type_names,
                        };
                        visitors.push((visitor, syntaxt_file));
                    }
                }
            }
        }
    }
    Ok(visitors)
}

fn order_types(this: &Item, other: &Item) -> Ordering {
    match (this, other) {
        (Item::Enum(this), Item::Enum(other)) => this.ident.cmp(&other.ident),
        (Item::Struct(this), Item::Struct(other)) => this.ident.cmp(&other.ident),
        _ => {
            let this_discriminant = unsafe { *(this as *const Item as *const u8) };
            let other_discriminant = unsafe { *(other as *const Item as *const u8) };
            this_discriminant.cmp(&other_discriminant)
        }
    }
}
