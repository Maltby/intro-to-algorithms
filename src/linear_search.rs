pub mod linear_search {
    pub fn linear_search(a: Vec<isize>, x: isize) -> Option<usize> {
        for i in 0..a.len() {
            if a[i] == x {return Some(i)};
        }
        return None;
    }
}