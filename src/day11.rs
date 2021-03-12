use crate::read_lines;
use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Into<&'static str> for Cell {
    fn into(self) -> &'static str {
        match self {
            Cell::Floor => ".",
            Cell::Empty => "L",
            Cell::Occupied => "#",
        }
    }
}

#[derive(Debug)]
struct Grid {
    dimensions: (usize, usize),
    data: Vec<Cell>,
}

impl Grid {
    fn new_from_file(filename: &str) -> Result<Self> {
        let (mut rows, mut columns) = (0, 0);
        let mut data = Vec::new();
        for line in read_lines(filename)? {
            let line = line?;
            if columns == 0 {
                columns = line.len();
            }
            for each in line.chars() {
                match each {
                    'L' => {
                        data.push(Cell::Empty);
                    }
                    '.' => {
                        data.push(Cell::Floor);
                    }
                    '#' => {
                        data.push(Cell::Empty);
                    }
                    _ => {
                        return Err(anyhow::Error::msg("Got invalid cell type!"));
                    }
                }
            }
            rows += 1;
        }
        Ok(Self {
            dimensions: (rows, columns),
            data,
        })
    }
    fn get(&self, row: usize, column: usize) -> Result<Cell> {
        let index = row * self.dimensions.1 + column;
        if index >= self.data.len() {
            Err(anyhow::Error::msg("Out of bound!"))
        } else {
            Ok(self.data[index])
        }
    }
    fn set(&mut self, row: usize, column: usize, value: Cell) {
        let index = row * self.dimensions.1 + column;
        self.data[index] = value;
    }
    fn occupied_steats(&self) -> usize {
        self.data.iter().filter(|e| **e == Cell::Occupied).count()
    }
    fn update(&mut self) -> Result<usize> {
        let (rows, columns) = self.dimensions;
        let mut changes = Vec::new();
        for row in 0..rows {
            for column in 0..columns {
                let mut num_neighbours = 0;
                for i in -1 as isize..=1 {
                    for j in -1 as isize..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        } else {
                            let new_r = row as isize + i;
                            let new_c = column as isize + j;
                            if new_r >= 0
                                && new_r < rows as isize
                                && new_c >= 0
                                && new_c < columns as isize
                            {
                                let cell = self.get(new_r as usize, new_c as usize)?;
                                if cell == Cell::Occupied {
                                    num_neighbours += 1;
                                }
                            }
                        }
                    }
                }
                match self.get(row, column)? {
                    Cell::Empty => {
                        if num_neighbours == 0 {
                            changes.push((row, column, Cell::Occupied));
                        }
                    }
                    Cell::Occupied => {
                        if num_neighbours >= 4 {
                            changes.push((row, column, Cell::Empty));
                        }
                    }
                    Cell::Floor => (),
                }
            }
        }
        for (row, column, cell) in &changes {
            self.set(*row, *column, *cell);
        }
        Ok(changes.len())
    }
}

use std::fmt;
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dsp = String::new();
        let (rows, columns) = self.dimensions;
        for row in 0..rows {
            for column in 0..columns {
                dsp.push_str(self.get(row, column).unwrap().into());
            }
            dsp.push_str("\n");
        }
        write!(f, "{}", dsp)
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut grid = Grid::new_from_file(filename)?;
    loop {
        let num_updates = grid.update()?;
        if num_updates == 0 {
            break;
        }
    }
    Ok(grid.occupied_steats())
}
pub fn part2(filename: &str) -> Result<usize> {
    Ok(0)
}
