// This file defines a LineSegment from a tuple of two Points. 
// The LineSegment will exist in the same spatial dimension as
// the Point with a higher number of spatial dimensions. The
// LineSegment can also serve as an Edge for making a Shape.

use crate::Point;


#[derive (Debug)]
pub struct LineSegment<'a>
{

	name: String,
	points: (&'a Point, &'a Point),

}






impl LineSegment<'_>
{

	pub fn test (arg1: u8)
	
	{
	
		unimplemented!();
	
	}

}
