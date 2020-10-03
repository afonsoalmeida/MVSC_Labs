// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers


pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x]}
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[ self.data.len() - 1 ] != 0
        }
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into
    // a number, by removing trailing zeros. The `mut` declaration for `v` here is just like the
    // one in `let mut ...`: We completely own `v`, but Rust still asks us to make our intention of
    // modifying it explicit. This `mut` is *not* part of the type of `from_vec` - the caller has
    // to give up ownership of `v` anyway, so they don't care anymore what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        while v.len()>0 && v[v.len()-1] == 0 {
            v.pop();
        }

        let mut v_inv : Vec<u64> = Vec::<u64>::new();
        
        for e in v.iter().rev() {
            v_inv.push( *e );
        }

        BigInt { data: v_inv }
    }

    pub fn num_digits(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn non_zero_digits(&self) -> i32 {
        let mut nz = 0;
        for e in &self.data {
            if *e != 0 { nz=nz+1 }
        }
        nz
    }

    pub fn largest_digit(&self) -> u64 {
        let mut ld = 0;

        for e in &self.data {
            if *e == 9 { return 9 }
            else if *e>ld { ld = *e }
        }

        ld
    }

    pub fn smallest_digit(&self) -> u64 {
        let mut sd = 9;

        for e in &self.data {
            if *e == 0 { return 0 }
            else if *e<sd { sd = *e }
        }

        sd
    }

    pub fn print(&self) {
        for i in &self.data {
            print!("{} ", *i)
        }
        println!()
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec((&v).clone());
    for e in v {
        print!("{} ", e);
    }
    println!();
    b1.print();
    b2.print();
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`.
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the
// number of digits? The number of non-zero digits? The smallest/largest digit? Of course, these
// should all take `self` as a shared reference (i.e., in borrowed form).

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}



pub fn main() {

    let mut vec = vec![1,2,3,0,4,5,6,0];
    let bigint = BigInt::from_vec(vec);
    bigint.print();
    bigint.print();
    println!("Number of digits: {}", bigint.num_digits());
    println!("Number of non zero digits: {}", bigint.non_zero_digits());
    println!("Largest digit: {}", bigint.largest_digit());
    println!("Smallest digit: {}", bigint.smallest_digit());
}
