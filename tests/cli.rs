use std::process::Command;

#[test]
fn cli_accepts_valid_input() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .arg("2025-10-01T12:00:00Z")
        .output()
        .expect("failed to run reltime");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be utf8");
    assert!(!stdout.trim().is_empty(), "expected non-empty relative output");
}

#[test]
fn cli_rejects_invalid_input() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .arg("not-a-date")
        .output()
        .expect("failed to run reltime");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be utf8");
    assert!(stderr.contains("error: invalid timestamp format"));
}

#[test]
fn cli_help_displays_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .arg("--help")
        .output()
        .expect("failed to run reltime --help");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be utf8");
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Examples:"));
    assert!(stdout.contains("--exact"));
}

#[test]
fn cli_accepts_exact_long_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .args(["--exact", "2000-01-01T00:00:00Z"])
        .output()
        .expect("failed to run reltime --exact");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be utf8");
    assert!(stdout.contains("ago"));
}

#[test]
fn cli_accepts_exact_short_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .args(["-e", "2000-01-01T00:00:00Z"])
        .output()
        .expect("failed to run reltime -e");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be utf8");
    assert!(stdout.contains("ago"));
}

#[test]
fn cli_rejects_invalid_input_in_exact_mode() {
    let output = Command::new(env!("CARGO_BIN_EXE_reltime"))
        .args(["--exact", "not-a-date"])
        .output()
        .expect("failed to run reltime --exact invalid");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be utf8");
    assert!(stderr.contains("error: invalid timestamp format"));
}
