use std::path::PathBuf;
use clap::{Parser, Subcommand};
use std::process::Command;
use std::sync::mpsc;
use std::{io, thread};

#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: XtaskCommand,
}

#[derive(Subcommand, Clone)]
pub enum XtaskCommand {
    Run {
        #[clap(short, long, default_value_t = false)]
        release: bool,
    },
    RunGame {
        #[clap(short, long, default_value_t = false)]
        release: bool,
    },
    KeyGen {
        #[clap(short, long, default_value = "assets/tls/cert.pem")]
        cert: PathBuf,
        #[clap(short, long, default_value = "assets/tls/key.pem")]
        key: PathBuf,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { cmd } = Cli::parse();
    match cmd {
        XtaskCommand::Run { release } => run(release),
        XtaskCommand::RunGame { release } => run_game(release),
        XtaskCommand::KeyGen { cert, key } => key_gen(cert, key),
    }
}

fn run(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let release = format!("{}", if release { "--release" } else { "" });

    let (tx, rx) = mpsc::channel();

    let sdk_tx = tx.clone();
    let sdk_release = release.clone();
    let sdk_handle = thread::spawn(move || {
        let mut sdk = Command::new("cargo")
            .args(["run", "--bin", "sdk-server", sdk_release.as_str()])
            .spawn()?;

        sdk.wait()?;

        sdk_tx.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    let dispatch_tx = tx.clone();
    let dispatch_release = release.clone();
    let dispatch_handle = thread::spawn(move || {
        let mut dispatch = Command::new("cargo")
            .args(["run", "--bin", "dispatch-server", dispatch_release.as_str()])
            .spawn()?;

        dispatch.wait()?;

        dispatch_tx.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    let gate_tx = tx.clone();
    let gate_release = release.clone();
    let gate_handle = thread::spawn(move || {
        let mut gate = Command::new("cargo")
            .args(["run", "--bin", "gate-server", gate_release.as_str()])
            .spawn()?;

        gate.wait()?;

        gate_tx.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    rx.recv().expect("failed to receive from channel");

    sdk_handle.join().expect("failed to join sdk-server thread")?;
    dispatch_handle.join().expect("failed to join sdk-server thread")?;
    gate_handle.join().expect("failed to join sdk-server thread")?;

    Ok(())
}

fn run_game(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let release = format!("{}", if release { "--release" } else { "" });

    let (tx, rx) = mpsc::channel();
    let game_handle = thread::spawn(move || {
        let mut game = Command::new("cargo")
            .args(["run", "--bin", "game-server", release.as_str()])
            .spawn()?;

        game.wait()?;

        tx.send(()).expect("failed to send completion signal");

        Ok::<(), io::Error>(())
    });

    rx.recv().expect("failed to receive from channel");

    game_handle.join().expect("failed to join sdk-server thread")?;

    Ok(())
}

fn key_gen(cert: PathBuf, key: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("openssl");
    cmd
        .args(["req", "-x509", "-nodes", "-days", "365", "-newkey", "rsa:2048"])
        .args(["-keyout", key.to_str().unwrap()])
        .args(["-out", cert.to_str().unwrap()]);

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
