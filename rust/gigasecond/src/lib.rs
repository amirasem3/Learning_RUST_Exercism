use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {

    let final_date = start + Duration::seconds(1000000000);
    // println!("The giga second after {} is {}",start,final_date);

    final_date
}
