#![allow(unused)]
#![allow(dead_code)]
//! This struct variant does not work in all programming languages.
//! Languages that use GC is very vulnerable for this pattern. This pattern
//! will cause GC slownes and use much memory because each change of state
//! will create a new object, and pass the other one for the GC to clean up.
//! In rust we can reuse the allocated memory, so using structs over enums are
//! actually a memory win here.
//!
//! Using this pattern gives you compile time validation.

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

// -------------------------- example -------------------------- //

fn example() {
    let new_order = Order::new(67);
    // new_order.ship(); this dont work, we have compile time errors

    let confirmed_order = new_order.confirmed();
    // If you cancel the order, all the other actions under will give compile time errors
    // this gives a very good error handling system
    // let cancelled_order = confirmed_order.cancel();

    // cant cancel here
    let shipped_order = confirmed_order.ship();
    // cant cancel here
    let delivered_order = shipped_order.deliver();
}
