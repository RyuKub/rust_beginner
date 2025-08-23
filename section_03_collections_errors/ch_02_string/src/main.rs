fn main() {
    // String - owned, mutable
    let mut greeting = String::new();
    greeting.push_str("Hello");
    greeting.push(' ');
    greeting.push_str("Rust");

    // การสร้างจาก string literal
    let message = String::from("Learning Rust");

    // การต่อ string
    let full_message = format!("{} - {}", greeting, message);
    println!("{}", full_message);

    // การทำงานกับ UTF-8 - 1 ตัวอักษรภาษาไทย เป็น 3 bytes
    // สวัสดีครับ เป็น 3 bytes * 10 = 30 bytes
    // Space เป็น 1 byte
    // Emoji เป็น 4 bytes
    let thai_text = "สวัสดีครับ 🦀";
    println!("length in bytes: {}", thai_text.len());
    println!("number of characters: {}", thai_text.chars().count());

    // การแยก string
    let words: Vec<&str> = "Rust is a safe language".split(' ').collect();
    println!("all words: {:?}", words);
}
