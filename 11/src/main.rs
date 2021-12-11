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

  let mut n_flashes = 0;
  for _ in 0 .. 100 {
    n_flashes += iterate(&mut grid);
  }
  println!("{}", n_flashes);

  for i in 101 .. 1000000 {
    if iterate(&mut grid) == 100 {
      println!("{}", i);
      break;
    }
  }

  Ok(())
}


#[allow(dead_code)]
fn debug_grid(grid: &Vec<Vec<i64>>) {
  for row in grid {
    for cell in row {
      print!(" {}", cell)
    }
    println!("");
  }
  println!("");
}



fn iterate(grid: &mut Vec<Vec<i64>>) -> i64 {
  let height = grid.len();
  let width = grid[0].len();

  let mut has_flashed = Vec::new();
  for _ in 0 .. height {
    let mut has_flashed_row = Vec::new();
    for _ in 0 .. width {
      has_flashed_row.push(false);
    }
    has_flashed.push(has_flashed_row);
  }

  for y in 0 .. height {
    for x in 0 .. width {
      grid[y][x] += 1;
    }
  }

  let mut n_flashes = 0;
  loop {
    let mut any_flashes = false;

    for y in 0 .. height {
      for x in 0 .. width {
        if (grid[y][x] > 9) && !has_flashed[y][x] {
          n_flashes += 1;
          has_flashed[y][x] = true;
          any_flashes = true;

          for (dx, dy) in &[(-1, -1), (0, -1), (1, -1),
                            (-1, 0),           (1, 0),
                            (-1, 1),  (0, 1),  (1, 1)]
          {
            let adjacent_x = x as i64 + dx;
            let adjacent_y = y as i64 + dy;

            if (adjacent_x >= 0) && (adjacent_x < width as i64)
              && (adjacent_y >= 0) && (adjacent_y < height as i64)
            {
              grid[adjacent_y as usize][adjacent_x as usize] += 1;
            }
          }
        }
      }
    }

    if !any_flashes {
      break;
    }
  }

  for y in 0 .. height {
    for x in 0 .. width {
      if has_flashed[y][x] {
        grid[y][x] = 0;
      }
    }
  }

  n_flashes
}

