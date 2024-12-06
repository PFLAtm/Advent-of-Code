use std::fs;

fn main() {
    let res = get_data();
    println!("{:#?}", res);
}

fn search(y_offset: i32, x_offset: i32) -> bool {
    let comp_vec = vec!['X', 'M', 'A', 'S'];
}

fn get_data() -> Vec<Vec<char>> {
    let raw_data = fs::read_to_string("data.txt").unwrap();
    let mut res = vec![vec![]];

    for (i, line) in raw_data.lines().enumerate() {
        res.push(vec![]);
        for char in line.chars() {
            res[i].push(char);
        }
    }

    res
}
