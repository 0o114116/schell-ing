use rand::Rng;
use std::{cmp::PartialEq, fmt};

// TODO: eventually, allow comparison and switching between different neighborhoods
// TODO: eventually, allow for more specific preferences (indifference, mixes, etc.)

// TODO: add incremental preferences (if it cannot find X neighbors, try for X-1)

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Color::Black => "B",
            Color::White => "W",
        };

        write!(f, "{}", symbol)
    }
}

struct Neighborhood(Vec<Vec<Option<Color>>>);

impl Neighborhood {
    fn random(width: usize, height: usize) -> Self {
        let mut neighborhood = Neighborhood(vec![vec![None; width]; height]);

        for row in neighborhood.0.iter_mut() {
            for cell in row.iter_mut() {
                let mut rng = rand::rng();

                if rng.random_bool(1.0 / 3.0) {
                    if rng.random_bool(0.5) {
                        *cell = Some(Color::Black);
                    } else {
                        *cell = Some(Color::White);
                    }
                } else {
                    *cell = None;
                }
            }
        }

        neighborhood
    }

    fn optimize(&mut self, pref: [u32; 2]) {
        let mut has_changes = true;

        while has_changes {
            has_changes = false;

            for i in 0..self.0.len() {
                for j in 0..self.0[i].len() {
                    match self.0[i][j] {
                        None => {}
                        Some(c) => {
                            if self.equal_neighbors(i, j, c, None) < pref[c as usize] {
                                if self.switch_spots(i, j, pref) {
                                    has_changes = true;

                                    println!("{}", self)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn count_neighbors(
        &self,
        row: usize,
        cell: usize,
        color: Color,
        condition: fn(Color, Color) -> bool,
        og_pos: Option<[usize; 2]>,
    ) -> u32 {
        let mut count = 0;
        let neighbors: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for coord in neighbors {
            let y = coord.0 + row as isize;
            let x = coord.1 + cell as isize;

            if y >= 0 && x >= 0 && y < self.0.len() as isize && x < self.0[0].len() as isize {
                match self.0[y as usize][x as usize] {
                    None => {}
                    Some(c) => {
                        match og_pos {
                            None => {
                                if condition(c, color) {
                                    count += 1;
                                }
                            }
                            Some(pos) => {
                                // TODO: i'm pretty sure this isn't implemented correctly
                                if pos[0] as isize != y && pos[1] as isize != x {
                                    if condition(c, color) {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn equal_neighbors(
        &self,
        row: usize,
        cell: usize,
        color: Color,
        og_pos: Option<[usize; 2]>,
    ) -> u32 {
        self.count_neighbors(row, cell, color, |c, c2| c == c2, og_pos)
    }

    fn diff_neighbors(
        &self,
        row: usize,
        cell: usize,
        color: Color,
        og_pos: Option<[usize; 2]>,
    ) -> u32 {
        self.count_neighbors(row, cell, color, |c, c2| c != c2, og_pos)
    }

    fn switch_spots(&mut self, row: usize, cell: usize, pref: [u32; 2]) -> bool {
        let color = self.0[row][cell].unwrap();

        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                match self.0[i][j] {
                    None => {
                        // TODO: The None here should be Some([row, cell]) after I fix check_neighbors
                        if self.equal_neighbors(i, j, color, None)
                            >= pref[color as usize]
                        {
                            self.0[i][j] = Some(color);
                            self.0[row][cell] = None;

                            return true;
                        }
                    }
                    Some(_) => {}
                }
            }
        }

        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                match self.0[i][j] {
                    None => {}
                    Some(c) => {
                        if color != c {
                            // TODO: The None here should be Some([row, cell]) when I fix count_neighbors
                            if self.diff_neighbors(i, j, c, None) >= pref[color as usize] {
                                // TODO: The None here should be Some([i, j]) when I fix count_neighbors
                                if self.diff_neighbors(row, cell, color, Some([i, j]))
                                    >= pref[c as usize]
                                {
                                    self.0[i][j] = Some(color);
                                    self.0[row][cell] = Some(c);

                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

impl fmt::Display for Neighborhood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                match cell {
                    Some(color) => write!(f, "{}", color)?,
                    None => write!(f, "X")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    Neighborhood::random(10, 5).optimize([2, 2]);
}
