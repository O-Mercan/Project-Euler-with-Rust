// the prime factors of 13195 are 5, 7, 13, 29.
// What is the largest prime factor of the number 600851475143?
fn main() {
    let mut number: i64 = 600851475143;
    let mut factor: i64 = 2;
    let mut largest_factor: i64 =1;

    while number >1 {
        if number % factor == 0 {
            largest_factor = factor;
            while number % factor == 0{
                number /= factor;
            }
        }
        factor += 1;
    }

    println!("The largest prime factor is {}", largest_factor);
}
