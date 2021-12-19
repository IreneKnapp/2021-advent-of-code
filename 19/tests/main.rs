use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_19")?;

  command.arg("small");
  command.assert().success().stdout(
      "79\n\
      3621\n");



  Ok(())
}

