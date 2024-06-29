fn main() {
    let mut sum = 0;
    let limit = 1000;

    for num in 1..limit {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }

    println!("Sum of multiples of 3 or 5 below {}: {}", limit, sum);
}

//If we list all the natural numbers below 10 that are multiples of 3 or 5 we get 3,5,6, and 9.
// The sum of these multiples is 23. Find the sum of all the multiples of 3 or 5 below 1000.