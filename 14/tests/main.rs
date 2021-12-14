use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_14")?;

  command.arg("input");
  command.assert().success().stdout(
      "2797\n\
      2926813379532\n");

  Ok(())
}

