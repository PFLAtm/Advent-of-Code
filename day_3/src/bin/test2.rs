fn main() {
    let mut a = 10;
    change(&mut a);
    println!("{a}");
}

fn change(i:&mut i32){
    *i = 5;
}