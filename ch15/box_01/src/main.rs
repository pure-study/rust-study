#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cons(l0, l1), Self::Cons(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basic_usage() {
        let list = Cons(1, Box::new(Nil));

        if let Cons(x, next) = list {
            assert_eq!(x, 1);
            assert_eq!(*next, Nil);
        } else {
            panic!("Should not go here.");
        }
    }

    #[test]
    fn test_box_deref_further() {
        // let list = Nil;  // fail it intentionally
        let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

        if let Cons(_, next) = list {
            assert_eq!(*next, Cons(2, Box::new(Nil)));
        } else {
            panic!("Should not go here.");
        }
    }
}
