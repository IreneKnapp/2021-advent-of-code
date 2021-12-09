use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut grid = Vec::new();
  for line in &input {
    let mut row = Vec::new();
    for c in line.chars() {
      let value: i64 = c.to_digit(10).unwrap() as i64;
      row.push(value);
    }
    grid.push(row);
  }

  let height = grid.len();
  let width = grid[0].len();

  let mut sum = 0;
  let mut low_points: Vec<(usize, usize)> = Vec::new();
  for y in 0 .. height {
    for x in 0 .. width {
      let here: i64 = grid[y][x];

      if x > 0 {
        let left = grid[y][x - 1];
        if here >= left {
          continue;
        }
      }

      if x + 1 < width {
        let right = grid[y][x + 1];
        if here >= right {
          continue;
        }
      }

      if y > 0 {
        let up = grid[y - 1][x];
        if here >= up {
          continue;
        }
      }

      if y + 1 < height {
        let down = grid[y + 1][x];
        if here >= down {
          continue;
        }
      }

      let risk = here + 1;

      low_points.push((x, y));
      sum += risk;
    }
  }
  println!("{}", sum);

  let mut basin_sizes: Vec<i64> = Vec::new();
  for (x, y) in &low_points {
    let mut visited_grid = Vec::new();
    for _ in 0 .. height {
      let mut visited_row = Vec::new();
      for _ in 0 .. width {
        visited_row.push(false);
      }
      visited_grid.push(visited_row);
    }

    let size = basin_size(*x, *y, &grid, &mut visited_grid);
    basin_sizes.push(size);
  }
  basin_sizes.sort();

  let mut result = 1;
  result *= basin_sizes[basin_sizes.len() - 1];
  result *= basin_sizes[basin_sizes.len() - 2];
  result *= basin_sizes[basin_sizes.len() - 3];
  println!("{}", result);

  Ok(())
}


fn basin_size(x: usize, y: usize, grid: &Vec<Vec<i64>>,
              mut visited_grid: &mut Vec<Vec<bool>>)
  -> i64
{
  let mut size = 1;
  visited_grid[y][x] = true;

  let height = grid.len();
  let width = grid[0].len();
  let here = grid[y][x];

  if x > 0 {
    let left_visited = visited_grid[y][x - 1];
    let left = grid[y][x - 1];
    if !left_visited && (here < left) && (left != 9) {
      size += basin_size(x - 1, y, &grid, &mut visited_grid);
    }
  }

  if x + 1 < width {
    let right_visited = visited_grid[y][x + 1];
    let right = grid[y][x + 1];
    if !right_visited && (here < right) && (right != 9) {
      size += basin_size(x + 1, y, &grid, &mut visited_grid);
    }
  }

  if y > 0 {
    let up_visited = visited_grid[y - 1][x];
    let up = grid[y - 1][x];
    if !up_visited && (here < up) && (up != 9) {
      size += basin_size(x, y - 1, &grid, &mut visited_grid);
    }
  }

  if y + 1 < height {
    let down_visited = visited_grid[y + 1][x];
    let down = grid[y + 1][x];
    if !down_visited && (here < down) && (down != 9) {
      size += basin_size(x, y + 1, &grid, &mut visited_grid);
    }
  }

  size
}

