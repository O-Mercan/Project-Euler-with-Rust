// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisble by all of the numbers from 1 to 20? 

fn main() {
    let n = 20;
    let result = smallest_multiple(n);
    println!("The smallest positive number that is evenly divisible by all of the numbers from 1 to {} is: {}", n, result);
}

fn gcd(num1: u32, num2: u32) -> u32 {
    if num2 == 0 {
        return num1;
    }
    gcd(num2, num1 % num2)
}

fn lcm(num1: u32, num2: u32) -> u32 {
    (num1 * num2) / gcd( num1, num2)
}

fn smallest_multiple(num: u32) -> u32 {
    (1..num).fold(1, |acc, x| lcm(acc, x))
}