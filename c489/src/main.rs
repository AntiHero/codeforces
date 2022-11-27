use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut i = stdin.lock().lines();

    let a: Vec<i16> = i
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i16>().unwrap())
        .collect();

    let m = a[0];
    let mut s = a[1];

    if s > m * 9 || (m > 1 && s == 0) {
        println!("-1 -1");

        return Ok(());
    }

    let mut max = String::new();
    let mut z: i16 = 0;

    for n in 1..=m {
        if s - 9 < 0 {
            max += &s.to_string();
        } else {
            max += "9";
        }

        s -= 9;

        if s <= 0 {
            z = m - n;
            break;
        }
    }

    let mut min = max.chars().rev().collect::<String>();
    let c = min.chars().nth(0).unwrap().to_string();

    let mut f = false;

    if z >= 1 {
        min = min[1..].to_string();

        if c.parse::<u8>().unwrap() > 1 {
            f = true;
            min = (c.parse::<u8>().unwrap() - 1).to_string() + &min;
        } else {
            min = "0".to_string() + &min;
        }

        for n in 1..=z {
            if n == z {
                min = if f { "1".to_string() } else { c.to_string() } + &min;
            } else {
                min = "0".to_string() + &min;
            }
        }
    }

    for _ in 1..=z {
        max += "0";
    }

    println!("{} {}", min, max);

    Ok(())
}
