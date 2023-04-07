#![allow(dead_code, unused_variables)]

use rand::Rng;

fn fill_rnd_vec(vec: &mut Vec<i32>, range: (i32, i32), size: i32) {
    let (bot, top) = range;

    for _ in 0..size {
        let num = rand::thread_rng().gen_range(bot..=top);
        vec.push(num);
    }
    vec.sort();
}

fn binsearch(vec: &Vec<i32>, val: i32, left: usize, right: usize) -> Result<usize, String> {
    let mid: usize = (left + right) / 2;
    let cur = vec[mid];
    let err = format!("not found! mid: {}, left: {}, right: {}", mid, left, right);

    if val == cur {
        return Ok(mid);
    } else if left > right {
        return Err(err);
    } else if val < cur {
        return binsearch(vec, val, left, mid - 1);
    } else if val > cur && mid < vec.len() {
        return binsearch(vec, val, mid + 1, right);
    }
    return Err(err);
}

fn binsearch_use_opt(vec: &Vec<i32>, val: i32, left: usize, right: usize) -> Option<usize> {
    let mid: usize = (left + right) / 2;
    let cur = vec[mid];

    if val == cur {
        return Some(mid);
    } else if left > right {
        return None;
    } else if val < cur {
        return binsearch_use_opt(vec, val, left, mid - 1);
    } else if val > cur && mid < vec.len() {
        return binsearch_use_opt(vec, val, mid + 1, right);
    }
    return None;
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let range = (0, 20);
    let size = 20;
    let val = rand::thread_rng().gen_range(range.0..=range.1);

    fill_rnd_vec(&mut vec, range, size);

    println!("vec: {:?}", vec);
    println!("val: {:?}", val);

    match binsearch(&vec, val, 0, vec.len()) {
        Ok(i) => println!("found it! index: {:?}", i),
        Err(err) => println!("{}", err),
    }

    match binsearch_use_opt(&vec, val, 0, vec.len()) {
        Some(i) => println!("found it! index: {:?}", i),
        None => println!("Not found!"),
    }
}
