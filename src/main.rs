mod commands;
mod connection;
mod markers;
mod setup;
mod terminal;

use clap::Parser;
use commands::{
    ensure_directory_exists, read_commands, write_initial_commands, COMMANDS_PATH,
};
use connection::handle_connection;
use std::net::TcpListener;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[clap(short, long, default_value = "0.0.0.0")]
    ip: String,

    #[clap(short, long, default_value = "8443")]
    port: String,
}

fn main() {
    let args = Args::parse();
    let address = format!("{}:{}", args.ip, args.port);

    ensure_directory_exists("/opt/catch/").unwrap();

    if !Path::new(COMMANDS_PATH).exists() {
        write_initial_commands(COMMANDS_PATH).expect("[-] Failed to create initial commands.json");
    }

    let commands = read_commands(COMMANDS_PATH).expect("[-] Could not read commands.json");

    let listener = TcpListener::bind(&address);

    let listener = match listener {
        Ok(l) => l,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AddrInUse => {
                    eprintln!("[-] The address {} is already in use.", address);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!(
                        "[-] Permission denied while trying to bind to {}. Maybe use sudo?",
                        address
                    );
                }
                _ => {
                    eprintln!("[-] Failed to bind to {}: {}", address, e);
                }
            }
            process::exit(1);
        }
    };

    println!("[i] Listening on {}...", address);

    match listener.accept() {
        Ok((stream, addr)) => {
            println!("[+] Connected by {}", addr);
            handle_connection(stream, &commands);
        }
        Err(e) => println!("[-] Connection failed: {}", e),
    }
}
