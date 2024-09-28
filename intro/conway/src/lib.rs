#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            grid: vec![T::default(); rows*cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value; 
    }
    
    pub fn in_scope(&self, row: usize, col: usize) -> bool {
        if row >= 0 && row < self.rows && col >= 0 && col < self.cols {
            return true;
        }
        false
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::<(usize, usize)>::new();
            
        let mut row_start: usize = if row > 0 {
            row-1
        } else {
            row
        };

        let mut col_start: usize = if col > 0 {
            col-1
        } else {
            col
        };
    
        for i in row_start..=row+1 {
            for j in col_start..=col+1 {
                if i == row && col == j {
                    continue;
                }
                
                if self.in_scope(i, j) == true {
                    // println!("{i}, {j}");
                    res.push((i, j));
                    // println!("{:?}", res);
                }
            }
        }

        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self {
            grid: grid.clone(),
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn neighbours_alive(&self, row: usize, col: usize) -> usize {
        let neighbours = self.grid.neighbours(row, col);
        let mut count: usize = 0;
        for n in neighbours {
            let val: Cell = *self.grid.get(n.0, n.1);
            if val == Cell::Alive {
                count += 1;
            }
        }

        count
    }

    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let na = self.neighbours_alive(i, j);
                if na < 2 || na > 3 && *self.grid.get(i, j) == Cell::Alive {
                    new_grid.set(Cell::Dead, i, j);
                } else if na == 3 {
                    new_grid.set(Cell::Alive, i, j);
                }
            }
        }

        self.grid = new_grid;
    }
}
