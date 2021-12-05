use advent_lib::prelude::*;

use std::collections::HashMap;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut line_specs = Vec::new();
  for line in &input {
    let words: Vec<&str> = line.split_whitespace().collect();
    let start = parse_coords(words[0])?;
    let end = parse_coords(words[2])?;
    line_specs.push((start, end));
  }

  {
    let mut counts: HashMap<(i64, i64), i64> = HashMap::new();
    for ((x_start, y_start), (x_end, y_end)) in &line_specs {
      if x_start == x_end {
        let x = *x_start;

        let (y_min, y_max) = if y_start <= y_end {
          (y_start, y_end)
        } else {
          (y_end, y_start)
        };

        for y in *y_min .. y_max + 1 {
          let n = match counts.get(&(x, y)) {
            Some(n) => *n,
            None => 0,
          };
          counts.insert((x, y), n + 1);
        }
      } else if y_start == y_end {
        let y = *y_start;

        let (x_min, x_max) = if x_start <= x_end {
          (x_start, x_end)
        } else {
          (x_end, x_start)
        };

        for x in *x_min .. x_max + 1 {
          let n = match counts.get(&(x, y)) {
            Some(n) => *n,
            None => 0,
          };
          counts.insert((x, y), n + 1);
        }
      }
    }

    let mut n_intersections = 0;
    for cell in counts.values() {
      if *cell > 1 {
        n_intersections += 1;
      }
    }
    println!("{}", n_intersections);
  }

  {
    let mut counts: HashMap<(i64, i64), i64> = HashMap::new();
    for ((x_start, y_start), (x_end, y_end)) in &line_specs {
      if x_start == x_end {
        let x = *x_start;

        let (y_min, y_max) = if y_start <= y_end {
          (y_start, y_end)
        } else {
          (y_end, y_start)
        };

        for y in *y_min .. y_max + 1 {
          let n = match counts.get(&(x, y)) {
            Some(n) => *n,
            None => 0,
          };
          counts.insert((x, y), n + 1);
        }
      } else if y_start == y_end {
        let y = *y_start;

        let (x_min, x_max) = if x_start <= x_end {
          (x_start, x_end)
        } else {
          (x_end, x_start)
        };

        for x in *x_min .. x_max + 1 {
          let n = match counts.get(&(x, y)) {
            Some(n) => *n,
            None => 0,
          };
          counts.insert((x, y), n + 1);
        }
      } else {
        let (x_start, x_end, y_start, y_end) = if x_start <= x_end {
          (x_start, x_end, y_start, y_end)
        } else {
          (x_end, x_start, y_end, y_start)
        };

        if y_start <= y_end {
          for i in 0 .. x_end + 1 - x_start {
            let x = x_start + i;
            let y = y_start + i;

            let n = match counts.get(&(x, y)) {
              Some(n) => *n,
              None => 0,
            };
            counts.insert((x, y), n + 1);
          }
        } else {
          for i in 0 .. x_end + 1 - x_start {
            let x = x_start + i;
            let y = y_start - i;

            let n = match counts.get(&(x, y)) {
              Some(n) => *n,
              None => 0,
            };
            counts.insert((x, y), n + 1);
          }
        }
      }
    }

    let mut n_intersections = 0;
    for cell in counts.values() {
      if *cell > 1 {
        n_intersections += 1;
      }
    }
    println!("{}", n_intersections);
  }

  Ok(())
}


fn parse_coords(word: &str) -> Result<(i64, i64)> {
  let values: Vec<&str> = word.split(',').collect();
  let x = values[0].parse::<i64>()?;
  let y = values[1].parse::<i64>()?;
  Ok((x, y))
}

