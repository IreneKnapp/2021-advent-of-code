use advent_lib::prelude::*;

use std::collections::HashMap;


#[derive(Debug, Clone)]
struct GameState {
  next_roll: i64,
  roll_odometer: i64,
  next_player_index: i64,
  players: Vec<PlayerState>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct NondeterministicState {
  next_player_index: i64,
  players: Vec<PlayerState>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PlayerState {
  position: i64,
  score: i64,
}

#[derive(Debug, Clone)]
struct GameParameters {
  die_size: i64,
  required_score: i64,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut players = Vec::new();
  for line in &input {
    let words: Vec<&str> = line.split_whitespace().collect();
    let position = words[4].parse::<i64>().unwrap();
    players.push(PlayerState {
      position: position,
      score: 0,
    });
  }
  let initial_state = GameState {
    next_roll: 1,
    roll_odometer: 0,
    next_player_index: 0,
    players,
  };

  let mut state = initial_state.clone();
  let parameters = GameParameters {
    die_size: 100,
    required_score: 1000,
  };
  loop {
    iterate(&mut state, &parameters);
    if is_done(&state, &parameters) {
      break;
    }
  }

  let result = losing_player(&state, &parameters).score * state.roll_odometer;
  println!("{}", result);

  let parameters = GameParameters {
    die_size: 3,
    required_score: 21,
  };
  let mut all_states = HashMap::new();
  all_states.insert(NondeterministicState {
    next_player_index: initial_state.next_player_index,
    players: initial_state.players.clone(),
  }, 1);
  let mut victories = vec![0, 0];
  loop {
    if all_states.len() == 0 {
      break;
    }
    //println!("{:?}", all_states);

    let (new_all_states, new_victories) = iterate_nondeterministic(
      &all_states, &parameters);
    for (i, n) in new_victories.iter().enumerate() {
      victories[i] += n;
    }
    all_states = new_all_states;
  }

  let mut max_victories = victories[0];
  if victories[1] > max_victories {
    max_victories = victories[1];
  }
  println!("{}", max_victories);

  Ok(())
}


fn iterate(state: &mut GameState, parameters: &GameParameters) {
  let player = &mut state.players[state.next_player_index as usize];

  let mut roll = 0;
  for _ in 0 .. 3 {
    roll += state.next_roll;
    state.next_roll = (state.next_roll % parameters.die_size) + 1;

    state.roll_odometer += 1;
  }

  player.position = (player.position + roll - 1) % 10 + 1;
  player.score += player.position;

  state.next_player_index = (state.next_player_index + 1) % 2;
}


fn is_done(state: &GameState, parameters: &GameParameters) -> bool {
  for player in &state.players {
    if player.score >= parameters.required_score {
      return true;
    }
  }

  return false;
}


fn losing_player<'a>(state: &'a GameState, parameters: &GameParameters)
  -> &'a PlayerState
{
  for player in &state.players {
    if player.score < parameters.required_score {
      return player;
    }
  }

  panic!("everybody wins");
}


fn iterate_nondeterministic(all_states: &HashMap<NondeterministicState,i64>,
    parameters: &GameParameters)
  -> (HashMap<NondeterministicState,i64>, Vec<i64>)
{
  let mut new_all_states = HashMap::new();
  let mut victory_counts = vec![0, 0];

  for (state, count_in) in all_states.iter() {
    for (roll, count_here) in [
      (3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].iter()
    {
      let mut new_state = state.clone();
      let player_index = new_state.next_player_index as usize;

      new_state.players[player_index].position =
        (new_state.players[player_index].position + roll - 1) % 10 + 1;
      new_state.players[player_index].score +=
        new_state.players[player_index].position;

      new_state.next_player_index = (new_state.next_player_index + 1) % 2;

      let new_score = new_state.players[player_index].score;

      let unrelated_count = match new_all_states.get(&new_state) {
        None => 0,
        Some(n) => *n,
      };

      let count_out = unrelated_count + count_in*count_here;

      if new_score >= parameters.required_score {
        //println!("logging victory for {} x{}", player_index, count_out);
        victory_counts[player_index] += count_out;
      } else {
        //println!("logging x{} {:?}", count_out, new_state);
        new_all_states.insert(new_state.clone(), count_out);
      }
    }
  }

  (new_all_states, victory_counts)
}

