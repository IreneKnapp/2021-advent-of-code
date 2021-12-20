use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_20")?;

  command.arg("small");
  command.assert().success().stdout(
      "5573\n\
      20097\n");



  Ok(())
}

