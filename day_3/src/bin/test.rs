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
    let mut end = false;
    for c in data.chars().into_iter() {
        if c == comp_vec[comp_index] {
            comp_index += 1;
        } else {
            end = true;
        }

        if !(args_taken && ((comp_vec[comp_index] == '(') || (comp_vec[comp_index] == ','))) {
            arg_vec.push(c);
            end = false;
        }
        if old_index != comp_index {
            old_index = comp_index;
            args_taken = true;
            if comp_vec[comp_index] == '(' {
                arg_vec.iter().for_each(|v| {
                    num_a.push(*v);
                });
                println!("arg: {:?}",num_a);
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
            //comp_index = 0;
        }
        

        if end && !(comp_vec[comp_index] == ')'){
            comp_index = 0;
            arg_vec = vec![];
        }
    }
}
/*fn enable(c:char, enable_index: &mut usize, disable_index: &mut usize, is_do: &mut bool, is_dont: &mut bool) -> EnableCmd {
    //let comp_do = vec!['d','o'];
    let comp_disable = vec!['d','o','n','\'','t','(',')',' '];
    let comp_enable = vec!['d','o','(',')',' '];
    
    if c == comp_enable[*enable_index]{
        *enable_index += 1;
        *is_do = true;
    } else {
        *is_do = false;
        *enable_index = 0;
    }

    if c == comp_disable[*disable_index] {
        *disable_index += 1;
        *is_dont = true;
    } else {
        *is_dont = false;
        *disable_index = 0;
    }

    
    println!("enable_index: {enable_index}, comp_char: {},char: {c}, end: {is_do}",comp_enable[*enable_index]);





    if *is_do && *enable_index == 4{
        *enable_index = 0;
        EnableCmd::Do
    } else if *is_dont && *disable_index == 7 {
        *enable_index = 0;
        EnableCmd::Dont
    } else {
        *enable_index = 0;
        EnableCmd::None
    }
    


}*/
