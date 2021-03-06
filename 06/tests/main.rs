use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_06")?;

  command.arg("input");
  command.assert().success().stdout(
      "386536\n\
      1732821262171\n");

  Ok(())
}

