fn main() {
    println!("Hello, world! 888222");
    println!("Welcome {}","Ryu");

    print!("Hello, ");
    print!("world\n");
    print!("\taaa");

    println!("-- Basic Output --");

    println!("Welcome {} {}", "Riw", "Oh");
    println!("{0} is a friend of {1}. And {1} is also a friend of {0}.", "Hum", "Noi");
    println!("-- Named Arguments --");
    println!("My name is {name}", name = "Riw");
    println!("I am {old} year old. I live in {city}.", old = 25, city = "Cambodia");

    println!("-- Formatting Numbers --");

    println!("Pi is approximately {value:.2}", value = 3.14159);
    println!("Number with padding: {:05}", 32);
    println!("Binary: {:b}", 32);
    println!("Hexadecimal: {:x}", 32);
    println!("Octal: {:o}", 32);
    
    println!("-- Formatting Text --");
    println!("Left aligend: '{:<10}'", "Hello");
    println!("Right aligend: '{:>10}'", "Hello");
    println!("Center aligend: '{:^10}'", "Hello");

    println!("-- Escape Character --");
    println!("Quote: \"Hello World\" ");
    println!("backslash: \\");
    println!("New Line:\nSecond Line");
    println!("Tab:\tTabbed text");

    

}
