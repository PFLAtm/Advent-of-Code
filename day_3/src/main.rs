use std::fs;


enum EnableCmd{
    Do,
    Dont,
    None

}

fn main(){
    let data = fs::read_to_string("data.txt").unwrap();
    //println!("{data}");

    let comp_vec = vec!['m', 'u', 'l', '(', ',', ')',' '];
    let comp_disable = vec!['d','o','n','\'','t','(',')',' '];
    let comp_enable = vec!['d','o','(',')',' '];
    
    let mut comp_index = 0;

   /*let mut enable_index = 0;
    let mut disable_index = 0;
    let mut is_do;
    let mut is_dont ;*/

    let mut num_a = String::new();
    let mut num_b = String::new();
    let mut end;
    let mut sum = 0;
    let mut enabled=true;

    for (_i,c) in data.chars().into_iter().enumerate() {
        
        
     


        
     
        if c == comp_vec[comp_index] {
            comp_index += 1;
            end = false;
        } else {
            end = true;
        }

        if comp_index == 4 && c.is_numeric() {
            num_a.push(c);
            end = false;
            
            
        }

        if comp_index == 5 && c.is_numeric() {
            num_b.push(c);
            end = false;
        }


        //println!("durchgang: {_i}, comp_index: {comp_index}, char: {c}, comp_char: {}, end: {end}",comp_vec[comp_index]);

        

        if comp_index == 6 && enabled {
            println!("num_a: {num_a}, num_b: {num_b}");
            sum += num_a.parse::<i32>().unwrap() * num_b.parse::<i32>().unwrap();
            end = true;
        }
        if end {
            comp_index = 0;
            num_a = String::new();
            num_b = String::new();
        }



    }

    println!("sum: {sum}");


}




