use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut start_positions = Vec::new();
  for word in input[0].split(',') {
    let position = word.parse::<i64>()?;
    start_positions.push(position);
  }
  start_positions.sort();

  {
    let median = start_positions[start_positions.len() / 2];
    let mut cost = 0;
    for position in &start_positions {
      cost += (median - position).abs();
    }
    println!("{}", cost);
  }

  {
    let min = start_positions[0];
    let max = start_positions[start_positions.len() - 1];
    let mut best_cost = None;

    for destination in min .. max + 1 {
      let mut cost = 0;
      for position in &start_positions {
        let distance = (destination - position).abs();
        cost += distance * (distance + 1) / 2;
      }

      match best_cost {
        None => {
          best_cost = Some(cost);
        },
        Some(old_best) => {
          if cost < old_best {
            best_cost = Some(cost);
          }
        },
      }
    }

    println!("{}", best_cost.unwrap());
  }

  Ok(())
}
