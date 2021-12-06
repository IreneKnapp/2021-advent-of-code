use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut time_buckets = Vec::new();
  for word in input[0].split(',') {
    let time = word.parse::<i64>()?;
    while time_buckets.len() <= time as usize {
      time_buckets.push(0);
    }
    time_buckets[time as usize] += 1;
  }

  for _ in 0 .. 80 {
    time_buckets = iterate(&time_buckets);
  }

  println!("{}", population_size(&time_buckets));

  for _ in 80 .. 256 {
    time_buckets = iterate(&time_buckets);
  }

  println!("{}", population_size(&time_buckets));

  Ok(())
}


fn iterate(old_time_buckets: &Vec<i64>) -> Vec<i64> {
  let mut new_time_buckets = Vec::new();

  while new_time_buckets.len() <= 8 {
    new_time_buckets.push(0);
  }

  for (i, n) in old_time_buckets.iter().enumerate() {
    if i == 0 {
      new_time_buckets[6] += n;
      new_time_buckets[8] += n;
    } else {
      new_time_buckets[i - 1] += n;
    }
  }

  new_time_buckets
}


fn population_size(time_buckets: &Vec<i64>) -> i64 {
  let mut result = 0;

  for n in time_buckets {
    result += n;
  }

  result
}

