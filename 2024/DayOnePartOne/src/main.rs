use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();
    let mut left_list: Vec<i64> = vec![];
    let mut right_list: Vec<i64> = vec![];
    let mut result: i64 = 0;

    input.iter().for_each(|x| {
        let vec = x.split("   ").collect::<Vec<&str>>();

        left_list.push(vec[0].parse::<i64>().unwrap());
        right_list.push(vec[1].parse::<i64>().unwrap());
    });

    right_list.sort();
    left_list.sort();

    for (i, el) in left_list.iter().enumerate() {
        result = result + (el - right_list[i]).abs();
    }

    dbg!(result);
}
