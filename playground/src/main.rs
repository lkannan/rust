mod math;
mod util;

use crate::math::areas::area_of_square;
use crate::util::{add_five, subtract_ten};

fn main() {
    // by default variables are immutable
    let y: u32 = 5;
    println!("Result: {}", add_five(y));

    // need to explicitly declare mutable variable
    let mut x: u32 = 15;
    x = subtract_ten(x);
    println!("Result: {}", x);

    let side: u32 = 5;
    println!("Area of square: {}", area_of_square(side));
}
