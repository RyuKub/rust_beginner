fn main() {
    println!("-----------------------------------------");

    let a: i32 = 100;

    let b = 200;

    println!("integer with explicit type: {}", a);
    println!("integer with explicit type: {}", b);

    println!("-----------------------------------------");

    let temp: f64 = 3.14989989;
    let presure: f32 = 10.983939398933;

    println!("floating-point number: {}", temp);
    println!("floating-point number: {}", presure);

    println!("-----------------------------------------");

    let balance: i32 = -500;

    println!("Signed integer: {}", balance);
    
    let age: u8 = 0;
    println!("Unsigned integer: {}", age);
    
    let small_signed_number: i8 = 127;

    let medium_unsigned_number: u16 = 65_535;

    println!("small_signed_number: {}", small_signed_number);
    println!("medium_unsigned_number: {}", medium_unsigned_number);

    println!("-----------------------------------------");

    let is_active: bool = true;
    let is_logged_in: bool = false;
    println!("Is active: {}", is_active);
    println!("Is logged: {}", is_logged_in);

    println!("-----------------------------------------");

    let letter: char = 'r';
     let emoji: char = '😊';
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);

    println!("-----------------------------------------");

     let greeting: String = "Hello, world!".to_string();
    println!("Greeting: {}", greeting);

     let x = 5;
    println!("The value of x is: {}", x);

    println!("-----------------------------------------");

     let name = "Riww";
    let age = 25;

    println!("My name is {} and I am {} years old.", name, age);

    println!("{0} is a friend of {1}. And {1} is also a friend of {0}.", "Riw", "OH");

    println!("-----------------------------------------");

    let immutable_var = 10;
    println!("Immutable variable value: {}", immutable_var);

    let mut mutable_var = 15;
    println!("Original value: {}", mutable_var);

    mutable_var = 25;
    mutable_var += 5;
    mutable_var *= 2; 
    mutable_var -= 10; 
    mutable_var /= 5; 

    println!("Mutable variable value: {}", mutable_var);

    const MAX_POINTS: u32 = 100_000;
    const MIN_POINTS: u32 = 0;

    println!("The maximum points are: {}", MAX_POINTS);
    println!("The minimum points are: {}", MIN_POINTS);

    println!("-----------------------------------------");

    let x = 5;
    println!("The value of x is: {}", x);

    // 'x' ตัวใหม่นี้จะ "บดบัง" (shadow) 'x' ตัวเดิม
    // เราสามารถเปลี่ยนชนิดข้อมูลได้ด้วย
    let x = "hello"; 
    println!("The value of x is now: {}", x);

    println!("-----------------------------------------");
    let outer_var = "I'm outside";

    { // เริ่ม scope ใหม่
    let inner_var = "I'm inside";
    println!("{}", outer_var); // เข้าถึงตัวแปรภายนอกได้
    println!("{}", inner_var); // เข้าถึงตัวแปรภายในได้
    } // จบ scope
    println!("{}", outer_var);
    // บรรทัดนี้จะ error เพราะ 'inner_var' อยู่นอก scope แล้ว
    println!("-----------------------------------------");
    let distance: Kilometers = 42;
    let anther_distance: i32 = 100;
    println!("distance in kilometer: {}", distance);
    println!("anther distance: {}", anther_distance);

    println!("-----------------------------------------");
     // การจัดการข้อผิดพลาด
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),}
}
/*
* Rust Error Codes (การจัดการข้อผิดพลาด)
*/
// Rust เน้นการจัดการข้อผิดพลาดผ่าน Result<T, E> และ Option<T> enum แทนการใช้ error code แบบดั้งเดิม
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        // คืนค่า Error
        Err("Cannot divide by zero!")
    } else {
        // คืนค่าที่สำเร็จ
        Ok(numerator / denominator)
    }
}


type Kilometers = i32;


#[allow(dead_code)]
#[allow(unused_variables)]
fn unused_function() {
    #[allow(unused_variables)]
    let unused_var = 42; // ตัวแปรที่ไม่ได้ใช้งาน
    println!("This function is not used, but Rust won't complain.");

}