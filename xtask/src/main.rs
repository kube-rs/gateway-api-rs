use std::{collections::BTreeMap, env};

use codegen::{Function, Scope};

fn main() {
    let task = env::args().nth(1);

    match task.as_deref() {
        Some("gen_enum_defaults") => gen_enum_defaults().unwrap(),
        _ => print_help(),
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

gen_enum_defaults generates Default trait impls for enums
"
    )
}

type DynError = Box<dyn std::error::Error>;

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

    println!("// WARNING: generated file - manual changes will be overriden\n");

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
