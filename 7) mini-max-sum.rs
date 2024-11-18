use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();
    println!("{} {}", sum - *max as i64, sum - *min as i64);
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
