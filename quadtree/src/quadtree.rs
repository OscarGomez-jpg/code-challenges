#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundary {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Boundary {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, point: Point) -> bool {
        return point.x >= self.x
            && point.x <= self.x + self.w
            && point.y >= self.y
            && point.y <= self.y + self.h;
    }

    pub fn intersects(&self, range: Boundary) -> bool {
        return !(range.x - range.w > self.x + self.w
            || range.x + range.w < self.x - self.w
            || range.y - range.h > self.y + self.h
            || range.y + range.h < self.y - self.h);
    }
}

pub struct Quadtree {
    pub boundary: Boundary,
    pub capacity: usize,
    pub divided: bool,
    pub points: Vec<Point>,
    pub northwest: Option<Box<Quadtree>>,
    pub northeast: Option<Box<Quadtree>>,
    pub southwest: Option<Box<Quadtree>>,
    pub southeast: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(boundary: Boundary, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            divided: false,
            points: Vec::new(),
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    pub fn subdivide(&mut self) {
        //He said that this was innecesary, but honestly, makes the code better
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        let ne = Boundary::new(x + w / 2., y, w / 2., h / 2.);
        let nw = Boundary::new(x, y, w / 2., h / 2.);
        let se = Boundary::new(x + w / 2., y + h / 2., w / 2., h / 2.);
        let sw = Boundary::new(x, y + h / 2., w / 2., h / 2.);

        self.northeast = Some(Box::new(Quadtree::new(ne, self.capacity)));
        self.northwest = Some(Box::new(Quadtree::new(nw, self.capacity)));
        self.southeast = Some(Box::new(Quadtree::new(se, self.capacity)));
        self.southwest = Some(Box::new(Quadtree::new(sw, self.capacity)));

        self.divided = true;
    }

    pub fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }

        // println!("Hi with point {:?}, from {:?}", point, self.boundary);

        if self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        } else {
            if !self.divided {
                self.subdivide();
            }

            if self.northeast.as_mut().unwrap().insert(point) {
                return true;
            } else if self.northwest.as_mut().unwrap().insert(point) {
                return true;
            } else if self.southeast.as_mut().unwrap().insert(point) {
                return true;
            } else {
                self.southwest.as_mut().unwrap().insert(point);
                return true;
            }
        }

        // println!("points len: {}", self.points.len());
    }

    pub fn query(&mut self, range: Boundary) -> Vec<Point> {
        let mut found: Vec<Point> = Vec::new();

        if !self.boundary.intersects(range) {
            return found;
        } else {
            for point in &self.points {
                if range.contains(*point) {
                    found.push(*point);
                }
            }

            if self.divided {
                if let Some(nw) = self.northwest.as_deref_mut() {
                    found.append(&mut nw.query(range));
                }
                if let Some(ne) = self.northeast.as_deref_mut() {
                    found.append(&mut ne.query(range));
                }
                if let Some(sw) = self.southwest.as_deref_mut() {
                    found.append(&mut sw.query(range));
                }
                if let Some(se) = self.southeast.as_deref_mut() {
                    found.append(&mut se.query(range));
                }
            }

            return found;
        }
    }

    /// This function receives and mutates a Vec<(f32, f32, f32, f32)> and returns
    /// the entire tree subdivisions in it
    pub fn show(&mut self, res: &mut Vec<(f32, f32, f32, f32)>) {
        res.push((
            self.boundary.x,
            self.boundary.y,
            self.boundary.w,
            self.boundary.h,
        ));

        if let Some(ne) = self.northeast.as_deref_mut() {
            // println!("Going to ne");
            ne.show(res);
        }

        if let Some(nw) = self.northwest.as_deref_mut() {
            // println!("Going to nw");
            nw.show(res);
        }

        if let Some(se) = self.southeast.as_deref_mut() {
            // println!("Going to se");
            se.show(res);
        }

        if let Some(sw) = self.southwest.as_deref_mut() {
            // println!("Going to sw");
            sw.show(res);
        }
    }
}
