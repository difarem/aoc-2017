use std::io;
use io::BufRead;

fn main() {
    let stdin = io::stdin();

    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let split: Vec<u64> = line.split_whitespace().map(|i| i.parse::<u64>().unwrap()).collect();
        let max = split.iter().max().unwrap_or(&0);
        let min = split.iter().min().unwrap_or(&0);

        sum1 += max - min;
        
        for i in 0 .. split.len() {
            for j in 0 .. i {
                let a = split[i].max(split[j]);
                let b = split[i].min(split[j]);
                
                if a % b == 0 {
                    sum2 += a / b;
                }
            }
        }
    }

    println!("{} {}", sum1, sum2);
}
