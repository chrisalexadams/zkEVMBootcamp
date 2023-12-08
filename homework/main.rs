fn main() {
    println!("Welcome");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut i = 1;
    let mut a = 0;
    while i < 302 {
        if i % 15 == 0 {
            println!("fizz buzz");
            a += 1;
        } else if i % 3 == 0 {
            println!("fizz")            
        } else if i % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", i);
        }

        i += 1;
    }

    println!("{}", a);
}