pub fn square_of_sum(n: u32) -> u32 {
     one_to_n_sum(n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result :u32 = 0;
    for i in 1..=n  {
        result += i.pow(2)

    }
    result
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);

    square_of_sum - sum_of_squares
}

pub fn one_to_n_sum (n: u32) -> u32{
    let mut result : u32 = 0;
    for i   in 1..=n {
        result += i;
    }
    result
}