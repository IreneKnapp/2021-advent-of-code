use advent_lib::prelude::*;

use std::collections::BTreeSet;


#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

#[derive(Debug, Clone)]
struct Rotation {
  top: RotationAngle,
  front: RotationAngle,
  right: RotationAngle,
}

#[derive(Debug, Clone, Copy)]
enum RotationAngle {
  None,
  Ninety,
  OneEighty,
  TwoSeventy,
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

  let mut scanners = Vec::new();
  for group in &input {
    let mut scanner = Vec::new();
    for line in group.iter().skip(1) {
      let values: Vec<&str> = line.split(",").collect();
      let x = values[0].parse::<i64>().unwrap();
      let y = values[1].parse::<i64>().unwrap();
      let z = values[2].parse::<i64>().unwrap();
      scanner.push(Point { x, y, z });
    }
    scanners.push(scanner);
  }

  let (merged_points, largest_distance) =
    compare_all_scanners(&scanners, false);
  println!("{}", merged_points.len());
  println!("{}", largest_distance);

  Ok(())
}


fn compare_all_scanners(all_scanners: &Vec<Vec<Point>>, perform_sort: bool)
  -> (Vec<Point>, i64)
{
  let mut unvisited_scanners = all_scanners.clone();
  let mut visited_scanner_offsets = Vec::new();

  visited_scanner_offsets.push(Point { x: 0, y: 0, z: 0 });

  loop {
    let mut found_match = false;

    //println!("{} scanners remain", unvisited_scanners.len());

    for i in 0 .. unvisited_scanners.len() - 1 {
      for j in i + 1 .. unvisited_scanners.len() {
        //println!("comparing {} vs {}", i, j);
        let scanner_a = &unvisited_scanners[i];
        let scanner_b = &unvisited_scanners[j];

        match compare_scanners(scanner_a, scanner_b) {
          None => { },
          Some((a_b_unioned_points, offset)) => {
            //println!("matched {} vs {} for {} beacons", i, j,
            //  a_b_unioned_points.len());

            unvisited_scanners.remove(j);
            unvisited_scanners[i] = a_b_unioned_points;
            if perform_sort {
              unvisited_scanners.sort_by(|a, b|
                a.len().partial_cmp(&b.len()).unwrap());
            }

            visited_scanner_offsets.push(offset);

            found_match = true;
          },
        }

        if found_match {
          break;
        }
      }

      if found_match {
        break;
      }
    }

    if !found_match {
      break;
    }
  }

  let mut largest_distance = None;
  for i in 0 .. visited_scanner_offsets.len() - 1 {
    for j in i + 1 .. visited_scanner_offsets.len() {
      let a = &visited_scanner_offsets[i];
      let b = &visited_scanner_offsets[j];
      let new_distance = compute_distance(a, b);

      match largest_distance {
        None => {
          largest_distance = Some(new_distance);
        }
        Some(old_distance) => {
          if new_distance > old_distance {
            largest_distance = Some(new_distance);
          }
        }
      }
    }
  }

  (unvisited_scanners[0].clone(), largest_distance.unwrap())
}


fn compare_scanners(scanner_a: &Vec<Point>, scanner_b: &Vec<Point>)
  -> Option<(Vec<Point>,Point)>
{
  let all_rotations = compute_all_rotations();

  for rotation in &all_rotations {
    let mut sorted_scanner_a = scanner_a.clone();
    sorted_scanner_a.sort();

    let mut sorted_scanner_b = Vec::new();
    for point in scanner_b {
      let rotated_point = apply_rotation(point, rotation);
      sorted_scanner_b.push(rotated_point);
    }
    sorted_scanner_b.sort();

    for a_i in 0 .. sorted_scanner_a.len() {
      for b_i in 0 .. sorted_scanner_b.len() {
        let a_origin_point = sorted_scanner_a[a_i].clone();
        let b_origin_point = sorted_scanner_b[b_i].clone();

        let mut n_matches = 0;
        let mut a_j = 0;
        let mut b_j = 0;
        loop {
          if a_j >= sorted_scanner_a.len() {
            break;
          }
          if b_j >= sorted_scanner_b.len() {
            break;
          }

          let a_point = point_difference(
            &sorted_scanner_a[a_j], &a_origin_point);
          let b_point = point_difference(
            &sorted_scanner_b[b_j], &b_origin_point);

          if a_point == b_point {
            n_matches += 1;
            a_j += 1;
            b_j += 1;
          } else if a_point < b_point {
            a_j += 1;
          } else {
            b_j += 1;
          }
        }

        if n_matches >= 12 {
          let b_offset = point_difference(&a_origin_point, &b_origin_point);

          let mut unioned_points = BTreeSet::new();
          for point in sorted_scanner_a {
            unioned_points.insert(point.clone());
          }

          for point in sorted_scanner_b {
            let adjusted_point = point_sum(&point, &b_offset);
            unioned_points.insert(adjusted_point);
          }

          let mut result = Vec::new();
          for point in unioned_points {
            result.push(point);
          }
          result.sort();

          return Some((result, b_offset));
        }
      }
    }
  }

  None
}


fn point_difference(left: &Point, right: &Point) -> Point {
  Point {
    x: left.x - right.x,
    y: left.y - right.y,
    z: left.z - right.z,
  }
}


fn point_sum(left: &Point, right: &Point) -> Point {
  Point {
    x: left.x + right.x,
    y: left.y + right.y,
    z: left.z + right.z,
  }
}


/* top 0, top 1, top 2, top 3 (front, right 0)
 * top 0, top 1, top 2, top 3 (front 1, right 0)
 * top 0, top 1, top 2, top 3 (front 2, right 0)
 * top 0, top 1, top 2, top 3 (front 3, right 0)
 * top 0, top 1, top 2, top 3 (front 0, right 1)
 * top 0, top 1, top 2, top 3 (front 0, right 3)
 */
fn compute_all_rotations() -> Vec<Rotation> {
  let mut result = Vec::new();

  let all_angles = vec![
    RotationAngle::None,
    RotationAngle::Ninety,
    RotationAngle::OneEighty,
    RotationAngle::TwoSeventy,
  ];

  for front in 0 .. 4 {
    for top in 0 .. 4 {
      result.push(Rotation {
        top: all_angles[top],
        front: all_angles[front],
        right: RotationAngle::None,
      });
    }
  }

  for right in [1, 3].iter() {
    for top in 0 .. 4 {
      result.push(Rotation {
        top: all_angles[top],
        front: RotationAngle::None,
        right: all_angles[*right],
      });
    }
  }

  result
}


/*
 * x is left-right, y is top-bottom, z is front-back.
 * top, front, right have their standard Rubik's cube meanings.
 */
fn apply_rotation(point: &Point, rotation: &Rotation) -> Point {
  let mut result = point.clone();

  match rotation.top {
    RotationAngle::None => { }
    RotationAngle::Ninety => {
      let temp_x = result.x;
      result.x = result.z;
      result.z = -temp_x;
    }
    RotationAngle::OneEighty => {
      result.x = -result.x;
      result.z = -result.z;
    }
    RotationAngle::TwoSeventy => {
      let temp_x = result.x;
      result.x = -result.z;
      result.z = temp_x;
    }
  }

  match rotation.front {
    RotationAngle::None => { }
    RotationAngle::Ninety => {
      let temp_x = result.x;
      result.x = result.y;
      result.y = -temp_x;
    }
    RotationAngle::OneEighty => {
      result.x = -result.x;
      result.y = -result.y;
    }
    RotationAngle::TwoSeventy => {
      let temp_x = result.x;
      result.x = -result.y;
      result.y = temp_x;
    }
  }

  match rotation.right {
    RotationAngle::None => { }
    RotationAngle::Ninety => {
      let temp_z = result.z;
      result.z = result.y;
      result.y = -temp_z;
    }
    RotationAngle::OneEighty => {
      result.z = -result.z;
      result.y = -result.y;
    }
    RotationAngle::TwoSeventy => {
      let temp_z = result.z;
      result.z = -result.y;
      result.y = temp_z;
    }
  }

  result
}


fn compute_distance(point_a: &Point, point_b: &Point) -> i64 {
  (point_a.x - point_b.x).abs()
    + (point_a.y - point_b.y).abs()
    + (point_a.z - point_b.z).abs()
}
