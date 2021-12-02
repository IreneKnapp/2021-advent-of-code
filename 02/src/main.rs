use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  {
    let mut depth = 0;
    let mut horizontal = 0;

    for line in &input {
      let mut words = line.split(' ');
      let command = words.next().unwrap();
      let parameter = words.next().unwrap().parse::<i64>().unwrap();

      match command {
        "forward" => {
          horizontal += parameter;
        },
        "down" => {
          depth += parameter;
        },
        "up" => {
          depth -= parameter;
        },
        _ => { },
      }
    }

    println!("{}", horizontal * depth);
  }

  {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for line in &input {
      let mut words = line.split(' ');
      let command = words.next().unwrap();
      let parameter = words.next().unwrap().parse::<i64>().unwrap();

      match command {
        "forward" => {
          horizontal += parameter;
          depth += aim * parameter;
        },
        "down" => {
          aim += parameter;
        },
        "up" => {
          aim -= parameter;
        },
        _ => { },
      }
    }

    println!("{}", horizontal * depth);
  }

  Ok(())
}

