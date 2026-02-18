use serde_json::Value;
use std::process::Command;
use tccutil_rs::tcc::TccEntry;

/// Helper: run the `tccutil-rs` binary with given args, returning (stdout, stderr, success).
fn run_tcc(args: &[&str]) -> (String, String, bool) {
    let bin = env!("CARGO_BIN_EXE_tccutil-rs");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to execute tccutil-rs binary");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

// ── tccutil-rs services ─────────────────────────────────────────────

#[test]
fn services_runs_and_lists_known_services() {
    let (stdout, _stderr, success) = run_tcc(&["services"]);
    assert!(success, "tccutil-rs services should exit 0");

    // Header row
    assert!(stdout.contains("INTERNAL NAME"), "should have header");
    assert!(
        stdout.contains("DESCRIPTION"),
        "should have description header"
    );

    // Spot-check a handful of well-known service names
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

// ── tccutil-rs list ─────────────────────────────────────────────────

#[test]
fn list_runs_without_error() {
    // list reads the user TCC DB — may return entries or "No entries found."
    // Either way it should not crash.
    let (stdout, _stderr, success) = run_tcc(&["--user", "list"]);
    assert!(success, "tccutil-rs --user list should exit 0");
    // Output is either the table or the empty-state message
    assert!(
        stdout.contains("SERVICE") || stdout.contains("No entries found"),
        "expected table header or empty message, got: {}",
        stdout
    );
}

#[test]
fn list_compact_runs_without_error() {
    let (_stdout, _stderr, success) = run_tcc(&["--user", "list", "--compact"]);
    assert!(success, "tccutil-rs --user list --compact should exit 0");
}

#[test]
fn list_with_client_filter_runs() {
    let (_stdout, _stderr, success) = run_tcc(&["--user", "list", "--client", "apple"]);
    assert!(
        success,
        "tccutil-rs --user list --client apple should exit 0"
    );
}

#[test]
fn list_with_service_filter_runs() {
    let (_stdout, _stderr, success) = run_tcc(&["--user", "list", "--service", "Camera"]);
    assert!(
        success,
        "tccutil-rs --user list --service Camera should exit 0"
    );
}

// ── tccutil-rs info ─────────────────────────────────────────────────

#[test]
fn info_shows_macos_version_and_db_paths() {
    let (stdout, _stderr, success) = run_tcc(&["info"]);
    assert!(success, "tccutil-rs info should exit 0");

    assert!(
        stdout.contains("macOS version:"),
        "should show macOS version"
    );
    assert!(stdout.contains("User DB:"), "should show User DB path");
    assert!(stdout.contains("System DB:"), "should show System DB path");
    assert!(stdout.contains("TCC.db"), "should mention TCC.db");
    assert!(stdout.contains("SIP status:"), "should show SIP status");
}

// ── Error cases ──────────────────────────────────────────────────────

#[test]
fn no_subcommand_prints_help_and_fails() {
    let (_stdout, stderr, success) = run_tcc(&[]);
    assert!(!success, "tccutil-rs with no args should fail");
    // clap prints usage to stderr
    assert!(
        stderr.contains("Usage") || stderr.contains("usage"),
        "should print usage info"
    );
}

#[test]
fn unknown_subcommand_fails() {
    let (_stdout, _stderr, success) = run_tcc(&["bogus"]);
    assert!(!success, "tccutil-rs bogus should fail");
}

#[test]
fn version_flag_prints_version() {
    let (stdout, _stderr, success) = run_tcc(&["--version"]);
    assert!(success, "tccutil-rs --version should exit 0");
    assert!(
        stdout.contains("tccutil-rs"),
        "version output should mention tccutil-rs"
    );
}

// ── tccutil-rs list --json ──────────────────────────────────────────

const EXPECTED_JSON_FIELDS: &[&str] = &[
    "service_raw",
    "service_display",
    "client",
    "auth_value",
    "last_modified",
    "is_system",
];

#[test]
fn list_json_outputs_valid_json_array() {
    let (stdout, _stderr, success) = run_tcc(&["--user", "list", "--json"]);
    assert!(success, "tccutil-rs --user list --json should exit 0");

    // Always assert: output is valid JSON and is an array
    let parsed: Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
    let arr = parsed.as_array().expect("JSON output should be an array");

    // If entries exist, verify each has the expected fields
    for (i, entry) in arr.iter().enumerate() {
        assert!(
            entry.is_object(),
            "entry at index {} should be an object",
            i
        );
        for field in EXPECTED_JSON_FIELDS {
            assert!(
                entry.get(field).is_some(),
                "entry at index {} missing field '{}'",
                i,
                field
            );
        }
    }

    // Unconditional field check: serialize an actual TccEntry and verify
    // that EXPECTED_JSON_FIELDS matches the real struct fields. If TccEntry
    // gains, loses, or renames a field, this will catch the mismatch.
    let entry = TccEntry {
        service_raw: String::new(),
        service_display: String::new(),
        client: String::new(),
        auth_value: 0,
        last_modified: String::new(),
        is_system: false,
    };
    let serialized = serde_json::to_value(&entry).expect("TccEntry should serialize");
    let obj = serialized
        .as_object()
        .expect("serialized TccEntry should be an object");
    for field in EXPECTED_JSON_FIELDS {
        assert!(
            obj.contains_key(*field),
            "TccEntry serialization missing expected field '{}'",
            field
        );
    }
    assert_eq!(
        obj.len(),
        EXPECTED_JSON_FIELDS.len(),
        "TccEntry has {} fields but EXPECTED_JSON_FIELDS lists {} (add/remove entries to keep in sync)",
        obj.len(),
        EXPECTED_JSON_FIELDS.len()
    );
}

#[test]
fn list_json_service_filter_returns_valid_structure() {
    // Use a service that almost certainly exists (Accessibility is one of the oldest TCC services).
    // Even if zero rows match, the output must still be a valid JSON array.
    let (stdout, _stderr, success) =
        run_tcc(&["--user", "list", "--json", "--service", "Accessibility"]);
    assert!(success);

    let parsed: Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
    let arr = parsed.as_array().expect("JSON output should be an array");

    for (i, entry) in arr.iter().enumerate() {
        assert!(entry.is_object(), "entry {} should be an object", i);
        assert!(
            entry.get("service_raw").is_some(),
            "entry {} missing service_raw",
            i
        );
    }
}

#[test]
fn list_compact_and_json_conflict() {
    let (_stdout, stderr, success) = run_tcc(&["--user", "list", "--compact", "--json"]);
    assert!(!success, "passing both --compact and --json should fail");
    assert!(
        stderr.contains("cannot be used with"),
        "clap should report argument conflict, got: {}",
        stderr
    );
}

#[test]
fn list_json_with_client_filter_only_contains_matching_entries() {
    let (stdout, _stderr, success) = run_tcc(&["--user", "list", "--json", "--client", "apple"]);
    assert!(
        success,
        "tccutil-rs --user list --json --client apple should exit 0"
    );

    // Unconditional: output is valid JSON array regardless of DB contents
    let parsed: Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
    let arr = parsed.as_array().expect("should be an array");

    // Every returned entry (if any) must be an object with a "client" field
    // containing the filter string. This verifies filter correctness structurally,
    // even if the result set is empty (no assertions are skipped).
    for (i, entry) in arr.iter().enumerate() {
        assert!(
            entry.is_object(),
            "entry at index {} should be an object",
            i
        );
        let client = entry
            .get("client")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("entry at index {} missing 'client' string field", i));
        assert!(
            client.to_lowercase().contains("apple"),
            "filtered entry at index {} should contain 'apple', got: {}",
            i,
            client
        );
    }

    // Unconditional: verify filtering with a guaranteed-no-match client
    // produces a valid empty JSON array (exercises the filter code path
    // even when the DB is empty)
    let (stdout2, _stderr2, success2) = run_tcc(&[
        "--user",
        "list",
        "--json",
        "--client",
        "zzz_nonexistent_client_zzz",
    ]);
    assert!(success2, "filter with no-match client should still exit 0");
    let parsed2: Value =
        serde_json::from_str(&stdout2).expect("no-match output should be valid JSON");
    let arr2 = parsed2
        .as_array()
        .expect("no-match output should be an array");
    assert!(
        arr2.is_empty(),
        "filtering by nonexistent client should return empty array, got {} entries",
        arr2.len()
    );
}
