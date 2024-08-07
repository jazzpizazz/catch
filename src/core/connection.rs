use crate::core::commands::{get_command, get_command_names, Command};
use crate::core::markers::{END_MARKER, START_MARKER};
use crate::core::setup::{disable_history, tty_upgrade};
use crate::core::terminal::{reset_terminal, set_raw_mode, show_shortcut_menu};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const CTRL_UNDERSCORE: u8 = 0x1F;
const CTRL_Y: u8 = 0x19;

pub fn send_raw(stream: &mut TcpStream, data: &str) {
    stream.write_all(data.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
}

pub fn send_command(stream: &mut TcpStream, command: &str) {
    let start_marker = START_MARKER.lock().unwrap();
    let end_marker = END_MARKER.lock().unwrap();
    let command = format!(
        "echo -n {};{}; echo -n {}",
        *start_marker, command, *end_marker
    );
    stream.write_all(command.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
}

pub fn recv_until_marker(stream: &mut TcpStream) -> String {
    let start_marker = START_MARKER.lock().unwrap().clone();
    let end_marker = END_MARKER.lock().unwrap().clone();
    let mut buffer = Vec::new();
    let mut data = [0; 1024];

    while let Ok(size) = stream.read(&mut data) {
        if size == 0 {
            break;
        }
        buffer.extend_from_slice(&data[..size]);

        if buffer
            .windows(end_marker.len())
            .any(|window| window == end_marker.as_bytes())
        {
            break;
        }
    }

    let buffer_str = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };

    buffer_str
        .split(&start_marker)
        .nth(1)
        .and_then(|after_start| after_start.split(&end_marker).next())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

pub fn handle_connection(mut stream: TcpStream, commands: &[Command]) {
    send_raw(&mut stream, "stty -echo\n");
    tty_upgrade(&mut stream);
    disable_history(&mut stream);

    set_raw_mode();

    let commands = commands.to_vec();
    let mut stdout = io::stdout();
    let mut network_buffer = [0; 1024];

    stream
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let (tx_command, rx_command) = mpsc::channel::<Vec<u8>>();
    let (tx_response, _rx_response) = mpsc::channel::<Vec<u8>>();
    let handle = thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0; 1024];
        let commands = commands.clone();
        loop {
            match stdin.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    if buffer[..size] == [CTRL_UNDERSCORE] {
                        print!("\n\r[!] Ctrl+_ detected, exiting catch!\n\r");
                        break;
                    }
                    if buffer[..size] != [CTRL_Y] {
                        tx_command.send(buffer[..size].to_vec()).unwrap();
                    } else {
                        if let Some(selection) =
                            show_shortcut_menu(&get_command_names(&commands)).unwrap()
                        {
                            let command = get_command(&commands, selection.as_str()).unwrap();
                            tx_command.send(command.as_bytes().to_vec()).unwrap();
                        }
                    }
                }
                _ => continue,
            }
        }
    });

    loop {
        match stream.read(&mut network_buffer) {
            Ok(size) if size > 0 => {
                tx_response
                    .send(network_buffer[..size].to_vec())
                    .expect("Failed to send response");
                stdout.write_all(&network_buffer[..size]).unwrap();
                stdout.flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => break,
        }

        match rx_command.try_recv() {
            Ok(data) => {
                stream.write_all(&data).unwrap();
                stream.flush().unwrap();
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    reset_terminal();
}
