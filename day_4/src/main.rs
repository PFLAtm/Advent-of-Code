use std::fs;

fn main() {
    let res = get_data();
    let mut sum = 0;
    for (i,l) in res.iter().enumerate() {
        for (j,c) in l.iter().enumerate() {
            if *c == 'X' {
                if search(res.clone(), j, i, 0, 0) {
                    sum +=1;
                }
            }
        }
    }

    println!("sum: {sum}");
}

fn search(data: Vec<Vec<char>>,x: usize , y:usize, x_offset: usize, y_offset: usize) -> bool {
    let comp_vec = vec!['X', 'M', 'A', 'S'];
    let mut end = true;

    let i = 1;

    while i < comp_vec.len() {
        if data[x+x_offset][y+y_offset] == comp_vec[i]{
            end = false;
        }

        if end {
            return false;
        }
    }
     

    true
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
