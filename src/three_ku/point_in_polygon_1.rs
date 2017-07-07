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

参考：

https://en.wikipedia.org/wiki/Point_in_polygon
https://www.zhihu.com/question/26551754
http://blog.csdn.net/wsh6759/article/details/5490951


判断点在多边形内外是计算机图形学的最基本算法，在计算机图形处理、模式识别、 CAD 、科学计算可视化以及 GIS 中有着广泛的应用。判断点在多边形内外的算法有主要有定向射线法、角度法、数学公式计算法和网格索引法等方法。角度法要使用复杂的三角运算，计算量大；在工程上应用最多的是定向射线法，这种方法简单、可靠，但其难以处理对边界点及边界与射线共线等特殊情况的处理。

常见的算法是射线法（ray-crossing）

该算法顾名思义，即从给定点引一条射线，计算出该点与多边形交点的个数。若与多边形各边交点为偶数个，则在多边形外，否则就在多边形内。算法需要考虑一些边界条件：射线若正好通过多边形的顶点，射线与多边形的边重合等。若射线穿过多边形的顶点时，若共享顶点的两边在射线的同一侧，则交点计数为 2 ，否则为 1 。具体计数时，当一条边的两个端点 y 值都大于 y 0 ，即边处于上方时，计数加 1 ，否则不加。当射线与多边形边重合时，可以判断其重合边的前一节点和后一节点，若为与射线同一侧，计数为 0 ，否则为 1 。通过以上的补充原则，我们可以正确的判断点位于多边形内的测试。

*/

// solution 1

type Point = (f32, f32);

fn point_in_poly(poly: &[Point], point: Point) -> bool {
    // 此变量用于统计目标点向右画射线与多边形相交次数
    let mut n_cross = 0;

    // 遍历多边形每一个节点
    for i in 0..poly.len() {
        // 定义当前节点和下一个节点
        let p1 = poly[i];
        // (i+1)%poly.len() 通过这种方式来选择多边形下一个顶点，是一个循环，不会有越界
        let p2 = ploy[(i+1)%poly.len()];
        // 如果这条边是水平的则跳过
        // 因为是元组，所以使用.1，表示的是y坐标
        if p1.1 == p2.1 {continue;}
        // 如果目标点低于此线段则跳过
        // point为目标点
        if point.1 < p1.1.min(p2.1) { continue; }
        // 如果目标点高于此线段则跳过
        if point.1 >= p1.1.max(p2.1) { continue; }
        // 如果 过p1画水平线， 过p2画水平线，目标点在这两条线中间
        // 过目标点，画一条水平线，x是这条线与多边形当前边的交点x坐标
        // 因为是元组，所以使用.0，表示的是x坐标，以下是交点公式求交点横坐标
        let x = (point.1 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1) + p1.0;
        // //如果交点在右边，统计加一
        // 这等于从目标点向右发一条射线（ray），与多边形各边的相交（crossing）次数
        if x > point.0 { n_cross += 1; }
    }
    // 如果是奇数，说明在多边形里，否则在多边形外 或 边上
    if (n_cross % 2 == 1) {true} else {false}
}

// solution 2

fn point_in_poly_2(poly: &[Point], (x, y): Point) -> bool {
  poly.iter().zip(poly.iter().cycle().skip(1))
    .filter(|&(&(x1, y1), &(x2, y2))| {
      (y1 > y) != (y2 > y) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1)
    })
    .count() & 1 == 1
}

// solution 3

fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let mut c = false;
    let (x, y) = point;

    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i+1)%poly.len()];

        if (y1>y) == (y2>y) { continue; }

        if x < (x2 - x1) * (y - y1) / (y2 - y1) + x1 {
            c = !c;
        }
    }
    c
}


// for test

#[test]
fn simple_square() {
  let poly = [(-5., -5.), (5., -5.),
              (5., 5.), (-5., 5.)];
  show_and_test(&poly, (-6., 0.), false);
  show_and_test(&poly, (-1., 1.), true);
}
