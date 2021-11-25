pub mod rootfn {

    const MAX_RECURSION_DEPTH: u8 = 100;

    pub fn root(number: f32, power: u32) -> f32 {
        if number < 0.0 {
            panic!("Invalid input.");
        }
        fn cycle(from: f32, to: f32, depth: u8, original_number: f32, power: u32) -> f32 {
            let difference = to - from;
            let calc = from + difference / 2.0;

            if depth == 0{
                return calc;
            }
            let powered = calc.powi(power as i32);
            let mut next_from = from;
            let mut next_to = calc;
            if powered < original_number{
                next_from = calc;
                next_to = to;
            }
            return cycle(next_from, next_to, depth -1, original_number, power);

        }
        return cycle(0.0, number, MAX_RECURSION_DEPTH, number, power);
    }
}

#[cfg(test)]
mod tests {
    use crate::rootfn;

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
        let right = (rootfn::root(val as f32, 2) * 100.0).round() / 100.0;
        assert_eq!(left, right);
    }
}
