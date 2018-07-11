extern crate dyndns;

use dyndns::*;

fn main() -> Result<(), String> {
    let token = std::env::var("DO_TOKEN")
        .expect("please set DO_TOKEN env var");

    // TODO: let's not hardcode this
    let ip = DynDns::new(token)?.set_domain("pi.dylanh.art")?;
    println!("set ip to {}", ip);

    Ok(())
}
