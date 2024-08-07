# Catch
Rust-based linux reverse shell listener.

## Installation

Will be improved but something like

```bash
mkdir /opt/catch/
cd /opt/catch/
git clone https://github.com/jazzpizazz/catch.git
cargo build -r
```
## Usage
Wihtout any arguments `catch` will start listening on `0.0.0.0:8443`, you can however specify the IP and port you want to listen on:
```bash
$ target/release/catch -h
Rust-based reverse shell listener offering enhanced operational features.

Usage: catch [OPTIONS]

Options:
  -i, --ip <IP>      [default: 0.0.0.0]
  -p, --port <PORT>  [default: 8443]
  -h, --help         Print help
  -V, --version      Print version

```
`catch` does not work with sessions, if you want to get multiple shells, just start multiple listeners.

## Usage
When running `catch` it will start listening for an incoming shell. Once a connection is made, `catch` will attempt to upgrade to a fully interactive shell using either `python3`, `python3` or `script`. After that it will unset the histfile, set the terminal to raw mode and drop into a fully interactive shell:
```bash
$ target/release/catch   
[i] Listening on 0.0.0.0:8443...
[+] Connected by 127.0.0.1:35262
[i] Attempting TTY upgrade...
[+] Binary 'python3' is installed, upgrading shell...
[i] Unsetting histfile...
┌──(kali㉿kali)-[~]
└─$    
```
> **Note**: Situations where the tty upgrade fails are not yet handled correctly

## Shortcuts
When in a shell you can press the magic **ctrl+y** key combination to open up a special menu:
![shortcuts](https://github.com/user-attachments/assets/f3be2960-fee4-4f6c-a961-6b521c6cba99)
In here you can search for a "shortcut" either by arrow keys or by typing (notice the search query in the bottom left) after you hit **enter** to directly send it to the shell!
### Adding your own shortcuts
Currently the commands are stored in `/opt/catch/commands.json` and you can add your own commands to the JSON file which is being loaded on startup. I'm considering moving this list to its own repository so additional commands can be added through PR's. 

## Exiting
To exit a connected reverse shell hit **ctr+_** this will attempt to revert your terminal to its default settings.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change. Please keep in mind that `catch` is still in development.

