// This file defines a Line or segment of a Line from two Points. 
// The members will be a Vector of Points, where the number of 
// Points indicates the number of spatial dimensions. Point will
// also have a bool where true = line, and false = line segment.
// The Line segment will serve as an edge for creating a Shape. 

use crate::Point;
use std::fmt::{self, Formatter, Display};

#[derive (Debug)]
pub struct Line
{

	name: String,
	points: Vec<Point>,
	is_line: bool, // True when it is implemented as an actual line.
				   // False when it is implemented as a line segment.

}






impl Line
{
	// Create a new Line
	// @PARAM n: A String holding the name of the Line
	// @PARAM vals: A Vector of Points. The length of the vector is the spatial dimensions 
	// @PARAM line: True indicates a real line, False indicates a segment of a Line
	// @RETURN: The Line or Line segment
	pub fn new_line (n: String, vals: Vec<Point>, line: bool) -> Self 
	
	{
	
		Line
		{

			name: n,
			points: vals,
			is_line: line,

		}
	
	}

}






impl Display for Line
{

	fn fmt (&self, f: &mut Formatter) -> fmt::Result
	
	{
	
		unimplemented! ();
	
	}

}

















