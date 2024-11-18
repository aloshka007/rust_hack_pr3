use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    // Create a counts array for bird types 1 through 5 (index 0 is unused)
    let mut counts = vec![0; 6];

    // Count occurrences of each bird type
    for &bird in arr {
        if bird >= 1 && bird <= 5 { // Ensure bird types are within the valid range
            counts[bird as usize] += 1;
        }
    }

    // Find the bird type with the maximum count
    // Starting from index 1 since 0 is unused
    let mut max_count = 0;
    let mut bird_type = 0;

    for (index, &count) in counts.iter().enumerate().skip(1) {
        if count > max_count || (count == max_count && (index as i32) < bird_type) {
            max_count = count;
            bird_type = index as i32;
        }
    }

    bird_type
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
