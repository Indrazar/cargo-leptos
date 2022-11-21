use crate::{
    config::Config,
    logger::GRAY,
    util::{run_interruptible, CommandAdditions},
};
use anyhow::{Context, Result};
use tokio::process::Command;

// for capturing the cargo output see util::CommandAdditions

pub async fn build(config: &Config, lib: bool) -> Result<()> {
    let args = args("build", config, lib);

    let (handle, process) = Command::new("cargo")
        .args(&args)
        .spawn_cargo_parsed()
        .context("Could not spawn command")?;
    run_interruptible("Cargo", process)
        .await
        .context(format!("cargo {}", &args.join(" ")))?;
    handle.await?;
    log::info!(
        "Cargo finished {}",
        GRAY.paint(format!("cargo {}", args.join(" ")))
    );
    Ok(())
}

pub async fn run(config: &Config) -> Result<()> {
    cmd("run", config, false).await
}

pub async fn test(config: &Config) -> Result<()> {
    cmd("test", config, false).await
}

async fn cmd(command: &str, config: &Config, lib: bool) -> Result<()> {
    let args = args(command, config, lib);

    let process = Command::new("cargo")
        .args(&args)
        .spawn()
        .context("Could not spawn command")?;
    run_interruptible("Cargo", process)
        .await
        .context(format!("cargo {}", &args.join(" ")))?;
    log::info!(
        "Cargo finished {}",
        GRAY.paint(format!("cargo {}", args.join(" ")))
    );
    Ok(())
}

fn args<'a>(command: &'a str, config: &Config, lib: bool) -> Vec<&'a str> {
    let features = match (lib, config.cli.csr, config.watch) {
        (false, _, true) => "--features=ssr,leptos_autoreload",
        (false, _, false) => "--features=ssr",
        (true, false, true) => "--features=hydrate,leptos_autoreload",
        (true, false, false) => "--features=hydrate",
        (true, true, true) => "--features=csr,leptos_autoreload",
        (true, true, false) => "--features=csr",
    };
    let mut args = vec![command, "--no-default-features", features];

    if lib {
        args.push("--lib");
        args.push("--target=wasm32-unknown-unknown");
    }

    config.cli.release.then(|| args.push("--release"));
    args
}