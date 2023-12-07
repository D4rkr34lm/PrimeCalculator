use std::collections::LinkedList;
use std::io::stdin;
use std::time::Instant;

fn main() {
    println!("Enter upper bound for checking");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");

    let max = input.trim().parse::<u64>().unwrap();

    let now = Instant::now();

    let mut primes : LinkedList<u64> = LinkedList::new();

    primes.push_back(2);
    
    for curr in 3..max {

        if is_prime(&primes, curr) {
            primes.push_back(curr);
            //let prime_text = curr.to_string();
            //println!("{}", prime_text);
        } 
    }

    println!("Calculation finished after {} ms", now.elapsed().as_millis());
}


fn is_prime (primes : &LinkedList<u64>, number : u64) -> bool{
    let mut prime_iter = primes.iter();
    let mut curr_prime_option = prime_iter.next();

    let mut is_prime = true;
    while curr_prime_option.is_some_and(|prime| prime*prime <= number){
        if curr_prime_option.is_some_and(|prime| number % prime == 0){
            is_prime = false;
            break;
        }
        curr_prime_option = prime_iter.next();
    }

    return is_prime;
}