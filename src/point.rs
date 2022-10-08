// This file will define a struct called Point
//
// Point will represent a point in space with
// up to 3 spatial dimensions. 
//
// Each dimension will have f32 precision.
use std::fmt::{self, Formatter, Display};

#[derive (Debug, Clone)]
pub struct Point
{

	name: String,
	x: f32,
	y: f32,
	z: f32,

}

  




impl Point
{

	// Create a new Point
	// @NOTE: A Point must have at least 1 value, and a name of at least length 1. 
	// @PARAM n: A String containing the name of the point
	// @PARAM i, j, k: Set of 32-bit floats designating the values of the points
	// @RETURN: Result<Self, String>
	//			|_ Ok()  = The Point
	//			|_ Err() = String containing an error message
	pub fn new_point (n: String, i: f32, j: f32, k: f32) -> Result<Self, String> 
	
	{
		if n.len() > 0
		{

			Ok(
				Point
				{

					name: n,
					x: i,
					y: j,
					z: k,

				}
			)

		}
		else
		{
			// Not sure how to clean up this line. Pressing Enter and Tab will print \n and \t
			Err(format!("Error: A Point must have a name of length 1 or greater. You supplied an empty String for your Point's name."))
		
		}
	
	}






	// Getter function for the name
	// @PARAM &self: Reference to the struct
	// @RETURN: Clone of String holding the name
	pub fn get_name (&self) -> String {	self.name.clone() }





	// Getter function for the x-value.
	// @PARAM &self: Reference to the struct
	// @RETURN: 32-bit float holding the x-value
	pub fn get_x (&self) -> f32 { self.x }






	// Getter function for the y-value
	// @PARAM &self: Reference to the struct
	// @RETURN: 32-bit float holding the y-value
	pub fn get_y (&self) -> f32 { self.y }






	// Getter function for the z-value
	// @PARAM &self: Reference to the struct
	// @RETURN: 32-bit float holding the z-value
	pub fn get_z (&self) -> f32 { self.z }






	// Setter function for the x-value
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_x: 32-bit float containing the new x-value
	// @RETURN: None
	pub fn set_x (&mut self, new_x: f32) { self.x = new_x; }






	// Setter function for the y-value
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_y: 32-bit float containing the new y-value
	// @RETURN: None
	pub fn set_y (&mut self, new_y: f32) { self.y = new_y; }






	// Setter function for the z-value
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_z: 32-bit float containing the new z-value
	// @RETURN: None
	pub fn set_z (&mut self, new_z: f32) { self.z = new_z; }






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





/*
	// Copy the Point
	// @PARAM &self: A reference to the Point
	// @RETURN: A copy of the Point
	pub fn clone (&self) -> Point 
	
	{
	
		Point
		{

			name: self.get_name(),
			x: self.get_x(),
			y: self.get_y(),
			z: self.get_z(),

		}
	
	}

*/




	// Calculate the distance between two Points
	// @PARAM start &Point: A reference to the first Point
	// @PARAM end &Point: A reference to the second Point
	// @RETURN: 32-bit floating point holding the distance between the two Points
	pub fn distance_between (start: &Point, end: &Point) -> f32 
	
	{
	
		let p1 = (start.get_x(), start.get_y(), start.get_z()); // Starting Point
		let p2 = (end.get_x(),   end.get_y(),   end.get_z()); // Ending Point
		
		( (p2.0 - p1.0).powi(2) + 
		  (p2.1 - p1.1).powi(2) + 
		  (p2.2 - p1.2).powi(2) ).sqrt() // Euclidean distance formula

	}

}






impl Display for Point
{
	// This function allows you to print the Point inside a macro with {}
	// @PARAM &self: A reference to the struct
	// @PARAM &mut Formatter: Mutatable reference to a Formatter
	// @Return: fmt::Result ... basically prints the point
	fn fmt (&self, f: &mut Formatter) -> fmt::Result 
		
	{
	
		write!(f, "{} = ({}, {}, {})", self.name, self.x, self.y, self.z)
	
	}		

}
