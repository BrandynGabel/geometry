// This file will define a Shape
// A Shape will consist of an arbitrary number of Points
// The points will represent the vertices of the Shape
// A Shape will also need to have its edges defined
// with a Vector of Lines.

use crate::Point;
use crate::Line;
use std::fmt::{self, Formatter, Display};

#[derive (Debug)]
pub struct Shape
{

	name: String,
	vertices: Vec<Point>,
	edges: Vec<Line>,

}






impl Shape
{

	pub fn new_shape (n: String, vert: Vec<Point>, edg: Vec<Line>) -> Self 
	
	{ 
		Shape
		{
			
			name: n,
			vertices: vert,
			edges: edg,

		}
	
	}

}






impl Display for Shape
{

	fn fmt (&self, f: &mut Formatter) -> fmt::Result
	
	{
	
		unimplemented! ();
	
	}

}
