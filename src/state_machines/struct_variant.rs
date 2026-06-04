#![allow(unused)]
#![allow(dead_code)]

pub struct Pending;
pub struct Confirmed;
pub struct Shipped;
pub struct Delivered;
pub struct Cancelled;

struct Order<S> {
    id: u128,
    state: S,
}

impl Order<Pending> {
    fn new(id: u128) -> Order<Pending> {
        Self { id, state: Pending }
    }

    fn confirmed(self) -> Order<Confirmed> {
        Order {
            id: self.id,
            state: Confirmed,
        }
    }

    fn cancel(self) -> Order<Cancelled> {
        Order {
            id: self.id,
            state: Cancelled,
        }
    }
}

impl Order<Confirmed> {
    fn ship(self) -> Order<Shipped> {
        Order {
            id: self.id,
            state: Shipped,
        }
    }

    fn cancel(self) -> Order<Cancelled> {
        Order {
            id: self.id,
            state: Cancelled,
        }
    }
}

impl Order<Shipped> {
    fn deliver(self) -> Order<Delivered> {
        Order {
            id: self.id,
            state: Delivered,
        }
    }
}

fn example() {
    let new_order = Order::new(67);
    let confirmed_order = new_order.confirmed();
    let shipped_order = confirmed_order.ship();
    let delivered_order = shipped_order.deliver();
}
