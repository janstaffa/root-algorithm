use std::{
    self,
    io::{stdin, stdout, Write},
};

fn main() {
    loop {
        let mut input = String::new();
        print!("number to sqrt > ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut input)
            .expect("Didn't enter a valid string.");

        let input_number: f32 = input.trim().parse().unwrap();
        println!("=> root of {} is {}", input.trim(), sqrt(input_number));
    }
}

const MAX_PRECISION: f32 = 0.001;
const SPLITS_PER_CYCLE: u16 = 10;
const MAX_RECURSION_DEPTH: u8 = 10;

pub fn sqrt(number: f32) -> f32 {
    if number < 0.0 {
        panic!("Invalid input.");
    }
    fn cycle(from: f32, to: f32, depth: u8, original_number: f32) -> f32 {
        let difference = to - from;
        let step = difference / SPLITS_PER_CYCLE as f32;
        let mut count = from;
        let mut best_count = to;

        let mut iter = 0;
        while count < to {
            if iter > SPLITS_PER_CYCLE {
                break;
            }
            let powered = count.powi(2);
            let difference = original_number - powered;

            let best_count_difference = original_number - best_count.powi(2);
            if difference.abs() < best_count_difference.abs() {
                best_count = count;
            }

            if difference.abs() < MAX_PRECISION {
                break;
            }
            count += step;
            iter += 1;
        }

        if depth > 0 && step > MAX_PRECISION {
            let prev_count = best_count - step;
            let next_count = best_count + step;
            let next_difference = original_number - next_count.powi(2);
            let best_difference = original_number - best_count.powi(2);

            let mut from = prev_count;
            let mut to = best_count;

            if best_difference > 0.0 && 0.0 > next_difference {
                from = best_count;
                to = next_count;
            }

            return cycle(from, to, depth - 1, original_number);
        }
        return best_count;
    }
    return cycle(0.0, number, MAX_RECURSION_DEPTH, number);
}

#[cfg(test)]
mod tests {
    use crate::sqrt;

    extern crate test_case;
    use test_case::test_case;

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(6)]
    #[test_case(8)]
    #[test_case(9)]
    #[test_case(10)]
    #[test_case(30)]
    #[test_case(100)]
    #[test_case(99)]
    #[test_case(200)]
    #[test_case(333)]
    #[test_case(1000)]
    #[test_case(10000)]
    #[test_case(81)]
    #[test_case(21)]
    #[test_case(18)]
    #[test_case(16)]
    #[test_case(71)]
    #[test_case(89)]
    #[test_case(91)]
    #[test_case(43)]
    #[test_case(73)]
    #[test_case(29)]
    #[test_case(1856)]
    #[test_case(2425)]
    #[test_case(75275)]
    #[test_case(464764)]
    #[test_case(75754)]
    #[test_case(785)]
    #[test_case(546)]
    #[test_case(6785)]
    #[test_case(11)]
    fn test_sqrt(val: i32) {
        let left = ((val as f32).sqrt() * 100.0).round() / 100.0;
        let right = (sqrt(val as f32) * 100.0).round() / 100.0;
        assert_eq!(left, right);
    }
}
