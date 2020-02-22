fn main() {
    let foo: i32 = Default::default();
    println!("foo: {}", foo);

    let pizza: PizzaConfig = Default::default();
    println!("wants_cheese: {}", pizza.wants_cheese);

    println!("number_of_olives: {}", pizza.number_of_olives);

    println!("special message: {}", pizza.special_message);

    let crust_type = match pizza.crust_type {
        CrustType::Thin => "Nice and thin",
        CrustType::Thick => "Extra thick and extra filling",
    };
    println!("crust_type: {}", crust_type);

    let custom_pizza = PizzaConfig {
        number_of_olives: 12,
        ..Default::default()
    };
    println!("{:?}", custom_pizza);

    let deluxe_custom_pizza = PizzaConfig {
        number_of_olives: 12,
        wants_cheese: true,
        special_message: "Will you marry me?".to_owned(),
        ..Default::default()
    };
    println!("{:?}", deluxe_custom_pizza);
}

#[derive(Default, Debug)]
struct PizzaConfig {
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

#[derive(Debug)]
enum CrustType {
    Thin,
    Thick,
}

impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::Thin
    }
}