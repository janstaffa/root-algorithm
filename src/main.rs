use root_fn::rootfn;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("number to sqrt > ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut input)
            .expect("Invalid input.");

        let input_number: f32 = input.trim().parse().unwrap();
        println!(
            "=> root of {} is {}",
            input.trim(),
            rootfn::root(input_number, 2)
        );
    }
}
