use advent_lib::prelude::*;

use std::collections::HashMap;
use std::io::Write;

#[derive(Debug,Clone,Eq,PartialEq)]
enum Instruction {
  Inp(Variable),
  Add(Variable, Expression),
  Mul(Variable, Expression),
  Div(Variable, Expression),
  Mod(Variable, Expression),
  Eql(Variable, Expression),
}

#[derive(Debug,Clone,Eq,PartialEq)]
enum Expression {
  Variable(Variable),
  Literal(i64),
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum Variable {
  W,
  X,
  Y,
  Z,
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct State {
  w: i64,
  x: i64,
  y: i64,
  z: i64,
  program_counter: usize,
  input: Vec<i64>,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut program = Vec::new();
  for line in &input {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words[0] {
      "inp" => {
        let a = parse_variable(words[1]);
        program.push(Instruction::Inp(a));
      },
      "add" => {
        let a = parse_variable(words[1]);
        let b = parse_expression(words[2]);
        program.push(Instruction::Add(a, b));
      },
      "mul" => {
        let a = parse_variable(words[1]);
        let b = parse_expression(words[2]);
        program.push(Instruction::Mul(a, b));
      },
      "div" => {
        let a = parse_variable(words[1]);
        let b = parse_expression(words[2]);
        program.push(Instruction::Div(a, b));
      },
      "mod" => {
        let a = parse_variable(words[1]);
        let b = parse_expression(words[2]);
        program.push(Instruction::Mod(a, b));
      },
      "eql" => {
        let a = parse_variable(words[1]);
        let b = parse_expression(words[2]);
        program.push(Instruction::Eql(a, b));
      },
      _ => panic!("not an instruction"),
    }
  }

  let initial_state = State {
    w: 0,
    x: 0,
    y: 0,
    z: 0,
    input: Vec::new(),
    program_counter: 0,
  };

  /*
  run_nondeterminsitically(&initial_state, &program, false);
  run_nondeterminsitically(&initial_state, &program, true);
  */
  let mut state = initial_state.clone();
  //for n in vec![9, 9, 9, 9, 5, 9, 6, 9, 9, 1, 9, 3, 2, 6].iter().rev() {
  //for n in vec![4, 8, 1, 1, 1, 5, 1, 4, 7, 1, 9, 1, 1, 1].iter().rev() {
  for n in vec![7, 4, 8, 3, 2, 1, 9, 8, 6, 4, 5, 7, 3, 3].iter().rev() {
    state.input.push(*n);
  }
  for stop in [18, 36, 54, 72, 90, 108, 126, 144, 162, 180, 198, 216, 234,
      252].iter()
  {
    run_until(&mut state, &program, *stop);
    debug_state(&state);
  }

  Ok(())
}


fn parse_expression(word: &str) -> Expression {
  match word {
    "w" => Expression::Variable(Variable::W),
    "x" => Expression::Variable(Variable::X),
    "y" => Expression::Variable(Variable::Y),
    "z" => Expression::Variable(Variable::Z),
    _ => Expression::Literal(word.parse::<i64>().unwrap()),
  }
}


fn parse_variable(word: &str) -> Variable {
  match word {
    "w" => Variable::W,
    "x" => Variable::X,
    "y" => Variable::Y,
    "z" => Variable::Z,
    _ => panic!("not a variable name"),
  }
}


#[allow(dead_code)]
fn run(state: &mut State, program: &Vec<Instruction>) {
  loop {
    if is_done(state, program) {
      break;
    }
    iterate(state, program);
  }
}


#[allow(dead_code)]
fn run_until(state: &mut State, program: &Vec<Instruction>, end: usize) {
  loop {
    if state.program_counter == end {
      break;
    }
    iterate(state, program);
  }
}


fn iterate(state: &mut State, program: &Vec<Instruction>) {
  match &program[state.program_counter] {
    Instruction::Inp(a) => {
      //debug_state(state);

      let result = state.input.pop().unwrap();
      set_variable(state, &a, result);
    },
    Instruction::Add(a, b) => {
      let a_value = evaluate_variable(state, &a);
      let b_value = evaluate_expression(state, &b);
      let result = a_value + b_value;
      set_variable(state, &a, result);
    },
    Instruction::Mul(a, b) => {
      let a_value = evaluate_variable(state, &a);
      let b_value = evaluate_expression(state, &b);
      let result = a_value * b_value;
      set_variable(state, &a, result);
    },
    Instruction::Div(a, b) => {
      let a_value = evaluate_variable(state, &a);
      let b_value = evaluate_expression(state, &b);
      let result = a_value / b_value;
      set_variable(state, &a, result);
    },
    Instruction::Mod(a, b) => {
      let a_value = evaluate_variable(state, &a);
      let b_value = evaluate_expression(state, &b);
      let result = a_value % b_value;
      set_variable(state, &a, result);
    },
    Instruction::Eql(a, b) => {
      let a_value = evaluate_variable(state, &a);
      let b_value = evaluate_expression(state, &b);
      let result = if a_value == b_value { 1 } else { 0 };
      set_variable(state, &a, result);
    },
  }

  state.program_counter += 1;
}


fn evaluate_expression(state: &State, expression: &Expression) -> i64 {
  match expression {
    Expression::Variable(Variable::W) => state.w,
    Expression::Variable(Variable::X) => state.x,
    Expression::Variable(Variable::Y) => state.y,
    Expression::Variable(Variable::Z) => state.z,
    Expression::Literal(n) => *n,
  }
}


fn evaluate_variable(state: &State, variable: &Variable) -> i64 {
  match variable {
    Variable::W => state.w,
    Variable::X => state.x,
    Variable::Y => state.y,
    Variable::Z => state.z,
  }
}


fn set_variable(state: &mut State, variable: &Variable, value: i64) {
  match variable {
    Variable::W => { state.w = value; },
    Variable::X => { state.x = value; },
    Variable::Y => { state.y = value; },
    Variable::Z => { state.z = value; },
  }
}


fn is_done(state: &State, program: &Vec<Instruction>) -> bool {
  if state.program_counter >= program.len() {
    return true;
  }

  false
}


#[allow(dead_code)]
fn debug_input(input: &Vec<i64>) {
  for digit in input.iter().rev() {
    print!("{}", digit);
  }
  println!("");
}


#[allow(dead_code)]
fn debug_state(state: &State) {
  println!("line {}:   {}    {}    {}    {}", state.program_counter, state.w,
    state.x, state.y, state.z);
}


#[allow(dead_code)]
fn run_fake(state: &mut State) {
  //           0    1   2    3   4   5   6    7   8   9  10   11   12   13
  let m = vec![1,   1,  1,  26,  1, 26,  1,  26,  1,  1, 26,  26,  26,  26];
  let n = vec![10, 13, 15, -12, 14, -2, 13, -12, 15, 11, -3, -13, -12, -13];
  let o = vec![10,  5, 12,  12,  6,  4, 15,   3,  7, 11,  2,  12,   4,  11];
  for i in 0 .. 14 {
    state.w = state.input.pop().unwrap();
    state.x = state.z % 26;
    state.z /= m[i];
    state.x = if state.x + n[i] != state.w { 1 } else { 0 };
    state.z = state.z * (25 * state.x + 1) + (state.w + o[i]) * state.x;
  }


  /* The largest input allowed by this matrix is: 99995969919326
   * The smallest is: 48111514719111
   *
   * Below is a trace of the state at the start of each iteration, when
   * invoked with the largest allowed input. Note that z (the final column)
   * is the only state actually passed between iterations, but for the purpose
   * of reverse-engineering, the below trace shows all four registers.
   *
   * line 18:   9    1    19    19
   * line 36:   9    1    14    508
   * line 54:   9    1    21    13229
   * line 72:   9    0    0    508
   * line 90:   5    1    11    13219
   * line 108:   9    0    0    508
   * line 126:   6    1    21    13229
   * line 144:   9    0    0    508
   * line 162:   9    1    16    13224
   * line 180:   1    1    12    343836
   * line 198:   9    0    0    13224
   * line 216:   3    0    0    508
   * line 234:   2    0    0    19
   * line 252:   6    0    0    0
   *
   * Below is a similar trace invoked with the smallest allowed input.
   *
   * line 18:   4    1    14    14
   * line 36:   8    1    13    377
   * line 54:   1    1    13    9815
   * line 72:   1    0    0    377
   * line 90:   1    1    7    9809
   * line 108:   5    0    0    377
   * line 126:   1    1    16    9818
   * line 144:   4    0    0    377
   * line 162:   7    1    14    9816
   * line 180:   1    1    12    255228
   * line 198:   9    0    0    9816
   * line 216:   1    0    0    377
   * line 234:   1    0    0    14
   * line 252:   1    0    0    0
   *
   * Here is a trace of a disallowed input.
   *
   * line 18:   7    1    17    17
   * line 36:   4    1    9    451
   * line 54:   8    1    20    11746
   * line 72:   3    1    15    11741
   * line 90:   2    1    8    305274
   * line 108:   1    1    5    305271
   * line 126:   9    1    24    7937070
   * line 144:   8    1    11    7937057
   * line 162:   6    1    13    206363495
   * line 180:   4    1    15    5365450885
   * line 198:   5    1    7    5365450877
   * line 216:   7    1    19    5365450889
   * line 234:   3    1    7    5365450877
   * line 252:   3    1    14    5365450884
   *
   */
}


#[allow(dead_code)]
fn cmp_histories(a: &Vec<i64>, b: &Vec<i64>) -> std::cmp::Ordering {
  for i in 0 .. a.len() {
    if a[i] > b[i] {
      return std::cmp::Ordering::Greater;
    } else if b[i] > a[i] {
      return std::cmp::Ordering::Less;
    }
  }

  std::cmp::Ordering::Equal
}


#[allow(dead_code)]
fn run_nondeterminsitically(initial_state: &State, program: &Vec<Instruction>,
    seek_min: bool)
{
  let mut all_states: HashMap<i64, Vec<i64>> = HashMap::new();
  all_states.insert(0, Vec::new());
  let mut last_program_counter = 0;
  for stop in [18, 36, 54, 72, 90, 108, 126, 144, 162, 180, 198, 216, 234,
      252].iter()
  {
    println!("going until {} with {} states in", stop, all_states.len());
    let mut new_all_states: HashMap<i64, Vec<i64>> = HashMap::new();

    let mut odometer = 0;
    for (z_in, history) in all_states.iter() {
      odometer += 1;
      if odometer % 100000 == 0 {
        print!("  {}", odometer);
        let _ = std::io::stdout().flush();
      }

      for w in 1 .. 10 {
        let mut state = initial_state.clone();
        state.z = *z_in;
        state.input.push(w);
        state.program_counter = last_program_counter;

        run_until(&mut state, &program, *stop);

        let z_out = state.z;

        let mut new_history = history.clone();
        new_history.push(w);

        match new_all_states.get(&z_out) {
          None => {
            new_all_states.insert(z_out, new_history);
          }
          Some(matching_history) => {
            let mut ordering = cmp_histories(&new_history, matching_history);
            if seek_min {
              ordering = ordering.reverse();
            }
            match ordering {
              std::cmp::Ordering::Less => {
                let matching_history = matching_history.clone();
                new_all_states.insert(z_out, matching_history);
              }
              _ => {
                new_all_states.insert(z_out, new_history);
              }
            }
          }
        }
      }
    }
    println!("");

    all_states = new_all_states;
    last_program_counter = *stop;
  }

  let winning_history = all_states.get(&0).unwrap();
  for n in winning_history {
    print!("{}", n);
  }
  println!("");
}

