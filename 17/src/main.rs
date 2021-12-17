use advent_lib::prelude::*;


#[derive(Debug)]
struct TargetArea {
  min_x: i64,
  max_x: i64,
  min_y: i64,
  max_y: i64,
}


#[derive(Debug, Clone, Copy)]
struct Point {
  x: i64,
  y: i64,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let words: Vec<&str> = input[0].split_whitespace().collect();
  let x_parts: Vec<&str> = words[2].strip_prefix("x=").unwrap()
    .strip_suffix(",").unwrap().split("..").collect();
  let y_parts: Vec<&str> = words[3].strip_prefix("y=").unwrap()
    .split("..").collect();
  let min_x = x_parts[0].parse::<i64>().unwrap();
  let max_x = x_parts[1].parse::<i64>().unwrap();
  let min_y = y_parts[0].parse::<i64>().unwrap();
  let max_y = y_parts[1].parse::<i64>().unwrap();
  let target_area = TargetArea { min_x, max_x, min_y, max_y };

  let (best_apogee, n_successes) = search_for_best_apogee(&target_area);
  println!("{}", best_apogee);
  println!("{}", n_successes);

  Ok(())
}


fn search_for_best_apogee(target_area: &TargetArea) -> (i64, i64) {
  let mut best_apogee = None;
  let mut n_successes = 0;

  for dx in 1 .. target_area.max_x + 1 {
    for dy in -100 .. {
      let trajectory = Point { x: dx, y: dy };

      let apogee = find_apogee(trajectory, target_area);

      match (best_apogee, apogee) {
        (Some(old_apogee), Some(new_apogee)) => {
          if new_apogee > old_apogee {
            best_apogee = apogee;
          }

          n_successes += 1;
        }
        (None, Some(_)) => {
          best_apogee = apogee;
          n_successes += 1;
        }
        (_, None) => { }
      }

      if should_stop_y(trajectory, target_area) {
        break;
      }

      if dy > 5000 {
        break;
      }
    }
  }

  (best_apogee.unwrap(), n_successes)
}


fn should_stop_y(trajectory: Point, target_area: &TargetArea) -> bool {
  let mut trajectory = trajectory;
  let mut position = Point { x: 0, y: 0 };

  loop {

    if is_in_target_area(&position, target_area) {
      return false;
    }

    if is_unable_to_reach_target_area(&position, &trajectory, target_area) {
      return true;
    }

    if is_past_target_area(&position, target_area) {
      if is_above_target_area(&position, target_area) {
        return true;
      } else {
        return false;
      }
    }

    iterate(&mut position, &mut trajectory);
  }
}


fn find_apogee(trajectory: Point, target_area: &TargetArea)
  -> Option<i64>
{
  let mut trajectory = trajectory;
  let mut position = Point { x: 0, y: 0 };
  let mut apogee = 0;

  loop {
    if position.y > apogee {
      apogee = position.y;
    }

    if is_in_target_area(&position, target_area) {
      return Some(apogee);
    }

    if is_unable_to_reach_target_area(&position, &trajectory, target_area) {
      return None;
    }

    if is_past_target_area(&position, target_area) {
      return None;
    }

    iterate(&mut position, &mut trajectory);
  }
}


fn iterate(position: &mut Point, trajectory: &mut Point) {
  position.x += trajectory.x;
  position.y += trajectory.y;

  if trajectory.x > 0 {
    trajectory.x -= 1;
  } else if trajectory.x < 0 {
    trajectory.x += 1;
  }

  trajectory.y -= 1;
}


fn is_in_target_area(position: &Point, target_area: &TargetArea) -> bool {
  (position.x >= target_area.min_x)
    && (position.x <= target_area.max_x)
    && (position.y >= target_area.min_y)
    && (position.y <= target_area.max_y)
}


fn is_past_target_area(position: &Point, target_area: &TargetArea) -> bool {
  (position.x > target_area.max_x) || (position.y < target_area.min_y)
}


fn is_unable_to_reach_target_area(position: &Point, trajectory: &Point,
    target_area: &TargetArea)
  -> bool
{
  (position.x < target_area.min_x) && (trajectory.x == 0)
}


fn is_above_target_area(position: &Point, target_area: &TargetArea)
  -> bool
{
  position.y > target_area.max_y
}

