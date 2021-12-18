use advent_lib::prelude::*;

use crate::types::{Pair, Value};
#[macro_use] extern crate lalrpop_util;

pub mod types;
lalrpop_mod!(pub expression);


#[derive(Debug, Clone)]
enum Step {
  Left,
  Right,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let parser = expression::PairParser::new();

  let mut result = None;
  for line in &input {
    let expression = Box::new(parser.parse(line)?);

    match result {
      None => {
        result = Some(expression);
      }
      Some(previous_result) => {
        let mut new_expression = add(previous_result, expression);
        reduce(&mut new_expression);
        result = Some(new_expression);
      }
    }
  }

  let output = magnitude(&result.unwrap());
  println!("{}", output);

  let mut expressions = Vec::new();
  for line in &input {
    expressions.push(Box::new(parser.parse(line)?));
  }

  let mut max_sum = None;
  for i in 0 .. expressions.len() - 2 {
    for j in i + 1 .. expressions.len() - 1 {
      let left = expressions[i].clone();
      let right = expressions[j].clone();
      let mut sum = add(left, right);
      reduce(&mut sum);
      let sum = magnitude(&sum);
      match max_sum {
        None => {
          max_sum = Some(sum);
        }
        Some(old_sum) => {
          if sum > old_sum {
            max_sum = Some(sum);
          }
        }
      }
    }
  }

  println!("{}", max_sum.unwrap());

  Ok(())
}


fn add(left: Box<Pair>, right: Box<Pair>) -> Box<Pair> {
  Box::new(Pair {
    left: Value::Pair(left),
    right: Value::Pair(right),
  })
}


fn reduce(expression: &mut Box<Pair>) {
  loop {
    match find_four_x_nested(&expression, Vec::new(), 0) {
      None => { }
      Some((path, left_addend, right_addend)) => {
        match subtree_to_left(&path) {
          None => { }
          Some(left_path) => {
            let subtree = get_subtree(expression, &left_path);
            add_to_rightmost_value(subtree, left_addend);
          }
        }

        match subtree_to_right(&path) {
          None => { }
          Some(right_path) => {
            let subtree = get_subtree(expression, &right_path);
            add_to_leftmost_value(subtree, right_addend);
          }
        }

        replace_with_zero(expression, &path);

        continue;
      }
    }

    if perform_split(expression) {
      continue;
    }

    break;
  }
}


fn find_four_x_nested(expression: &Pair, path_so_far: Vec<Step>, depth: usize)
  -> Option<(Vec<Step>, i64, i64)>
{
  if depth >= 4 {
    match (&expression.left, &expression.right) {
      (Value::Literal(left), Value::Literal(right)) => {
        return Some((path_so_far, *left, *right))
      }
      //_ => { panic!("you don't say"); }
      _ => { return None; }
    }
  }

  match &expression.left {
    Value::Literal(_) => { }
    Value::Pair(left) => {
      let mut path = path_so_far.clone();
      path.push(Step::Left);
      let result = find_four_x_nested(left.as_ref(), path, depth + 1);
      if result.is_some() {
        return result;
      }
    }
  }

  match &expression.right {
    Value::Literal(_) => { }
    Value::Pair(right) => {
      let mut path = path_so_far.clone();
      path.push(Step::Right);
      let result = find_four_x_nested(right.as_ref(), path, depth + 1);
      if result.is_some() {
        return result;
      }
    }
  }

  None
}


fn subtree_to_left(path: &Vec<Step>) -> Option<Vec<Step>> {
  let mut result = path.clone();

  loop {
    match result[result.len() - 1] {
      Step::Left => {
        result.pop();

        if result.len() == 0 {
          return None;
        }
      }
      Step::Right => {
        result.pop();
        result.push(Step::Left);
        return Some(result);
      }
    }
  }
}


fn subtree_to_right(path: &Vec<Step>) -> Option<Vec<Step>> {
  let mut result = path.clone();

  loop {
    match result[result.len() - 1] {
      Step::Left => {
        result.pop();
        result.push(Step::Right);
        return Some(result);
      }
      Step::Right => {
        result.pop();

        if result.len() == 0 {
          return None;
        }
      }
    }
  }
}


fn get_subtree<'a>(pair: &'a mut Box<Pair>, path: &Vec<Step>) -> &'a mut Value {
  let mut here: &mut Box<Pair> = pair;

  for (i, step) in path.iter().enumerate() {
    if i == path.len() - 1 {
      match step {
        Step::Left => {
          return &mut here.left;
        }
        Step::Right => {
          return &mut here.right;
        }
      }
    } else {
      match step {
        Step::Left => {
          match &mut here.left {
            Value::Pair(left) => {
              here = left;
            }
            _ => { panic!("left is not a pair"); }
          }
        }
        Step::Right => {
          match &mut here.right {
            Value::Pair(right) => {
              here = right;
            }
            _ => { panic!("right is not a pair"); }
          }
        }
      }
    }
  }

  panic!("empty path");
}


fn add_to_rightmost_value(subtree: &mut Value, addend: i64) {
  match subtree {
    Value::Literal(value) => {
      *value += addend;
    }
    Value::Pair(pair) => {
      add_to_rightmost_value(&mut pair.right, addend);
    }
  }
}


fn add_to_leftmost_value(subtree: &mut Value, addend: i64) {
  match subtree {
    Value::Literal(value) => {
      *value += addend;
    }
    Value::Pair(pair) => {
      add_to_leftmost_value(&mut pair.left, addend);
    }
  }
}


fn replace_with_zero(pair: &mut Box<Pair>, path: &Vec<Step>) {
  let mut here: &mut Box<Pair> = pair;

  for (i, step) in path.iter().enumerate() {
    if i == path.len() - 1 {
      match step {
        Step::Left => {
          here.left = Value::Literal(0);
        }
        Step::Right => {
          here.right = Value::Literal(0);
        }
      }
    } else {
      match step {
        Step::Left => {
          match &mut here.left {
            Value::Pair(left) => {
              here = left;
            }
            _ => { panic!("left is not a pair"); }
          }
        }
        Step::Right => {
          match &mut here.right {
            Value::Pair(right) => {
              here = right;
            }
            _ => { panic!("right is not a pair"); }
          }
        }
      }
    }
  }
}


fn perform_split(expression: &mut Box<Pair>) -> bool {
  match &mut expression.left {
    Value::Literal(n) => {
      if *n >= 10 {
        expression.left = Value::Pair(Box::new(Pair {
          left: Value::Literal((*n as f64 / 2.0).floor() as i64),
          right: Value::Literal((*n as f64 / 2.0).ceil() as i64),
        }));
        return true;
      }
    }
    Value::Pair(left) => {
      if perform_split(left) {
        return true;
      }
    }
  }

  match &mut expression.right {
    Value::Literal(n) => {
      if *n >= 10 {
        expression.right = Value::Pair(Box::new(Pair {
          left: Value::Literal((*n as f64 / 2.0).floor() as i64),
          right: Value::Literal((*n as f64 / 2.0).ceil() as i64),
        }));
        return true;
      }
    }
    Value::Pair(right) => {
      if perform_split(right) {
        return true;
      }
    }
  }

  false
}


fn magnitude(expression: &Box<Pair>) -> i64 {
  let mut result = 0;

  result += 3 * match &expression.left {
    Value::Literal(n) => *n,
    Value::Pair(left) => magnitude(left),
  };

  result += 2 * match &expression.right {
    Value::Literal(n) => *n,
    Value::Pair(right) => magnitude(right),
  };

  result
}

