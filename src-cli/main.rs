use clap::{Parser, Subcommand};
use std::io::prelude::Write;
use std::os::unix::net::UnixStream;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Enable,
    Disable,
}

fn main() {
    let cli = Cli::parse();

    let socket_path = "/tmp/numlock-cli-socket";
    let mut stream = UnixStream::connect(socket_path).unwrap();

    let message = match &cli.command {
        Command::Enable => "1",
        Command::Disable => "0",
    };

    stream.write_all(message.as_bytes()).unwrap();
}
