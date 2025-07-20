#[cfg(test)]
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use std::process::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("--help").assert().success();
}

#[test]
fn test_version() {
    let expected_version = "bigly 0.0.1\n";
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("--version").assert().stdout(expected_version);
}

#[test]
fn test_search_functionality() {
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("search").assert().success().stdout(predicate::str::contains("search"));
}

#[test]
fn test_search_with_file_option() {
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("--file").arg("CLAUDE.md").arg("command")
        .assert().success().stdout(predicate::str::contains("CLAUDE.md"));
}

#[test]
fn test_hazard_exit_code() {
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("hazard").assert().code(0);
}

#[test]
fn test_hazard_stdout() {
    let hazard_predicate = predicate::function(|x: &str| {
        if x == "You got it right!\n" || x == "You got it wrong!\n" {
            return true;
        } else {
            return false;
        }
    });
    let mut cmd = Command::cargo_bin("bigly").expect("Calling binary failed");
    cmd.arg("hazard").assert().stdout(hazard_predicate);
}
