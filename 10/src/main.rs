use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut total_score = 0;
  let mut incomplete_lines = Vec::new();
  for line in &input {
    let mut chars = line.chars().peekable();
    let score = parse(&mut chars);
    total_score += score;

    if score == 0 {
      incomplete_lines.push(line.clone());
    }
  }
  println!("{}", total_score);

  let mut completion_scores = Vec::new();
  for line in &incomplete_lines {
    let mut chars = line.chars().peekable();
    let completion = repair(&mut chars, true).unwrap();
    let mut score: i64 = 0;
    for c in completion.chars() {
      score *= 5;
      score += match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
      }
    }
    completion_scores.push(score);
  }
  completion_scores.sort();
  let second_score = completion_scores[completion_scores.len() / 2];
  println!("{}", second_score);

  Ok(())
}


fn parse(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> i64 {
  match chars.next() {
    None => {
      return 0;
    }
    Some(open) => {
      let open = open.clone();
      loop {
        match chars.peek() {
          None => {
            return 0;
          }
          Some(c) => {
            let c = *c;
            if "([{<".contains(|d| c == d) {
              let score = parse(chars);
              if score != 0 {
                return score;
              }
            } else {
              let _ = chars.next();
              match (open, c) {
                ('(', ')') => { return 0; }
                ('[', ']') => { return 0; }
                ('{', '}') => { return 0; }
                ('<', '>') => { return 0; }
                (_, ')') => { return 3; }
                (_, ']') => { return 57; }
                (_, '}') => { return 1197; }
                (_, '>') => { return 25137; }
                _ => { }
              }
            }
          }
        }
      }
    }
  }
}


fn repair(chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    is_top_level: bool)
  -> Option<String>
{
  loop {
    match chars.next() {
      None => {
        return Some(String::new());
      }
      Some(open) => {
        let open = open.clone();
        loop {
          match chars.peek() {
            None => {
              let mut result = String::new();
              result.push(match open {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("alas"),
              });
              return Some(result);
            }
            Some(c) => {
              let c = *c;
              if "([{<".contains(|d| c == d) {
                match repair(chars, false) {
                  None => { }
                  Some(mut result) => {
                    result.push(match open {
                      '(' => ')',
                      '[' => ']',
                      '{' => '}',
                      '<' => '>',
                      _ => panic!("alas"),
                    });
                    return Some(result);
                  }
                }
              } else {
                let _ = chars.next();
                if is_top_level {
                  break;
                } else {
                  return None;
                }
              }
            }
          }
        }
      }
    }
  }
}

