// TODO: should create a 'Sort' trait, each algorithm having it's own 'Impl'
pub mod insertion_sort {
    pub fn sort(mut a: Vec<isize>, n: usize) -> Vec<isize> {
        for i in 2..n {
            let key = a[i];
            let mut j = i - 1;
            while j > 0 && a[j] > key {
                a[j + 1] = a[j];
                j = j - 1;
            }
            a[j + 1] = key;
        }
        a
    }
}