use std::{fs, i32, usize};
fn main() {
    let res = get_data();
    let mut sum = 0;
    
    for (y_i,y) in res.iter().enumerate() {
        for (x_i,_x) in y.iter().enumerate(){
            if search(res.clone(), x_i as i32, y_i as i32, -1, 0){
                sum +=1;
            }
        }
    }
    

    println!("sum: {sum}");
}

fn search(data: Vec<Vec<char>>, mut x: i32, mut y: i32, x_offset: i32, y_offset: i32) -> bool {
    let comp_vec = vec!['X', 'M', 'A', 'S'];
    let mut end;
    for i in 1..comp_vec.len() {
        end = true;
        if x < data[y as usize].len() as i32 - 1 && x > 0 && y<data.len() as i32 -1 && y > 0  {
            if data[(y + y_offset) as usize][(x + x_offset) as usize] == comp_vec[i] {
                end = false;
                x += x_offset;
                y += y_offset;
            }
        }

        if end {
            return false;
        }
    }

    println!("x:{x},y:{y}");

    true
}

/*fn search(data: Vec<Vec<char>>,x: usize , y:usize, x_offset: usize, y_offset: usize) -> bool {
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
}*/

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
