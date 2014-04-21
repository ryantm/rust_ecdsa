use ecdsa::point::Point;

pub struct Group {
    name: ~str,
    generator: Option<Point>
}
