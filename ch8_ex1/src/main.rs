#[derive(Debug)]
struct Result {
    mean : i32,
    median : i32,
    mode : i32
}

fn get_mean(l: &Vec<i32>) -> i32 {
    let mut acc = 0;
    for i in l {
        acc += i;
    }
    acc / (l.len() as i32)
}

fn get_median(l: &Vec<i32>) -> i32 {
    let mut sorted = l.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

fn get_mode(l: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut hist : HashMap<i32, i32> = HashMap::new();
    for i in l {
        let e = hist.entry(*i).or_insert(0);
        *e += 1;
    }
    let mut max = (0, 0);
    for (k,v) in hist.iter() {
        if *v > max.1 {
            max.1 = *v;
            max.0 = *k;
        }
    }
    max.0
}

fn process_list(l : &Vec<i32>) -> Result {
    Result{ mean: get_mean(l), median: get_median(l), mode: get_mode(l) }
}

fn main() {
    let v = vec![2, 51, 99, 1, 342, 99, 23, 55, 1, 75, 99];
    let result = process_list(&v);
    println!("Input vector is {:?}", v);
    println!("Result is {:?}", result);
}
