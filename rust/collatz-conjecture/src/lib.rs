pub fn collatz(n: u64) -> Option<u64> {
    let mut step_counter = 0;
    let mut n_mutable = n;
    if n == 0 { return None; }
    while n_mutable != 1 {
        if n_mutable % 2 == 0 {
            n_mutable /= 2;
            step_counter += 1;
            continue;
        } else {
            n_mutable = n_mutable * 3 + 1;
            step_counter += 1;
        }
    }
    Some(step_counter)
}
