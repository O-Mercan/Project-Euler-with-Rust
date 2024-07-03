//The sum of the primes below 10 is  2+3+5+7=17.
//Find the sum of all primes below two million

fn main() {
    let limit = 2000000;
    let mut primes = vec![true; limit];
    let mut sum = 0;

    for num in 2..limit{
        if primes[num] {
            sum += num;
            let mut multiple = num * num;{
                while multiple < limit {
                    primes[multiple] = false;
                    multiple += num;
                }
            }
        }
    }

    println!("the sum of all primes below two million is {}", sum);
}

