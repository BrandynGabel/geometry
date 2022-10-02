mod point;

use crate::point::Point;


// A test of the Point struct

fn main ()
        
{

    println!("Hello, world!");

    // Create some values for point_a
    let vals: Vec<f32> = vec! [1.1, 2.2, 3.3];
    let name_a: String = String::from("point_a");

    // Create point_a
    let mut point_a = Point::new_point (vals, name_a);

    // Test the "simple" getters
    println! ("point_a has {} spatial dimension", point_a.get_dimensions());
    println! ("point_a has values of ({}, {}, {})",
              point_a.get_x(), point_a.get_y(), point_a.get_z() );

    // Test the "simple" setters
    point_a.set_x(9.9);
    point_a.set_y(-1.1);
    point_a.set_z(0.5);

    println! ("The new values of point_a are: ({}, {}, {})", 
              point_a.get_x(), point_a.get_y(), point_a.get_z());




    // Move on to point_b





    // Create point_b from literals
    let mut point_b = Point::new_point(vec![1.1, 2.2, 3.3, 4.4, 5.5], String::from("point_b"));

    // Test print_all()
    point_b.print_all();

    println! ("The values of {} will be squared.", point_b.get_name());

    for dimension in 1 ..= point_b.get_dimensions()
    {
        let curr_dim = dimension.into();
        let new_val  = f32::powf(point_b.get_val_at(curr_dim), 2.0); // Square the values
        point_b.set_val_at(curr_dim, new_val);

    }


    // Test setter and getter for name
    point_b.set_name(String::from("point_c"));
    println! ("\nThe new name for point_b is: {}", point_b.get_name());

    // Create point_c
    let point_c = point_b;
    point_c.print_all();


    // Create point_d and test the implicit constructor
    let point_d = Point::new_point(vec![0.2, 0.4], String::from("point_d"));
    point_d.print_all();




}