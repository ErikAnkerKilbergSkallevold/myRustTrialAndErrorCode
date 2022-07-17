mod random;

use std::collections::HashSet;
use random::random_range;

pub type Posisition = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Posisition>,
    mines: HashSet<Posisition>,
    flagged_fields: HashSet<Posisition>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width, 
            height, 
            open_fields: HashSet::new(), 
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines}, 
            flagged_fields: HashSet::new()}
    }

    pub fn neighbors(&self, (x, y): Posisition) -> impl Iterator<Item = Posisition> {
        (x.max(1) - 1..=(x + 1).min(self.width - 1))
        .flat_map(|i| {
            (y.max(1) - 1..=(y + 1).min(self.height - 1)).map(move |j| (i, j))
        })
    }

    pub fn open(&mut self, posistion: Posisition) -> OpenResult {
        self.open_fields.insert(posistion);

        let is_mine = self.mines.contains(&posistion);

        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }
}


#[cfg(test)]

mod tests {
    use crate::Minesweeper;

    #[test]

    fn test() {
        let ms = Minesweeper::new(10, 10, 5);

        println!("{:?}", ms);
    }
}