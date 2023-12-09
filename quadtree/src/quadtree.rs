/// Native struct to represent a position
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

///Native struct to represent a range, zone, boundary, etc.
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

    /// Function to check if a point is contained
    /// inside a Boundary
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check
    ///
    /// # Returns
    ///
    /// True if the point is inside the function
    /// False otherwise
    pub fn contains(&self, point: Point) -> bool {
        return point.x >= self.x
            && point.x <= self.x + self.w
            && point.y >= self.y
            && point.y <= self.y + self.h;
    }

    /// Function to check if another range is contained
    /// inside a Boundary
    ///
    /// # Arguments
    ///
    /// * `range` - The range to check
    ///
    /// # Returns
    ///
    /// A boolen confirming if the range is contained inside
    /// the boundary
    pub fn intersects(&self, range: Boundary) -> bool {
        return !(range.x - range.w > self.x + self.w
            || range.x + range.w < self.x - self.w
            || range.y - range.h > self.y + self.h
            || range.y + range.h < self.y - self.h);
    }
}

/// Primary struct that contains all the points to check
/// It has 4 directions: north west, north east, south east, and sout west.
/// It will subdivide when the it gets filled. Once filled, it will distribute
/// the quadtree into 4 quadtrees recursiveley
pub struct Quadtree {
    pub boundary: Boundary,
    capacity: usize,
    divided: bool,
    points: Vec<Point>,
    northwest: Option<Box<Quadtree>>,
    northeast: Option<Box<Quadtree>>,
    southwest: Option<Box<Quadtree>>,
    southeast: Option<Box<Quadtree>>,
}

impl Quadtree {
    /// Creates a new Quadtree. By default,
    /// northwest, northeast, southwest, southeast is None,
    /// once the capacity is filled, the quadtree will subdivide into
    /// those directions.
    ///
    /// # Arguments
    ///
    /// * `boundary` - The initial range of the Quadtree, once divided the quadtree wont exceed
    /// that range
    /// * `capacity` - The initial capacity of each subdivision, cannot be changed once defined
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

    /// This function divides the quadtree into 4 quadtrees, north east,
    /// north west, south east, south west, the subdivision will have the same
    /// capacity as their father.
    fn subdivide(&mut self) {
        //He said that this was innecesary, but honestly, makes the code better
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        // Set of the subdivisions
        let ne = Boundary::new(x + w / 2., y, w / 2., h / 2.);
        let nw = Boundary::new(x, y, w / 2., h / 2.);
        let se = Boundary::new(x + w / 2., y + h / 2., w / 2., h / 2.);
        let sw = Boundary::new(x, y + h / 2., w / 2., h / 2.);

        // Filling of all directions
        self.northeast = Some(Box::new(Quadtree::new(ne, self.capacity)));
        self.northwest = Some(Box::new(Quadtree::new(nw, self.capacity)));
        self.southeast = Some(Box::new(Quadtree::new(se, self.capacity)));
        self.southwest = Some(Box::new(Quadtree::new(sw, self.capacity)));

        // Field to know if the tree is divided
        self.divided = true;
    }

    /// This function inserts the a point into the quadtree.
    /// If the quadtree is filled it will divide it into 4 four directions
    /// north west, north east, south west, south east. It also discriminates a
    /// point, so if a point lands between two borders, it only be added to one of
    /// the two ranges in which it is contained
    ///
    /// # Arguments
    ///
    /// * `point` - Point to be added
    pub fn insert(&mut self, point: Point) -> bool {
        //If the quadtree doesn't contains the point
        //it automatically returns
        if !self.boundary.contains(point) {
            return false;
        }

        //If the quadtree is not filled it will add the point to the actual
        //quadtree, otherwise it will just return
        if self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        } else {
            //This let us divide just one time, otherwise the query function
            //won't work
            if !self.divided {
                self.subdivide();
            }

            //Subdivision according to the directions. It must mutable borrow
            //The direction to add the point
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
    }

    /// Queries the quadtree for points within the specified range.
    ///
    /// This function takes a `Boundary` range as an argument and returns a vector
    /// containing all the points within the specified range. The search is performed
    /// efficiently using the quadtree structure.
    ///
    /// # Arguments
    ///
    /// * `range` - The `Boundary` range for the query. Points within this range will be included in the result.
    ///
    /// # Returns
    ///
    /// A vector of `Point` instances that fall within the specified range.
    ///
    /// # Example
    ///
    /// ```
    /// use quadtree::{Quadtree, Boundary, Point};
    ///
    /// // Assuming quadtree is an instance of your Quadtree structure
    /// let range = Boundary::new(/* define your range parameters here */);
    /// let result = quadtree.query(range);
    /// println!("{:?}", result);
    /// ```
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

    /// Returns a vector of `Boundary` instances representing the boundaries of the quadtree nodes.
    ///
    /// This function traverses the quadtree recursively, starting from the current node,
    /// and collects the boundaries of each node in a vector. The result is a vector
    /// containing all the boundaries in the quadtree hierarchy.
    ///
    /// # Returns
    ///
    /// A vector of `Boundary` instances representing the boundaries of the quadtree nodes.
    ///
    /// # Example
    ///
    /// ```
    /// use quadtree::{Quadtree, Boundary};
    ///
    /// // Assuming quadtree is an instance of your Quadtree structure
    /// let boundaries = quadtree.show();
    /// println!("{:?}", boundaries);
    /// ```
    pub fn show(&self) -> Vec<Boundary> {
        let mut res: Vec<Boundary> = Vec::new();

        res.push(self.boundary);

        if let Some(ne) = self.northeast.as_ref() {
            res.append(&mut ne.show());
        }

        if let Some(nw) = self.northwest.as_ref() {
            res.append(&mut nw.show());
        }

        if let Some(se) = self.southeast.as_ref() {
            res.append(&mut se.show());
        }

        if let Some(sw) = self.southwest.as_ref() {
            res.append(&mut sw.show());
        }

        return res;
    }

    /// This function returns the capacity of the quadtree
    ///
    /// # Returns
    ///
    /// A usize corresponding to the capacity
    #[allow(dead_code)]
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}
