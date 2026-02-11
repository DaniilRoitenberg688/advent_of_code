use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let splited: Vec<&str> = input.split("\n").collect();
    let ind = find_ranges_and_nums(splited);
    let ranges = ind.0;
    let newr = ranges.clone();
    let nums = ind.1;
    let result1 = first_part(ranges, nums);
    println!("{result1}");

    let result1 = second_part(newr);
    println!("{result1}")



}

fn find_ranges_and_nums(input: Vec<&str>) ->  (Vec<Vec<u64>>, Vec<u64>) {
    let mut is_nums = false;
    let mut ranges: Vec<Vec<u64>> = Vec::new();
    let mut nums: Vec<u64> = Vec::new();
    for line in input.iter() {
        match *line {
            "" => is_nums = true,
            _ => {
                if is_nums {
                    let n: u64 = line.parse().unwrap(); 
                    nums.push(n);
                } else {
                    let range: Vec<u64> = line.split("-").map(|x| x.parse().unwrap()).collect();
                    ranges.push(range);
                }
            }
        }
    }
    return (ranges, nums)
}




fn first_part(ranges: Vec<Vec<u64>>, nums: Vec<u64>) -> u64 {
    let mut res = 0;
    for n in nums.iter() {
        for r in ranges.iter() {
            if r[0] <= *n && *n <= r[1] {
                res += 1;
                break;
            }
        }
    }
    return res
}


fn match_to_inters(r: &Vec<u64>, i: &Vec<u64>) -> Option<Vec<u64>> {
    if r[0] >= i[0] && r[1] <= i[1] {
        return Some(vec![i[0], i[1]]) 
    }
    if r[0] < i[0] && r[1] > i[1] {
        return Some(vec![r[0], r[1]])
    }
    if r[0] < i[0] && r[1] >= i[0] {
        return Some(vec![r[0], i[1]])
    }
    if r[1] > i[1] && r[0] <= i[1] {
        return Some(vec![i[0], r[1]])
    }
    None
}


fn second_part(ranges: Vec<Vec<u64>>) -> u64 {
    let mut allr: HashSet<Vec<u64>> = HashSet::new();
    allr.insert(ranges[0].clone());
    for r in ranges.iter() {
        let mut k = r.clone();
        for i in allr.clone().iter() {
            //println!("{:?}", k);
            match match_to_inters(&k, i) {
                Some(l) => {
                    allr.remove(&k);
                    k = l.clone();
                    allr.remove(i);
                    allr.insert(l);
                }
                None => {
                    allr.insert(k.to_vec());
                }
            }

        }
        //println!("\n");
    }
    println!("{:?}", allr);
    let mut res = 0;
    for v in allr.iter() {
        res += v[1] - v[0] + 1
    }
    res

}



