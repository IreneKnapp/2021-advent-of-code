use advent_lib::prelude::*;

use std::collections::HashMap;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::group_lines_by_blanks(
    advent_lib::read_lines_file(&filename)?);

  let template = input[0][0].clone();

  let mut rules: HashMap<(char, char),char> = HashMap::new();
  for line in &input[1]{
    let words: Vec<&str> = line.split_whitespace().collect();
    let lhs: Vec<char> = words[0].chars().collect();
    let rhs: Vec<char> = words[2].chars().collect();
    let left = lhs[0];
    let right = lhs[1];
    let inserted = rhs[0];
    rules.insert((left, right), inserted);
  }

  let mut pairized = pairize(&template);
  for _ in 0 .. 10 {
    pairized = pairized_iterate(&pairized, &rules);
  }
  let answer = pairized_score(&template, &pairized);
  println!("{}", answer);

  for _ in 10 .. 40 {
    pairized = pairized_iterate(&pairized, &rules);
  }
  let answer = pairized_score(&template, &pairized);
  println!("{}", answer);

  Ok(())
}


#[allow(dead_code)]
fn iterate(template: &str, rules: &HashMap<(char, char), char>) -> String {
  let mut result = String::new();
  let length = template.len();

  for i in 0 .. length - 1 {
    let left = template.chars().nth(i).unwrap();
    let right = template.chars().nth(i + 1).unwrap();
    let inserted = rules.get(&(left, right));

    result.push(left);

    match inserted {
      None => { }
      Some(c) => {
        result.push(*c);
      }
    }

    if i == length - 2 {
      result.push(right);
    }
  }

  result
}


#[allow(dead_code)]
fn score(template: &str) -> i64 {
  let mut buckets = HashMap::new();

  for c in template.chars() {
    let value = buckets.get(&c);
    match value {
      None => {
        buckets.insert(c, 1);
      }
      Some(total) => {
        let new_total = total + 1;
        buckets.insert(c, new_total);
      }
    }
  }

  let mut min_value = None;
  let mut max_value = None;
  for (_, value) in &buckets {
    match min_value {
      None => {
        min_value = Some(value);
      }
      Some(old_value) => {
        if value < old_value {
          min_value = Some(value);
        }
      }
    }

    match max_value {
      None => {
        max_value = Some(value);
      }
      Some(old_value) => {
        if value > old_value {
          max_value = Some(value);
        }
      }
    }
  }

  max_value.unwrap() - min_value.unwrap()
}


fn pairize(template: &str) -> HashMap<(char, char), usize> {
  let mut result: HashMap<(char, char), usize> = HashMap::new();

  for i in 0 .. template.len() - 1 {
    let left = template.chars().nth(i).unwrap();
    let right = template.chars().nth(i + 1).unwrap();
    let key = (left, right);

    match result.get(&key) {
      None => {
        result.insert(key, 1);
      }
      Some(count) => {
        let new_count = count + 1;
        result.insert(key, new_count);
      }
    }
  }

  result
}


fn pairized_iterate(
    pairized: &HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>)
  -> HashMap<(char, char), usize>
{
  let mut result: HashMap<(char, char), usize> = HashMap::new();

  for (key, value) in pairized {
    match rules.get(&key) {
      None => {
        result.insert(key.clone(), *value);
      }
      Some(inserted) => {
        let (left, right) = key;
        let left_key = (*left, *inserted);
        let right_key = (*inserted, *right);

        match result.get(&left_key) {
          None => {
            result.insert(left_key, *value);
          }
          Some(old_value) => {
            let new_value = old_value + value;
            result.insert(left_key, new_value);
          }
        }

        match result.get(&right_key) {
          None => {
            result.insert(right_key, *value);
          }
          Some(old_value) => {
            let new_value = old_value + value;
            result.insert(right_key, new_value);
          }
        }
      }
    }
  }

  result
}


fn pairized_score(original: &str, pairized: &HashMap<(char, char), usize>)
  -> usize
{
  let mut buckets = HashMap::new();

  let leftmost = original.chars().nth(0).unwrap();
  buckets.insert(leftmost, 1);

  for ((_, c), value) in pairized {
    match buckets.get(&c) {
      None => {
        buckets.insert(*c, *value);
      }
      Some(total) => {
        let new_total = total + value;
        buckets.insert(*c, new_total);
      }
    }
  }

  let mut min_value = None;
  let mut max_value = None;
  for (_, value) in &buckets {
    match min_value {
      None => {
        min_value = Some(value);
      }
      Some(old_value) => {
        if value < old_value {
          min_value = Some(value);
        }
      }
    }

    match max_value {
      None => {
        max_value = Some(value);
      }
      Some(old_value) => {
        if value > old_value {
          max_value = Some(value);
        }
      }
    }
  }

  max_value.unwrap() - min_value.unwrap()
}

