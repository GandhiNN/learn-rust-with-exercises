#![allow(unused)]
// Define a struct named `Order` with the following fields:
// - `price`: an unsigned integer
// - `quantity`: an unsigned integer
//
// It should also have a method named `is_available` that
// returns `true` if the `quantity` is greater than 0, and `false` otherwise.

struct Order {
    price: u64,
    quantity: u64,
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
