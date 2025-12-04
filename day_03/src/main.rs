use std::fs::read_to_string;
use std::str::Split;

fn parse_line_part_1(text: &str) -> i32 {
    let text = String::from(text).replace("R", "").replace("L", "-");
    let movement: i32 = text.parse().unwrap();
    movement
}

fn get_largest_joltage(line: &str) -> u32 {
    0
}

fn part_1(lines: &Split<&str>) -> u32 {
    let mut dial_position: i32 = 50;
    let mut count: u32 = 0;
    lines
        .clone()
        .for_each(|line| {
            let movement = parse_line_part_1(line);
            dial_position += movement;
            dial_position %= 100;
            dial_position += 100;
            dial_position %= 100;
            if(dial_position == 0) {
                count += 1;
            }
        });
    count
}

fn main() {
    let text_file_string = read_to_string("./day_01/input.txt").expect("It was supposed to work");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_result = part_1(&sample_input);
    println!("WHAT IS part_1_result?? {part_1_result}");
    /*
       WHAT IS part_1_result?? ???
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_largest_joltage_test() {
        assert_eq!(get_largest_joltage("987654321111111"), 98);
        assert_eq!(get_largest_joltage("811111111111119"), 89);
        assert_eq!(get_largest_joltage("234234234234278"), 78);
        assert_eq!(get_largest_joltage("818181911112111"), 92);
    }
    #[test]
    fn part_1_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 357);
    }
}
