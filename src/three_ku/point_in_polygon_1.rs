/*
https://www.codewars.com/kata/point-in-polygon-1

The problem

In this kata, you're going write a function called pointInPoly to test if a point is inside a polygon.

Points will be represented as [x,y] arrays.

The polygon will be an array of points which are the polygon's vertices. The last point in the array connects back to the first point.

You can assume:

The polygon will be a valid simple polygon. That is, it will have at least three points, none of its edges will cross each other, and exactly two edges will meet at each vertex.
In the tests, the point will never fall exactly on an edge of the polygon.
Testing

To help you visualize your test cases, the showAndTest(poly,point,inside) function is preloaded. It draws the polygon and point and then calls Test.expect automatically.

So if you call:

showAndTest([[-5, -5], [5, -5], [0, 5]], [0,0], true)
then you'll see:

!()[images/1.png]

The drawing window is 14x14 units wide and centered at the origin.
*/

// type Point = (f32, f32);



// fn point_in_poly(poly: &[Point], point: Point) -> bool {
//   false
// }
//
// #[test]
// fn simple_square() {
//   let poly = [(-5., -5.), (5., -5.),
//               (5., 5.), (-5., 5.)];
//   show_and_test(&poly, (-6., 0.), false);
//   show_and_test(&poly, (-1., 1.), true);
// }
