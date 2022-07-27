use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type Handler = Result<(), Box<dyn std::error::Error>>;

#[test]
fn run_with_defaults() -> Handler {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Handler {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "non-existing-file"])
        .assert()
        .failure();
    Ok(())
}
