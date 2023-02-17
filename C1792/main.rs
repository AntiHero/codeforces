use std::io;

fn main() {
    let stdin = io::stdin();
    let mut t = String::new();

    stdin.read_line(&mut t).expect("String");

    let t = t.trim().parse::<u16>().expect("Number");
    let mut m = String::new();
    let mut input_data: Vec<Vec<u32>> = Vec::new();
    let mut nums = String::new();

    for _ in 0..t {
        stdin.read_line(&mut m).expect("Number of test cases");
        stdin.read_line(&mut nums).expect("String of numbers");

        let mut arr: Vec<u32> = Vec::new();

        nums.split_whitespace()
            .for_each(|x| arr.push(x.parse::<u32>().expect("Number")));

        nums = String::from("");
        input_data.push(arr);
    }
}
