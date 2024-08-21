pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_facors = Vec::new();
    let mut input_mut  = n;
    let mut second_loop_counter = 3;
    if n == 1 { return prime_facors}
    else {
        while input_mut % 2 ==0 {
            prime_facors.push(2);
            input_mut /=2;
        }

            for i in (3..=(input_mut as f64).sqrt() as u64).step_by(2) {
                while input_mut % i == 0 {
                    prime_facors.push(i);
                    input_mut = input_mut / i;
                }
            }

        if input_mut > 2 {
            prime_facors.push(input_mut);
        }
        return prime_facors;
        
    }


    // todo!("This should calculate the prime factors of {n}")
}
