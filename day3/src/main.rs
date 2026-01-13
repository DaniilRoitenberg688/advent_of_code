use std::fs;


fn main() {
    let f = fs::read_to_string("input").unwrap();   
    let d: Vec<&str> = f.split("\n").collect();
    let p = d.clone();
    let res = first_part(d);
    println!("{}",res);
    let res = second_part(p);
    println!("{}",res);
}


fn index_of<T>(d: &Vec<T>, i: T) -> Option<usize> 
where 
    T: PartialEq
{
    for index in 0..d.len() {
        if d[index] == i {
            return Some(index);
        }
    }
    None

}

fn find_max(l: Vec<u64>, index: usize) -> u64 {
    if l.len() == 1{
        return l[0]
    } 
    if index == 1 {
        return *l.iter().max().unwrap();
    }
    if index == 0 {
        return 0;
    }
    let ca = l[0..(l.len()-index+1)].to_vec();
    let mut m = ca.iter().max().unwrap();
    if l.len() <= index {
        m = &l[0];
    }
    let ind = index_of(&l, *m).unwrap();
    let left = l[ind+1..l.len()].to_vec();
    println!("array {:?}", l);
    println!("cutted array {:?}", ca);
    println!("max number found in array {:?}", m);
    println!("index: {:?}", index);
    println!("left: {:?}", left);
    println!();
    return m * 10_u64.pow(index as u32 - 1) + find_max(left, index-1);

}






fn second_part(data: Vec<&str>) -> u64 {
    let mut res = 0;
    for line in data.iter() {
        if *line == ""{
            continue;
        };
        println!("{line}");
        let line_ints: Vec<u64> = line.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
        let c = find_max(line_ints, 12);
        println!("{c}");
        res += c
    };
    return res;
}


fn first_part(data: Vec<&str>) -> u32 {
    let mut res = 0;
    for line in data.iter() {
        if *line == ""{
            continue;
        };
        let chars_line: Vec<char> = line.chars().collect();
        let mut c = 0;
        for x in 0..line.len() {
            for y in 1+x..line.len() {
                let fs: u32 = chars_line[x].to_digit(10).unwrap(); 
                let ss: u32 = chars_line[y].to_digit(10).unwrap(); 
                let num = fs * 10 + ss;
                if num > c {
                    c = num
                }
            }
        }
        res += c
    }
    return res;
}

