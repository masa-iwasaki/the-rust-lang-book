use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(3);
    v.push(4);
    v.push(4);
    v.push(4);

    v.sort();

    println!("Mean: {}", &mean(&v));
    println!("Median: {}", &median(&v));
    println!("Mode: {}", &mode(&v));
}

fn mean(values: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in values {
        sum += i;
    }

    sum
}

fn median(values: &Vec<i32>) -> i32 {
    values[values.len()/2]
}

fn mode(values: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut mode:i32 = 0;
    let mut max_count:i32 = 0;

    for i in values {
        let count = counts.entry(*i).or_insert(1);
        *count += 1
    }

    for (key, value) in &counts {
        if value > &max_count {
            mode = *key;
            max_count = *value
        }
    }

    mode
}
