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
	dimensions: u8, // Attempt to handle up to 255 spatial dimensions

}

  




impl Point
{

	// Create a new Point
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
					dimensions: vals.len() as u8,
					values: vals, // vals.len() must be used first in 
								  // order to avoid losing ownership of vals

				}
			)

		}
		else if n.len() == 0
		{
		
			Err(format!("Error: A Point must have a name of at least length 1. You supplied an empty String for your Point's name."))
		
		}
		else if vals.len() < 1 || vals.len() > 255
		{
			
			// Not sure how to clean up this line. Pressing Enter and Tab will print \n and \t
			Err(format!("Error: A Point must have at least 1 dimension, and no more than 255 dimensions. You supplied a Vector of length {}", vals.len()))
		
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
	// @RETURN: 32-bit float holding the y-value
	pub fn get_y (&self) -> Option<f32>
	
	{
		if self.values.len() > 1
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
	// @RETURN: 32-bit float holding the z-value
	pub fn get_z (&self) -> f32 
	
	{
	
		self.values[2]
	
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
		
		if dim < self.values.len()
		{
		
			Ok(self.values[dim - 1])
		
		}
		else
		{
		
			Err(format!("Error: You requested the value of dimension {},
						but the Point named {} only has {} dimensions.",
						dim, self.name, self.values.len()))
		
		}
	
	}






	// Getter for number of dimensions
	// @PARAM &self: Reference to the struct
	// @RETURN: unsigned 8-bit integer holding the number of dimensions
	pub fn get_dimensions (&self) -> u8 
	
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
		
		self.values[dim - 1] = new_val;
	
	}






	// Setter function for the name
	// @PARAM &mut self: Mutatable reference to the struct
	// @PARAM new_name: String holding the new name
	// @RETURN: None
	pub fn set_name (&mut self, new_name: String) 
	
	{
	
		self.name = new_name;
	
	}






	// Print information about the Point
	// @PARAM: &self: Reference to the struct
	// @RETURN: None
	pub fn print_all (&self)
	
	{
	
		println!("{} has {} spatial dimensions", self.name, self.dimensions);
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
	    											  // empty and can have maximum length of 255.
	
	}

}





// @TODO: Finish fmt and write comments
impl Display for Point
{

	fn fmt (&self, f: &mut Formatter) -> fmt::Result 
		
	{
	
		let name  = self.get_name();
		let dims  = self.get_dimensions();
		let mut point = self.values.iter().map(|val| format!("{}", val)).collect::<Vec<String>>().join(", ");
		point.pop(); point.pop();

		write!(f, "{0} has {1} spatial dimensions\n{0} = ({2})", name, dims, point)

	
	}		

}
