advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;

    for line in lines {
        total += match first_and_last_digits(line){
            Some(i) => i,
            None => { return None },
        };
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn first_digit_loc(line: &str) -> Option<usize> {
    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            return Some(index);
        }
    }

    None
}

fn last_digit_loc(line: &str) -> Option<usize> {

    for (index, char) in line.chars().rev().enumerate() {
        if char.is_numeric() {
            return Some(line.len() - 1 - index);
        }
    }

    None
}

fn first_and_last_digits(line: &str) -> Option<u32> {

    let first_digit = match first_digit_loc(line) {
        Some(i) => line.chars().nth(i).unwrap(),
        None => { return None; },
    };

    let last_digit = match last_digit_loc(line) {
        Some(i) => line.chars().nth(i).unwrap(),
        None => { return None; },
    };

    Some((format!("{}{}", first_digit, last_digit)).parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142_u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
