#![allow (dead_code, unused_imports, unused_variables)]

mod point;
mod line;
mod shape;

use std::io;
use crate::point::Point;
use crate::line::Line;
use crate::shape::Shape;


// This file will be reorganized to separate struct/impl tests into their own functions
// These functions will then be called in main(). 

fn main ()
        
{

    println!("Hello, world!");
    // test_point(); // All tests of Point appear successful



}





// Function that tests the Point struct and impl
// Expressions that return a Result<T, E> will end in .unwrap() to bypass error handling 
// Expressions that will generate a panic! are identified to the right of the expression
// panic!s are not handled in this test, they are just shown for functionality purposes
// Uncomment expressions to test them at your behest
fn test_point () 

{

    // let no_name = Point::new_point(String::from(""), vec![0.0]).unwrap(); //<- panic! no name error
    // let no_vals = Point::new_point(String::from("no_vals"), vec![]).unwrap(); //<- panic! no values error
    // let mut x = Point::new_point(String::from("point1d"), vec![1.1]).unwrap();
    // let mut y = Point::new_point(String::from("point2d"), vec![-3.3, -4.4]).unwrap();
    // let mut z = Point::new_point(String::from("point3d"), vec![5.5, 6.6, 7.7]).unwrap();
    // let mut f = Point::new_point(String::from("point4d"), vec![8.8, 9.9, 0.0, 2.2]).unwrap();

    // println!("{}\n{}\n{}\n", x, y, z);

    // x.set_name(String::from("")).unwrap(); //<- panic! no name error
    // x.set_name(String::from("Point_x")).unwrap();
    // y.set_name(String::from("Point_y")).unwrap();
    // z.set_name(String::from("Point_z")).unwrap();
    // f.set_name(String::from("Point_f")).unwrap();

    // println!("{}\n{}\n{}\n{}\n", x, y, z, f);

    // x = y.clone();
    // println!("{}\n{}\n{}\n", x, y, z);

    // print_dist(&z, &f, 4);
    // print_dist(&y, &z, 0);
    // print_dist(&f, &x, 1);
    // print_dist(&x, &y, 3);
    // print_dist(&y, &x, 3)
    // print_dist(&y, &y, 50);

    // x.print_all();
    // y.print_all();
    // z.print_all();
    // f.print_all();

    // x.set_val_at(1, 4.4).unwrap();
    // println!("{}", x);
    // x.set_val_at(x.get_dimensions(), -3.0).unwrap();
    // println!("{}", x);
    // f.set_val_at(3, -999.9).unwrap();
    // f.set_val_at(f.get_dimensions(), -3.0).unwrap();
    // println!("{}", f);
    // println!("{}", f);
    // x.set_val_at(0, 6.6).unwrap(); //<- panic! dimension 0 does not exist error
    // x.set_val_at(2, 6.6).unwrap(); //<- panic! dimension 2 does not exist error
    // z.set_val_at(4, 8.8).unwrap(); //<- panic! dimension 4 does not exist error

    // x.set_x(1.9);
    // println!("{}", x);

    // y.set_x(9.2);
    // println!("{}", y);
    // y.set_y(2.9).unwrap();
    // println!("{}", y);

    // z.set_x(1.5);
    // println!("{}", z);
    // z.set_y(3.5).unwrap();
    // println!("{}", z);
    // z.set_z(-7.5).unwrap();
    // println!("{}", z);

    // f.set_x(4.0);
    // println!("{}", f);
    // f.set_y(4.0).unwrap();
    // println!("{}", f);
    // f.set_z(-4.0).unwrap();
    // println!("{}", f);

    // x.set_y(2.9).unwrap(); //<- panic! y-value does not exist error
    // println!("{}", x);
    // y.set_z(-7.5).unwrap(); //<- panic! z-value does not exist error
    // println!("{}", y);

}
// Print the distance between two Points with precision 'pr'
fn print_dist (p1: &Point, p2: &Point, pr: usize){
    let distance = p1.distance_to(&p2);
    let dim = std::cmp::max(p1.get_dimensions(), p2.get_dimensions());
    println!("The distance between {0} and {1} is {2:.3$} in {4}-D space", p1.get_name(), p2.get_name(), distance, pr, dim);
}






fn test_line ()

{

    unimplemented!();

}






fn test_shape ()

{

    unimplemented!();

}
