use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Galaxy(pub usize, pub usize);

impl Galaxy {
    pub fn shortest_path(&self, other: &Self) -> usize {
        let Galaxy(r, c) = *self;
        let Galaxy(o_r, o_c) = *other;

        (r.max(o_r) - r.min(o_r)) + (c.max(o_c) - c.min(o_c))
    }
}

#[derive(Debug, Clone)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = (0..=self.max_row()).peekable();

        while let Some(row) = rows.next() {
            for col in 0..=self.max_col() {
                if self.galaxies.contains(&Galaxy(row, col)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            if rows.peek().is_some() {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl Universe {
    pub fn new(galaxies: Vec<Galaxy>) -> Self {
        Self { galaxies }
    }

    pub fn max_row(&self) -> usize {
        self.galaxies
            .iter()
            .map(|Galaxy(r, _)| *r)
            .max()
            .unwrap_or(0)
    }

    pub fn max_col(&self) -> usize {
        self.galaxies
            .iter()
            .map(|Galaxy(_, c)| *c)
            .max()
            .unwrap_or(0)
    }

    pub fn expand_by(&mut self, value: usize) {
        let mut empty_rows = Vec::new();

        for row in 0..=self.max_row() {
            if !self.galaxies.iter().any(|Galaxy(r, _)| r == &row) {
                empty_rows.push(row);
            }
        }

        let mut empty_cols = Vec::new();
        for col in 0..=self.max_col() {
            if !self.galaxies.iter().any(|Galaxy(_, c)| c == &col) {
                empty_cols.push(col);
            }
        }

        let down_shift = |r| {
            let empty_rows = empty_rows.iter().take_while(|v| v < &&r).count();
            (value * empty_rows) - empty_rows
        };

        let right_shift = |c| {
            let empty_cols = empty_cols.iter().take_while(|v| v < &&c).count();
            (value * empty_cols) - empty_cols
        };

        self.galaxies.iter_mut().for_each(|Galaxy(r, c)| {
            *r = *r + down_shift(*r);
            *c = *c + right_shift(*c);
        });
    }

    pub fn expand(&mut self) {
        self.expand_by(2)
    }

    pub fn pairs(&self) -> impl Iterator<Item = (Galaxy, Galaxy)> + Clone + '_ {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(skip, g1)| self.galaxies.iter().skip(skip + 1).map(|g2| (*g1, *g2)))
    }
}

fn sum_of_paths(universe: &Universe) -> usize {
    universe.pairs().map(|(g1, g2)| g1.shortest_path(&g2)).sum()
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let mut galaxies = Vec::new();
    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Galaxy(row, col));
            }
        }
    }

    let mut universe = Universe::new(galaxies);
    let mut older_universe = universe.clone();

    universe.expand();
    older_universe.expand_by(1_000_000);

    println!("{}", sum_of_paths(&universe));
    println!("{}", sum_of_paths(&older_universe));

    Ok(())
}
