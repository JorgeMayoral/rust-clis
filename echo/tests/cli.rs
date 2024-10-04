use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn error_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: echo <TEXT>"));
    Ok(())
}

#[test]
fn ok_with_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("test").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = std::fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn single_input() -> TestResult {
    run(&["Test output"], "tests/expected/output1.txt")
}

#[test]
fn multiple_input_multiple_space() -> TestResult {
    run(&["Test", "output"], "tests/expected/output2.txt")
}

#[test]
fn single_input_multiple_space_no_newline() -> TestResult {
    run(&["-n", "Test  output"], "tests/expected/output3.txt")
}

#[test]
fn multiple_input_no_newline() -> TestResult {
    run(&["-n", "Test", "output"], "tests/expected/output4.txt")
}
