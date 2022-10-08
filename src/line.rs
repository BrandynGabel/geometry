// This file defines a Line from a tuple of two Points. 


use crate::Point;
use std::fmt::{self, Formatter, Display};

#[derive (Debug, Clone)]
pub struct Line<'a>
{

	name: String,
	points: (&'a Point, &'a Point),

}



//**********************************
//
// @TODO: LEARN ABOUT LIFETIMES AND
//		  POINT REFERENCES IN TUPLE
//
//		  FIX new_line(...)
//
//**********************************
impl<'a> Line<'a>
{
	// Create a new Line
	// @PARAM n: A String holding the name of the Line
	// @PARAM vals: A tuple of Points. 
	// @RETURN: Result<Self, String>
	//			|_ Ok()  = The Line
	//			|_ Err() = String containing an error message 
	pub fn new_line (n: String, vals: (&'a Point, &'a Point)) -> Result<Self, String> 
	
	{
		if n.len() > 0
		{
		
			Ok(

				Line
				{

					name: n,
					points: vals,

				}

			)

		}
		else
		{
		
			Err(format!("Error: A Line must have a name of length 1 or greater. You supplied an empty String for your Line's name."))
		
		}
	
	}






	// Getter function for the name
	// @PARAM &self: Reference to the struct
	// @RETURN: Clone of String holding the name
	pub fn get_name (&self) -> String { self.name.clone() }





	// Getter function for the Vector of Points supplied to make the Line
	// @PARAM &self: Reference to the Line
	// @RETURN: Vector of Points
	pub fn get_points (&self) -> (&Point, &Point) { self.points.clone() }






	// 








	// Setter function for the name
	// @PARAM &mut self: A mutable reference to the Line
	// @PARAM new_name: A String containing the new name for the Line
	// @RETURN: None
	pub fn set_name (&mut self, new_name: String) { self.name = new_name; }






	 

}






impl<'a> Display for Line<'a>
{

	fn fmt (&self, f: &mut Formatter) -> fmt::Result
	
	{
	
		let points = format!("{}, {}", self.points.0, self.points.1);
		write!(f, "{}", points)
	
	}

}

















