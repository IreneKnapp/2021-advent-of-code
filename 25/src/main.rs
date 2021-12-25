use advent_lib::prelude::*;

#[derive(Debug,Clone,Eq,PartialEq)]
enum Cell {
  Empty,
  East,
  South,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut initial_grid = Vec::new();
  for line in &input {
    let mut row = Vec::new();
    for c in line.chars() {
      row.push(match c {
        '.' => Cell::Empty,
        '>' => Cell::East,
        'v' => Cell::South,
        _ => panic!("Unrecognized character."),
      });
    }
    initial_grid.push(row);
  }

  let mut i = 0;
  let mut grid = initial_grid.clone();
  loop {
    i += 1;
    let (anything_moved, new_grid) = iterate(&grid);
    grid = new_grid;

    println!("iteration {}", i);
    debug_grid(&grid);

    if !anything_moved {
      break;
    }
  }
  println!("{}", i);

  Ok(())
}


fn iterate(grid: &Vec<Vec<Cell>>) -> (bool, Vec<Vec<Cell>>) {
  let mut anything_moved = false;

  let height = grid.len();
  let width = grid[0].len();

  let mut new_grid = Vec::new();
  for _ in 0 .. height {
    let mut row = Vec::new();
    for _ in 0 .. width {
      row.push(Cell::Empty);
    }
    new_grid.push(row);
  }

  for y in 0 .. height {
    for x in 0 .. width {
      if grid[y][x] == Cell::East {
        let new_x = (x + 1) % width;
        if grid[y][new_x] == Cell::Empty {
          new_grid[y][new_x] = Cell::East;
          anything_moved = true;
        } else {
          new_grid[y][x] = Cell::East;
        }
      }
    }
  }

  for y in 0 .. height {
    for x in 0 .. width {
      if grid[y][x] == Cell::South {
        let new_y = (y + 1) % height;
        if new_grid[new_y][x] == Cell::Empty
          && grid[new_y][x] != Cell::South
        {
          new_grid[new_y][x] = Cell::South;
          anything_moved = true;
        } else {
          new_grid[y][x] = Cell::South;
        }
      }
    }
  }

  (anything_moved, new_grid)
}


#[allow(dead_code)]
fn debug_grid(grid: &Vec<Vec<Cell>>) {
  for row in grid {
    for cell in row {
      match cell {
        Cell::Empty => print!("."),
        Cell::East => print!(">"),
        Cell::South => print!("v"),
      }
    }
    println!("");
  }
  println!("");
}

