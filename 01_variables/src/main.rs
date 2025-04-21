fn main1() {
    let message = "Name: Cesar, Weight: ";
    let weight = 155.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);
}

fn main() {
    let mut message = String::from("Name: Cesar, Weight: ");
    message.clear();
    let mut height = 171;
    height = 182;
    println!("{}{}", message, height);
}
