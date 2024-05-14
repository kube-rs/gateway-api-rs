use std::{collections::BTreeMap, env};

use codegen::{Enum, Function, Scope, Variant};

fn main() {
    let task = env::args().nth(1);

    match task.as_deref() {
        Some("gen_enum_defaults") => gen_enum_defaults().unwrap(),
        Some("gen_condition_constants") => gen_condition_constants().unwrap(),
        _ => print_help(),
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

gen_enum_defaults generates Default trait impls for enums
gen_constants generates constants used for Conditions
"
    )
}

type DynError = Box<dyn std::error::Error>;

fn gen_condition_constants() -> Result<(), DynError> {
    let gateway_condition_types = env::var("GATEWAY_CONDITION_CONSTANTS")?;
    let gateway_reason_types = env::var("GATEWAY_REASON_CONSTANTS")?;
    let listener_condition_types = env::var("LISTENER_CONDITION_CONSTANTS")?;
    let listener_reason_types = env::var("LISTENER_REASON_CONSTANTS")?;

    let mut scope = Scope::new();
    gen_const_enums(&mut scope, gateway_condition_types);
    gen_const_enums(&mut scope, gateway_reason_types);
    gen_const_enums(&mut scope, listener_condition_types);
    gen_const_enums(&mut scope, listener_reason_types);
    println!("{}", gen_generated_file_warning());
    println!("{}", scope.to_string());
    Ok(())
}

fn gen_const_enums(scope: &mut Scope, constants: String) {
    let enum_type_and_variants: Vec<&str> = constants.split('=').collect();
    let enum_type = enum_type_and_variants[0];
    let variants: Vec<&str> = enum_type_and_variants[1].split(',').collect();

    let mut enumeration = Enum::new(enum_type);
    enumeration.derive("Debug");
    enumeration.vis("pub");

    for variant in variants {
        let var = Variant::new(variant);
        enumeration.push_variant(var);
    }
    scope.push_enum(enumeration);

    gen_display_impl(scope, enum_type);
}

fn gen_display_impl(scope: &mut Scope, ty: &str) {
    let mut func = Function::new("fmt".to_string());
    func.arg_ref_self();
    func.arg("f", "&mut std::fmt::Formatter");
    func.ret("std::fmt::Result");
    func.line("write!(f, \"{:?}\", self)");
    scope
        .new_impl(ty)
        .impl_trait("std::fmt::Display")
        .push_fn(func);
}

fn gen_enum_defaults() -> Result<(), DynError> {
    // GATEWAY_API_ENUMS provides the enum names along with their default variant to be used in the
    // generated Default impl. For eg: GATEWAY_API_ENUMS=enum1=default1,enum2=default2.
    let gw_api_enums = env::var("GATEWAY_API_ENUMS")?;
    let enums_with_defaults = get_enums_with_defaults_map(gw_api_enums);

    let mut scope = Scope::new();
    let mut httproute_enums = vec![];
    let mut grpcroute_enums = vec![];

    for (e, d) in enums_with_defaults {
        // The `fn default()` function.
        let mut func = Function::new("default".to_string());
        func.ret("Self").line(format!("{}::{}", e, d));

        // The impl Default for <enum> implementation.
        scope
            .new_impl(e.as_str())
            .impl_trait("Default")
            .push_fn(func);

        // Determine which enums belong to the httproute module and which belong to the
        // grpcroute module.
        if e.starts_with("HTTPRoute") {
            httproute_enums.push(e);
        } else if e.starts_with("GRPCRoute") {
            grpcroute_enums.push(e);
        }
    }

    println!("{}", gen_generated_file_warning());

    // Generate use statements for the enums.
    if httproute_enums.len() > 0 {
        let use_http_stmt = gen_use_stmt(httproute_enums, "httproutes".to_string());
        println!("{}\n", use_http_stmt);
    }
    if grpcroute_enums.len() > 0 {
        let use_grpc_stmt = gen_use_stmt(grpcroute_enums, "grpcroutes".to_string());
        println!("{}\n", use_grpc_stmt);
    }

    println!("{}", scope.to_string());
    Ok(())
}

fn gen_generated_file_warning() -> String {
    "// WARNING: generated file - manual changes will be overriden\n".into()
}

fn gen_use_stmt(items: Vec<String>, module: String) -> String {
    let mut stmt = String::from(format!("use super::{}::{{", module));
    for item in items {
        stmt.push_str(format!("{}, ", item).as_str());
    }
    stmt.push_str("};");
    stmt
}

fn get_enums_with_defaults_map(env_var_val: String) -> BTreeMap<String, String> {
    let mut enums_with_defaults = BTreeMap::new();
    env_var_val.split(',').for_each(|enum_with_default| {
        let enum_and_default: Vec<&str> = enum_with_default.split('=').collect();
        enums_with_defaults.insert(
            enum_and_default[0].to_string(),
            enum_and_default[1].to_string(),
        );
    });

    enums_with_defaults
}
