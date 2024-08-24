

pub fn plants(diagram: &str, student: &str) -> Vec<String> {
    let mut first_line = String::new();
    let mut second_line  = String::new();
    let start_second_line  = diagram.len()/2;
    let each_line_start_index = student_decoder(student);
    let diagram_string = String::from(diagram);
    let mut vector_result = Vec::new();
    first_line = diagram_string[0..start_second_line].to_string();
    second_line  =diagram_string[start_second_line +1..diagram_string.len()].to_string();
    let mut first_line_plants = first_line[each_line_start_index..each_line_start_index+2].to_string();
    let mut second_line_plants = second_line[each_line_start_index..each_line_start_index+2].to_string();
    for c in first_line_plants.chars(){
        vector_result.push(plant_decoder(c));
    }
    for c in second_line_plants.chars() {
        vector_result.push(plant_decoder(c));
    }
    vector_result
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for");
}

pub fn plant_decoder (code:char) -> String{

    match code {
        'V' => String::from("violets"),
        'R' => String::from("radishes"),
        'C' => String::from("clover"),
        'G' => String::from("grass"),
        _ => String::from("Invalid Plant Code!")
    }
}

pub fn student_decoder (student: &str) -> usize {
    // Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, and Larry.
    match student {
        "Alice"=>0,
        "Bob" => 2,
        "Charlie" =>4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" =>16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => 25
    }
}
