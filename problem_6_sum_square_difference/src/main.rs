//The sum of the squares of the first ten natural numbers is, 1^2+2^2+ ... + 10^2 = 385
//The square of the sum of the first ten natural numbers is, (1+2+3+ ... +10)^2 = 3025
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is, 3025 -385 = 2640
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum


fn main() {
    let n = 100;

    let sum_of_squares: u64 = (1..=n).map(|x| x * x).sum();

    let square_of_sum: u64 = (1..=n).sum::<u64>().pow(2);

    let difference = square_of_sum - sum_of_squares;

    println!("The difference between the sum of the squares and the square of the sum for the first {} natural numbers is: {}", n, difference);
}

