//By listing the first six prime numbers: 2,3,5,7,11 and 13, we can see that the 6th prime is 13.
//What is the 100001st prime number?

fn main() {
    let nth = 100001;
    let prime = nth_prime(nth);

    println!("The {}th prime number is {}", nth, prime);
}

fn is_prime(n: u64) ->bool {
    if n <= 1 {
        return false
    }
    if n <= 3 {
        return true
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false
    } 
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn nth_prime(n:usize) -> u64 {
    let mut count = 0;
    let mut candidate = 1;
    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}