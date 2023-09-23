use std::time::Instant;
                                            // ASCII :: 1,2,3,4,5,6,7,8,9
const ASCII_DIGIT_TABLE: [u8; 9] = [48+1,48+2,48+3,48+4,48+5,48+6,48+7,48+8,48+9];
fn verify_pandigital(number_str: &str) -> bool {
    if number_str.len() != 9 {
        return false;
    }
    let number_bytes = number_str.as_bytes();
    for &digit in &ASCII_DIGIT_TABLE {
        if !number_bytes.contains(&digit) {
            return false;
        }
    }
    true
}
fn main() {
    let mut biggest_pandigital = 0_u32;
    let start = Instant::now();
    'out_loop: for num in 193..=std::u32::MAX {
        let mut number_str = format!("{}", num);
        if number_str.len() > 5 {
            break 'out_loop;
        }
        let mut coef = 2_u32;
        'inner_loop: while number_str.len() <= 9 {
            if verify_pandigital(&number_str) {
                let parsed_str = number_str.parse::<u32>().unwrap();
                if parsed_str > biggest_pandigital {
                    biggest_pandigital = parsed_str;
                }
                break 'inner_loop;
            } else {
                number_str.push_str(&(coef * num).to_string());
                coef += 1;
            }
        }
    }
    println!("Biggest Found = {}", biggest_pandigital);
    println!("Time Elapsed : {} ms",(start.elapsed().as_millis()))
}
