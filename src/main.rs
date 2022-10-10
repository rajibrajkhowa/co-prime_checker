use std::io;


fn gcd(a: u64,b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a == 0 && b == 0 {

        return 0;
    }
    if b > a {
        gcd(b,a)
    }
    else {
        gcd(b,a%b)
    }
}

fn coprime(a: u64,b: u64) -> bool {
    if gcd(a,b) == 1 {
        true
    }
    else {
        false
    }
}

fn main() {
    let mut a = String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut a).expect("Number not entered");
    let a: u64 = a.trim().parse().expect("Please type a number");

    let mut b = String::new();
    println!("Please enter the second number:");
    io::stdin().read_line(&mut b).expect("Number not entered");
    let b: u64 = b.trim().parse().expect("Please type a number");

    
    println!("GCD of {} and {} is {}",a, b, coprime(a,b));
}