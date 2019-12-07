use std::collections::HashSet;

type Point = (i32, i32);
type VisitedPoint = (Point, usize);

fn main() {
    let mut input_str = String::from(include_str!("input.txt"));
    // Remove last \n
    input_str.pop();

    let inputs: Vec<&str> = input_str.split("\n").collect();

    part_1(&inputs);
    part_2(&inputs);
}

fn get_intersections(wires: &Vec<&str>) -> HashSet::<Point> {
    let initial_point: Point = (0, 0);
    let mut wire_positions: HashSet<VisitedPoint> = HashSet::new();
    let mut intersections: HashSet<Point> = HashSet::new();

    let mut wire_nums = wires.iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>();

    for (i, wire) in wires.iter().enumerate() {
        let mut current_position = initial_point.clone();

        for step in wire.split(",").collect::<Vec<&str>>().iter() {
            let (instruction, steps) = step.split_at(1); 

            for _ in 0..steps.parse::<i32>().unwrap() {
                match instruction {
                    "L" => current_position.0 = current_position.0 - 1,
                    "R" => current_position.0 = current_position.0 + 1,
                    "D" => current_position.1 = current_position.1 - 1,
                    "U" => current_position.1 = current_position.1 + 1,
                    _ => panic!("Invalid instruction!"),
                }
                for wire_num in wire_nums.iter() {
                    if wire_num != &i {
                        if wire_positions.contains(&(current_position, *wire_num)) {
                            intersections.insert(current_position);
                        }
                    }
                }
                wire_positions.insert((current_position, i));
            }
        }
    }

    intersections
}

fn part_1(wires: &Vec<&str>) {
    let intersections = get_intersections(wires);

    let mut intersection_distances = intersections.into_iter().map(|point| manhattan_distance(&(0, 0), &point)).collect::<Vec<i32>>();
    intersection_distances.sort();

    println!("Closest intersection: {:?} steps", intersection_distances[0]);
}

fn part_2(wires: &Vec<&str>) {
    let intersections = get_intersections(wires);
    let mut closest_steps = i32::max_value();

    for intersection in intersections.iter() {
        let mut current_steps = 0;
        for wire in wires.iter() {
             let mut current_position = (0,0);
 
             'step_loop: for step in wire.split(",").collect::<Vec<&str>>().iter() {
                 let (instruction, steps) = step.split_at(1); 
 
                 for _ in 0..steps.parse::<i32>().unwrap() {
                     match instruction {
                         "L" => current_position.0 = current_position.0 - 1,
                         "R" => current_position.0 = current_position.0 + 1,
                         "D" => current_position.1 = current_position.1 - 1,
                         "U" => current_position.1 = current_position.1 + 1,
                         _ => panic!("Invalid instruction!"),
                     }

                     current_steps += 1;

                     if &current_position == intersection {
                         break 'step_loop;
                     }
                 }
             }
         }
         if current_steps < closest_steps {
             closest_steps = current_steps;
         }
    }

    println!("Least steps to closest intersection: {:?} steps", closest_steps);
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    return (b.0.abs() - a.0.abs()) + (b.1.abs() - a.1.abs())
}

