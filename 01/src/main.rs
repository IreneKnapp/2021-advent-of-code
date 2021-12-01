use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_int_file(&filename)?;

  {
    let mut last_item = None;
    let mut n_increases = 0;
    for item in &input {
      match last_item {
        Some(value) => {
          if *item > value {
            n_increases += 1;
          }
        }
        None => { }
      }

      last_item = Some(*item);
    }

    println!("{}", n_increases);
  }

  {
    let mut window = Vec::new();
    let mut last_sum = None;
    let mut n_increases = 0;
    for item in &input {
      window.push(item);
      if window.len() > 3 {
        let _ = window.remove(0);
      }

      if window.len() == 3 {
        let mut sum = 0;
        for a in &window {
          sum += *a;
        }

        match last_sum {
          None => { }
          Some(value) => {
            if sum > value {
              n_increases += 1;
            }
          }
        }

        last_sum = Some(sum);
      }
    }

    println!("{}", n_increases);
  }

  Ok(())
}

