use read_data::read_data;

mod read_data;
fn main() {
    let mut safe_count = 0;
    
    'a: for i in read_data("data.txt").iter() {
        
        let mut report:Vec<i32> = i.split_whitespace().map(|v|{
            v.parse().unwrap()
        }).collect();

        let mut decrease = true;
        if report[0] > report[1] {
            decrease = true;
        } else if report[0] < report[1] {
            decrease = false;
        }

        let mut dampened = false;
        let mut i=0;
        'b: while i < report.len()-1{
            if decrease {
                if !((report[i]-report[i+1] < 4)){
                    if !(report[i]>report[i+1]) && !dampened{
                        report.remove(i+1);
                        dampened = true;
                        i = 0;
                        continue 'b;
                    }else {
                        continue 'a;
                    }
                    
                }
            } else {
                if !((report[i+1]-report[i] < 4)) {
                    if !(report[i]<report[i+1]) && !dampened {
                        report.remove(i+1);
                        dampened = true;
                        i = 0;
                        continue 'b;
                    } else {
                        continue 'a;
                    }
                }
            }
            
            i += 1;
        }
        safe_count += 1;         
   }

   println!("{safe_count}");
   
}
