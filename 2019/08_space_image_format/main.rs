struct InputData {
    width: usize,
    height: usize,
    layers: Vec<String>,
}

impl InputData {
    fn new(width: usize, height: usize, image_data: String) -> Self {
        let layers = sub_strings(&image_data, width * height);
        Self {
            width,
            height,
            layers,
        }
    }
}

fn main() {
    let mut image_data = String::from(include_str!("input.txt"));
    // Remove trailing \n
    image_data.pop();

    let data = InputData::new(25, 6, image_data);

    part1(&data);
    part2(&data);
}

fn part1(data: &InputData) {
    println!("Part 1: {}", decode_image(&data.layers));
}

fn part2(data: &InputData) {
    println!("Part 2: ");

    let mut final_message = String::new();
    for pixel in 0..(data.width * data.height) {
        let mut visible_color = '0'; 
        for layer in data.layers.iter().rev() {
            match layer.chars().nth(pixel).expect("No input in column") {
                '1' => visible_color = '1',
                '0' => visible_color = '0',
                _ => (),
            }
        }
        final_message.push(visible_color);
    }

    for row in sub_strings(&final_message, data.width) {
        let readable_row = row.chars().map(|c| match c {
            '0' => ' ',
            '1' => '■',
            _ => '0',
        }).collect::<String>();

        println!("{:?}", readable_row);
    }
}

fn decode_image(image: &Vec<String>) -> u32 {
    let mut lowest_zeros = ("", u32::max_value());
    for layer in image.iter() {
        let zeros = count_char_occurence(layer, '0');
        if zeros < lowest_zeros.1 {
            lowest_zeros = (layer, zeros)
        }
    }

    let n_ones = count_char_occurence(lowest_zeros.0, '1');
    let n_twos = count_char_occurence(lowest_zeros.0, '2');

    n_ones * n_twos
}

fn count_char_occurence(input: &str, c: char) -> u32 {
    let mut sum = 0;
    for ch in input.chars() {
        if ch == c {
            sum += 1;
        }
    }
    sum
}

// Credit: @juggle-tux https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/9
fn sub_strings(string: &str, sub_len: usize) -> Vec<String> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(String::from(&string[pos..pos + len]));
        pos += len;
    }
    subs
}
