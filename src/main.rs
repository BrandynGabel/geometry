#![allow (dead_code, unused_imports, unused_variables)]

mod point;
mod line;
mod line_segment;
mod shape;

use std::io;
use crate::point::Point;
use crate::line::Line;
use crate::line_segment::LineSegment;
//use crate::shape::Shape;


// This file will be reorganized to separate struct/impl tests into their own functions
// These functions will then be called in main(). 

fn main ()
        
{

    println!("Hello, world!");
    // test_point(); // All tests were successful
    test_line();



}





// Function that tests the Point struct and impl
// Expressions that return a Result<T, E> will end in .unwrap() to bypass error handling 
// Expressions that will generate a panic! are identified to the right of the expression
// panic!s are not handled in this test, they are just shown for functionality purposes
// Uncomment expressions to test them at your behest
fn test_point () 

{
    // let no_name = Point::new_point(String::from(""), 1.0, 2.0, 3.0).unwrap(); // <- panic! No name error

    // let x = Point::new_point(String::from("x"), 1.0, 2.0, 3.0).unwrap();
    // println!("{}", x);

    // let mut x_clone = x.clone();
    // // x_clone.set_name(String::from("")).unwrap(); // <- panic! No name error

    // x_clone.set_name(String::from("x_clone")).unwrap();
    // println!("{}\n", x_clone);

    // let mut y = Point::new_point(String::from("y"), 0.0, 0.0, 0.0).unwrap();
    // println!("{}", y);
    // y.set_x(4.4);
    // y.set_y(-5.5);
    // y.set_z(6.9);
    // println!("{}", y);
    // println!("{} {} {} {}\n", y.get_name(), y.get_x(), y.get_y(), y.get_z());

    // let x_y = Point::distance_between(&x, &y);
    // println!("{}", x_y);

    // println!("{}", Point::distance_between(&x, &x));


}






fn test_line ()

{

    let x = Point::new_point(String::from("x"), 1.0, 2.0, 3.0).unwrap();
    let y = Point::new_point(String::from("y"), -9.0, -8.0, -7.0).unwrap();

    let line_a = Line::new_line(String::from("line_1"), (&x, &y)).unwrap();

    println!("{}", line_a);

}






fn test_shape ()

{

    unimplemented!();

}
