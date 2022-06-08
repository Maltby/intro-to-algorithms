pub mod add_binary_integers {
    pub fn add_binary_integers(mut a: Vec<u8>, mut b: Vec<u8>) -> Vec<u8> {
        a.reverse();
        b.reverse();

        let mut overflow: u8 = 0;
        let mut c: Vec<u8> = vec!();
        for i in 0..a.len() {
            let overflow_c = overflow;
            overflow = 0;
            let sum = a[i] + b[i] + overflow_c;
            match sum {
                0 => { c.push(0) },
                1 => { c.push(1) },
                2 => {
                    c.push(0);
                    overflow = 1;
                }
                3 => {
                    c.push(1);
                    overflow = 1;
                }
                _ => panic!()
            }
        }
        if overflow == 1 { c.push(1) }
        c.reverse();
        return c;
    }
}