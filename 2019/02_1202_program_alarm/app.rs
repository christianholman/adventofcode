use std::fs;

fn main() {
    let mut nums_string = fs::read_to_string("./input.txt")
                        .expect("Could not read input file");
    // Remove \n character
    nums_string.pop();
    let nums: Vec<i32> = nums_string.split(",").map(|s| s.parse::<i32>().expect("Could not parse input")).collect();

    println!("Part 1: {:?}", part1(&nums));
    println!("Part 2: {:?}", part2(&nums));
}

fn part1(nums: &Vec<i32>) -> Vec<i32> {
    let mut our_nums = nums.clone();
    let mut i = 0;
        
    loop {
        let start_index = i * 4;
        let output_location = our_nums[start_index + 3 as usize];
        match our_nums[start_index] {
            1 => our_nums[output_location as usize] = our_nums[our_nums[start_index + 1 as usize] as usize] + our_nums[our_nums[start_index + 2 as usize] as usize],
            2 => our_nums[output_location as usize] = our_nums[our_nums[start_index + 1 as usize] as usize] * our_nums[our_nums[start_index + 2 as usize] as usize],
            99 => return our_nums,
            _ => break,
        }
        i += 1;
    }

    our_nums
}

fn part2(nums: &Vec<i32>) -> Vec<i32> {
    let mut our_nums = nums.clone();

    for x in 1..100 {
        our_nums[1] = x;
        for y in 1..100 {
            our_nums[2] = y;
            if part1(&our_nums)[0] == 19690720 {
                return part1(&our_nums)
            }
        }
    }

    our_nums
}
