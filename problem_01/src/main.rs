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

fn main() {
    problem_10();
}
