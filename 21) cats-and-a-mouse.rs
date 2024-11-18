use std::io::{self, Write};

fn cat_and_mouse(x: i32, y: i32, z: i32) -> String {
    let distance_a = (z - x).abs(); 
    let distance_b = (z - y).abs();

    if distance_a < distance_b {
        "Cat A".to_string()
    } else if distance_b < distance_a {
        "Cat B".to_string()
    } else {
        "Mouse C".to_string()
    }
}

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x, y, z) = (values[0], values[1], values[2]);

        let result = cat_and_mouse(x, y, z);
        writeln!(handle, "{}", result).unwrap();
    }
}
