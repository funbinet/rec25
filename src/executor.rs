//! Command executor — builds, validates, and runs external commands.

use anyhow::{bail, Result};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Result of a command execution.
#[derive(Debug)]
pub struct ExecResult {
    pub stdout:    String,
    pub stderr:    String,
    pub exit_code: i32,
    pub duration:  Duration,
}

/// Replace all `{placeholder}` tokens in a command template.
///
/// `inputs`  — map from placeholder name to user-supplied value.
/// `out`     — value to substitute for `{output_file}`.
/// `ts`      — value to substitute for `{timestamp}`.
pub fn build_command(
    template:  &str,
    inputs:    &HashMap<String, String>,
    out:       &str,
    ts:        &str,
) -> String {
    let mut cmd = template.to_string();
    for (key, val) in inputs {
        cmd = cmd.replace(&format!("{{{key}}}"), val);
    }
    cmd = cmd.replace("{output_file}", out);
    cmd = cmd.replace("{timestamp}", ts);
    cmd
}

/// Check whether `binary` is on PATH using `which`.
pub fn check_binary(binary: &str) -> bool {
    std::process::Command::new("which")
        .arg(binary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run `cmd` through `sh -c`, enforcing `timeout_secs`.
///
/// The command is executed on a worker thread. If the timeout fires the
/// thread is abandoned (the child process continues briefly) and an error
/// is returned — acceptable for a v1 sequential tool.
pub fn run_command(cmd: &str, timeout_secs: u64) -> Result<ExecResult> {
    let start = Instant::now();
    let cmd_owned = cmd.to_string();
    let (tx, rx) = mpsc::channel::<Result<ExecResult>>();

    thread::spawn(move || {
        let result = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd_owned)
            .output();

        let res = match result {
            Ok(out) => Ok(ExecResult {
                stdout:    String::from_utf8_lossy(&out.stdout).into_owned(),
                stderr:    String::from_utf8_lossy(&out.stderr).into_owned(),
                exit_code: out.status.code().unwrap_or(-1),
                duration:  start.elapsed(),
            }),
            Err(e) => Err(anyhow::anyhow!("Failed to spawn command: {e}")),
        };
        let _ = tx.send(res);
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(res) => res,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            bail!("Command timed out after {timeout_secs}s")
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            bail!("Command thread disconnected unexpectedly")
        }
    }
}

/// Install hint for a missing binary (best-effort).
pub fn install_hint(binary: &str) -> String {
    // Common package name overrides.
    let pkg = match binary {
        "rustscan"       => "rustscan",
        "eyewitness"     => "eyewitness",
        "theHarvester"   => "theharvester",
        "reconftw.sh"    => "reconftw",
        "sniper"         => "sn1per",
        "paramspider"    => "paramspider",
        "knockpy"        => "knockpy",
        other            => other,
    };
    format!("sudo apt install {pkg} -y   OR   pip install {pkg}")
}
