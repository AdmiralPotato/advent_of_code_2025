use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn parse_line_range(text: &str) -> RangeInclusive<u64> {
    let (a, b) = text.split_once("-").unwrap();
    let a: u64 = a.parse().unwrap();
    let b: u64 = b.parse().unwrap();
    a..=b
}

fn part_1(ranges: &Vec<RangeInclusive<u64>>, ingredients: &Vec<u64>) -> u64 {
    let mut count: u64 = 0;
    'outer: for ingredient in ingredients.iter() {
        for range in ranges.iter() {
            if range.contains(ingredient) {
                count += 1;
                continue 'outer;
            }
        }
    }
    count
}

fn main() {
    let text_file_string = read_to_string("./day_05/input.txt").expect("It was supposed to work");
    let (ranges, ingredients) = prep_input(text_file_string);
    let part_1_result = part_1(&ranges, &ingredients);
    println!("WHAT IS part_1_result?? {part_1_result}");
    /*
       WHAT IS part_1_result?? 821
    */
}

fn prep_input(text: String) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ingredients) = text.trim().split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = ranges.trim().split("\n").map(parse_line_range).collect();
    let ingredients: Vec<u64> = ingredients.trim().split("\n").map(|s| s.parse::<u64>().unwrap()).collect();
    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_range_test() {
        assert_eq!(parse_line_range("3-5"), 3..=5);
        assert_eq!(parse_line_range("10-14"), 10..=14);
        assert_eq!(parse_line_range("16-20"), 16..=20);
        assert_eq!(parse_line_range("12-18"), 12..=18);
    }
    #[test]
    fn part_1_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let (ranges, ingredients) = prep_input(text_file_string);
        let part_1_result = part_1(&ranges, &ingredients);
        assert_eq!(part_1_result, 3);
    }
}
