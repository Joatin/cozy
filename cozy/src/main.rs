#[macro_use]
extern crate slog;

use sloggers::Build;
use std::error::Error;
use sloggers::terminal::TerminalLoggerBuilder;
use std::process::{Command, Stdio};
use slog::Logger;
use std::io::{BufReader, BufRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let logger = TerminalLoggerBuilder::new().build()?;

    info!(logger, "Starting cozy");

    build_admin_interface(&logger)?;

    Ok(())
}

fn build_admin_interface(logger: &Logger) -> Result<(), anyhow::Error> {
    let mut child = Command::new("yarn")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir("admin_gui")
        .args(&["build"])
        .spawn()?;


    {
        let stdout = child.stdout.as_mut().unwrap();
        let stderr = child.stderr.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        let stdout_lines = stdout_reader.lines();
        let stderr_lines = stderr_reader.lines();

        for line in stdout_lines {
            if let Ok(t) = line {
                info!(logger, "ADMIN_GUI: {}", t);
            }
        }

        for line in stderr_lines {
            if let Ok(t) = line {
                error!(logger, "ADMIN_GUI: {}", t);
            }
        }
    }

    let status = child.wait()?;
    if !status.success() {
        Err(anyhow::anyhow!("Failed to build frontend"))
    } else {
        Ok(())
    }
}