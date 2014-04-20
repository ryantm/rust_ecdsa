use ecdsa::finitepoint::Point;

pub struct Group {
    name: ~str,
    generator: Option<Point>
}
