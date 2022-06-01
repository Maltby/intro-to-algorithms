mod insertion_sort;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_sanity() {
        let input = vec!(1,3,5,2,4);
        let len = input.len();
        let output = insertion_sort::insertion_sort::sort(input, len);
        assert_eq!(vec!(1,2,3,4,5), output);
    }
}