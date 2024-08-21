use rand::Rng;


pub fn private_key(p: u64) -> u64 {
    let mut private_key_selctor = rand::thread_rng();
    let random_number : u64 = private_key_selctor.gen_range(2..=p-1);

    random_number
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {  // If exp is odd, multiply the base with the result
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus; // Square the base
    }

    result
}