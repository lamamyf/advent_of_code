use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    let mut dial = 50;
    let mut zero_count = 0;

    input.lines().for_each(|line| {
        if let Some(moves) = line.strip_prefix("L") {
            let number = moves.parse::<i32>().unwrap();
            dial = (dial - number) % 100;
        } else  if let Some(moves) = line.strip_prefix("R") {
            let number = moves.parse::<i32>().unwrap();
            dial = (dial + number) % 100;
        }

        if (dial == 0) {
            zero_count += 1;
        }
    });

    dbg!(zero_count);
}
