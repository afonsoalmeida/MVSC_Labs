use std::thread;

fn binary_search(v: &Vec<i32>, mut l: i32, mut r: i32, x: i32) -> i32 {
    while l <= r { 
        let mut m = l + (r - l) / 2; 
  
        if v[m as usize] == x {
            return m;
        }
  
        if v[m as usize] < x {
            l = m + 1;
        }
        else {
            r = m - 1;
        }
    } 
  
    // if we reach here, then element was 
    // not present 
    return -1; 
}

fn print(val: i32, element: i32) {
    if val != -1 {
        println!("Found element <{}> at index {}!", element, val);
    } else {
        println!("Could not find element <{}> in the array!", element);
    }
}

pub fn main() {
    let vec = vec![1,3,5,7,9,11,13,15];

    //let res = binary_search(&vec,0,(vec.len()-1) as i32,11);

    let mut tids = Vec::with_capacity(5);

    for i in 0..5 {
        let cvec= vec.clone();
        tids.push( thread::spawn(move|| {
            print( binary_search(&cvec,0,(&cvec.len()-1) as i32, i), i);
        }));
    }

    for tid in tids {
        tid.join().unwrap();
    }
    /*
    let t1 = thread::spawn(move || {
       print(binary_search(&vec,0,(&vec.len()-1) as i32,11));
    });

    let t2 = thread::spawn(move || {
        print(binary_search(&vec2,0,(&vec2.len()-1) as i32,16));
    });

    t1.join().unwrap();
    t2.join().unwrap();
    */
    
}