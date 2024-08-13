use std::env;
use gateway_api::Duration;
use std::str::FromStr;

/// Simple example of using the gateway_api::Duration: just parse the duration
/// string given on the command line, then print it back out (which formats it).
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

    // Parse the duration input using gateway_api::Duration
    match Duration::from_str(&args[1]) {
        Ok(duration) => {
            println!("Parsed duration: {}", duration);
        }
        Err(err) => {
            println!("Failed to parse duration: {}", err);
        }
    }
}