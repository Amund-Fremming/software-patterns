#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug)]
enum OrderState {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug)]
enum OrderEvent {
    Confirm,
    Ship,
    Deliver,
    Cancel,
}

struct Order {
    id: u32,
    state: OrderState,
}

impl Order {
    fn new(id: u32) -> Self {
        Self {
            id,
            state: OrderState::Pending,
        }
    }

    fn handle(&mut self, event: OrderEvent) -> Result<(), String> {
        let next = match (&self.state, event) {
            (OrderState::Pending, OrderEvent::Confirm) => OrderState::Confirmed,
            (OrderState::Pending, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Confirmed, OrderEvent::Ship) => OrderState::Shipped,
            (OrderState::Confirmed, OrderEvent::Cancel) => OrderState::Cancelled,
            (OrderState::Shipped, OrderEvent::Deliver) => OrderState::Delivered,

            (state, event) => {
                return Err(format!("Invalid event {:?} for state {:?}", event, state));
            }
        };

        self.state = next;
        Ok(())
    }
}

fn example() {
    let mut new_order = Order::new(67);
    new_order.handle(OrderEvent::Confirm);
    new_order.handle(OrderEvent::Deliver);
}
