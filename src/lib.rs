mod insertion_sort;
mod linear_search;
mod add_binary_integers;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_sanity() {
        let input = vec!(4,2,5,3,1);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_increasing(input, len);
        assert_eq!(vec!(1,2,3,4,5), output);
    }
    #[test]
    fn insertion_sort_single() {
        let input = vec!(1);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_increasing(input, len);
        assert_eq!(vec!(1), output)
    }
    #[test]
    fn insertion_sort_no_change() {
        let input = vec!(1,2,3);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_increasing(input, len);
        assert_eq!(vec!(1,2,3), output)
    }
    #[test]
    fn insertion_sort_negatives() {
        let input = vec!(1,-2,3);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_increasing(input, len);
        assert_eq!(vec!(-2,1,3), output)
    }
    #[test]
    fn insertion_sort_blank() {
        let input = vec!();
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_increasing(input, len);
        assert_eq!(vec!() as Vec<isize>, output)
    }
    #[test]
    fn insertion_sort_sanity_decreasing() {
        let input = vec!(4,2,5,3,1);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_decreasing(input, len);
        assert!(vec!(1,2,3,4,5).iter().rev().eq(output.iter()));
    }
    #[test]
    fn insertion_sort_single_decreasing() {
        let input = vec!(1);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_decreasing(input, len);
        assert_eq!(vec!(1), output)
    }
    #[test]
    fn insertion_sort_no_change_decreasing() {
        let input = vec!(1,2,3);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_decreasing(input, len);
        assert!(vec!(1,2,3).iter().rev().eq(output.iter()));
    }
    #[test]
    fn insertion_sort_negatives_decreasing() {
        let input = vec!(1,-2,3);
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_decreasing(input, len);
        assert!(vec!(-2,1,3).iter().rev().eq(output.iter()));
    }
    #[test]
    fn insertion_sort_blank_decreasing() {
        let input = vec!();
        let len = input.len();
        let output = insertion_sort::insertion_sort::monotonically_decreasing(input, len);
        assert_eq!(vec!() as Vec<isize>, output);
    }
    #[test]
    fn linear_search_sanity() {
        let input = vec!(5,5,5,5,1,5);
        let x = 1;
        let output = linear_search::linear_search::linear_search(input, x);
        assert_eq!(Some(4), output);
    }
    #[test]
    fn linear_search_insanity() {
        let input = vec!(5,5,5,5,1,5);
        let x = 2;
        let output = linear_search::linear_search::linear_search(input, x);
        assert_eq!(None, output);
    }
    #[test]
    fn add_binary_integers_sanity() {
        let a = vec!(1,0,0,1,0,1);
        let b = vec!(1,0,0,0,0,1);
        let output = add_binary_integers::add_binary_integers::add_binary_integers(a, b);
        assert_eq!(vec!(1,0,0,0,1,1,0), output);
    }
}