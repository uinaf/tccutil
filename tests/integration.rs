use std::process::Command;

/// `HOME` is overridden to a fresh tempdir so the user TCC.db path resolves to
/// a non-existent file. That way `list --user` returns an empty result without
/// depending on the host machine's privacy data or Full Disk Access state, and
/// the test outcome is deterministic across local dev boxes and CI runners.
fn run_tcc(args: &[&str]) -> (String, String, bool) {
    let bin = env!("CARGO_BIN_EXE_tccutil-rs");
    let home = tempfile::tempdir().expect("create tempdir for HOME");
    let output = Command::new(bin)
        .args(args)
        .env("HOME", home.path())
        .output()
        .expect("failed to execute tccutil-rs binary");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

fn assert_basic_json_shape(stdout: &str) {
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{'),
        "JSON output should start with '{{'"
    );
    assert!(trimmed.ends_with('}'), "JSON output should end with '}}'");
}

#[test]
fn services_runs_and_lists_known_services() {
    let (stdout, _stderr, success) = run_tcc(&["services"]);
    assert!(success, "tccutil-rs services should exit 0");

    assert!(stdout.contains("INTERNAL NAME"), "should have header");
    assert!(
        stdout.contains("DESCRIPTION"),
        "should have description header"
    );

    assert!(
        stdout.contains("Accessibility"),
        "should list Accessibility"
    );
    assert!(stdout.contains("Camera"), "should list Camera");
    assert!(stdout.contains("Microphone"), "should list Microphone");
    assert!(
        stdout.contains("Screen Recording"),
        "should list Screen Recording"
    );
    assert!(
        stdout.contains("Full Disk Access"),
        "should list Full Disk Access"
    );
}

#[test]
fn empty_list_variants_render_the_same_empty_state() {
    let cases = [
        &["--user", "list"][..],
        &["--user", "list", "--compact"][..],
        &["--user", "list", "--client", "apple"][..],
        &["--user", "list", "--service", "Camera"][..],
    ];

    for args in cases {
        let (stdout, stderr, success) = run_tcc(args);
        assert!(success, "args: {args:?}; stderr: {stderr}");
        assert_eq!(stdout, "No entries found.\n", "args: {args:?}");
    }
}

#[test]
fn no_subcommand_prints_help_and_fails() {
    let (_stdout, stderr, success) = run_tcc(&[]);
    assert!(!success, "tccutil-rs with no args should fail");
    assert!(
        stderr.contains("Usage") || stderr.contains("usage"),
        "should print usage info"
    );
}

#[test]
fn unknown_subcommand_fails() {
    let (_stdout, stderr, success) = run_tcc(&["bogus"]);
    assert!(!success, "tccutil-rs bogus should fail");
    assert!(stderr.contains("unrecognized subcommand 'bogus'"));
}

#[test]
fn version_flag_prints_version() {
    let (stdout, _stderr, success) = run_tcc(&["--version"]);
    assert!(success, "tccutil-rs --version should exit 0");
    assert_eq!(
        stdout,
        format!("tccutil-rs {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn services_json_mode_returns_valid_json() {
    let (stdout, stderr, success) = run_tcc(&["services", "--json"]);
    assert!(success, "tccutil-rs services --json should exit 0");
    assert!(
        stderr.trim().is_empty(),
        "stderr should be empty in JSON mode"
    );

    assert_basic_json_shape(&stdout);
    assert!(stdout.contains("\"ok\":true"));
    assert!(stdout.contains("\"command\":\"services\""));
    assert!(stdout.contains("\"data\":{\"services\":["));
    assert!(stdout.contains("\"error\":null"));
}

#[test]
fn list_json_mode_returns_valid_json() {
    let (stdout, stderr, success) = run_tcc(&["--user", "list", "--json"]);
    assert!(success, "tccutil-rs --user list --json should exit 0");
    assert!(
        stderr.trim().is_empty(),
        "stderr should be empty in JSON mode"
    );

    assert_basic_json_shape(&stdout);
    assert!(stdout.contains("\"ok\":true"));
    assert!(stdout.contains("\"command\":\"list\""));
    assert!(stdout.contains("\"data\":{\"count\":"));
    assert!(stdout.contains("\"entries\":["));
    assert!(stdout.contains("\"warnings\":["));
    assert!(stdout.contains("\"error\":null"));
}

#[test]
fn grant_json_force_unknown_service_still_errors() {
    let (stdout, stderr, success) = run_tcc(&[
        "grant",
        "DefinitelyNotARealService",
        "com.example.app",
        "--json",
        "--force",
    ]);
    assert!(!success);
    assert!(stderr.trim().is_empty());
    assert!(stdout.contains("\"ok\":false"));
    assert!(stdout.contains("\"kind\":\"UnknownService\""));
}

#[test]
fn grant_json_mode_failure_has_error_shape() {
    let (stdout, stderr, success) = run_tcc(&[
        "grant",
        "DefinitelyNotARealService",
        "com.example.app",
        "--json",
    ]);
    assert!(!success, "grant with unknown service should fail");
    assert!(
        stderr.trim().is_empty(),
        "stderr should be empty in JSON mode"
    );

    assert_basic_json_shape(&stdout);
    assert!(stdout.contains("\"ok\":false"));
    assert!(stdout.contains("\"command\":\"grant\""));
    assert!(stdout.contains("\"data\":null"));
    assert!(stdout.contains("\"error\":{\"kind\":"));
    assert!(stdout.contains("\"message\":\""));
}
