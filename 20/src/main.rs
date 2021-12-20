use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut iterator = input.iter();

  let mut dictionary = Vec::new();
  for c in iterator.next().unwrap().chars() {
    dictionary.push(match c {
      '#' => 1,
      '.' => 0,
      _ => panic!("unexpected character"),
    });
  }

  let _ = iterator.next();

  let mut image = Vec::new();
  for line in iterator {
    let mut row = Vec::new();
    for c in line.chars() {
      row.push(match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("unexpected character"),
      });
    }
    image.push(row);
  }

  let mut background = 0;
  for _ in 0 .. 2 {
    let (new_image, new_background) =
      iterate(&image, background, &dictionary);
    image = new_image;
    background = new_background;
  }
  println!("{}", count_pixels(&image));

  for _ in 2 .. 50 {
    let (new_image, new_background) =
      iterate(&image, background, &dictionary);
    image = new_image;
    background = new_background;
  }
  println!("{}", count_pixels(&image));

  Ok(())
}


fn iterate(input: &Vec<Vec<u8>>, background: u8, dictionary: &Vec<u8>)
  -> (Vec<Vec<u8>>, u8)
{
  let height = input.len() as i64;
  let width = input[0].len() as i64;

  let mut output = Vec::new();
  for y in -2 .. height+2 {
    let mut row = Vec::new();
    for x in -2 .. width+2 {
      let mut index: usize = 0;

      for offset_y in -1 .. 2 {
        for offset_x in -1 .. 2 {
          let computed_y: i64 = y + offset_y;
          let computed_x: i64 = x + offset_x;


          index *= 2;
          if (computed_y >= 0) && (computed_y < height)
            && (computed_x >= 0) && (computed_x < width)
          {
            index += input[computed_y as usize][computed_x as usize] as usize;
          } else {
            index += background as usize;
          }
        }
      }

      row.push(dictionary[index]);
    }
    output.push(row);
  }

  let output_background = dictionary[background as usize];

  (output, output_background)
}


fn count_pixels(image: &Vec<Vec<u8>>) -> i64 {
  let mut count = 0;

  for row in image {
    for cell in row {
      count += *cell as i64;
    }
  }

  count
}


#[allow(dead_code)]
fn debug_image(image: &Vec<Vec<u8>>) {
  for row in image {
    for cell in row {
      if *cell == 1 {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
  println!("");
}

