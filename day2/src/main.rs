use std::{collections, fs};

fn main() {
    let f = fs::read_to_string("input-ex").unwrap();   
    let ranges = f.split(",");
    let mut res = 0;
    for r in ranges {
        let sr = r.strip_suffix("\n").unwrap_or(r);
        let range_d: Vec<&str> = sr.split("-").collect();
        let s: u64 = range_d[0].parse().unwrap();
        let e: u64 = range_d[range_d.len() - 1].parse().unwrap();
        for n in s..=e {
            if !is_right_second_part(n) {
                res += n;
            }
               
        }
    }
    println!("{res}")

}



fn get_dels(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..=(n / 2) {
        if n % i == 0 {
            res.push(i);
        }
    }
    return res;
}

fn is_right_first_part(n: u64) -> bool {
    let n_str: String = n.to_string();
    if n_str.len() % 2 != 0 {
        return true;
    }
    let first_part = &n_str[0..n_str.len() / 2];
    let second_part = &n_str[n_str.len() / 2..];
    if first_part == second_part {
        return false;
    }
    true
}


fn is_right_second_part(n: u64) -> bool {
    let n_str: String = n.to_string();
    let dels = get_dels(n_str.len());
    if dels.len() == 0 {
        return true;
    }

    
    for del in dels.iter() {
        let mut subs = collections::HashSet::new();
        let mut s = 0;
        let mut i = *del;
        while i <= n_str.len() {
            let a = &n_str[s..i];
            subs.insert(a);
            s = i;
            i += del;
        }
        if subs.len() == 1 {
            return false;
        }
    }
    

    return true;
}
