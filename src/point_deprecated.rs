/*
This file was originally intended for euclidean geometry, however the calculations required
for dimensions higher than 3 (e.g. coplanarity) are beyond the scope of this project, and 
require more advanced linear algebra algorithms. Therefore Point will be rewritten to hold
a maximum of 3 dimensions. 

Additionally, the choice of using Vec<f32> data type to store the
data was a poor choice due to the fact that that such calculations will result in O(n) or
greater time complexity which is extremely inefficient. Instead the new Point struct for the
the linear algebra project will use HashMap. The Point for euclidean geometry will just use
three members x, y, z, all of which will be f32. 

This file will not be deleted because it was a good learning experience, and I want to be 
able to reference it in case I forget something, and as a starting point for implementing
the new linear algebra Point. Lastly, the testing of the original Point struct will be 
removed from main.rs and copied to the end of this file.




// This file will define a struct called Point
//
// Point will represent a point in space with
// up to 255 spatial dimensions. 
//
// Each dimension will have f32 precision.
use std::fmt::{self, Formatter, Display};

#[derive (Debug, Clone)]
pub struct Point
{

	name: String,
	values: Vec<f32>,
	dimensions: usize, // dimensions is defined as usize, which is about 184 quintillion,
					   // but the instantiation of a Point is limited to u8 which is 255.
					   // It is defined as usize because that is what the Rust std lib uses
					   // as the parameter type for uints, and I want to avoid using .into()

}

  




impl Point
{

	// Create a new Point
	// @NOTE: A Point must have at least 1 value, and a name of at least length 1. 
	// @PARAM n: A String containing the name of the point
	// @PARAM vals: Set of 32-bit floats designating the values of the points
	//				vals[0] = first dimensions, vals[1] = second dimension, etc.
	// @RETURN: Result<Self, String>
	//			|_ Ok()  = The Point
	//			|_ Err() = String containing an error message
	pub fn new_point (n: String, vals: Vec<f32>) -> Result<Self, String> 
	
	{
		if vals.len() > 0 && vals.len() < 256 && n.len() > 0
		{

			Ok(
				Point
				{

					name: n,
					dimensions: vals.len(),
					values: vals, // vals.len() must be used first in 
								  // order to avoid losing ownership of vals

				}
			)

		}
		else if n.len() == 0
		{
			// Not sure how to clean up this line. Pressing Enter and Tab will print \n and \t
			Err(format!("Error: A Point must have a name of length 1 or greater. You supplied an empty String for your Point's name."))
		
		}
		else if vals.len() < 1 || vals.len() > 255
		{
			
			
			Err(format!("Error: A Point must have at least 1 dimension, and no more than {} dimensions. You supplied a Vector of length {}", u8::MAX, vals.len()))
		
		}
		else
		{
		
			Err(format!("Error: An unknown error occurred."))
		
		}
	
	}






	// Getter function for the name
	// @PARAM &self: Reference to the struct
	// @RETURN: Clone of String holding the name
	pub fn get_name (&self) -> String 
	
	{
	
		self.name.clone()

	}





	// Getter function for the x-value.
	// @NOTE: Only use for 1D, 2D, or 3D Point
	// @PARAM &self: Reference to the struct
	// @RETURN: 32-bit float holding the x-value
	pub fn get_x (&self) -> f32 // Learning note: &self is shorthand for self: &Self
								// Using Self in place of Point only works inside the
	{							// impl block
	
		self.values[0]
	
	}






	// Getter function for the y-value
	// @NOTE: Only use for 2D or 3D Point
	// @PARAM &self: Reference to the struct
	// @RETURN: Option<f32>
	//			|_ Some() = 32-bit float holding the y-value
	//			|_ None if it's a 1D Point
	pub fn get_y (&self) -> Option<f32>
	
	{
		if self.dimensions > 1
		{
		
			Some(self.values[1])
		
		}
		else
		{
		
			None
		
		}
	
	}






	// Getter function for the z-value
	// @NOTE: Only use for 3D Point
	// @PARAM &self: Reference to the struct
	// @RETURN: Option<f32>
	//			|_ Some() = 32-bit float holding the z-value
	//			|_ None if it's a 1D or 2D Point
	pub fn get_z (&self) -> Option<f32> 
	
	{
	
		if self.dimensions > 2
		{
		
			Some(self.values[2])
		
		}
		else
		{
		
			None
		
		}
	
	}





	// Getter function for the value at dimension 'dim'
	// @NOTE: Can be used for accessing value of any dimension that exists
	// @PARAM &self: Reference to the struct
	// @PARAM dim: System default integer size holding the dimension of interest
	// @RETURN: Result<f32, String>
	//			|_ Ok()  = 32-bit float holding the value at dimension 'dim'
	//			|_ Err() = String containing an error message
	pub fn get_val_at (&self, dim: usize) -> Result<f32, String> 
	
	{
		
		if dim <= self.dimensions.into()
		{
		
			Ok(self.values[dim - 1])
		
		}
		else
		{
		
			Err(format!("Error: You requested the value of dimension {}, but the Point named {} only has {} dimensions.",
						dim, self.name, self.dimensions))
		
		}
	
	}






	// Getter for number of dimensions
	// @PARAM &self: Reference to the struct
	// @RETURN: unsigned 8-bit integer holding the number of dimensions
	pub fn get_dimensions (&self) -> usize 
	
	{
	
		self.dimensions
	
	}






	// Getter function for the values vector
	// @PARAM &self: Reference to the struct
	// @RETURN: Clone of values
	pub fn get_all_vals (&self) -> Vec<f32> 
	
	{
	
		self.values.clone()
	
	}






	// Setter function for the x-value
	// @NOTE: Only use for 1D, 2D, or 3D Point
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_x: 32-bit float containing the new x-value
	// @RETURN: None
	pub fn set_x (&mut self, new_x: f32)

	{

		self.values[0] = new_x;

	}






	// Setter function for the y-value
	// @NOTE: Only use for 2D or 3D Point
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_y: 32-bit float containing the new y-value
	// @RETURN: Result<(), String>
	//			|_ Ok()  = () unit value
	//			|_ Err() = String containing an error message
	pub fn set_y (&mut self, new_y: f32) -> Result<(), String>
	
	{
	
		if self.dimensions > 1
		{
		
			self.values[1] = new_y;
			Ok(())
		
		}
		else
		{
		
			Err(format!("Error: You cannot change the y-value of the Point named {} because it does not exist.", self.name))
		
		}
	
	}






	// Setter function for the z-value
	// @NOTE: Only use for 3D Point
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_z: 32-bit float containing the new z-value
	// @RETURN: Result<(), String>
	//			|_ Ok()  = () unit value
	//			|_ Err() = String containing an error message
	pub fn set_z (&mut self, new_z: f32) -> Result<(), String>
	
	{
	
		if self.dimensions > 2
		{
		
			self.values[2] = new_z;
			Ok(())
		
		}
		else
		{
		
			Err(format!("Error: You cannot change the z-value of the Point named {} because it does not exist.", self.name))
		
		}
	
	}





	// Setter function for the value at dimension 'dim'
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM dim: System default integer size holding the dimension of interest
	// @PARAM new_val: 32-bit float containing the new value for dimension 'dim'
	// @RETURN: Result<(), String>
	//			|_ Ok()  = () unit value
	//			|_ Err() = String containing an error message
	pub fn set_val_at (&mut self, dim: usize, new_val: f32) -> Result<(), String>
	
	{
		
		if self.dimensions >= dim && dim > 0
		{
		
			self.values[dim - 1] = new_val;
			Ok(())
		
		}
		else
		{
		
			Err(format!("Error: You cannot change dimension {} of the Point named {} because it does not exist.", dim, self.name))
		
		}
		
	
	}






	// Setter function for the name
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_name: String holding the new name
	// @RETURN: Result<(), String>
	//			|_ Ok()  = () unit value
	//			|_ Err() = String containing an error message
	pub fn set_name (&mut self, new_name: String) -> Result<(), String>
	
	{
		
		if new_name.len() > 0
		{
		
			self.name = new_name;
			Ok(())
		
		}
		else
		{
		
			Err(format!("Error: A Point must have a name of length 1 or greater. You tried to change the name of the Point named {} to an empty String.", self.name))
		
		}
	
	}






	// Print information about the Point
	// @PARAM: &self: Reference to the struct
	// @RETURN: None
	pub fn print_all (&self)
	
	{
		
		if self.dimensions == 1
		{
		
			println!("{} has 1 spatial dimension.", self.name);
		
		}
		else
		{
		
			println!("{} has {} spatial dimensions.", self.name, self.dimensions);
		
		}
	    print!("{} = (", self.name);

	    for index in 1 .. self.dimensions
	    {

	        match self.get_val_at(index.into())
	        {
	          
	        	Ok(val)  => print!("{}, ", val),
	          	
	          	Err(err) => print!("{}", err),
	          
	        };

	    }

	    println!("{})", self.values.last().unwrap()); // This will crash the program if self.values is empty.
	    											  // Creating a Point with an empty Vector should return
	    											  // a String via Err() explaining the Vector cannot be
	    											  // empty and can only have a maximum length of 255.
	
	}






	// Copy the Point
	// @PARAM &self: A reference to the Point
	// @RETURN: A copy of the Point
	pub fn clone (&self) -> Point 
	
	{
	
		Point
		{

			name: self.get_name(),
			dimensions: self.get_dimensions(),
			values: self.get_all_vals(),

		}
	
	}




	// Consider changing from &self to p1: &Point

	// Calculate the distance between two Points
	// @NOTE: If the Points do not have the same number of
	//		  dimensions, the calculation will be carried
	//		  out in the higher spacial dimension of the two
	// @PARAM &self: A reference to the first Point
	// @PARAM p2 &Point: A reference to the second Point
	// @RETURN: 32-bit floating point holding the distance between the two Points
	pub fn distance_to (&self, p2: &Point) -> f32 
	
	{
	
		let mut vec1 = self.get_all_vals();
		let mut vec2 = p2.get_all_vals();
		let mut distance: f32 = 0.0;

		// if-elseif block ensures Points are calculated in the higher dimension
		if vec1.len() > vec2.len()
		{
		
			vec2.resize(vec1.len(), 0.0);
		
		}
		else if vec1.len() < vec2.len()
		{
		
			vec1.resize(vec2.len(), 0.0);
		
		}

		
		for index in 0 .. vec1.len()
		{
		
			distance = distance + (vec2[index] - vec1[index]).powi(2);
		
		}

		distance.sqrt()
	
	}

}





// @TODO: Finish fmt and write comments
impl Display for Point
{
	// This function allows you to print the Point inside a macro with {}
	// @PARAM &self: A reference to the struct
	// @PARAM &mut Formatter: Mutatable reference to a Formatter
	// @Return: fmt::Result ... basically prints the point
	fn fmt (&self, f: &mut Formatter) -> fmt::Result 
		
	{
	
		// This statement collects all the values for each dimension into a string and separates them with a comma and space.
		let point = self.values.iter().map(|val| format!("{}", val)).collect::<Vec<String>>().join(", ");

		if self.dimensions == 1
		{
		
			write!(f, "{} = ({})", self.name, point)
		
		}
		else
		{
		
			write!(f, "{} = ({})", self.name, point)
		
		}
	
	}		

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
*/
