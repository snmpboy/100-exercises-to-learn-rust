use std::fmt::Debug;
use std::sync::{Arc, RwLock};
// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;


pub fn sum(slice: &'static [i32]) -> i32
{

    let data = Arc::new(RwLock::new(0));
    let tup = slice.split_at(slice.len()/2);
    let array_2d = [tup.0, tup.1];
    for a in array_2d {
        let data_clone = data.clone();
        thread::spawn(move|| {
            for i in a {
                let mut total = data_clone.write().unwrap();
                *total += i;
            }
        }).join().unwrap();
    }
    let sum = data.read().unwrap();
    *sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
