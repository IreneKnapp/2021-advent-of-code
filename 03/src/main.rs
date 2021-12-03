use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  {
    let mut frequencies = Vec::new();

    for _ in 0 .. input[0].len() {
      frequencies.push(0);
    }

    for line in &input {
      let mut i = 0;
      for c in line.chars() {
        match c {
          '1' => {
            frequencies[i] += 1;
          },
          _ => { }
        }

        i += 1;
      }
    }

    let n_lines = input.len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0 .. frequencies.len() {
      let bit_value = 1 << frequencies.len() - i - 1;
      if frequencies[i] > n_lines / 2 {
        gamma += bit_value;
      } else {
        epsilon += bit_value;
      }
    }

    println!("{}", gamma * epsilon);
  }

  {
    let n_bits = input[0].len();

    let generator_line = filter(&input, 0, true);
    let mut generator = 0;

    for i in 0 .. n_bits {
      let bit_value = 1 << n_bits - i - 1;
      if generator_line.chars().nth(i).unwrap() == '1' {
        generator += bit_value;
      }
    }

    let scrubber_line = filter(&input, 0, false);
    let mut scrubber = 0;

    for i in 0 .. n_bits {
      let bit_value = 1 << n_bits - i - 1;
      if scrubber_line.chars().nth(i).unwrap() == '1' {
        scrubber += bit_value;
      }
    }

    println!("{}", generator * scrubber);
  }

  Ok(())
}


fn filter(input: &Vec<String>, bit_offset: usize, want_most: bool) -> String {
  if input.len() == 1 {
    return input[0].clone();
  }

  let mut frequency = 0;

  for line in input {
    match line.chars().nth(bit_offset).unwrap() {
      '1' => {
        frequency += 1;
      },
      _ => { }
    }
  }

  let is_tied = frequency * 2 == input.len();
  let one_is_most = frequency > input.len() / 2;

  let mut new_input = Vec::new();
  for line in input {
    let is_one = line.chars().nth(bit_offset).unwrap() == '1';
    let is_wanted = if is_tied {
      if want_most {
        is_one
      } else {
        !is_one
      }
    } else {
      if want_most {
        if one_is_most {
          is_one
        } else {
          !is_one
        }
      } else {
        if one_is_most {
          !is_one
        } else {
          is_one
        }
      }
    };
    if is_wanted {
      new_input.push(line.clone());
    }
  }

  return filter(&new_input, bit_offset + 1, want_most);
}

