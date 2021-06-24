mod variables;
mod guess_number;
mod data_type;
mod function;
mod ownership;
mod reference;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use crate::ownership::owner_ship;
use crate::reference::test_reference;

fn main() {
    // guess_number::guess_number()
    // variables::variables()
    // data_type::data_type()
    // function::function_test()
    // owner_ship()
    test_reference()
}
