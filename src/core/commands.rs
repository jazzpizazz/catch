use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    name: String,
    command: String,
}

static DEFAULT_COMMAND_NAMES: &[&str] = &["Find SUID binaries", "Find files with capabilities"];

static DEFAULT_COMMANDS: &[&str] = &[
    "find / -perm -u=s -type f -exec ls -la {} \\; 2>/dev/null\n",
    "getcap -r / 2>/dev/null\n",
];

fn default_commands() -> Vec<Command> {
    DEFAULT_COMMAND_NAMES
        .iter()
        .zip(DEFAULT_COMMANDS.iter())
        .map(|(&name, &command)| Command {
            name: name.to_string(),
            command: command.to_string(),
        })
        .collect()
}

pub const COMMANDS_PATH: &str = "/opt/catch/commands.json";

pub fn ensure_directory_exists(path_str: &str) -> io::Result<()> {
    let path = Path::new(path_str);
    if !path.exists() {
        println!("[-] {} does not exist", path_str);
        exit(1)
    }
    Ok(())
}

pub fn write_initial_commands(path: &str) -> io::Result<()> {
    let commands = default_commands();
    let json_data = serde_json::to_string_pretty(&commands)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn read_commands(path: &str) -> io::Result<Vec<Command>> {
    let data = fs::read_to_string(path)?;
    let commands: Vec<Command> = serde_json::from_str(&data)?;
    Ok(commands)
}

pub fn get_command_names(commands: &[Command]) -> Vec<String> {
    commands.iter().map(|cmd| cmd.name.clone()).collect()
}

pub fn get_command<'a>(commands: &'a [Command], name: &str) -> Option<&'a str> {
    commands
        .iter()
        .find(|cmd| cmd.name == name)
        .map(|cmd| cmd.command.as_str())
}
