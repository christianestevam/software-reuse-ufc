#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub items: Vec<String>,
    pub total_cost: f32,
}

pub struct Waiter {
    pub orders: Vec<Order>,
}

impl Waiter {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn take_order(&mut self, order: Order) {
        println!("Order received: {:?}", order);
        self.orders.push(order);
    }

    pub fn serve_order(&self, order_id: u32) -> Result<(), String> {
        match self.orders.iter().find(|&order| order.id == order_id) {
            Some(order) => {
                println!("Serving order: {:?}", order);
                Ok(())
            }
            None => Err(format!("Order {} not found.", order_id)),
        }
    }
}
