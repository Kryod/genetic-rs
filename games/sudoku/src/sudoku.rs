use std::default::Default;
use std::fmt::{Debug, Display};
use rand::thread_rng;

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Col {
    Left = 0, Center = 1, Right = 2
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Row {
    Up = 0, Center = 1, Down = 2
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Distance(pub u8);

impl Row {
    fn from(x: u8) -> Row {
        match x {
            0 => Row::Up,
            1 => Row::Center,
            2 => Row::Down,
            _ => Row::Up
        }
    }
}

impl Col {
    fn from(x: u8) -> Col {
        match x {
            0 => Col::Left,
            1 => Col::Center,
            2 => Col::Right,
            _ => Col::Left
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub data: [u8;9],
    row: Row,
    col: Col,
    pub immutables: Vec<usize>
}

impl Cell {
    fn new(data: &[u8;9], row: u8, col: u8) -> Self {
        let mut new_data = [0;9];
        let mut immutables = Vec::with_capacity(9);
        for x in 0..3 {
            for y in 0..3 {
                new_data[x*3+y] = data[x*3+y];
                if (1..=9).contains(&data[x*3+y]) {
                    immutables.push(x*3+y);
                }
            }
        }

        Self {
            data: new_data,
            row: Row::from(row),
            col: Col::from(col),
            immutables
        }
    }

    fn validate(&self) -> Result<(), Distance> {
        let mut error = 0;

        for i in 1..self.data.len() {
            if self.data[i..].contains(&self.data[i - 1]) || !(1..=9).contains(&self.data[i]) {
                error+=1;
            }
        }

        if error > 0 {
            Err(Distance(error))
        } else {
            Ok(())
        }
    }

    fn randomize(&mut self) {
        let mut to_permute: Vec<_> = (0..9).into_iter().filter(|x| !self.immutables.contains(x)).collect();

        if to_permute.len() < 2 {
            return;
        }
        let mut rng = thread_rng();
        to_permute.shuffle(&mut rng);

        while to_permute.len() > 1 {
            self.data.swap(to_permute.pop().unwrap(), to_permute.pop().unwrap());
        }

    }

    fn fill(&mut self) {
        let mut missing_vals: Vec<_> = (1..=9).into_iter().filter(|x| !self.data.contains(x)).collect();

        for d in &mut self.data {
            if *d == 0 {
                *d = missing_vals.pop().expect("No more missing values");
            }
        }

    }

}

impl Default for Cell {
    fn default() -> Self {

        Self {
            data: [0;9],
            row: Row::Up,
            col: Col::Left,
            immutables: vec![],
        }
     }
}

#[test]
fn test_cell() {
    let cell = Cell {
        data: [1, 2, 3, 4, 5, 6 ,7, 8, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    let new_cell = Cell::new(&[1, 2, 3,
                                4, 5, 6,
                                7, 8, 9], 1, 2);
    assert_eq!(cell, new_cell);

    let cell = Cell {
        data: [1, 0, 0, 0, 0, 0, 0, 0, 0],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![0],
    };

    let new_cell = Cell::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0], 1, 2);
    assert_eq!(cell, new_cell);
}

#[test]
fn test_cell_validate() {
    let cell = Cell {
        data: [1, 2, 3, 4, 5, 6 ,7, 8, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![],
    };
    assert_eq!(cell.validate(), Ok(()));

    let cell = Cell {
        data: [1, 2, 3, 4, 3, 6 ,2, 8, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![],
    };
    assert_eq!(cell.validate(), Err(Distance(2)));


    let cell = Cell {
        data: [1, 2, 3, 4, 0, 6 ,7, 8, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![],
    };
    assert_eq!(cell.validate(), Err(Distance(1)));
}

#[test]
fn test_fill() {
    let mut cell = Cell {
        data: [1, 2, 3, 0, 5, 0 ,7, 8, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![0,1,2,4,6,7,8],
    };

    assert_eq!(cell.validate(), Err(Distance(3)));
    cell.fill();
    assert_eq!(cell.data, [1, 2, 3, 6, 5, 4 ,7, 8, 9]);
    assert_eq!(cell.validate(), Ok(()));
}

/*#[test]
fn test_randomize() {
    let mut cell = Cell {
        data: [1, 0, 3, 0, 5, 0 ,7, 0, 9],
        row: Row::Center,
        col: Col::Right,
        immutables: vec![0,2,4,6,8],
    };

    assert_eq!(cell.validate(), Err(Distance(7)));
    cell.fill();
    assert_eq!(cell.data, [1, 8, 3, 6, 5, 4 ,7, 2, 9]);
    assert_eq!(cell.validate(), Ok(()));
    cell.randomize();
    assert_eq!(cell.validate(), Ok(()));
}*/

#[derive(Debug, Clone, PartialEq)]
pub struct Sudoku {
    pub cells: [Cell; 9],
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut datas: [[u8; 9]; 9] = [[0;9]; 9];

        let mut row = 0;
        let mut col = 0;

        for cell in &self.cells {
            let mut subrow = 0;
            let mut xcounter = 0;
            for data in cell.data {
                datas[row*3 + subrow][col*3 + xcounter] = data;
                xcounter += 1;
                if xcounter > 2 {
                    xcounter = 0;
                    subrow += 1;
                }
            }
            col += 1;
            if col > 2 {
                col = 0;
                row += 1;
            }
        }

        for tab in datas {
            write!(f, "{:?}", tab)?
        }

        Ok(())
    }
}

impl Sudoku {

    pub fn new(data: [[u8;9];9]) -> Self {
        let mut cells = [Cell::default(), Cell::default(), Cell::default(),
                                    Cell::default(), Cell::default(), Cell::default(),
                                    Cell::default(), Cell::default(), Cell::default()];

        for x in 0..3 {
            for y in 0..3 {
                cells[x*3+y] = Cell::new(&data[x*3+y], x as u8, y as u8);
            } 
        }

        Self {
            cells
        }
    }

    fn validate_data(&self, data: [u8; 9]) -> Result<(), Distance> {
        let cell = Cell {
            data,
            row: Row::Up,
            col: Col::Left,
            immutables: vec![],
        };

        cell.validate()
    }

    fn validate_col(&self, cells: Vec<&Cell>, sub_col: Col) -> Result<(), Distance> {
        let sub_col_num = sub_col as usize;
        let mut datas = [0;9];

        let mut i = 0;

        for cell in cells {
            for x in 0..3 {
                datas[i] = cell.data[sub_col_num + x*3];
                i+=1;
            }
        }

        self.validate_data(datas)
    }

    fn validate_row(&self, cells: Vec<&Cell>, sub_row: Row) -> Result<(), Distance> {
        let sub_row_num = sub_row as usize;
        let mut datas = [0;9];

        let mut i = 0;

        for cell in cells {
            for x in 0..3 {
                datas[i] = cell.data[sub_row_num*3 + x];
                i+=1;
            }
        }

        self.validate_data(datas)
    }

    pub fn validate(&self) -> Result<(), Distance> { 

        let mut error = 0;

        //validate cells
        for cell in &self.cells {
            if let Err(Distance(d)) = cell.validate() {
                error += d;
            }
        }

        for col in [Col::Left, Col::Center, Col::Right] {
            for sub_col in [Col::Left, Col::Center, Col::Right] {
                let cells: Vec<_> = self.cells.iter().filter(|c| c.col == col).collect();
                if let Err(Distance(d)) = self.validate_col(cells, sub_col) {
                    error += d;
                }
            }
        }

        for row in [Row::Up, Row::Center, Row::Down] {
            for sub_row in [Row::Up, Row::Center, Row::Down] {
                let cells: Vec<_> = self.cells.iter().filter(|c| c.row == row).collect();
                if let Err(Distance(d)) = self.validate_row(cells, sub_row) {
                    error += d;
                }
            }
        }


        if error > 0 {
            Err(Distance(error))
        } else {
            Ok(())
        }
    }

    pub fn fill(&self) -> Sudoku {
        let mut new_sudoku = self.clone();

        for cell in &mut new_sudoku.cells {
            cell.fill();
        }

        new_sudoku
    }

    pub fn randomize(&self) -> Sudoku {
        let mut new_sudoku = self.clone();

        for cell in &mut new_sudoku.cells {
            cell.randomize();
        }

        new_sudoku
    }
}

#[test]
fn test_sudoku_validate_col_ok() {

    let cell1 = [1, 2, 3,
        4, 5, 6,
        7, 8, 9];
    let cell2 = [0;9];
    let cell3 = [0;9];
    let cell4 = [2, 3, 1,
        5, 6, 4,
        8, 9, 7];
    let cell5 = [0;9];
    let cell6 = [0;9];
    let cell7 = [3, 1, 2,
        6, 4, 5,
        9, 7, 8];
    let cell8 = [0;9];
    let cell9 = [0;9];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    let cells: Vec<_> = sudoku.cells.iter().filter(|c| c.col == Col::Left).collect();
    assert_eq!(sudoku.validate_col(cells, Col::Right), Ok(()));
}

#[test]
fn test_sudoku_validate_col_nok() {

    let cell1 = [1, 2, 1,
        4, 5, 6,
        7, 8, 9];
    let cell2 = [0;9];
    let cell3 = [0;9];
    let cell4 = [1, 3, 1,
        5, 6, 4,
        8, 9, 7];
    let cell5 = [0;9];
    let cell6 = [0;9];
    let cell7 = [0;9];
    let cell8 = [0;9];
    let cell9 = [0;9];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    let cells: Vec<_> = sudoku.cells.iter().filter(|c| c.col == Col::Left).collect();
    assert_eq!(sudoku.validate_col(cells, Col::Right), Err(Distance(4)));
}

#[test]
fn test_sudoku_validate_row_ok() {

    let cell1 = [1, 2, 3,
        4, 5, 6,
        7, 8, 9];
    let cell2 = [4, 5, 6,
        7, 8, 9,
        1, 2, 3];
    let cell3 = [7, 8, 9,
        1, 2, 3,
        4, 5, 6];
    let cell4 = [0;9];
    let cell5 = [0;9];
    let cell6 = [0;9];
    let cell7 = [0;9];
    let cell8 = [0;9];
    let cell9 = [0;9];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    let cells: Vec<_> = sudoku.cells.iter().filter(|c| c.row == Row::Up).collect();
    assert_eq!(sudoku.validate_row(cells, Row::Center), Ok(()));
}

#[test]
fn test_sudoku_validate_row_nok() {

    let cell1 = [1, 2, 3,
        4, 5, 6,
        7, 8, 9];
    let cell2 = [4, 5, 6,
        7, 8, 9,
        1, 2, 3];
    let cell3 = [7, 8, 9,
        1, 2, 3,
        4, 5, 6];
    let cell4 = [0;9];
    let cell5 = [0;9];
    let cell6 = [0;9];
    let cell7 = [0;9];
    let cell8 = [0;9];
    let cell9 = [0;9];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    let cells: Vec<_> = sudoku.cells.iter().filter(|c| c.row == Row::Up).collect();
    assert_eq!(sudoku.validate_row(cells, Row::Center), Ok(()));
}

#[test]
fn test_sudoku_validate() {

    let cell1 = [7, 3, 5,
        8, 4, 2,
        9, 6, 1];
    let cell2 = [6, 1, 4,
        9, 7, 3,
        2, 8, 5];
    let cell3 = [8, 9, 2,
        5, 6, 1,
        3, 7, 4];
    let cell4 = [2, 8, 6,
        4, 1, 3,
        5, 7, 9];
    let cell5 = [3, 4, 9,
        8, 5, 7,
        1, 2, 6];
    let cell6 = [1, 5, 7,
        9, 2, 6,
        4, 3, 8];
    let cell7 = [1, 5, 7,
        6, 9, 4,
        3, 2, 8];
    let cell8 = [4, 9, 2,
        7, 3, 8,
        5, 6, 1];
    let cell9 = [6, 8, 3,
        2, 1, 5,
        7, 4, 9];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    assert_eq!(sudoku.validate(), Ok(()));
}

#[test]
fn test_sudoku_validate_fail() {

    let cell1 = [7, 3, 5,
        8, 4, 2,
        9, 6, 1];
    let cell2 = [6, 1, 4,
        9, 7, 3,
        2, 8, 5];
    let cell3 = [8, 9, 2,
        5, 6, 1,
        3, 7, 4];
    let cell4 = [2, 8, 6,
        4, 1, 3,
        5, 7, 9];
    let cell5 = [3, 4, 9,
        8, 5, 7,
        1, 2, 6];
    let cell6 = [1, 5, 7,
        9, 2, 6,
        4, 3, 8];
    let cell7 = [1, 5, 7,
        6, 9, 4,
        3, 2, 8];
    let cell8 = [4, 9, 2,
        7, 3, 8,
        5, 6, 1];
    let cell9 = [6, 8, 9,
        2, 1, 5,
        7, 4, 3];
    let sudoku = Sudoku::new([cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9]);

    assert_eq!(sudoku.validate(), Err(Distance(2)));
}
