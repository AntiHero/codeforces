use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Bad input");

    let str = input.trim();

    let mut i = 0;

    let mut has_ab = false;
    let mut has_ba = false;

    let mut x1 = -1;
    let mut x2 = -1;

    if str.len() < 3 {
        return println!("NO");
    }

    while i < str.len() {
        if i > str.len() - 2 {
            break;
        }

        if has_ab {
            if &str[i..i + 2] == "BA" {
                if i as i32 - x1 >= 2 {
                    return println!("YES");
                }
            }
        }

        if has_ba {
            if &str[i..i + 2] == "AB" {
                if i as i32 - x2 >= 2 {
                    return println!("YES");
                }
            }
        }

        if !has_ab && &str[i..i + 2] == "AB" {
            has_ab = true;
            x1 = i as i32;
        } else if !has_ba && &str[i..i + 2] == "BA" {
            has_ba = true;
            x2 = i as i32;
        }

        i += 1;
    }

    println!("NO");
}
