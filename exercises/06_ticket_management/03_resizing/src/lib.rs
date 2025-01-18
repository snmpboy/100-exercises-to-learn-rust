#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(5);
        let g =  mem::size_of::<Vec<i32>>();
        let b = mem::size_of::<Vec<i32>>() + v.capacity() * std::mem::size_of::<i32>();
        v.push(1);
        v.push(2);
        v.push(2);
        v.push(2);
        v.push(2);// max capacity reached
        assert_eq!(v.capacity(), 5);

        v.push(3); // beyond capacity, needs to resize
        let a = mem::size_of::<Vec<i32>>() + v.capacity() * std::mem::size_of::<i32>();

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        assert_eq!(v.capacity(), 10);
    }
}
