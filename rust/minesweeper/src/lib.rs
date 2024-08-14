pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let number_of_rows  = minefield.len();
    let number_of_columns = minefield.get(0).map_or(0, |row| row.len());
    
    let mut result  = Vec::with_capacity(number_of_rows);

    for (y, row) in minefield.iter().enumerate() {
        let mut annotated_row  = String::with_capacity(number_of_columns);
        for (x,&cell) in row.as_bytes().iter().enumerate() {
            if cell == b'*' { 
                annotated_row.push('*');
            }else { 
                let mut mine_count = 0;

                for dy  in -1..=1 {
                    for dx in -1..=1{
                        if dy == 0 && dx == 0 { continue; }
                        let ny = y.wrapping_add(dy as usize);
                        let nx = x.wrapping_add(dx as usize);
                        
                        if ny < number_of_rows && nx < number_of_columns && minefield[ny].as_bytes()[nx] == b'*' {
                            mine_count+=1;
                        }
                    }
                }
                if mine_count > 0 {
                    annotated_row.push((b'0' + mine_count as u8) as char);
                }else {
                    annotated_row.push(' ');
                }
            }
        }
        result.push(annotated_row);
    }

    result

}


