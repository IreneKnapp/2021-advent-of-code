use advent_lib::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Cave {
  Start,
  End,
  Small(String),
  Large(String),
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut connections: HashMap<Cave,Vec<Cave>> = HashMap::new();
  for line in &input {
    let words: Vec<&str> = line.split('-').collect();
    let a = parse_cave(words[0]);
    let b = parse_cave(words[1]);

    match connections.get_mut(&a) {
      None => {
        let mut destinations = Vec::new();
        destinations.push(b.clone());
        connections.insert(a.clone(), destinations);
      }
      Some(destinations) => {
        destinations.push(b.clone());
      }
    }

    match connections.get_mut(&b) {
      None => {
        let mut destinations = Vec::new();
        destinations.push(a.clone());
        connections.insert(b.clone(), destinations);
      }
      Some(destinations) => {
        destinations.push(a.clone());
      }
    }
  }

  let path_count = tour_caves(Cave::Start, &connections, HashSet::new());
  println!("{}", path_count);

  let longer_path_count = longer_tour_caves(Cave::Start, &connections,
    HashMap::new(), false);
  println!("{}", longer_path_count);

  Ok(())
}


fn parse_cave(name: &str) -> Cave {
  if name == "start" {
    Cave::Start
  } else if name == "end" {
    Cave::End
  } else if name.chars().nth(0).unwrap().is_uppercase() {
    Cave::Large(name.to_string())
  } else {
    Cave::Small(name.to_string())
  }
}


fn tour_caves(here: Cave, map: &HashMap<Cave,Vec<Cave>>,
    visited: HashSet<Cave>)
  -> i64
{
  if here == Cave::End {
    return 1;
  }

  let mut path_count = 0;

  for destination in map.get(&here).unwrap() {
    if !visited.contains(destination) {
      let mut new_visited = visited.clone();
      match here {
        Cave::Large(_) => { },
        _ => {
          new_visited.insert(here.clone());
        }
      }
      path_count += tour_caves(destination.clone(), map, new_visited);
    }
  }

  path_count
}


fn longer_tour_caves(here: Cave, map: &HashMap<Cave,Vec<Cave>>,
    visited: HashMap<Cave,usize>, have_double_visited: bool)
  -> i64
{
  if here == Cave::End {
    return 1;
  }

  let mut path_count = 0;

  for destination in map.get(&here).unwrap() {
    let (should_go, new_have_double_visited) = match visited.get(destination)
    {
      None => (true, have_double_visited),
      Some(1) => {
        if have_double_visited {
          (false, true)
        } else {
          (true, true)
        }
      }
      _ => (false, have_double_visited),
    };

    if should_go {
      let mut new_visited = visited.clone();
      let here_visit_count = visited.get(&here);
      match here {
        Cave::Large(_) => { },
        Cave::Small(_) => {
          new_visited.insert(here.clone(), match here_visit_count {
            None => 1,
            Some(n) => n + 1,
          });
        }
        Cave::Start => {
          new_visited.insert(here.clone(), 2);
        }
        Cave::End => {
          new_visited.insert(here.clone(), 2);
        }
      }

      path_count += longer_tour_caves(destination.clone(), map, new_visited,
        new_have_double_visited);
    }
  }

  path_count
}

