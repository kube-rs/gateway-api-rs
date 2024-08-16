use gateway_api::Duration;
use std::env;
use std::str::FromStr;

/// Simple example of using the gateway_api::Duration: just parse the duration
/// string given on the command line, then print it back out (which formats it).
///
/// See the format specification here: https://gateway-api.sigs.k8s.io/geps/gep-2257/
///
/// Good things to try:
/// cargo run --example gep2257 1h (should print "Parsed duration: 1h")
/// cargo run --example gep2257 1h30m (should print "Parsed duration: 1h30m")
/// cargo run --example gep2257 30m1h10s5s (should print "Parsed duration: 1h30m15s")
fn main() {
    // Get the command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a duration input");
        return;
    }

    let value = &args[1];

    // Parse the duration input using gateway_api::Duration
    match Duration::from_str(value) {
        Ok(duration) => {
            println!("Parsed duration: {}", duration);
        }
        Err(error) => {
            eprintln!(
                "Failed to parse duration from: {}\nError: {:#?}",
                value, error
            );
            std::process::exit(1);
        }
    }
}
