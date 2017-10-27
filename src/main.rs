extern crate itertools;

use std::cell::Cell;
use std::fmt;
use itertools::Itertools;

fn main() {
    let mut a = Grid::new_from_pattern(20,20,vec![(8,9),(10,10),(8,11),(9,9),(9,11),(10,9),(10,11)]);
    println!("{:?}", [0; 20]);
    println!("{}", a);
    println!("{:?}", [0; 20]);
    for _ in 0..5000 {
        //std::thread::sleep(std::time::Duration::from_millis(250));
        a = a.next_gen();
        //println!("{}",a);
        //println!("{:?}",[0;20]);
    }
    println!("{}", a);
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Grid {
    grid: Vec<Vec<Cell<bool>>>,
    size: (usize, usize),
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.grid {
            for case in line {
                write!(
                    f,
                    "{}",
                    if case.get() == true {
                        format!("{}", 0)
                    } else {
                        " ".to_string()
                    }
                ).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(n: usize, p: usize) -> Grid {
        let mut grid = vec![];
        for _ in 0..n {
            let mut line = vec![];
            for _ in 0..p {
                line.push(Cell::new(false));
            }
            grid.push(line)
        }
        Grid {
            grid: grid,
            size: (n, p),
        }
    }

    pub fn new_from_pattern<T>(n: usize, p: usize, pos: T) -> Grid 
    where
        T: IntoIterator<Item = (usize,usize)>,
    {
        let grid = Self::new(n,p);
        pos.into_iter().foreach(|(x,y)| grid.case(x,y).set(true));
        grid
    }

    pub fn case<'a>(&'a self, x: usize, y: usize) -> Case<'a> {
        Case {
            grid: self,
            pos: (x, y),
        }
    }

    pub fn next_gen(&self) -> Grid {
        let (xmax, ymax) = self.size;
        let grid = Grid::new(xmax, ymax);
        for x in 0..xmax - 1 {
            for y in 0..ymax - 1 {
                grid.case(x, y).set(self.case(x, y).next());
            }
        }
        return grid;
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }
}

struct Case<'a> {
    grid: &'a Grid,
    pos: (usize, usize),
}

impl<'a> Case<'a> {
    pub fn set(&self, n: bool) {
        self.grid.grid[self.pos.0][self.pos.1].set(n);
    }

    pub fn value(&self) -> bool {
        self.grid.grid[self.pos.0][self.pos.1].get()
    }

    pub fn adj_count(&self) -> usize {
        let (x, y) = self.pos;
        let grid = self.grid;
        let (xmax, ymax) = grid.size();
        let mut count = 0;
        if x > 0 {
            count += grid.case(x - 1, y).value() as usize;
            if y > 0 {
                count += grid.case(x - 1, y - 1).value() as usize
            }
            if y < ymax {
                count += grid.case(x - 1, y + 1).value() as usize
            }
        };
        if x < xmax {
            count += grid.case(x + 1, y).value() as usize;
            if y > 0 {
                count += grid.case(x + 1, y - 1).value() as usize
            }
            if y < ymax {
                count += grid.case(x + 1, y + 1).value() as usize
            }
        };
        if y > 0 {
            count += grid.case(x, y - 1).value() as usize
        };
        if y < ymax {
            count += grid.case(x, y + 1).value() as usize
        };
        count
    }

    pub fn next(&self) -> bool {
        let c = self.adj_count();
        if c == 3 {
            return true;
        };
        if c == 2 {
            return self.value();
        }
        return false;
    }
}
