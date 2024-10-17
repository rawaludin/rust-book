pub fn fibonacci(n: u32) -> u64 {
    if n >= 1 {
        if n == 1 {
            return 1;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    } else {
        0
    }
}
