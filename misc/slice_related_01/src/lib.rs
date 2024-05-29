
#[cfg(test)]
mod tests {
    #[test]
    fn test_general_slice_01() {
        let a = [1, 2, 3, 4, 5];
        
        assert_eq!(&a[1..3], &[2, 3]);
    }

    #[test]
    fn test_mutability_of_string_on_heap_01() {
        let mut s = String::from("hello");
        s.push_str(", world!");

        assert_eq!(s, "hello, world!");
    }
}
