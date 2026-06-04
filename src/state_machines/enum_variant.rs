#![allow(unused)]
#![allow(dead_code)]
//! Naive runtime state machine — order lifecycle.
//! Events drive transitions, but correctness is on you.

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

            // Everything else is invalid — but this is runtime, not compile time
            (state, event) => {
                return Err(format!("Invalid event {:?} for state {:?}", event, state));
            }
        };

        self.state = next;
        Ok(())
    }
}

// -------------------------- example -------------------------- //

fn example() {
    let mut new_order = Order::new(67);

    new_order.handle(OrderEvent::Confirm);
    // This hides the error, or we need to handle it here and now.
    // This gives runtime error
    // This pattern also makes us developers need to remember what we can do in each cases
    // for every time we need to use the state. The struct pattern only makes us remember and
    // implement the rules once.
    new_order.handle(OrderEvent::Deliver);
}
