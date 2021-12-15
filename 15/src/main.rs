use advent_lib::prelude::*;

use std::collections::BinaryHeap;
use std::cmp::Reverse;


#[derive(Eq)]
struct Exploration {
  x: usize,
  y: usize,
  risk_so_far: i64,
}

impl Ord for Exploration {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    Reverse(&self.risk_so_far).cmp(&Reverse(&other.risk_so_far))
  }
}

impl PartialOrd for Exploration {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(Reverse(&self.risk_so_far).cmp(&Reverse(&other.risk_so_far)))
  }
}

impl PartialEq for Exploration {
  fn eq(&self, other: &Self) -> bool {
    self.risk_so_far == other.risk_so_far
  }
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut value_grid = Vec::new();
  let mut risk_grid: Vec<Vec<Option<i64>>> = Vec::new();
  for line in &input{
    let mut value_row = Vec::new();
    let mut risk_row = Vec::new();
    for c in line.chars() {
      let value = c.to_digit(10).unwrap() as i64;
      value_row.push(value);
      risk_row.push(None);
    }
    value_grid.push(value_row);
    risk_grid.push(risk_row);
  }

  let height = value_grid.len();
  let width = value_grid[0].len();

  risk_grid[0][0] = Some(0);
  find_path(0, 0, &value_grid, &mut risk_grid, false);
  let risk = risk_grid[height - 1][width - 1].unwrap();
  println!("{}", risk);

  let mut big_risk_grid = Vec::new();
  for _ in 0 .. height * 5 {
    let mut big_risk_grid_row = Vec::new();
    for _ in 0 .. width * 5 {
      big_risk_grid_row.push(None);
    }
    big_risk_grid.push(big_risk_grid_row);
  }
  big_risk_grid[0][0] = Some(0);
  find_path(0, 0, &value_grid, &mut big_risk_grid, true);
  let big_risk = big_risk_grid[height * 5 - 1][width * 5 - 1].unwrap();
  println!("{}", big_risk);

  Ok(())
}


#[allow(dead_code)]
fn debug_risk_grid(risk_grid: &Vec<Vec<Option<i64>>>) {
  for row in risk_grid {
    for cell in row {
      match cell {
        None => { print!(" ."); }
        Some(n) => { print!(" {}", n); }
      }
    }
    println!("");
  }
  println!("");
}


fn find_path(x: usize, y: usize, value_grid: &Vec<Vec<i64>>,
    risk_grid: &mut Vec<Vec<Option<i64>>>, repeat: bool)
{
  let mut height = value_grid.len();
  let mut width = value_grid[0].len();
  if repeat {
    width *= 5;
    height *= 5;
  }

  let mut exploration_queue = BinaryHeap::new();
  exploration_queue.push(Exploration { x, y, risk_so_far: 0 });

  loop {
    let Exploration { x, y, risk_so_far: _ } = match exploration_queue.pop() {
      None => { return; }
      Some(value) => { value }
    };
    let risk_so_far = risk_grid[y][x].unwrap();

    if x > 0 {
      let value_left = get_value(x - 1, y, value_grid);
      let new_risk_left = risk_so_far + value_left;

      let risk_left = risk_grid[y][x-1];
      let should_move = match risk_left {
        None => true,
        Some(prior_best_risk) => {
          new_risk_left < prior_best_risk
        }
      };
      if should_move {
        risk_grid[y][x-1] = Some(new_risk_left);

        exploration_queue.push(Exploration {
          x: x - 1,
          y,
          risk_so_far: new_risk_left
        });
      }
    }

    if y > 0 {
      let value_up = get_value(x, y - 1, value_grid);
      let new_risk_up = risk_so_far + value_up;

      let risk_up = risk_grid[y-1][x];
      let should_move = match risk_up {
        None => true,
        Some(prior_best_risk) => {
          new_risk_up < prior_best_risk
        }
      };
      if should_move {
        risk_grid[y-1][x] = Some(new_risk_up);

        exploration_queue.push(Exploration {
          x,
          y: y - 1,
          risk_so_far: new_risk_up
        });
      }
    }

    if x + 1 < width {
      let value_right = get_value(x + 1, y, value_grid);
      let new_risk_right = risk_so_far + value_right;

      let risk_right = risk_grid[y][x+1];
      let should_move = match risk_right {
        None => true,
        Some(prior_best_risk) => {
          new_risk_right < prior_best_risk
        }
      };
      if should_move {
        risk_grid[y][x+1] = Some(new_risk_right);

        exploration_queue.push(Exploration {
          x: x + 1,
          y,
          risk_so_far: new_risk_right
        });
      }
    }

    if y + 1 < height {
      let value_down = get_value(x, y + 1, value_grid);
      let new_risk_down = risk_so_far + value_down;

      let risk_down = risk_grid[y+1][x];
      let should_move = match risk_down {
        None => true,
        Some(prior_best_risk) => {
          new_risk_down < prior_best_risk
        }
      };
      if should_move {
        risk_grid[y+1][x] = Some(new_risk_down);

        exploration_queue.push(Exploration {
          x,
          y: y + 1,
          risk_so_far: new_risk_down
        });
      }
    }
  }
}


fn get_value(x: usize, y: usize, value_grid: &Vec<Vec<i64>>) -> i64 {
  let height = value_grid.len();
  let width = value_grid[0].len();

  let tile_x = x / width;
  let tile_y = y / height;
  let increment: i64 = (tile_x + tile_y) as i64;
  let raw_value = value_grid[y % height][x % width];
  (raw_value + increment - 1) % 9 + 1
}

