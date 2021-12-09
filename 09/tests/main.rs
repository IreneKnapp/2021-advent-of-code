use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_09")?;

  command.arg("input");
  command.assert().success().stdout(
      "502\n\
      1330560\n");

  Ok(())
}

