use advent_lib::prelude::*;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;


#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
enum Amphipod {
  A,
  B,
  C,
  D,
}

#[derive(Debug,Clone,Eq,PartialEq)]
enum Place {
  Room(usize),
  Hallway(usize),
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Parameters {
  room_size: usize,
}

#[derive(Debug,Clone,Eq)]
struct State {
  expense_so_far: usize,
  hallway: Vec<Option<Amphipod>>,
  rooms: Vec<Vec<Amphipod>>,
  last_destination: Option<Place>,
}

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
struct StaticState {
  hallway: Vec<Option<Amphipod>>,
  rooms: Vec<Vec<Amphipod>>,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    Reverse(&self.expense_so_far).cmp(&Reverse(&other.expense_so_far))
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(Reverse(&self.expense_so_far).cmp(&Reverse(&other.expense_so_far)))
  }
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.expense_so_far == other.expense_so_far
  }
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut hallway = Vec::new();
  for _ in 0 .. 11 {
    hallway.push(None);
  }

  let mut rooms = Vec::new();
  for i in 0 .. 4 {
    let mut room = Vec::new();
    let x = 3 + 2 * i;
    for j in 0 .. 2 {
      let y = 3 - j;
      let c = input[y].chars().nth(x).unwrap();
      let amphipod = match c {
        'A' => Amphipod::A,
        'B' => Amphipod::B,
        'C' => Amphipod::C,
        'D' => Amphipod::D,
        _ => { panic!("Unexpected character."); }
      };
      room.push(amphipod);
    }
    rooms.push(room);
  }

  let initial_state = State {
    expense_so_far: 0,
    hallway,
    rooms,
    last_destination: None,
  };

  let expense = find_steps(&Parameters { room_size: 2 }, &initial_state);
  println!("{}", expense);

  let mut harder_rooms = Vec::new();
  for i in 0 .. 4 {
    let mut harder_room = Vec::new();
    harder_room.push(initial_state.rooms[i][0]);
    match i {
      0 => {
        harder_room.push(Amphipod::D);
        harder_room.push(Amphipod::D);
      }
      1 => {
        harder_room.push(Amphipod::B);
        harder_room.push(Amphipod::C);
      }
      2 => {
        harder_room.push(Amphipod::A);
        harder_room.push(Amphipod::B);
      }
      3 => {
        harder_room.push(Amphipod::C);
        harder_room.push(Amphipod::A);
      }
      _ => { }
    }
    harder_room.push(initial_state.rooms[i][1]);
    harder_rooms.push(harder_room);
  }
  let harder_initial_state = State {
    expense_so_far: 0,
    hallway: initial_state.hallway.clone(),
    rooms: harder_rooms,
    last_destination: None,
  };

  let harder_expense = find_steps(&Parameters { room_size: 4 },
    &harder_initial_state);
  println!("{}", harder_expense);

  Ok(())
}


fn find_steps(parameters: &Parameters, initial_state: &State) -> usize {
  let mut exploration_queue = BinaryHeap::new();
  exploration_queue.push(initial_state.clone());

  let mut visited_states = HashMap::new();

  //let mut i = 0;
  loop {
    let state = match exploration_queue.pop() {
      None => { panic!("insoluble"); }
      Some(state) => { state }
    };

    if is_solved(parameters, &state) {
      return state.expense_so_far;
    }

    let static_state = StaticState {
      hallway: state.hallway.clone(),
      rooms: state.rooms.clone(),
    };

    match visited_states.get(&static_state) {
      None => { }
      Some(_) => { continue; }
    }

    visited_states.insert(static_state, state.expense_so_far);

    //i += 1;
    //if (parameters.room_size == 4) && (i % 10000 == 0) {
    //  println!("iteration {}, cost {}", i, state.expense_so_far);
    //  debug_state(parameters, &state);
    //}

    // This is where we would call is_unsolvable() if it worked.

    // All moves that start in a room.
    for start_room in 0 .. 4 {
      if state.rooms[start_room].len() > 0 {
        let start_room_occupancy = state.rooms[start_room].len();
        let amphipod = state.rooms[start_room][start_room_occupancy-1];
        let distance = 1 + parameters.room_size - start_room_occupancy;

        // Don't consider moves that begin at the most recent destination.
        match state.last_destination {
          Some(Place::Room(last_room)) => {
            if start_room == last_room {
              continue;
            }
          }
          _ => { }
        }

        // It only makes sense to move an amphipod out of their destination
        // room if there are amphipods that don't want to be there below
        // them.
        if amphipod_destination_room(&amphipod) == start_room {
          let mut should_skip = true;
          for i in 0 .. start_room_occupancy {
            let occupant = state.rooms[start_room][i];
            if occupant != amphipod {
              should_skip = false;
              break;
            }
          }
          if should_skip {
            continue;
          }
        }

        {
          // All *leftward* moves that start in a room.
          let mut hallway = hallway_left_of_room(start_room);
          let mut distance = distance + 1;

          loop {
            if state.hallway[hallway].is_some() {
              break;
            }

            // At each step along the hallway, enqueue a search state that
            // has the amphipod stopped at that location.
            enqueue_move_to_hallway(&state, &amphipod, Place::Room(start_room),
                hallway, distance, &mut exploration_queue, &visited_states);

            match adjacent_room_left_of_hallway(hallway) {
              None => { },
              Some(end_room) => {
                enqueue_move_to_room(parameters, &state, &amphipod,
                  Place::Room(start_room), end_room, distance,
                  &mut exploration_queue, &visited_states);
              }
            }

            match hallway_left_of_hallway(hallway) {
              None => { break; },
              Some(new_hallway) => {
                distance += hallway - new_hallway;
                hallway = new_hallway;
              }
            }
          }
        }

        {
          // All *rightward* moves that start in a room.
          let mut hallway = hallway_right_of_room(start_room);
          let mut distance = distance + 1;

          loop {
            if state.hallway[hallway].is_some() {
              break;
            }

            // At each step along the hallway, enqueue a search state that
            // has the amphipod stopped at that location.
            enqueue_move_to_hallway(&state, &amphipod, Place::Room(start_room),
                hallway, distance, &mut exploration_queue, &visited_states);

            match adjacent_room_right_of_hallway(hallway) {
              None => { },
              Some(end_room) => {
                enqueue_move_to_room(parameters, &state, &amphipod,
                  Place::Room(start_room), end_room, distance,
                  &mut exploration_queue, &visited_states);
              }
            }

            match hallway_right_of_hallway(hallway) {
              None => { break; },
              Some(new_hallway) => {
                distance += new_hallway - hallway;
                hallway = new_hallway;
              }
            }
          }
        }
      }
    }

    // All moves that start in a hallway.
    for start_hallway in 0 .. 11 {
      // Don't consider moves that begin at the most recent destination.
      match state.last_destination {
        Some(Place::Hallway(last_hallway)) => {
          if start_hallway == last_hallway {
            continue;
          }
        }
        _ => { }
      }

      match state.hallway[start_hallway] {
        None => { }
        Some(amphipod) => {
          let distance = 0;

          {
            // All *leftward* moves that start in a hallway.
            let mut hallway = start_hallway;
            let mut distance = distance;

            loop {
              match adjacent_room_left_of_hallway(hallway) {
                None => { },
                Some(end_room) => {
                  enqueue_move_to_room(parameters, &state, &amphipod,
                    Place::Hallway(start_hallway), end_room, distance,
                    &mut exploration_queue, &visited_states);
                }
              }

              match hallway_left_of_hallway(hallway) {
                None => { break; },
                Some(new_hallway) => {
                  if state.hallway[new_hallway].is_some() {
                    break;
                  }

                  distance += hallway - new_hallway;
                  hallway = new_hallway;
                }
              }
            }
          }

          {
            // All *rightward* moves that start in a hallway.
            let mut hallway = start_hallway;
            let mut distance = distance;

            loop {
              match adjacent_room_right_of_hallway(hallway) {
                None => { },
                Some(end_room) => {
                  enqueue_move_to_room(parameters, &state, &amphipod,
                    Place::Hallway(start_hallway), end_room, distance,
                    &mut exploration_queue, &visited_states);
                }
              }

              match hallway_right_of_hallway(hallway) {
                None => { break; },
                Some(new_hallway) => {
                  if state.hallway[new_hallway].is_some() {
                    break;
                  }

                  distance += new_hallway - hallway;
                  hallway = new_hallway;
                }
              }
            }
          }
        }
      }
    }
  }
}


fn enqueue_move_to_room(parameters: &Parameters, state: &State,
    amphipod: &Amphipod, start: Place, end_room: usize, distance: usize,
    exploration_queue: &mut BinaryHeap<State>,
    visited_states: &HashMap<StaticState,usize>)
{
  if room_is_viable_destination(parameters, state, amphipod, end_room) {
    let end_room_occupancy = state.rooms[end_room].len();
    let mut new_state = state.clone();

    match start {
      Place::Hallway(start_hallway) => {
        new_state.hallway[start_hallway] = None;
      }
      Place::Room(start_room) => {
        let _ = new_state.rooms[start_room].pop();
      }
    }

    new_state.rooms[end_room].push(*amphipod);
    let distance = distance + 1 + parameters.room_size - end_room_occupancy;
    new_state.expense_so_far += amphipod_expense(&amphipod, distance);
    new_state.last_destination = Some(Place::Room(end_room));

    enqueue_state(new_state, exploration_queue, visited_states);
  }
}


fn enqueue_move_to_hallway(state: &State, amphipod: &Amphipod, start: Place,
    end_hallway: usize, distance: usize,
    exploration_queue: &mut BinaryHeap<State>,
    visited_states: &HashMap<StaticState,usize>)
{
  let mut new_state = state.clone();
  match start {
    Place::Room(start_room) => {
      let _ = new_state.rooms[start_room].pop();
    }
    _ => { }
  }
  new_state.hallway[end_hallway] = Some(*amphipod);
  new_state.expense_so_far += amphipod_expense(&amphipod, distance);
  new_state.last_destination = Some(Place::Hallway(end_hallway));

  enqueue_state(new_state, exploration_queue, visited_states);
}


fn enqueue_state(state: State, exploration_queue: &mut BinaryHeap<State>,
    visited_states: &HashMap<StaticState,usize>)
{
  let should_enqueue = match visited_states.get(&StaticState {
    hallway: state.hallway.clone(),
    rooms: state.rooms.clone(),
  }) {
    None => { true }
    Some(already_tried_expense) => {
      state.expense_so_far < *already_tried_expense
    }
  };

  if should_enqueue {
    exploration_queue.push(state);
  }
}


fn room_is_viable_destination(parameters: &Parameters, state: &State,
    amphipod: &Amphipod, end_room: usize)
  -> bool
{
  let end_room_occupancy = state.rooms[end_room].len();

  if end_room_occupancy == parameters.room_size {
    return false;
  }

  if amphipod_destination_room(&amphipod) != end_room {
    return false;
  }

  for existing_occupant in &state.rooms[end_room] {
    if amphipod != existing_occupant {
      return false;
    }
  }

  true
}


fn is_solved(parameters: &Parameters, state: &State) -> bool {
  for hallway in 0 .. 11 {
    if state.hallway[hallway].is_some() {
      return false;
    }
  }

  for room in 0 .. 4 {
    let occupants = &state.rooms[room];
    if occupants.len() != parameters.room_size {
      return false;
    }
    for occupant in occupants {
      if amphipod_destination_room(occupant) != room {
        return false;
      }
    }
  }

  true
}


#[allow(dead_code)]
// This function does not work - it returns true in cases that are still
// solvable. It is retained as a salve to my ego.
fn is_unsolvable(state: &State) -> bool {
  // Consider pieces which are in rooms other than their destinations. There
  // are certain scenarios in which we can prove that these pieces can never
  // reach their destinations.
  for start_room in 0 .. 4 {
    for occupant in &state.rooms[start_room] {
      let end_room = amphipod_destination_room(occupant);

      // Break this into two cases, one in which we search left and an
      // almost-identical one in which we search right.
      if start_room > end_room {
        // First, check whether there is an available stash space to the
        // right. If there is, we can't prove anything useful, so keep
        // searching.
        let stash_hallway = hallway_right_of_room(start_room);
        match state.hallway[stash_hallway] {
          None => {
            continue;
          },
          Some(_) => { },
        }

        // Now that we know there is no stash space, check whether we and an
        // amphipod in the hallway will block each other.
        let mut hallway = hallway_left_of_room(start_room);
        loop {
          match state.hallway[hallway] {
            None => { },
            Some(blocker) => {
              if amphipod_destination_room(&blocker) == start_room {
                return true;
              }
            }
          }

          match adjacent_room_left_of_hallway(hallway) {
            None => { },
            Some(intermediate_room) => {
              if intermediate_room == end_room {
                break;
              }
            }
          }

          match hallway_left_of_hallway(hallway) {
            None => {
              break;
            },
            Some(new_hallway) => {
              hallway = new_hallway;
            }
          }
        }
      } else if start_room < end_room {
        // First, check whether there is an available stash space to the
        // left. If there is, we can't prove anything useful, so keep
        // searching.
        let stash_hallway = hallway_left_of_room(start_room);
        match state.hallway[stash_hallway] {
          None => {
            continue;
          },
          Some(_) => { },
        }

        // Now that we know there is no stash space, check whether we and an
        // amphipod in the hallway will block each other.
        let mut hallway = hallway_right_of_room(start_room);
        loop {
          match state.hallway[hallway] {
            None => { },
            Some(blocker) => {
              if amphipod_destination_room(&blocker) == start_room {
                return true;
              }
            }
          }

          match adjacent_room_right_of_hallway(hallway) {
            None => { }
            Some(intermediate_room) => {
              if intermediate_room == end_room {
                break;
              }
            }
          }

          match hallway_right_of_hallway(hallway) {
            None => {
              break;
            },
            Some(new_hallway) => {
              hallway = new_hallway;
            }
          }
        }
      }
    }
  }

  // Consider pieces which are in the hallway. There are certain cases in
  // which these pieces can never reach their destinations.
  for start_hallway in 0 .. 11 {
    match state.hallway[start_hallway] {
      None => { }
      Some(amphipod) => {
        let end_room = amphipod_destination_room(&amphipod);

        if is_room_left_of_hallway(end_room, start_hallway) {
          // This amphipod must move left to reach its destination.
          let mut hallway = start_hallway;

          loop {
            match adjacent_room_left_of_hallway(hallway) {
              None => { },
              Some(new_room) => {
                if new_room == end_room {
                  break;
                }
              }
            }

            match hallway_left_of_hallway(hallway) {
              None => {
                break;
              }
              Some(new_hallway) => {
                match state.hallway[new_hallway] {
                  None => { }
                  Some(blocker) => {
                    // If the blocker needs to go somewhere that's on the
                    // far side of the start point, the configuration is
                    // unsolvable.
                    let blocker_destination =
                      amphipod_destination_room(&blocker);
                    if is_room_right_of_hallway(
                        blocker_destination, start_hallway)
                    {
                      return true;
                    }
                  }
                }

                hallway = new_hallway;
              }
            }
          }
        } else {
          // This amphipod must move right to reach its destination.
          let mut hallway = start_hallway;

          loop {
            match adjacent_room_right_of_hallway(hallway) {
              None => { },
              Some(new_room) => {
                if new_room == end_room {
                  break;
                }
              }
            }

            match hallway_right_of_hallway(hallway) {
              None => {
                break;
              }
              Some(new_hallway) => {
                match state.hallway[new_hallway] {
                  None => { }
                  Some(blocker) => {
                    // If the blocker needs to go somewhere that's on the
                    // far side of the start point, the configuration is
                    // unsolvable.
                    let blocker_destination =
                      amphipod_destination_room(&blocker);
                    if is_room_left_of_hallway(
                        blocker_destination, start_hallway)
                    {
                      return true;
                    }
                  }
                }

                hallway = new_hallway;
              }
            }
          }
        }
      }
    }
  }

  false
}


fn amphipod_expense(amphipod: &Amphipod, distance: usize) -> usize {
  distance * match amphipod {
    Amphipod::A => 1,
    Amphipod::B => 10,
    Amphipod::C => 100,
    Amphipod::D => 1000,
  }
}


fn hallway_left_of_room(room_index: usize) -> usize {
  1 + room_index * 2
}

fn hallway_right_of_room(room_index: usize) -> usize {
  3 + room_index * 2
}

fn hallway_left_of_hallway(hallway_index: usize) -> Option<usize> {
  if hallway_index == 0 {
    None
  } else if hallway_index == 1 {
    Some(0)
  } else if hallway_index == 3 {
    Some(1)
  } else if hallway_index == 5 {
    Some(3)
  } else if hallway_index == 7 {
    Some(5)
  } else if hallway_index == 9 {
    Some(7)
  } else if hallway_index == 10 {
    Some(9)
  } else {
    panic!("Invalid hallway index {}", hallway_index);
  }
}

fn hallway_right_of_hallway(hallway_index: usize) -> Option<usize> {
  if hallway_index == 0 {
    Some(1)
  } else if hallway_index == 1 {
    Some(3)
  } else if hallway_index == 3 {
    Some(5)
  } else if hallway_index == 5 {
    Some(7)
  } else if hallway_index == 7 {
    Some(9)
  } else if hallway_index == 9 {
    Some(10)
  } else if hallway_index == 10 {
    None
  } else {
    panic!("Invalid hallway index {}", hallway_index);
  }
}

fn adjacent_room_right_of_hallway(hallway_index: usize) -> Option<usize> {
  if hallway_index == 1 {
    Some(0)
  } else if hallway_index == 3 {
    Some(1)
  } else if hallway_index == 5 {
    Some(2)
  } else if hallway_index == 7 {
    Some(3)
  } else {
    None
  }
}

fn adjacent_room_left_of_hallway(hallway_index: usize) -> Option<usize> {
  if hallway_index == 9 {
    Some(3)
  } else if hallway_index == 7 {
    Some(2)
  } else if hallway_index == 5 {
    Some(1)
  } else if hallway_index == 3 {
    Some(0)
  } else {
    None
  }
}

fn is_room_left_of_hallway(room_index: usize, hallway_index: usize) -> bool {
  hallway_right_of_room(room_index) <= hallway_index
}

fn is_room_right_of_hallway(room_index: usize, hallway_index: usize) -> bool {
  hallway_left_of_room(room_index) >= hallway_index
}


fn amphipod_destination_room(amphipod: &Amphipod) -> usize {
  match amphipod {
    Amphipod::A => 0,
    Amphipod::B => 1,
    Amphipod::C => 2,
    Amphipod::D => 3,
  }
}


#[allow(dead_code)]
fn debug_state(parameters: &Parameters, state: &State) {
  println!("#############");

  print!("#");
  for hallway in 0 .. 11 {
    match state.hallway[hallway] {
      None => { print!("."); }
      Some(amphipod) => { debug_amphipod(&amphipod); }
    }
  }
  println!("#");

  print!("###");
  for room in 0 .. 4 {
    if state.rooms[room].len() == parameters.room_size {
      debug_amphipod(&state.rooms[room][parameters.room_size - 1]);
    } else {
      print!(".");
    }
    print!("#");
  }
  println!("##");

  for i in 0 .. parameters.room_size - 1 {
    print!("  #");
    for room in 0 .. 4 {
      let index = parameters.room_size - 2 - i;
      if state.rooms[room].len() >= index + 1 {
        debug_amphipod(&state.rooms[room][index]);
      } else {
        print!(".");
      }
      print!("#");
    }
    println!("");
  }

  println!("  #########");

  println!("");
}


#[allow(dead_code)]
fn debug_amphipod(amphipod: &Amphipod) {
  match amphipod {
    Amphipod::A => { print!("A"); }
    Amphipod::B => { print!("B"); }
    Amphipod::C => { print!("C"); }
    Amphipod::D => { print!("D"); }
  }
}

