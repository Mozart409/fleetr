use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use toml::value::Array;

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

    /// Force overwrite of the fleet.toml file
    #[arg(short = 'f', long = "force")]
    force: bool,

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
        let _ = init(args.force);
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
    servers: HashMap<String, FleetServers>,
}
#[derive(Deserialize, Debug)]
struct FleetServers {
    ip: String,
    ssh_key: String,
    user: String,
    role: String,
    pkgs: Array,
}
fn init(force: bool) -> std::io::Result<()> {
    let file_path = "fleet.toml";
    if Path::new(file_path).exists() && !force {
        println!(
            "File {} already exists. Use --force to overwrite.",
            file_path
        );
        return Ok(());
    }

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

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(init_txt.as_bytes())?;
    println!("Wrote fleet.toml");

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

    for (name, server) in data.servers {
        println!("Server: {}", name);
        println!("  IP: {}", server.ip);
        println!("  SSH Key: {}", server.ssh_key);
        println!("  User: {}", server.user);
        println!("  Role: {}", server.role);
        println!("  Packages: {:?}", server.pkgs);
    }

    Ok(())
}
