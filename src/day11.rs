use crate::read_lines;
use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

use std::convert::TryFrom;

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Occupied),
            'L' => Ok(Cell::Empty),
            '.' => Ok(Cell::Floor),
            _ => Err(anyhow::Error::msg("Invalid character")),
        }
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        match self {
            Cell::Floor => '.',
            Cell::Empty => 'L',
            Cell::Occupied => '#',
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
                data.push(Cell::try_from(each)?);
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
    fn diagonal_neighbours(
        &self,
        postion: (usize, usize),
        direction: (isize, isize),
    ) -> Result<bool> {
        let (rows, columns) = self.dimensions;
        let (row, column) = postion;
        let (dx, dy) = direction;
        let mut temp_r = row as isize;
        let mut temp_c = column as isize;
        loop {
            temp_r += dx;
            temp_c += dy;
            if temp_r < 0 || temp_r >= rows as isize || temp_c < 0 || temp_c >= columns as isize {
                break;
            }
            match self.get(temp_r as usize, temp_c as usize)? {
                Cell::Empty => {
                    return Ok(false);
                }
                Cell::Occupied => {
                    return Ok(true);
                }
                Cell::Floor => (),
            }
        }
        Ok(false)
    }
    fn adjacent_neighbours(&self, row: usize, column: usize) -> Result<usize> {
        let mut num_neighbours = 0;
        let (rows, columns) = self.dimensions;
        for i in -1 as isize..=1 {
            for j in -1 as isize..=1 {
                if i == 0 && j == 0 {
                    continue;
                } else {
                    let new_r = row as isize + i;
                    let new_c = column as isize + j;
                    if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < columns as isize
                    {
                        let cell = self.get(new_r as usize, new_c as usize)?;
                        if cell == Cell::Occupied {
                            num_neighbours += 1;
                        }
                    }
                }
            }
        }
        Ok(num_neighbours)
    }
    fn update(
        &mut self,
        tolerance: usize,
        clo: impl Fn(&Self, usize, usize) -> Result<usize>,
    ) -> Result<usize> {
        let (rows, columns) = self.dimensions;
        let mut changes = Vec::new();
        for row in 0..rows {
            for column in 0..columns {
                let cell = self.get(row, column)?;
                if cell == Cell::Floor {
                    continue;
                }
                let neighbours = clo(&self, row, column)?;
                match cell {
                    Cell::Empty => {
                        if neighbours == 0 {
                            changes.push((row, column, Cell::Occupied));
                        }
                    }
                    Cell::Occupied => {
                        if neighbours >= tolerance {
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
                let each = self.get(row, column).or(Err(fmt::Error))?;
                dsp.push(each.into());
            }
            dsp.push('\n');
        }
        write!(f, "{}", dsp)
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut grid = Grid::new_from_file(filename)?;
    loop {
        let num_updates =
            grid.update(4, |grid, row, column| grid.adjacent_neighbours(row, column))?;
        if num_updates == 0 {
            break;
        }
    }
    Ok(grid.occupied_steats())
}
pub fn part2(filename: &str) -> Result<usize> {
    let mut grid = Grid::new_from_file(filename)?;
    loop {
        let num_updates = grid.update(5, |grid, row, column| {
            let mut neighbours = grid.diagonal_neighbours((row, column), (0, 1))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (0, -1))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (1, 0))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (-1, 0))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (1, 1))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (-1, -1))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (1, -1))? as usize;
            neighbours += grid.diagonal_neighbours((row, column), (-1, 1))? as usize;
            Ok(neighbours)
        })?;
        if num_updates == 0 {
            break;
        }
    }
    Ok(grid.occupied_steats())
}
