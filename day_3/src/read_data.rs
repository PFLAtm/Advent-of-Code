use std::fs;

pub fn read_data(file_path:&str) -> Vec<String>{
    let data = fs::read_to_string(file_path).expect("string reading failed");
    let mut res = Vec::new();
    for i in data.lines() {
        res.push(i.to_string());
    }
    res

}