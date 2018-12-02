use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let values = read_input();
    let sum = find_sum(&values);
    let timer = Instant::now();
    let repeated_freq = find_repeated_frequency(&values);
    println!("time to get repeated frequencies: {:?}", timer.elapsed());
    println!("sum: {}", sum);
    println!("repeated_freq: {}", repeated_freq);
}

fn read_input() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let iter = buffer.split_whitespace();
    iter.map(|s_value| s_value.parse::<i32>().unwrap())
        .collect()
}

fn find_sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn find_repeated_frequency(values: &[i32]) -> i32 {
    let mut frequencies = HashSet::new();
    let mut freq = 0;
    frequencies.insert(freq);
    loop {
        let mut value_iterator = values.iter();
        while let Some(value) = value_iterator.next() {
            freq += value;
            if !frequencies.contains(&freq) {
                frequencies.insert(freq);
            } else {
                return freq;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_sum_test() {
        let values = vec![1, 2, 3];
        assert_eq!(find_sum(&values), 6);
    }

    #[test]
    fn basic_repeated_freq_test() {
        let values = vec![1, -6, 4];
        assert_eq!(find_repeated_frequency(&values), 0);
    }
}
