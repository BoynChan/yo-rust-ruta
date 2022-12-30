use std::ops::Add;

#[derive(Debug, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point {
            x: x as f64,
            y: y as f64,
        }
    }
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dist(&self, p: Point) -> f64 {
        let dis_x = (self.x - p.x).abs();
        let dis_y = (self.y - p.y).abs();
        (dis_x * dis_x + dis_y * dis_y).sqrt()
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Polygon {
    points: Vec<Point>,
    index: usize,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon {
            points: Vec::new(),
            index: 0,
        }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            return None;
        }
        let mut p = &self.points[0];
        for ps in &self.points {
            if ps.x < p.x {
                p = ps;
            }
        }
        return Some(*p);
    }

    fn iter(&mut self) -> &mut Vec<Point> {
        &mut self.points
    }
}

pub struct Circle {
    center: Point,
    radius: usize,
}

impl Circle {
    fn new(center: Point, radius: usize) -> Circle {
        Circle { center, radius }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        // let points = poly.iter().cloned().collect::<Vec<_>>();
        // assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(Shape::Polygon(poly)),
            Shape::from(Shape::Circle(Circle::new(Point::new(10, 20), 5))),
        ];
        // let circumferences = shapes
        //     .iter()
        //     .map(Shape::circumference)
        //     .map(round_two_digits)
        //     .collect::<Vec<_>>();
        // assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}
