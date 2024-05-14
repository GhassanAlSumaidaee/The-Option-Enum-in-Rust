#[derive(Debug)]
// Define an enum representing different types of fruits
enum Fruit {
    Apple,
    Banana,
    Orange,
}

// Define an enum representing the state of fruit availability
enum FruitAvailability {
    Available(Fruit), // Variant with associated data of type Fruit
    NotAvailable,
}

// Function to check the availability of a fruit
fn check_availability(fruit: Fruit) -> FruitAvailability {
    // Simulate availability based on the fruit type
    match fruit {
        Fruit::Apple => FruitAvailability::Available(Fruit::Apple),
        Fruit::Banana => FruitAvailability::NotAvailable,
        Fruit::Orange => FruitAvailability::Available(Fruit::Orange),
    }
}

fn main() {
    let apple = Fruit::Apple;
    let banana = Fruit::Banana;
    let orange = Fruit::Orange;

    // Check the availability of each fruit
    let availability_apple = check_availability(apple);
    let availability_banana = check_availability(banana);
    let availability_orange = check_availability(orange);

    // Print the availability of each fruit
    print_availability(availability_apple);
    print_availability(availability_banana);
    print_availability(availability_orange);
}

// Function to print the availability of a fruit
fn print_availability(avail: FruitAvailability) {
    match avail {
        FruitAvailability::Available(fruit) => println!("{:?} is available", fruit),
        FruitAvailability::NotAvailable => println!("This fruit is not available"),
    }
}
