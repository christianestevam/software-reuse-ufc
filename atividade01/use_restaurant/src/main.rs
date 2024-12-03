use restaurant::front_of_house::hosting::Host;
use restaurant::front_of_house::serving::{Order, Waiter};
use restaurant::back_of_house;

fn main() {
    // Setup
    let mut host = Host::new();
    let mut waiter = Waiter::new();

    // Seat customers at a table
    host.seat_table(1).unwrap();
    host.seat_table(2).unwrap();

    // Create and take an order
    let order1 = Order {
        id: 1,
        items: vec!["Burger".to_string(), "Fries".to_string()],
        total_cost: 15.50,
    };
    waiter.take_order(order1);

    let order2 = Order {
        id: 2,
        items: vec!["Pizza".to_string()],
        total_cost: 12.00,
    };
    waiter.take_order(order2);

    // Serve orders
    waiter.serve_order(1).unwrap();
    waiter.serve_order(2).unwrap();

    // Back of house operations
    back_of_house::prepare_order(1);
    back_of_house::prepare_order(2);
    back_of_house::clean_kitchen();

    // Free tables
    host.free_table(1).unwrap();
    host.free_table(2).unwrap();

    println!("Restaurant operations completed successfully!");
}
