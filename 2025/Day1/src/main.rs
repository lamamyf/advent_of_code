use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    let mut dial = 50;
    let mut zero_count = 0;

    input.lines().for_each(|line| {
        if let Some(moves) = line.strip_prefix("L") {
            let number = moves.parse::<i32>().unwrap();

            for i in 1..=number {
                let pos = (dial - i).rem_euclid(100);
                if pos == 0 {
                    zero_count += 1;
                }
            }

            dial = (dial - number).rem_euclid(100);
        } else if let Some(moves) = line.strip_prefix("R") {
            let number = moves.parse::<i32>().unwrap();

            for i in 1..=number {
                let pos = (dial + i).rem_euclid(100);
                if pos == 0 {
                    zero_count += 1;
                }
            }

            dial = (dial + number).rem_euclid(100);
        }
    });

    dbg!(zero_count);
}