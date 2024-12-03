use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let sum = 0;

    let mut num_a = String::new();
    let mut num_b = String::new();

    let comp_vec = vec!['m', 'u', 'l', '(', ',', ')'];

    let mut comp_index = 0;
    let mut arg_vec: Vec<char> = vec![];
    let mut args_taken = false;
    let mut old_index = comp_index;
    for c in data.chars().into_iter() {
        if c == comp_vec[comp_index] {
            comp_index += 1;
        } else {
            comp_index = 0;
            arg_vec = vec![];
        }

        if !(args_taken && comp_vec[comp_index] == '(') {
            arg_vec.push(c);
        }
        if old_index != comp_index {
            old_index = comp_index;
            args_taken = true;
            if comp_vec[comp_index] == '(' {
                arg_vec.iter().for_each(|v| {
                    num_a.push(*v);
                });
                arg_vec = vec![];
            } else {
                arg_vec.iter().for_each(|v| {
                    num_b.push(*v);
                });

                arg_vec = vec![];
            }
        }
        println!("{}", comp_vec[comp_index]);
        if comp_vec[comp_index] == ')' {
            println!("a: {num_a}, b: {num_b}");
            comp_index = 0;
        }
    }
}
