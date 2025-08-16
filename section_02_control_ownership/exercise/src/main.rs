mod calculator;
fn main() {
    let apple_price: f64 = 15.50;
    let apple_quantity: u32 = 5;


    let banana_price: f64 = 8.75;
    let banana_quantity: u32 = 8;


    let total_apple = calculator::calculato_item_total(apple_price, apple_quantity);
    let total_banana = calculator::calculato_item_total(banana_price, banana_quantity);

    let grand_total = total_apple + total_banana;


    println!("ราคารวมของแอปเปิ้ล: {:.2}", total_apple);
    println!("ราคารวมของกล้วย: {:.2}", total_banana);
    println!("ยอดรวมทั้งหมด: {:.2}", grand_total);
}
