use std::process::Command;
use std::error::Error;

fn main() -> Result<(), anyhow::Error> {
    install_node_dependencies()?;

    Ok(())
}

fn build_package(dir: &str) -> Result<(), anyhow::Error> {
    let output = Command::new("yarn").current_dir(dir).args(&["build"]).output()?;
    println!("{:?}", output);
    if !output.status.success() {
        Err(anyhow::anyhow!("Failed to build frontend"))
    } else {
        Ok(())
    }
}

fn install_node_dependencies() -> Result<(), anyhow::Error> {
    let output = Command::new("yarn").args(&["install", "--frozen-lockfile"]).output()?;
    println!("{:?}", output);
    if !output.status.success() {
        Err(anyhow::anyhow!("Failed to install node dependencies"))
    } else {
        Ok(())
    }
}