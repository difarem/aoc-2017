use std::io;
use io::BufRead;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;

        let line = line.unwrap();
        let line = line.as_bytes();
        let len = line.len();

        for i in 0..len {
            let byte = line[i];

            let next1 = if i == len - 1 { 0 } else { i + 1 };
            let next1 = line[next1];

            if byte == next1 {
                sum1 += (byte as u32) - ('0' as u32);
            }

            let next2 = (i + len / 2) % len;
            let next2 = line[next2];
            if byte == next2 {
                sum2 += (byte as u32) - ('0' as u32);
            }
        }

        println!("{} {}", sum1, sum2);
    }
}
