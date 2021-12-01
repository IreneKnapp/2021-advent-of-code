use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_01")?;

  command.arg("input");
  command.assert().success().stdout(
      "1215\n\
      1150\n");

  Ok(())
}

