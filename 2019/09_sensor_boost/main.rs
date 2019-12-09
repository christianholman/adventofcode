fn main() {
    let mut input = String::from(include_str!("input.txt"));
    if input.ends_with('\n') {
        input.pop();
    }

    let initial_state = 2;

    let program = input.split(",").collect::<Vec<&str>>();
    let mut memory: Vec<i64> = program.iter().map(|v| v.parse::<i64>().expect("error parsing program")).collect();

    let mut extra_memory = vec![0i64; 1000000];
    memory.append(&mut extra_memory);

    let mut pointer = 0;
    let mut relative_base = 0i64;

    loop {
        let instruction = format!("{:0>5}", memory[pointer]);
        let (modes_str, opcode_str) = instruction.split_at(3);

        let modes_num = modes_str.parse::<usize>().expect("could not parse parameter mode");
        let modes: Vec<usize> = vec![
            (modes_num / 1) % 10,
            (modes_num / 10) % 10,
            (modes_num / 100) % 10,
        ];
        
        let get_locations = |n_params| -> Vec<usize> {
            let mut locations: Vec<usize> = vec![];
            for p in 0..n_params {
                match modes[p] {
                    0 => {
                        locations.push(memory[pointer + 1 + p] as usize)
                    }
                    1 => {
                        locations.push(pointer + 1 + p)
                    }
                    2 => {
                        locations.push((memory[pointer + 1 + p] + relative_base) as usize)
                    }
                    _ => {
                        panic!{"parametermode {} not implemented", modes[p]}
                    }
                }
            }
            locations
        };

        match opcode_str.parse::<usize>().expect("could not parse opcode") {
            1 => {
                let n_params = 3;
                let locations = get_locations(n_params);
                memory[locations[2]] = memory[locations[1]] + memory[locations[0]];
                pointer += 1 + n_params;
            },
            2 => {
                let n_params = 3;
                let locations = get_locations(n_params);
                memory[locations[2]] = memory[locations[1]] * memory[locations[0]];
                pointer += 1 + n_params;
            },
            3 => {
                let n_params = 1;
                let locations = get_locations(n_params);
                memory[locations[0]] = initial_state;
                pointer += 1 + n_params;
            },
            4 => {
                let n_params = 1;
                let locations = get_locations(n_params);
                println!("Debug: {}", memory[locations[0]]);
                pointer += 1 + n_params;
            },
            5 => {
                let n_params = 2;
                let locations = get_locations(n_params);
                if memory[locations[0]] != 0 {
                    pointer = memory[locations[1]] as usize;
                } else {
                    pointer += 1 + n_params;
                }
            },
            6 => {
                let n_params = 2;
                let locations = get_locations(n_params);
                if memory[locations[0]] == 0 {
                    pointer = memory[locations[1]] as usize;
                } else {
                    pointer += 1 + n_params;
                }
            },
            7 => {
                let n_params = 3;
                let locations = get_locations(n_params);
                if memory[locations[0]] < memory[locations[1]] {
                    memory[locations[2]] = 1;
                } else {
                    memory[locations[2]] = 0;
                }
                pointer += 1 + n_params;
            },
            8 => {
                let n_params = 3;
                let locations = get_locations(n_params);
                if memory[locations[0]] == memory[locations[1]] {
                    memory[locations[2]] = 1;
                } else {
                    memory[locations[2]] = 0;
                }
                pointer += 1 + n_params;
            },
            9 => {
                let n_params = 1;
                let locations = get_locations(n_params);
                relative_base += memory[locations[0]];
                pointer += 1 + n_params;
            }
            99 => {
                panic!("machine halted.");
            },
            _ => {
                panic!("opcode {} not implemented", opcode_str);
            }
        }
    }
}
