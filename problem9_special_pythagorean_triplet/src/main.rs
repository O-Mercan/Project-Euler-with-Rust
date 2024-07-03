//A pythagorean triplet is a set of three naturel numbers, a<b<c, for which, a^2+b^2=c^2
// for example, 3^2+4^2 = 9+16=25=5^2.
//There exist exactly one pythagorean triplet for which a+b+c=100.
//find the product abc.

fn main() {
    let mut found = false;
    for a in 1..1000 {
        for b in a+1..1000 {
            let c = 1000 - a - b;
            if a < b && b < c && a * a + b * b == c * c {
                println!("a: {}, b: {}, c: {}", a, b, c);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}

