use assert_cmd::Command;

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn hello_output() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("hello, world\n");
}
