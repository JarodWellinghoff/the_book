fn main() {
    println!("{}", fahrenheit_to_celsius(32.0));
    println!("{}", fibonacci(10));
    twleve_days_of_christmas();
}

// Cpnvert temperature from Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Generate the nth Fibonacci number
fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    return a;
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn twleve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if i == 0 {
                println!("{}", gifts[j]);
            } else if j == 0 {
                println!("and {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}
