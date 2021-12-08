use advent_lib::prelude::*;

use std::collections::BTreeSet;
use std::collections::BTreeMap;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut desired_digit_count = 0;
  for line in &input {
    let halves: Vec<&str> = line.split(" | ").collect();
    let rhs: Vec<&str> = halves[1].split(" ").collect();
    for word in rhs {
      match word.len() {
        2 => {
          desired_digit_count += 1;
        }
        3 => {
          desired_digit_count += 1;
        }
        4 => {
          desired_digit_count += 1;
        }
        7 => {
          desired_digit_count += 1;
        }
        _ => { }
      }
    }
  }

  println!("{}", desired_digit_count);

  let mut final_sum = 0;
  for line in &input {
    let halves: Vec<&str> = line.split(" | ").collect();
    let lhs: Vec<&str> = halves[0].split(" ").collect();
    let rhs: Vec<&str> = halves[1].split(" ").collect();

    let mut encoded_one = None;
    let mut encoded_four = None;
    let mut encoded_seven = None;
    let mut encoded_eight = None;
    let mut five_segment_encodings = Vec::new();
    let mut six_segment_encodings = Vec::new();
    for word in lhs {
      match word.len() {
        2 => {
          // decodes as 1
          encoded_one = Some(word_to_letter_set(word));
        }
        3 => {
          // decodes as 7
          encoded_seven = Some(word_to_letter_set(word));
        }
        4 => {
          // decodes as 4
          encoded_four = Some(word_to_letter_set(word));
        }
        5 => {
          five_segment_encodings.push(word_to_letter_set(word));
          // decodes as 2, 3, or 5
          // 2, 3, 5 share a, d, g
          // 2, 3 share a, c, d, g; differ on e, f
          // 2, 3 share c; 5 has b instead
          // 3, 5 share f; 2 has e instead
          // be -> 1
          // edb -> 7
          // cgeb -> 4
          // fdcge
          // fecdb
          // fabcd
          // cbdgef
          // fgaecd
          // agebfd
          // cfbegad -> 8
        }
        6 => {
          six_segment_encodings.push(word_to_letter_set(word));
          // decodes as 0, 6, or 9
        }
        7 => {
          // decodes as 8
          encoded_eight = Some(word_to_letter_set(word));
        }
        _ => { }
      }
    }
    let encoded_one = encoded_one.unwrap();
    let encoded_four = encoded_four.unwrap();
    let encoded_seven = encoded_seven.unwrap();
    let encoded_eight = encoded_eight.unwrap();

    let encoded_a: char = encoded_seven.difference(&encoded_one)
        .next().unwrap().clone();

    let mut horizontal_segments: BTreeSet<char> =
        five_segment_encodings[0].clone();
    for encoding in &five_segment_encodings {
      let mut new_intersection = BTreeSet::new();
      for c in horizontal_segments.intersection(encoding) {
        new_intersection.insert(c.clone());
      }
      horizontal_segments = new_intersection;
    }

    let mut encoded_dg = horizontal_segments.clone();
    encoded_dg.remove(&encoded_a);

    let encoded_d = encoded_four.intersection(&encoded_dg)
        .next().unwrap().clone();

    let mut encoded_g = encoded_dg.clone();
    encoded_g.remove(&encoded_d);
    let encoded_g: char = encoded_g.iter().next().unwrap().clone();

    let mut encoded_bf = None;
    for encoding in &six_segment_encodings {
      let mut vertical_segments = BTreeSet::new();
      for c in encoding.difference(&horizontal_segments) {
        vertical_segments.insert(c.clone());
      }

      if vertical_segments.len() == 4 {
        continue;
      }

      match encoded_bf {
        None => {
          encoded_bf = Some(vertical_segments);
        }
        Some(old_vertical_segments) => {
          let mut new_encoded_bf = BTreeSet::new();
          for c in old_vertical_segments.intersection(&vertical_segments) {
            new_encoded_bf.insert(c.clone());
          }
          encoded_bf = Some(new_encoded_bf);
        }
      }
    }
    let encoded_bf = encoded_bf.unwrap();

    let bf_vec: Vec<&char> = encoded_bf.iter().collect();
    let mut bf_zero_count = 0;
    for encoding in &five_segment_encodings {
      for c in encoding.intersection(&encoded_bf) {
        if c == bf_vec[0] {
          bf_zero_count += 1;
        }
      }
    }

    let (encoded_b, encoded_f): (char, char) = if bf_zero_count == 2 {
      (*bf_vec[1], *bf_vec[0])
    } else {
      (*bf_vec[0], *bf_vec[1])
    };

    let mut encoded_c = encoded_four.clone();
    encoded_c.remove(&encoded_b);
    encoded_c.remove(&encoded_d);
    encoded_c.remove(&encoded_f);
    let encoded_c = encoded_c.iter().next().unwrap().clone();

    let mut encoded_e = BTreeSet::new();
    for encoding in &six_segment_encodings {
      for c in encoding {
        encoded_e.insert(c);
      }
    }
    encoded_e.remove(&encoded_a);
    encoded_e.remove(&encoded_b);
    encoded_e.remove(&encoded_c);
    encoded_e.remove(&encoded_d);
    encoded_e.remove(&encoded_f);
    encoded_e.remove(&encoded_g);
    let encoded_e: char = *encoded_e.iter().next().unwrap().clone();

    let mut decoder_ring: BTreeMap<BTreeSet<char>, i64> = BTreeMap::new();
    decoder_ring.insert(encoded_one, 1);
    decoder_ring.insert(encoded_four, 4);
    decoder_ring.insert(encoded_seven, 7);
    decoder_ring.insert(encoded_eight, 8);

    let mut encoded_zero: BTreeSet<char> = BTreeSet::new();
    encoded_zero.insert(encoded_a);
    encoded_zero.insert(encoded_b);
    encoded_zero.insert(encoded_c);
    encoded_zero.insert(encoded_e);
    encoded_zero.insert(encoded_f);
    encoded_zero.insert(encoded_g);
    decoder_ring.insert(encoded_zero, 0);

    let mut encoded_two: BTreeSet<char> = BTreeSet::new();
    encoded_two.insert(encoded_a);
    encoded_two.insert(encoded_c);
    encoded_two.insert(encoded_d);
    encoded_two.insert(encoded_e);
    encoded_two.insert(encoded_g);
    decoder_ring.insert(encoded_two, 2);

    let mut encoded_three: BTreeSet<char> = BTreeSet::new();
    encoded_three.insert(encoded_a);
    encoded_three.insert(encoded_c);
    encoded_three.insert(encoded_d);
    encoded_three.insert(encoded_f);
    encoded_three.insert(encoded_g);
    decoder_ring.insert(encoded_three, 3);

    let mut encoded_five: BTreeSet<char> = BTreeSet::new();
    encoded_five.insert(encoded_a);
    encoded_five.insert(encoded_b);
    encoded_five.insert(encoded_d);
    encoded_five.insert(encoded_f);
    encoded_five.insert(encoded_g);
    decoder_ring.insert(encoded_five, 5);

    let mut encoded_six: BTreeSet<char> = BTreeSet::new();
    encoded_six.insert(encoded_a);
    encoded_six.insert(encoded_b);
    encoded_six.insert(encoded_d);
    encoded_six.insert(encoded_e);
    encoded_six.insert(encoded_f);
    encoded_six.insert(encoded_g);
    decoder_ring.insert(encoded_six, 6);

    let mut encoded_nine: BTreeSet<char> = BTreeSet::new();
    encoded_nine.insert(encoded_a);
    encoded_nine.insert(encoded_b);
    encoded_nine.insert(encoded_c);
    encoded_nine.insert(encoded_d);
    encoded_nine.insert(encoded_f);
    encoded_nine.insert(encoded_g);
    decoder_ring.insert(encoded_nine, 9);

    let mut value = 0;
    for word in &rhs {
      let digit = decode_word(word, &decoder_ring);
      value = value * 10 + digit;
    }
    final_sum += value;
  }
  println!("{}", final_sum);

  Ok(())
}


fn word_to_letter_set(word: &str) -> BTreeSet<char> {
  let mut result = BTreeSet::new();

  for c in word.chars() {
    result.insert(c);
  }

  result
}


fn decode_word(word: &str, decoder_ring: &BTreeMap<BTreeSet<char>,i64>)
  -> i64
{
  let letter_set = word_to_letter_set(word);
  *decoder_ring.get(&letter_set).unwrap()
}

