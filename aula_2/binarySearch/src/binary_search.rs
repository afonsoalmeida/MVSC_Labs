use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

struct BinSearch {
    found: bool,
    index: i32
}

/** Binary search to find the element or to find the right index
(after some arithmetics) to insert the element. */
fn binary_search(v: &Vec<i32>, mut l: i32, mut r: i32, x: i32) -> BinSearch {
    while l < r { 
        let mut m = (l+r) / 2; 
  
        if v[m as usize] == x {
            return BinSearch{found: true, index: m};
        }
        else if x < v[m as usize] {
            r = m;
        }
        else {
            l = m + 1;
        }
    } 
  
    // if we reach here, then element was 
    // not present, but returns where it should be inserted (after some arithmetic)
    return BinSearch{found:false, index:(-r)-1}; 
}

/** Prints where the element was found (if it was found) or where it can be inserted
(if it does not exist). */
fn print(res: &BinSearch, element: i32) {
    if res.found {
        println!("Found element <{}> at index {}!", element, res.index);
    } else {
        println!("Element <{}> can be inserted at index {}!", element, -(res.index+1));
    }
}

/** Prints the contents of the array. */
fn print_arr(v: &Vec<i32>) {
    for i in v.iter() {
        print!("{} ", i);
    }
    println!();
}

pub fn main() {
    let mut vec = vec![1,3,5,7,9,11,13,15];

    let mut tids = Vec::with_capacity(5);

    let arc = Arc::new(Mutex::new(vec));
    
    for i in 0..5 {
        tids.push (
            thread::spawn( {
                let carc = Arc::clone(&arc);
                move|| {
                    let mut x = carc.lock().unwrap();
                    let res = binary_search(&x, 0, 8, i);
                    print(&res, i);
                    if !res.found {
                        //Inserts in array and shifts to the right
                        x.insert(-(res.index+1) as usize, i);
                    }
                    print_arr(&x);
                } 
            }));
    }

    for tid in tids {
        tid.join().unwrap();
    }
    
}