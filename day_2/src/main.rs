use std::io::{self, BufRead, BufReader, Result};

const DUPLICATION_NUMBERS: [u8; 2] = [2, 3];

fn read_input() -> Result<Vec<String>> {
    let stdin = BufReader::new(io::stdin());
    stdin.lines().collect()
}

fn determine_count(duplication_numbers: &[u8], line: &str) -> Vec<u8> {
    let mut counts = vec![0; 26];
    let mut return_values: Vec<u8> = vec![0; duplication_numbers.len()];
    for character in line.chars() {
        counts[character as usize - 'a' as usize] += 1;
    }
    for (idx, value) in duplication_numbers.iter().enumerate() {
        if counts.iter().any(|count| count == value) {
            return_values[idx] = 1;
        }
    }
    return_values
}

fn determine_checksum(lines: &[String]) -> i64 {
    let mut totals = [0, 0];
    for line in lines {
        let counts = determine_count(&DUPLICATION_NUMBERS, &line);
        totals[0] += counts[0];
        totals[1] += counts[1];
    }

    i64::from(totals[0]) * i64::from(totals[1])
}

fn find_single_difference(lines: &[String]) -> Option<String> {
    let mut sorted_lines = Vec::with_capacity(lines.len());
    sorted_lines.extend_from_slice(lines);
    sorted_lines.sort_unstable();

    'outer: for pair in sorted_lines.windows(2) {
        let mut found_unmatching = false;
        let mut unmatching_idx = None;
        let mut pair0_chars = pair[0].chars().collect::<Vec<char>>();
        let pair1_chars = pair[1].chars().collect::<Vec<char>>();
        for idx in 0..pair[0].len() {
            if pair0_chars[idx] != pair1_chars[idx] {
                if found_unmatching {
                    continue 'outer;
                } else {
                    found_unmatching = true;
                    unmatching_idx = Some(idx);
                }
            }
        }
        if let Some(idx) = unmatching_idx {
            pair0_chars.remove(idx);
            return Some(pair0_chars.into_iter().collect::<String>());
        }
    }
    None
}

fn main() {
    let lines = read_input().unwrap();
    println!("checksum: {}", determine_checksum(&lines));
    let single_diff_string = find_single_difference(&lines);
    println!(
        "single diff remaining chars: {}",
        single_diff_string.unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_2_or_3() {
        let string = "abcdef";
        assert_eq!(vec![0, 0], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn both() {
        let string = "bababc";
        assert_eq!(vec![1, 1], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn a_2() {
        let string = "abbcde";
        assert_eq!(vec![1, 0], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn a_3() {
        let string = "abcccd";
        assert_eq!(vec![0, 1], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn two_2s() {
        let string = "aabcdd";
        assert_eq!(vec![1, 0], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn two_es() {
        let string = "abcdee";
        assert_eq!(vec![1, 0], determine_count(&DUPLICATION_NUMBERS, &string));
    }

    #[test]
    fn two_3s() {
        let string = "ababab";
        assert_eq!(vec![0, 1], determine_count(&DUPLICATION_NUMBERS, &string));
    }

}
