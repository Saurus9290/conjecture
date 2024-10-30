pub fn collatz(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0=> break,
            1 => return Some(i),
            even if even % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        
        }
    }
    None
}

fn main() {
    let test_numbers = vec![1, 6, 19, 27, 0];

    for &number in &test_numbers {
        match collatz(number) {
            Some(steps) => println!("Number {} reaches 1 in {} steps.", number, steps),
            None => println!("Number {} does not reach 1 within safe bounds.", number),
        }
    }
}