use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let tests_num: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..tests_num {
        let input_line: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (s1, s2) = (input_line[1], input_line[2]);

        let r: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (mut l1, mut l2, mut t1, mut t2, mut map) = (
            Vec::new(),
            Vec::new(),
            s1,
            s2,
            std::collections::HashMap::new(),
        );

        let mut sorted_r = r.clone();
        sorted_r.sort_by(|a, b| b.cmp(a));

        for (idx, el) in r.iter().enumerate() {
            map.entry(el).or_insert(Vec::new()).push(idx + 1);
        }

        for el in sorted_r.iter() {
            if let Some(idx) = map.get_mut(el).unwrap().pop() {
                let b = idx;
                if t1 > t2 {
                    l2.push(b);
                    t2 += s2;
                } else {
                    l1.push(b);
                    t1 += s1;
                }
            }
        }
        
        println!(
            "{} {}",
            l1.len(),
            l1.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "{} {}",
            l2.len(),
            l2.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
