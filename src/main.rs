use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use toml::value::Array;
use toml::Table;
use toml::Value;

#[derive(Parser, Debug)]
#[clap(author = "Mozart409", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,

    /// init a new fleet
    #[arg(short = 'i')]
    init: bool,

    /// Configure your fleet
    #[arg(short = 'r')]
    run: bool,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    if args.init {
        println!("INIT");
        let _ = init();
    }
    if args.run {
        println!("Up your fleet");
        let _ = run();
    }
    if args.name.is_some() {
        println!(
            "Hello {} (from fleetr)!",
            args.name.unwrap_or("world".to_string())
        );
    }
    println!("Read the help with --help.");
}

#[derive(Deserialize, Debug)]
struct FleetConfig {
    servers: FleetServers,
}
#[derive(Deserialize, Debug)]
struct FleetServers {
    ip: String,
    ssh_keys: String,
    user: String,
    role: String,
    pkgs: Array,
}
fn init() -> std::io::Result<()> {
    println!("Creating init file...");
    let init_txt = format!(
        r#"
[servers]
[servers.aubergine]
ip      = '10.0.0.10'
ssh_key = '~/.ssh/key'
user    = 'root'
role    = 'cntrl'
pkgs = ['nala', 'curl', 'wget', 'net-tools']
[servers.rhubarb]
ip      = '10.0.0.11'
ssh_key = '~/.ssh/key'
user    = 'root'
role    = 'agent'
pkgs = ['nala', 'curl', 'wget']
        "#
    );
    let mut file = match File::create_new("fleet.toml") {
        Err(err) => panic!("Could not create file {}", err),
        Ok(file) => file,
    };
    match file.write_all(init_txt.as_bytes()) {
        Err(err) => panic!("Could not write file {}", err),
        Ok(_) => println!("Wrote fleet.toml"),
    }
    Ok(())
}

fn run() -> std::io::Result<()> {
    println!("Configuring your fleet...");

    let mut file = match File::open("fleet.toml") {
        Err(err) => panic!("Could not open file. Create it with fleetr --init {}", err),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read into buffer");

    println!("{}", content);

    let data: FleetConfig = toml::from_str(&content).expect("Invalid toml format");

    println!("{:?}", data);

    Ok(())
}
