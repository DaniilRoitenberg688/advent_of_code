use std::fs;

const POSSIBLE: [(i32, i32); 8] = [
    (1, 1),
    (1, -1),
    (1, 0),
    (-1, 0),
    (-1, 1),
    (-1, -1),
    (0, -1),
    (0, 1),
];

fn main() {
    let input = fs::read_to_string("file").unwrap();
    let data = convert_to_list_list(input);
    let res = first_part(&data);
    let res2 = second_part(&data);
    println!("{res}");
    println!("{res2}");
}

fn convert_to_list_list(s: String) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    for i in s.split("\n") {
        res.push(i.chars().collect());
    }
    return res;
}

fn first_part(data: &Vec<Vec<char>>) -> u32 {
    let mut res = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let s = data[i][j];
            if s == '.' {
                continue;
            }
            let mut c = 0;
            for v in POSSIBLE.iter() {
                let l = match data.get((i as i32 - v.0) as usize) {
                    Some(v) => v,
                    None => continue,
                };
                match l.get((j as i32 - v.1) as usize) {
                    Some(v) => {
                        if *v == '@' {
                            c += 1
                        }
                    }
                    None => continue,
                };
            }
            if c < 4 {
                res += 1;
            }
        }
    }
    return res;
}

fn second_part(data: &Vec<Vec<char>>) -> u32 {
    let mut res: Vec<(usize, usize)> = vec![];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let s = data[i][j];
            if s == '.' {
                continue;
            }
            let mut c = 0;
            for v in POSSIBLE.iter() {
                let l = match data.get((i as i32 - v.0) as usize) {
                    Some(v) => v,
                    None => continue,
                };
                match l.get((j as i32 - v.1) as usize) {
                    Some(v) => {
                        if *v == '@' {
                            c += 1
                        }
                    }
                    None => continue,
                };
            }
            if c < 4 {
                res.push((i, j));
            }
        }
    }
    if res.len() == 0 {
        return 0;
    }
    let mut nd = data.clone();
    for i in 0..nd.len() {
        for j in 0..nd[i].len() {
            if res.contains(&(i, j)) {
                nd[i][j] = '.'
                
            }
        }
    }

    return res.len() as u32 + second_part(&nd);
}
