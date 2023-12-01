pub struct Garden {
    pub matrix: Vec<Vec<u32>>,
}

impl Garden {
    pub fn new(matrix_size: u32) -> Self {
        let matrix: Vec<Vec<u32>> = vec![vec![0; matrix_size as usize]; matrix_size as usize];
        Garden { matrix }
    }

    pub fn sparse_seed(&mut self, pos_x: u32, pos_y: u32) {
        let mut keep = true;
        let oportunity = self.search_river(pos_x, pos_y);

        //Making sure that we haven't visited that position
        if self.matrix[pos_y as usize][pos_x as usize] != 0 {
            //When the code cannot find an 0 near
            if oportunity.0 == pos_x && oportunity.1 == pos_y {
                return;
            } else {
                keep = false;
            }
        }

        if keep {
            self.matrix[pos_y as usize][pos_x as usize] = 1;

            //Checking the position of the actual run
            if pos_y + 1 < self.matrix.len() as u32 {
                if self.matrix[(pos_y + 1) as usize][pos_x as usize] != 1 {
                    self.sparse_seed(pos_x, pos_y + 1)
                }
            }

            if pos_x + 1 < self.matrix.len() as u32 {
                if self.matrix[pos_y as usize][(pos_x + 1) as usize] != 1 {
                    self.sparse_seed(pos_x + 1, pos_y)
                }
            }

            if self.matrix[pos_y.saturating_sub(1) as usize][pos_x as usize] != 1 {
                self.sparse_seed(pos_x, pos_y.saturating_sub(1))
            }

            if self.matrix[pos_y as usize][pos_x.saturating_sub(1) as usize] != 1 {
                self.sparse_seed(pos_x.saturating_sub(1), pos_y)
            }
        } else {
            self.sparse_seed(oportunity.0, oportunity.1);
        }
    }

    fn search_river(&mut self, pos_x: u32, pos_y: u32) -> (u32, u32) {
        let mut ans = (pos_x, pos_y);

        //Making sure that the position is not a previous filled slot
        if self.matrix[pos_y as usize][pos_x as usize] == 3 {
            self.matrix[pos_y as usize][pos_x as usize] = 2;
            //Checking the position of the actual run
            if pos_y + 1 < self.matrix.len() as u32
                && self.matrix[(pos_y + 1) as usize][pos_x as usize] == 0
            {
                ans = (pos_x, pos_y + 1);
            } else if pos_x + 1 < self.matrix.len() as u32
                && self.matrix[pos_y as usize][(pos_x + 1) as usize] == 0
            {
                ans = (pos_x + 1, pos_y);
            } else if self.matrix[pos_y.saturating_sub(1) as usize][pos_x as usize] == 0 {
                ans = (pos_x, pos_y.saturating_sub(1));
            } else if self.matrix[pos_y as usize][pos_x.saturating_sub(1) as usize] == 0 {
                ans = (pos_x.saturating_sub(1), pos_y);
            }
        }

        ans
    }

    /**
     * This function prints the matrix
     */
    pub fn to_string(&self) -> String {
        let mut msg: String = String::from("");
        for row in &self.matrix {
            for &value in row {
                msg.push_str(&value.to_string());
            }
            msg.push_str("\n");
        }
        msg
    }
}

impl Default for Garden {
    fn default() -> Self {
        Garden::new(5)
    }
}
