use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::group_lines_by_blanks(
    advent_lib::read_lines_file(&filename)?);

  let mut numbers_called = Vec::new();
  for word in input[0][0].split(',') {
    numbers_called.push(word.parse::<i64>()?);
  }

  let mut boards = Vec::new();
  for board_lines in &input[1..] {
    let mut board = Vec::new();
    for line in board_lines {
      let mut board_row = Vec::new();
      for word in line.split_whitespace() {
        board_row.push(word.parse::<i64>()?);
      }
      board.push(board_row);
    }
    boards.push(board);
  }

  {
    let mut final_score: Option<i64> = None;
    for i in 0 .. numbers_called.len() {
      let winning_numbers = &numbers_called[0 .. i];
      for board in &boards {
        if is_winner(&board, &winning_numbers) {
          final_score = Some(score(&board, &winning_numbers));
          break;
        }
      }

      match final_score {
        Some(_) => { break; }
        _ => { }
      }
    }

    println!("{}", final_score.unwrap());
  }

  {
    let mut final_score: Option<i64> = None;
    for i in 0 .. numbers_called.len() {
      let mut n_winners = 0;
      let mut last_non_winner = None;

      let winning_numbers = &numbers_called[0 .. i];
      for board in &boards {
        if is_winner(&board, &winning_numbers) {
          n_winners += 1;
        } else {
          last_non_winner = Some(board);
        }
      }

      if n_winners == boards.len() - 1 {
        let final_board = last_non_winner.unwrap();
        for j in i .. numbers_called.len() {
          let winning_numbers = &numbers_called[0 .. j];
          if is_winner(final_board, &winning_numbers) {
            final_score = Some(score(final_board, &winning_numbers));
            break;
          }
        }
        break;
      }
    }

    println!("{}", final_score.unwrap());
  }

  Ok(())
}


fn is_winner(board: &Vec<Vec<i64>>, winning_numbers: &[i64]) -> bool {
  for y in 0 .. 5 {
    let mut n_winners = 0;
    for x in 0 .. 5 {
      if winning_numbers.contains(&board[y][x]) {
        n_winners += 1;
      } else {
        break;
      }
    }

    if n_winners == 5 {
      return true;
    }
  }

  for x in 0 .. 5 {
    let mut n_winners = 0;
    for y in 0 .. 5 {
      if winning_numbers.contains(&board[y][x]) {
        n_winners += 1;
      } else {
        break;
      }
    }

    if n_winners == 5 {
      return true;
    }
  }

  return false;
}


fn score(board: &Vec<Vec<i64>>, winning_numbers: &[i64]) -> i64 {
  let mut sum = 0;

  for x in 0 .. 5 {
    for y in 0 .. 5 {
      if !winning_numbers.contains(&board[y][x]) {
        sum += board[y][x];
      }
    }
  }

  return sum * winning_numbers[winning_numbers.len() - 1];
}

