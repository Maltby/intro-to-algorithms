// TODO: should create a 'Sort' trait, each algorithm having it's own 'Impl'
pub mod insertion_sort {
    pub fn monotonically_increasing(mut a: Vec<isize>, n: usize) -> Vec<isize> {
        // [6, 5]
        for i in 1..n { // 1
            let key = a[i]; // '5'
            let mut j = i; // 1
            while j > 0 && a[j - 1] > key {
                j-=1; // 0
                a[j + 1] = a[j]; // [6, 6]
            }
            // j = 0, key = '5'
            a[j] = key; // [5, 6]
        }
        a
    }
    pub fn monotonically_decreasing(mut a: Vec<isize>, n: usize) -> Vec<isize> {
        // [6, 5]
        for i in 1..n { // 1
            let key = a[i]; // '5'
            let mut j = i; // 1
            while j > 0 && key >= a[j - 1] { // key = 5, a[j-1] = 6
                j-=1; // 0
                a[j + 1] = a[j]; // [6, 6]
            }
            // j = 0, key = '5'
            a[j] = key; // [5, 6]
        }
        a
    }
}