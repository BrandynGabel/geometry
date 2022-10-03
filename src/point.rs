// This file will define a struct called Point
//
// Point will represent a point in space with
// up to 255 spatial dimensions. 
//
// Each dimension will have f32 precision.
use std::fmt::{self, Formatter, Display};

#[derive (Debug)]
pub struct Point
{

	name: String,
	values: Vec<f32>,
	dimensions: u8, // Attempt to handle up to 255 spatial dimensions

}





impl Point
{

	// Create a new Point
	// @PARAM n: A String containing the name of the point
	// @PARAM vals: Set of 32-bit floats designating the values of the points
	//				vals[0] = first dimensions, vals[1] = second dimension, etc.
	// @RETURN: The Point
	pub fn new_point (n: String, vals: Vec<f32>) -> Self 
	
	{
	
		Point
		{

			name: n,
			dimensions: vals.len() as u8,
			values: vals, // vals.len() must be used first in 
						  // order to avoid losing ownership of vals

		}
	
	}






	// Getter for number of dimensions
	// @PARAM &self: Reference to the struct
	// @RETURN: unsigned 8-bit integer holding the number of dimensions
	pub fn get_dimensions (&self) -> u8 
	
	{
	
		self.dimensions
	
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
	// @RETURN: 32-bit float holding the y-value
	pub fn get_y (&self) -> f32 
	
	{
	
		self.values[1]
	
	}






	// Getter function for the z-value
	// @NOTE: Only use for 3D Point
	// @PARAM &self: Reference to the struct
	// @RETURN: 32-bit float holding the z-value
	pub fn get_z (&self) -> f32 
	
	{
	
		self.values[2]
	
	}





	// Getter function for the value at dimension 'dim'
	// @NOTE: Can be used for accessing value of any dimension
	// @PARAM &self: Reference to the struct
	// @PARAM dim: System default integer size holding the dimension of interest
	// @RETURN: 32-bit float holding the value at dimension 'dim'
	pub fn get_val_at (&self, dim: usize) -> f32 
	
	{
		
		//assert! (dim < self.values.len());
		self.values[dim - 1]
	
	}





	// @TODO: Fix and finish
	// Getter function for the values vector
	/*pub fn get_all_vals (&self) -> Vec<f32> 
	
	{
	
		self.values
	
	}*/






	// Getter function for the name
	// @PARAM &self: Reference to the struct
	// @RETURN: String holding the name
	pub fn get_name (&self) -> String 
	
	{
	
		let val = &self.name;
		String::from(val)

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
	// @RETURN: None
	pub fn set_y (&mut self, new_y: f32)
	
	{
	
		self.values[1] = new_y;
	
	}






	// Setter function for the z-value
	// @NOTE: Only use for 3D Point
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_z: 32-bit float containing the new z-value
	// @RETURN: None
	pub fn set_z (&mut self, new_z: f32)
	
	{
	
		self.values[2] = new_z;
	
	}





	// Setter function for the value at dimension 'dim'
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM dim: System default integer size holding the dimension of interest
	// @PARAM new_val: 32-bit float containing the new value for dimension 'dim'
	// @RETURN: None
	pub fn set_val_at (&mut self, dim: usize, new_val: f32)
	
	{
		
		//assert! (dim < self.values.len());
		self.values[dim - 1] = new_val;
	
	}






	// Setter function for the name
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM n: String holding the new name
	// @RETURN: None
	pub fn set_name (&mut self, n: String) 
	
	{
	
		self.name = n;
	
	}






	// Print information about the Point
	// @PARAM: &self: Reference to the struct
	// @RETURN: None
	pub fn print_all (&self)
	
	{
	
		println!("{} has {} spatial dimensions", self.get_name(), self.get_dimensions());
	    	print! ("{} = (", self.get_name());
	    	let dimensions = self.get_dimensions().into();

	    	for dimension in 1 .. dimensions
	    	{

	        	print! ("{}, ", self.get_val_at(dimension));  

	    	}

	    	println! ("{})", self.get_val_at(dimensions));
	
	}

}





// @TODO: Finish fmt and write comments
impl Display for Point
{

	fn fmt (&self, f: &mut Formatter) -> fmt::Result 
		
	{
	
		let name = self.get_name();
		let dims = self.get_dimensions();

		write! (f, "{0} has {1} spatial dimensions\n{0} = ", name, dims,)

	
	}		

}
