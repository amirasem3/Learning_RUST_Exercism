pub fn nth(n: u32) -> u32 {
    if n == 0 {
        2;
    }
   let mut primes :Vec<u32> = Vec::new();
    
    primes.push(2);
    let mut i = 3;
    while primes.len() <= n as usize {
        if !is_multiply_of(&i, &primes){
            primes.push(i)
        }
        i+=1;
    }
    primes[n as usize]
}

fn is_multiply_of(i: &u32, primes:&Vec<u32>) -> bool{
    for prime in primes {
        if i % prime == 0 {
            return true;
        }
    }
    false
}
