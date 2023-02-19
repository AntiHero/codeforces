use std::io;

fn main() {
    let stdin = io::stdin();
    let mut t = String::new();

    stdin.read_line(&mut t).expect("String");

    let t = t.trim().parse::<u16>().expect("Number");
    let mut l = String::new();
    let mut p = String::new();

    for _ in 0..t {
        stdin.read_line(&mut l).expect("Length of the permutation");
        let len = l.trim().parse::<usize>().expect("Number");

        stdin.read_line(&mut p).expect("Permutation");

        let mut pos: Vec<usize> = vec![0; len];

        // WE DON'T REALLY NEED THE PERMUTATION ITSELF, WE NEED ONLY THE POSITION OF IT'S ELEMENTS,
        // SO WE BUILD THE POSITION VECTOR INSTEAD
        p.split_whitespace()
            .enumerate()
            .for_each(|(i, x)| pos[(x.parse::<usize>().expect("Number")) - 1] = i);

        // MAX NEEDED SWAPS
        let mut n = pos.len() / 2;

        while n > 0 {
            if pos[n - 1] < pos[n] && pos[len - n - 1] < pos[len - n] {
                n -= 1;
                continue;
            }

            break;
        }

        l = String::from("");
        p = String::from("");

        println!("{n}");
    }
}
