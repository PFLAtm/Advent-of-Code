use std::{fs, path::absolute};

fn main() {
    let mut one: Vec<i32> = vec![];
    let mut two: Vec<i32> = vec![];

    let mut data = fs::read_to_string("data.txt");

    for i in data.unwrap().lines() {
        let mut split = i.split_whitespace();
        one.push(split.next().unwrap().parse().unwrap());
        two.push(split.next().unwrap().parse().unwrap());
    }
    let mut count = 0;
    let mut sum = 0;
    for i in one {
        count = 0;
        for j in two.iter() {
            if i == *j {
                count += 1;
            }
        }
        sum += i * count;
    }
    println!("{sum}");

    /* let a = bubble(one);
     let b = bubble(two);
     let mut sum = 0;
     for i in 0..a.len() {
         sum += if a[i] - b[i] < 0 {
             (a[i] - b[i]) * -1
         } else {
             a[i] - b[i]
         };
     }
     println!("{sum}");
    */
}

fn bubble(mut l: Vec<i32>) -> Vec<i32> {
    let mut sorted = true;
    let mut a = 1;
    while sorted {
        sorted = false;
        for i in 0..l.len() - a {
            if l[i + 1] < l[i] {
                (l[i + 1], l[i]) = (l[i], l[i + 1]);
                sorted = true;
            }
        }
        a += 1;
    }
    l
}
