#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Id {
    id: usize,
}

impl Id {
    fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub id: Id,
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        let id = Id::new(id);
        Self { id, x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    fn pair(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }
}

#[derive(Debug, Clone)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn new(point1: Point, point2: Point) -> Line {
        Line(point1, point2)
    }

    pub fn cross(&self, other: &Self) -> bool {
        cross(self, other)
    }
}

fn cross(line1: &Line, line2: &Line) -> bool {
    let (ax, ay) = line1.0.pair();
    let (bx, by) = line1.1.pair();
    let (cx, cy) = line2.0.pair();
    let (dx, dy) = line2.1.pair();

    if line1.0.id == line2.0.id
        || line1.0.id == line2.1.id
        || line1.1.id == line2.0.id
        || line1.1.id == line2.1.id
    {
        // 点を共有している
        return false;
    }

    let s = (ax - bx) * (cy - ay) - (ay - by) * (cx - ax);
    let t = (ax - bx) * (dy - ay) - (ay - by) * (dx - ax);

    if s * t > 0 {
        return false;
    }

    let s = (cx - dx) * (ay - cy) - (cy - dy) * (ax - cx);
    let t = (cx - dx) * (by - cy) - (cy - dy) * (bx - cx);
    if s * t > 0 {
        return false;
    }

    true
}
