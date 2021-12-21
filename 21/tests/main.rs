use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_21")?;

  command.arg("input");
  command.assert().success().stdout(
      "888735\n\
      647608359455719\n");

  Ok(())
}

