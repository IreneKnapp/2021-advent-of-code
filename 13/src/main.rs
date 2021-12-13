use advent_lib::prelude::*;

use std::collections::HashSet;


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Axis {
  X,
  Y,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::group_lines_by_blanks(
    advent_lib::read_lines_file(&filename)?);

  let mut dots: HashSet<(u64,u64)> = HashSet::new();
  for line in &input[0] {
    let words: Vec<&str> = line.split(',').collect();
    let x = words[0].parse::<u64>()?;
    let y = words[1].parse::<u64>()?;
    dots.insert((x, y));
  }

  let mut folds: Vec<(Axis,u64)> = Vec::new();
  for line in &input[1]{
    let words: Vec<&str> = line.split(' ').collect();
    let subwords: Vec<&str> = words[2].split('=').collect();
    let axis = match subwords[0] {
      "x" => Axis::X,
      "y" => Axis::Y,
      _ => panic!("hm"),
    };
    let value = subwords[1].parse::<u64>()?;
    folds.push((axis, value));
  }

  let (width, height) = bounds(&dots);

  let mut first_fold = Vec::new();
  first_fold.push(folds[0].clone());

  let rendering = render(width, height, &dots, &first_fold);
  println!("{}", rendering.len());

  let rendering = render(width, height, &dots, &folds);
  let (width, height) = bounds(&rendering);
  for y in 0 .. height {
    for x in 0 .. width {
      if rendering.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }

  Ok(())
}


fn bounds(dots: &HashSet<(u64, u64)>) -> (u64, u64) {
  let mut max_x = None;
  let mut max_y = None;
  for (x, y) in dots {
    max_x = match max_x {
      None => Some(x),
      Some(old_x) => if x > old_x {
        Some(x)
      } else {
        Some(old_x)
      }
    };

    max_y = match max_y {
      None => Some(y),
      Some(old_y) => if y > old_y {
        Some(y)
      } else {
        Some(old_y)
      }
    };
  }
  let width = max_x.unwrap() + 1;
  let height = max_y.unwrap() + 1;

  (width, height)
}


fn render(width: u64, height: u64, dots: &HashSet<(u64, u64)>,
    folds: &Vec<(Axis, u64)>)
  -> HashSet<(u64, u64)>
{
  let mut result = HashSet::new();

  for y in 0 .. height {
    for x in 0 .. width {
      if dots.contains(&(x, y)) {
        let (new_x, new_y) = apply_folds(x, y, folds);
        result.insert((new_x, new_y));
      }
    }
  }

  result
}


fn apply_folds(x: u64, y: u64, folds: &Vec<(Axis, u64)>) -> (u64, u64) {
  let mut transformed_x = x;
  let mut transformed_y = y;

  for fold in folds {
    let (new_x, new_y) = apply_fold(transformed_x, transformed_y, fold);
    transformed_x = new_x;
    transformed_y = new_y;
  }

  (transformed_x, transformed_y)
}


fn apply_fold(x: u64, y: u64, fold: &(Axis, u64)) -> (u64, u64) {
  let (axis, fold_value) = fold;
  match axis {
    Axis::X => {
      if x > *fold_value {
        (2 * *fold_value - x, y)
      } else {
        (x, y)
      }
    }
    Axis::Y => {
      if y > *fold_value {
        (x, 2 * *fold_value - y)
      } else {
        (x, y)
      }
    }
  }
}

