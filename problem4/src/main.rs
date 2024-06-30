// A palindromic number reads the same both ways. The largest palindrome made from the product
// of two 2-digit numbers in 9009 = 91 * 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let (max_palindrome, num1, num2) = largest_palindrome_product();

    println!("The largest palindrome made from the product of 3-digit numbers is: {}: {} * {}", max_palindrome, num1, num2);
}

fn is_palindrom(num: u32) -> bool {
    let num_str = num.to_string();
    let rev_str: String =num_str.chars().rev().collect();
    num_str == rev_str
}

fn largest_palindrome_product() -> (u32, u32, u32) {
    let mut max_palindrome = 0;
    let mut num1 = 0;
    let mut num2 = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let product = i* j;
            if is_palindrom(product) && product > max_palindrome {
                max_palindrome = product;
                num1 = i;
                num2 = j;
            }
        }
    }

    (max_palindrome, num1, num2)
}
