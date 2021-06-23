mod variables;
mod guess_number;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    guess_number::guessNumber()
}
