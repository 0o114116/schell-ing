use crate::color::Color;
use rand::Rng;
use std::fmt;

pub struct Neighborhood(Vec<Vec<Option<Color>>>);

impl Neighborhood {
    pub fn random(width: usize, height: usize) -> Self {
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

    pub fn optimize(&mut self, pref: [u32; 2]) -> &mut Self {
        let mut has_changes = true;

        while has_changes {
            has_changes = false;

            for row in 0..self.0.len() {
                // for every row in the neighborhood...
                for cell in 0..self.0[row].len() {
                    // for every cell in the row...
                    match self.0[row][cell] {
                        // if the cell is not empty...
                        Some(color) => {
                            // attempt to satisfy preferences
                            for i in 0..pref[color as usize] {
                                if self.equal_neighbors(row, cell, color, None)
                                    < pref[color as usize] - i
                                {
                                    // try to move
                                    if self.switch_spots(
                                        row,
                                        cell,
                                        [pref[0] - i, pref[1] - i],
                                        false,
                                    ) {
                                        has_changes = true;

                                        println!("{}", self);
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        self
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

        // for every possible neighbor of a cell...
        for coord in neighbors {
            let y = coord.0 + row as isize;
            let x = coord.1 + cell as isize;

            // if the absolute coordinates are within the bounds of the neighborhood matrix...
            if y >= 0 && x >= 0 && y < self.0.len() as isize && x < self.0[0].len() as isize {
                match self.0[y as usize][x as usize] {
                    None => {}
                    Some(c) => {
                        // if the neighbor cell is not empty...
                        match og_pos {
                            None => {
                                if condition(c, color) {
                                    count += 1;
                                }
                            }
                            // and, given an original position...
                            Some(pos) => {
                                // if the neighbor of a viable spot is not the unhappy individual...
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

    fn switch_spots(
        &mut self,
        row: usize,
        cell: usize,
        pref: [u32; 2],
        checked_empty: bool,
    ) -> bool {
        let color = self.0[row][cell].unwrap();

        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                match self.0[i][j] {
                    None => {
                        if !checked_empty {
                            if self.equal_neighbors(i, j, color, Some([row, cell]))
                                >= pref[color as usize]
                            {
                                self.0[i][j] = Some(color);
                                self.0[row][cell] = None;

                                return true;
                            }
                        }
                    }
                    Some(c) => {
                        if checked_empty {
                            if color != c {
                                if self.diff_neighbors(i, j, c, Some([row, cell]))
                                    >= pref[color as usize]
                                {
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
        }

        if !checked_empty {
            return self.switch_spots(row, cell, pref, true);
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
