struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define the laptops with their brand and price
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650_000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755_000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550_000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850_000,
        },
    ];

    // Customer purchases 3 laptops from each brand
    let quantity_per_brand = 3;

    // Calculate and print the total cost directly using the struct
    println!(
        "The total cost for purchasing 3 laptops from each brand is: ₦{}",
        laptops
            .iter()
            .map(|laptop| laptop.total_cost(quantity_per_brand))
            .sum::<u32>()
    );
}

