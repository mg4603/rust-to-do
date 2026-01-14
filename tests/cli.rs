use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn add_and_list_task() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["add", "Buy milk"])
        .assert()
        .success();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(contains("Buy milk"));
}

#[test]
fn complete_task() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["add", "Test"])
        .assert()
        .success();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["done", "1"])
        .assert()
        .success()
        .stdout(contains("Completed"));

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["list"])
        .assert()
        .stdout(contains("[x]"));
}

#[test]
fn delete_task() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["add", "Task1"])
        .assert()
        .success();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["delete", "1"])
        .assert()
        .success()
        .stdout(contains("Deleted"));

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
    cmd.current_dir(dir.path())
        .args(["list"])
        .assert()
        .stdout(contains("No tasks"));
}
