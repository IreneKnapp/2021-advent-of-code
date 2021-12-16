use advent_lib::prelude::*;

#[derive(Debug)]
struct Packet {
  version: u8,
  type_id: u8,
  content: Vec<u8>,
  subpackets: Vec<Packet>,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut bit_string: Vec<u8> = Vec::new();
  for c in input[0].chars() {
    let value = c.to_digit(16).unwrap() as u8;
    for i in 0 .. 4 {
      let bit = (value >> (3 - i)) & 1;
      bit_string.push(bit);
    }
  }

  let mut iter = bit_string.iter().peekable();
  let (version_total, _top_packet, result) = read_packet(&mut iter).unwrap();
  println!("{}", version_total);
  println!("{}", result);

  Ok(())
}


fn read_packet<'a>(iter: &mut impl Iterator<Item=&'a u8>) -> Result<(u64, Packet, u128)>
{
  let mut version_total: u64 = 0;

  let version = bits_to_u8(take_bits(iter, 3)?);
  let type_id = bits_to_u8(take_bits(iter, 3)?);

  version_total += version as u64;

  if type_id == 4 {
    let mut result = 0;
    loop {
      let bits = take_bits(iter, 5)?;

      let bits_value = bits_to_u128(bits[1..].to_vec());
      result = result << 4;
      result += bits_value;

      if bits[0] == 0 {
        break;
      }
    }

    Ok((version_total, Packet {
      version,
      type_id,
      content: Vec::new(),
      subpackets: Vec::new(),
    }, result))
  } else {
    let mut subpackets = Vec::new();
    let mut parameters = Vec::new();

    let length_type_id = bits_to_u8(take_bits(iter, 1)?);
    if length_type_id == 0 {
      let length_in_bits = bits_to_u64(take_bits(iter, 15)?);

      let mut content_bits: Vec<u8> = Vec::new();
      for bit in iter.take(length_in_bits as usize) {
        content_bits.push(*bit);
      }

      let mut content_bits = content_bits.iter().peekable();

      loop {
        match read_packet(&mut content_bits) {
          Err(_) => {
            break;
          }
          Ok((sub_version_total, subpacket, value)) => {
            version_total += sub_version_total;
            subpackets.push(subpacket);
            parameters.push(value);
          }
        }
      }
    } else {
      let n_subpackets = bits_to_u64(take_bits(iter, 11)?);

      for _ in 0 .. n_subpackets {
        match read_packet(iter) {
          Err(_) => {
            return Err(advent_lib::error::Error::Unspecified);
          }
          Ok((sub_version_total, subpacket, value)) => {
            version_total += sub_version_total;
            subpackets.push(subpacket);
            parameters.push(value);
          }
        }
      }
    }

    let mut result = 0;
    match type_id {
      0 => {
        result = 0;
        for parameter in &parameters {
          result += parameter;
        }
      }
      1 => {
        result = 1;
        for parameter in &parameters {
          result *= parameter;
        }
      }
      2 => {
        result = parameters[0];
        for parameter in &parameters {
          if *parameter < result {
            result = *parameter;
          }
        }
      }
      3 => {
        result = parameters[0];
        for parameter in &parameters {
          if *parameter > result {
            result = *parameter;
          }
        }
      }
      5 => {
        result = if parameters[0] > parameters[1] { 1 } else { 0 };
      }
      6 => {
        result = if parameters[0] < parameters[1] { 1 } else { 0 };
      }
      7 => {
        result = if parameters[0] == parameters[1] { 1 } else { 0 };
      }
      _ => { }
    }

    Ok((version_total, Packet {
      version,
      type_id,
      content: Vec::new(),
      subpackets: subpackets,
    }, result))
  }
}


fn take_bits<'a>(iter: &mut impl Iterator<Item=&'a u8>, count: usize)
  -> Result<Vec<u8>>
{
  let mut result: Vec<u8> = Vec::new();

  for _ in 0 .. count {
    match iter.next() {
      None => {
        return Err(advent_lib::error::Error::Unspecified);
      }
      Some(bit) => {
        result.push(*bit);
      }
    }
  }

  Ok(result)
}


fn bits_to_u8(bits: Vec<u8>) -> u8 {
  let mut result = 0;

  for bit in &bits {
    result *= 2;
    result += bit;
  }

  result
}


fn bits_to_u64(bits: Vec<u8>) -> u64 {
  let mut result = 0;

  for bit in &bits {
    result *= 2;
    result += *bit as u64;
  }

  result
}


fn bits_to_u128(bits: Vec<u8>) -> u128 {
  let mut result = 0;

  for bit in &bits {
    result *= 2;
    result += *bit as u128;
  }

  result
}

