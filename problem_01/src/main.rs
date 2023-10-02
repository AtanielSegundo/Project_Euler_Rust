use std::{fmt::Display, ops::Range, process::exit};

fn problem_1() {
    let numbers_arr: Vec<u64> = Range::<u64> {
        start: 3,
        end: 1000,
    }
    .collect();
    let only_multiples_sum: u64 = numbers_arr
        .into_iter()
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .sum();
    println!("The sum of numbers is = {}", only_multiples_sum)
}

fn problem_2() {
    let mut fib_slider = [1, 2];
    let mut fib_sum: u64 = 0;
    while (fib_slider[1] <= 4_000_000) {
        if fib_slider[1] % 2 == 0 {
            fib_sum += fib_slider[1];
        }
        fib_slider = [fib_slider[1], fib_slider[0] + fib_slider[1]];
    }
    println!("The sum of Fibonacci even-valued terms = {}", fib_sum);
}

fn problem_3() {
    fn is_prime(num: &i64) -> bool {
        if num % 2 == 0 {
            return false;
        }
        for factor in 2..=num / 2 {
            if num % factor == 0 {
                return false;
            }
        }
        return true;
    }
    const _number: i64 = 600851475143;
    let mut biggest_prime_factor = 1_i64;
    let mut aux = (_number as f64).sqrt() as i64;
    loop {
        if _number % aux == 0 && is_prime(&aux) {
            biggest_prime_factor = aux;
            break;
        }
        aux -= 1;
    }
    println!(
        "The biggest factor of {} is {}",
        _number, biggest_prime_factor
    );
}

fn problem_4() {
    fn is_palindrome<T: Display>(number: &T) -> bool {
        let string = format!("{}", number);
        if string == string.chars().rev().collect::<String>() {
            return true;
        }
        return false;
    }
    let num_arr = Range {
        start: 100,
        end: 999,
    }
    .collect::<Vec<i32>>();
    let mut biggest_prime = 0_i32;
    for rhs in &num_arr {
        for lhs in &num_arr {
            let product = rhs * lhs;
            if is_palindrome(&product) {
                biggest_prime = if product > biggest_prime {
                    product
                } else {
                    biggest_prime
                };
                break;
            }
        }
    }

    println!("The biggest 3 digits palindrome is {}", biggest_prime);
}

fn problem_5() {
    let divisisor_arr = (2..=20).rev().collect::<Vec<i32>>();
    dbg!(&divisisor_arr);
    'out_loop: for i in (20 * 2)..std::i32::MAX {
        for divisor in &divisisor_arr {
            if i % divisor != 0 {
                break;
            }
            if divisor == &divisisor_arr[divisisor_arr.len() - 1] {
                println!(
                    " the smallest positive number that is evenly divisible is {}",
                    i
                );
                break 'out_loop;
            }
        }
    }
}

fn problem_6() {
    let first_hundred = (1..=100).collect::<Vec<u64>>();
    let sum_squareds = first_hundred.iter().map(|x| x.pow(2)).sum::<u64>();
    dbg!(&sum_squareds);
    let squared_sum = first_hundred.iter().sum::<u64>().pow(2);
    println!(
        "square of sum {} - sum of squared {} = {}",
        &squared_sum,
        &sum_squareds,
        squared_sum - sum_squareds
    )
}

fn problem_7() {
    fn is_prime(num: &u64) -> bool {
        if *num < 4 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }
        for factor in 2..=num / 2 {
            if num % factor == 0 {
                return false;
            }
        }
        return true;
    }

    let mut prime_count = 0;
    for u in 1..=u64::MAX {
        if is_prime(&u) {
            println!("{}", &u);
            prime_count += 1;
        }
        if prime_count == 10_002 {
            println!("the 10_001st prime number is {}", u);
            break;
        }
    }
}

fn problem_8() {
    let splited_vec: Vec<&str> = Vec::new();
    let mut min = 0;
    let mut max = 12;
    let mut greatest_product = 0;
    let digit_1000_str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    'iterate: loop {
        if max == digit_1000_str.len() {
            break 'iterate;
        }
        let mut into_vec: Vec<u64> = Vec::new();
        for char in String::from(&digit_1000_str[min..=max]).chars() {
            //println!("{}", char.to_digit(10).unwrap());
            into_vec.push(char.to_digit(10).unwrap() as u64)
        }
        let product = {
            let mut aux = 1_u64;
            for i in into_vec {
                aux *= i;
            }
            aux
        };
        greatest_product = if product > greatest_product {
            product
        } else {
            greatest_product
        };
        min += 1;
        max += 1;
    }
    println!("greatest product = {}", greatest_product);
}

fn problem_9() {
    let triplet_check = |a: f64, b: f64, c: f64| (a + b + c) == 1000.0;
    let case_true = |a: u64, b: u64, c: u64| a * b * c;

    for b in (1..=500) {
        for a in 1..b {
            let c = (((a as u64).pow(2) + (b as u64).pow(2)) as f64).sqrt();
            if triplet_check(a as f64, b as f64, c) && (a < b) && ((b as f64) < c) {
                println!("a = {} , b = {} , c = {}", a, b, c);
                println!("{}", case_true(a, b, c as u64));
                exit(0);
            }
        }
    }
}

fn problem_10() {
    fn is_prime(num: u64) -> bool {
        if num < 4 {
            return num > 1;
        }
        if num % 2 == 0 {
            return false;
        }
        let mut factor = 3;
        while factor * factor <= num {
            if num % factor == 0 {
                return false;
            }
            factor += 2;
        }
        true
    }
    let mut sum: u128 = 2;
    for i in (1..=2_000_000_u64).step_by(2) {
        if is_prime(i) {
            sum += i as u128;
        }
    }
    println!("sum = {}", sum);
}

fn problem_11() {
    const UP: [i64; 2] = [-1, 0];
    const DOWN: [i64; 2] = [1, 0];
    const LEFT: [i64; 2] = [0, -1];
    const RIGHT: [i64; 2] = [0, 1];
    const DIAGONAl_PP: [i64; 2] = [1, 1];
    const DIAGONAl_NN: [i64; 2] = [-1, -1];
    const DIAGONAl_PN: [i64; 2] = [1, -1];
    const DIAGONAl_NP: [i64; 2] = [-1, 1];

    
    const directions: [[i64; 2];8] = [UP, DOWN, LEFT, RIGHT, DIAGONAl_PP,DIAGONAl_NN,DIAGONAl_PN,DIAGONAl_NP];

    let matrix_str = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

    let matrix_arr: Vec<u64> = matrix_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let matrix_chunked: Vec<_> = matrix_arr.chunks(20).collect();

    let matrix_len = matrix_chunked.len() as i64;
    let max_row = (matrix_len - 1) as i64;
    let max_col = (matrix_chunked[0].len() - 1) as i64;

    let validation = |arr:[i64;2]| -> bool {
        let row = arr[0];
        let col = arr[1];
        return (row >= 0) && (row <= max_row) && (col >= 0) && (col <= max_col);
    };

    let mut max_product = 1 as u64;
    let mut max_locations:[[i64;2];4] = [[0,0],[0,0],[0,0],[0,0]];

    for row in 0..=max_row {
        for col in 0..=max_col {
            for direction in directions {
                let r_hat = direction[0];
                let c_hat = direction[1];
                let possible_states  = [
                    [row,col],
                    [row + 1*r_hat, col + 1*c_hat],
                    [row + 2*r_hat, col + 2*c_hat],
                    [row + 3*r_hat, col + 3*c_hat],
                ];
                if possible_states.into_iter().all(validation) == true{
                    let mut aux_product = 1 as u64;
                    for valid_state in possible_states{
                        let row = valid_state[0] as usize;
                        let col = valid_state[1] as usize;
                        aux_product *= matrix_chunked[row][col]
                    }
                    if aux_product >= max_product{
                        max_product = aux_product;
                        max_locations = possible_states;
                    }
                }
            }
        }
    }
    println!("The biggest product among all directions is : {}",max_product);
    println!("factor, location: [ row ][ column ]");
    for location in max_locations{
        let row = location[0] as usize;
        let col = location[1] as usize;
        println!("  {}  , location: [  {} ][    {}   ]",matrix_chunked[row][col],row,col)
    }
}

fn main() {
    problem_11();
}
