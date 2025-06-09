use log::debug;
use log::info;
use log::trace;
use proc_macro2::{Ident, Span};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use syn::Fields;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::PathSegment;
use syn::Type;
use syn::visit;
use syn::visit::Visit;
use syn::visit_mut;

use syn::visit_mut::VisitMut;

pub struct StructVisitor<'ast, 'b> {
    pub name: String,
    pub structs: Vec<&'ast ItemStruct>,
    pub derived_type_names: &'b BTreeSet<String>,
}

pub struct StructEnumVisitor<'ast, 'b> {
    pub name: String,
    pub structs: Vec<&'ast ItemStruct>,
    pub enums: Vec<&'ast ItemEnum>,
    pub derived_type_names: &'b BTreeSet<String>,
}

pub struct StructRenamer {
    pub changed: bool,
    pub names: BTreeMap<String, String>,
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

impl<'ast, 'b> Visit<'ast> for StructEnumVisitor<'ast, 'b> {
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

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        info!("Visiting Enum name == {} {:?}", node.ident, node.variants);

        if node
            .variants
            .iter()
            .map(|f| &f.fields)
            .all(|f| *f == Fields::Unit)
        {
            self.enums.push(node);
        }
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
