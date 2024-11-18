use std::io;

fn get_money_spent(keyboards: &[i32], drives: &[i32], b: i32) -> i32 {
    let mut max_spent = -1;

    for &k in keyboards {
        for &d in drives {
            let total = k + d;
            if total <= b {
                max_spent = max_spent.max(total);
            }
        }
    }

    max_spent
}

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let bnm: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let b = bnm[0];
    let n = bnm[1];
    let m = bnm[2];

    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let keyboards: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

   
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let drives: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = get_money_spent(&keyboards, &drives, b);
    println!("{}", result);
}
