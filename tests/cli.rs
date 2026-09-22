// Startup errors must be visible without CURSOR_BRIDGE_DEBUG.
//
// Each test sets env vars only on the child process, so tests stay
// independent of each other and of whether `agent`/`claude` are installed.

use std::path::PathBuf;
use std::process::{Command, Output};

fn missing_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "cursor-bridge-missing-{name}-{}",
        std::process::id()
    ))
}

fn run_bridge(configure: impl FnOnce(&mut Command)) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cursor-bridge"));
    command.env_remove("CURSOR_BRIDGE_DEBUG");
    configure(&mut command);
    command.output().expect("run cursor-bridge")
}

#[test]
fn missing_agent_path_reports_error_without_debug_flag() {
    let output = run_bridge(|command| {
        command.env("AGENT_PATH", missing_path("agent"));
    });
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(stderr.contains("AGENT_PATH"), "stderr: {stderr}");
}

#[test]
fn missing_claude_path_reports_error_without_debug_flag() {
    // Any existing executable satisfies AGENT_PATH; the test binary itself
    // is an `.exe` on Windows and a plain file elsewhere.
    let existing_agent = std::env::current_exe().expect("current exe");
    let output = run_bridge(|command| {
        command
            .env("AGENT_PATH", existing_agent)
            .env("CLAUDE_PATH", missing_path("claude"));
    });
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(stderr.contains("CLAUDE_PATH"), "stderr: {stderr}");
}
