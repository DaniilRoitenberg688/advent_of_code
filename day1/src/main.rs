use std::{fs, process};


fn main() {
    let data = match fs::read_to_string("input.txt") {
        Ok(v) => v,
        _  => {
            println!("cannot read file");
            process::exit(1);
        }
    };
    let lines = data.split("\n");
    let mut counter = 50;
    let mut res = 0;
    let mut turn = 0;
    for l in lines {
        if l == "" {
            continue;
        }
        let c: Vec<char> = l.chars().collect();
        if c[0] == 'L' {
            let number: i32 = l[1..].parse().unwrap();
            turn = -number;
        } else if c[0] == 'R' {
            let number: i32 = l[1..].parse().unwrap();
            turn = number;
        }


        if counter == 0 {
            if turn < 0 {
                res += - turn / 100;
            } else {
                res += turn / 100;
            }
        } else {
            if turn < 0 {
                res += - turn / 100;
                if turn % 100 + counter <= 0{
                    res += 1
                }
            } else {
                res += turn / 100;
                if turn % 100 + counter >= 100 {
                    res += 1
                }
            }
        }


        counter += turn;
        if counter % 100 == 0 {
            counter = 0
        } else if counter < 0 {
            counter = 100 + counter % 100
        } else {
            counter = counter % 100
        }




    }

    println!("{res}")

}
