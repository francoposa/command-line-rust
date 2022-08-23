use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args_failure_prints_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn assert_expected_file_success(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn one_arg_success() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn echo_parity_single_quoted_arg_success() -> TestResult {
    let args = &["Hello there"];
    let expected_file = "tests/expected/hello1.txt";
    assert_expected_file_success(args, expected_file)
}

#[test]
fn echo_parity_double_arg_success() -> TestResult {
    let args = &["Hello", "there"];
    let expected_file = "tests/expected/hello2.txt";
    assert_expected_file_success(args, expected_file)
}

#[test]
fn echo_parity_single_quoted_arg_no_newline_success() -> TestResult {
    let args = &["Hello  there", "-n"];
    let expected_file = "tests/expected/hello1.n.txt";
    assert_expected_file_success(args, expected_file)
}

#[test]
fn echo_parity_double_arg_no_newline_success() -> TestResult {
    let args = &["-n", "Hello", "there"];
    let expected_file = "tests/expected/hello2.n.txt";
    assert_expected_file_success(args, expected_file)
}
