use std::fs::read_to_string;
use std::str::Split;

fn parse_line_part_1(text: &str) -> (u32, u32) {
    let (a, b) = text.split_once("-").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn detect_repeat(n: u32) -> bool {
    let string = n.to_string();
    let half: f32 = (string.len() as f32) / 2.0;
    if (half % 1.0 == 0.0) {
        let first_half = &string[0..(half as usize)];
        let repeated = format!("{first_half}{first_half}");
        return repeated == string;
    }
    false
}

fn part_1(lines: &Split<&str>) -> u64 {
    let mut count: u64 = 0;
    lines.clone().for_each(|line| {
        let (start, stop) = parse_line_part_1(line);
        for i in start..=stop {
            if (detect_repeat(i)) {
                count += i as u64;
            }
        }
    });
    count
}

fn detect_repeat_2(n: u32) -> bool {
    let string = n.to_string();
    let half: f32 = (string.len() as f32) / 2.0;
    if (half % 1.0 == 0.0) {
        let first_half = &string[0..(half as usize)];
        let repeated = format!("{first_half}{first_half}");
        return repeated == string;
    }
    false
}

fn part_2(lines: &Split<&str>) -> u64 {
    let mut count: u64 = 0;
    lines.clone().for_each(|line| {
        let (start, stop) = parse_line_part_1(line);
        for i in start..=stop {
            if (detect_repeat_2(i)) {
                count += i as u64;
            }
        }
    });
    count
}

fn main() {
    let text_file_string = read_to_string("./day_02/input.txt").expect("It was supposed to work");
    let sample_input = text_file_string.trim().split(",");
    let part_1_result = part_1(&sample_input);
    println!("WHAT IS part_1_result?? {part_1_result}");
    let part_2_result = part_2(&sample_input);
    println!("WHAT IS part_2_result?? {part_2_result}");
    /*
       WHAT IS part_1_result?? 9188031749
       WHAT IS part_2_result?? ??????????
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_part_1() {
        assert_eq!(parse_line_part_1("11-22"), (11, 22));
        assert_eq!(parse_line_part_1("38593856-38593862"), (38593856, 38593862));
    }
    #[test]
    fn test_detect_repeat() {
        assert_eq!(detect_repeat(1), false);
        assert_eq!(detect_repeat(13), false);
        assert_eq!(detect_repeat(123123), true);
        assert_eq!(detect_repeat(123124), false);
    }
    #[test]
    fn part_1_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split(",");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 1227775554);
    }
    #[test]
    fn test_detect_repeat_2() {
        assert_eq!(detect_repeat_2(1), false);
        assert_eq!(detect_repeat_2(11), true);
        assert_eq!(detect_repeat_2(13), false);
        assert_eq!(detect_repeat_2(111), true);
        assert_eq!(detect_repeat_2(123123), true);
        assert_eq!(detect_repeat_2(121212), true);
        assert_eq!(detect_repeat_2(1212121), false);
        assert_eq!(detect_repeat_2(123123123), true);
        assert_eq!(detect_repeat_2(123124), false);
    }
    #[test]
    fn part_2_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split(",");
        let part_1_result = part_2(&sample_input);
        assert_eq!(part_1_result, 4174379265);
    }
}
