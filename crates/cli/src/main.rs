use clap::Parser;
use std::{error::Error, process};
use tabled::{builder::Builder, Style};

mod builder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The hostname to check.
    host: String,

    /// CHECKTYPE <ping | http | tcp | dns | udp>
    #[arg(short, long, default_value = "ping")]
    _type: String,

    /// Maximum number of nodes used for the check. 0 = max
    #[arg(short, long, default_value_t = 0)]
    nodes: u8,

    /// Wait for seconds before checking the result.
    #[arg(short, long, default_value_t = 0)]
    wait: u8,
}

const DEF_TYPES: [&str; 5] = ["ping", "http", "tcp", "dns", "udp"];

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !DEF_TYPES.contains(&args._type.as_str()) {
        eprintln!("Unknown CHECKTYP, use `--help` to view allowed params");
        process::exit(1);
    }

    let hostname: &str = &args.host;
    let nodes_str: &str = &args.nodes.to_string();
    let check_type: &str = &args._type;

    let mut result: Builder = Builder::default();

    match check_type {
        "http" => {
            result = builder::check_http(hostname, args.nodes, args.wait)?;
        }

        "ping" => {
            result = builder::check_ping(hostname, args.nodes, args.wait)?;
        }

        "tcp" => {
            result = builder::check_tcp(hostname, args.nodes, args.wait)?;
        }

        _ => {}
    }

    let mut table = result.build();
    table.with(Style::modern());

    println!("Check-{}", check_type);
    println!("Hostname: {}", hostname);
    println!(
        "Max Nodes: {}",
        if args.nodes == 0 { "max" } else { nodes_str }
    );
    println!("{}", table);

    Ok(())
}
