use advent_lib::prelude::*;


#[derive(Debug, Clone)]
struct Step {
  set_to: bool,
  min: Point,
  max: Point,
}

#[derive(Debug, Clone)]
struct Region {
  min: Point,
  max: Point,
}

#[derive(Debug, Clone)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Span {
  min: i64,
  max: i64,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut directions = Vec::new();
  for line in &input {
    let mut step = Step {
      set_to: false,
      min: Point { x: 0, y: 0, z: 0 },
      max: Point { x: 0, y: 0, z: 0 },
    };

    let words: Vec<&str> = line.split_whitespace().collect();

    step.set_to = match words[0] {
      "on" => true,
      "off" => false,
      _ => panic!("hm"),
    };

    let components: Vec<&str> = words[1].split(",").collect();

    let parts: Vec<&str> = components[0].split("=").collect();
    let ends: Vec<&str> = parts[1].split("..").collect();
    step.min.x = ends[0].parse::<i64>().unwrap();
    step.max.x = ends[1].parse::<i64>().unwrap();

    let parts: Vec<&str> = components[1].split("=").collect();
    let ends: Vec<&str> = parts[1].split("..").collect();
    step.min.y = ends[0].parse::<i64>().unwrap();
    step.max.y = ends[1].parse::<i64>().unwrap();

    let parts: Vec<&str> = components[2].split("=").collect();
    let ends: Vec<&str> = parts[1].split("..").collect();
    step.min.z = ends[0].parse::<i64>().unwrap();
    step.max.z = ends[1].parse::<i64>().unwrap();

    directions.push(step);
  }

  let mut total_on = 0;
  for x in -50 .. 51 {
    for y in -50 .. 51 {
      for z in -50 .. 51 {
        let point = Point { x, y, z };
        if test_cube(&point, &directions) {
          total_on += 1;
        }
      }
    }
  }

  println!("{}", total_on);

  let total_on_everywhere = count_everywhere(&directions);
  println!("{}", total_on_everywhere);

  Ok(())
}


fn test_cube(point: &Point, directions: &Vec<Step>) -> bool {
  for step in directions.iter().rev() {
    if (point.x >= step.min.x) && (point.y >= step.min.y)
      && (point.z >= step.min.z) && (point.x <= step.max.x)
      && (point.y <= step.max.y) && (point.z <= step.max.z)
    {
      return step.set_to;
    }
  }

  false
}


fn count_everywhere(directions: &Vec<Step>) -> i64 {
  let mut lit_regions: Vec<Region> = Vec::new();

  for step in directions {
    if lit_regions.len() == 0 {
      if step.set_to {
        lit_regions.push(Region {
          min: step.min.clone(),
          max: step.max.clone(),
        });
      }
      continue;
    }

    let mut new_lit_regions = Vec::new();
    for lit_region in lit_regions.into_iter() {
      /* Depending on whether set_to is true or false, we may be doing union
       * or difference. However, we implement union as difference followed by
       * also including the entire subtracted volume. So difference is the
       * only operation that needs the per-axis breakdown of cases.
       *
       * Start by determining whether it's overlapping or not. The
       * non-overlapping cases are:
       *
       *         A==========B                          (lit region)
       *                         C=========D           (step)
       *
       *                         A=========B           (lit region)
       *         C==========D                          (step)
       *
       * Now eliminate cases where the step entirely covers the lit region:
       *
       *                     A======B                  (lit region)
       *               C==================D            (step)
       *
       * The following cases remain:
       *
       *               A==================B            (lit region)
       *                     C======D                  (step)
       *
       *               A==========B                    (lit region)
       *                     C=====.......?            (step)
       *
       *               A==========B                    (lit region)
       *        ?......=====D                          (step)
       */
      let mut lit_spans = Vec::new();
      let mut step_spans = Vec::new();
      let mut differenced_spans = Vec::new();
      for axis in 0 .. 3 {
        let mut relevant_spans = Vec::new();

        let (lit_min, lit_max, step_min, step_max) = match axis {
          0 => { (lit_region.min.x, lit_region.max.x, step.min.x, step.max.x) },
          1 => { (lit_region.min.y, lit_region.max.y, step.min.y, step.max.y) },
          2 => { (lit_region.min.z, lit_region.max.z, step.min.z, step.max.z) },
          _ => { panic!("run in circles scream and shout"); },
        };

        lit_spans.push(Span { min: lit_min, max: lit_max });
        step_spans.push(Span { min: step_min, max: step_max });

        if (lit_max < step_min) || (lit_min > step_max) {
          // Non-overlapping
          relevant_spans.push(Span { min: lit_min, max: lit_max });
        } else if (step_min <= lit_min) && (step_max >= lit_max) {
          // Step entirely covers lit
          // It is intentional that nothing is done here.
        } else if (lit_min < step_min) && (lit_max > step_max) {
          // Step is a segment in the middle of lit
          relevant_spans.push(Span { min: lit_min, max: step_min - 1 });
          relevant_spans.push(Span { min: step_max + 1, max: lit_max });
        } else if lit_min < step_min {
          relevant_spans.push(Span { min: lit_min, max: step_min - 1 });
        } else {
          relevant_spans.push(Span { min: step_max + 1, max: lit_max });
        }

        differenced_spans.push(relevant_spans);
      }

      if ((differenced_spans[0].len() == 1)
        && (differenced_spans[0][0] == lit_spans[0]))
        || ((differenced_spans[1].len() == 1)
            && (differenced_spans[1][0] == lit_spans[1]))
        || ((differenced_spans[2].len() == 1)
            && (differenced_spans[2][0] == lit_spans[2]))
      {
        new_lit_regions.push(Region {
          min: Point {
            x: lit_spans[0].min,
            y: lit_spans[1].min,
            z: lit_spans[2].min,
          },
          max: Point {
            x: lit_spans[0].max,
            y: lit_spans[1].max,
            z: lit_spans[2].max,
          },
        });
      } else {
        for z_span in &differenced_spans[2] {
          let region = Region {
            min: Point {
              x: lit_spans[0].min,
              y: lit_spans[1].min,
              z: z_span.min,
            },
            max: Point {
              x: lit_spans[0].max,
              y: lit_spans[1].max,
              z: z_span.max,
            },
          };
          new_lit_regions.push(region);
        }

        let z_min = std::cmp::max(lit_spans[2].min, step_spans[2].min);
        let z_max = std::cmp::min(lit_spans[2].max, step_spans[2].max);

        for y_span in &differenced_spans[1] {
          let region = Region {
            min: Point {
              x: lit_spans[0].min,
              y: y_span.min,
              z: z_min,
            },
            max: Point {
              x: lit_spans[0].max,
              y: y_span.max,
              z: z_max,
            },
          };
          new_lit_regions.push(region);
        }

        let y_min = std::cmp::max(lit_spans[1].min, step_spans[1].min);
        let y_max = std::cmp::min(lit_spans[1].max, step_spans[1].max);

        for x_span in &differenced_spans[0] {
          let region = Region {
            min: Point {
              x: x_span.min,
              y: y_min,
              z: z_min,
            },
            max: Point {
              x: x_span.max,
              y: y_max,
              z: z_max,
            },
          };
          new_lit_regions.push(region);
        }
      }

    }

    if step.set_to {
      let region = Region {
        min: step.min.clone(),
        max: step.max.clone(),
      };
      new_lit_regions.push(region);
    }


    lit_regions = new_lit_regions;
  }

  let mut total_lit = 0;
  for region in &lit_regions {
    let x_extent = region.max.x - region.min.x + 1;
    let y_extent = region.max.y - region.min.y + 1;
    let z_extent = region.max.z - region.min.z + 1;
    let volume = x_extent * y_extent * z_extent;
    total_lit += volume;
  }

  total_lit
}

