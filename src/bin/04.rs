use std::env;
use std::fs::File;
use std::io::{self,BufReader, BufRead, Seek, SeekFrom};
use aoc2025;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Paper,
    Empty,
    Unknown,
}

impl From<u8> for Cell {
    fn from(b : u8) -> Self {
        match b {
            b'@' => Self::Paper,
            b'.' => Self::Empty,
            _ => Self::Unknown,
        }
    }
}

trait PaperMill {
    fn width(&self) -> isize;
    fn height(&self) -> isize;
    fn at(&self, row : isize, col : isize) -> Cell;
    fn set(&mut self, row : isize, col : isize, cell : Cell);
    fn count_around(&self, row : isize, col : isize) -> u64;
}

fn new_mill( rows : isize, cols : isize ) -> Vec<Vec<Cell>> {
    vec![vec![Cell::Empty;cols as usize];rows as usize]
}

impl PaperMill for Vec<Vec<Cell>> {
    fn width(&self) -> isize {
        self[0].len() as isize
    }
    fn height(&self) -> isize {
        self.len() as isize
    }
    fn at(&self, row : isize, col : isize) -> Cell {
        if row < 0 || col < 0 || row >= self.height() || col >= self.width() {
            Cell::Empty
        } else {
            self[row as usize][col as usize]
        }
    }
    fn set(&mut self, row : isize, col : isize, cell : Cell) {
        if row < 0 || col < 0 || row >= self.height() || col >= self.width() {
            panic!("bounds error");
        } else {
            self[row as usize][col as usize] = cell;
        }
    }
    fn count_around(&self, row : isize, col : isize) -> u64 {
        let mut tot = 0;
        for nrow in row-1..=row+1 {
            for ncol in col-1..=col+1 {
                if nrow != row || ncol != col {
                    match self.at(nrow, ncol) {
                        Cell::Paper => tot += 1,
                        _ => {}
                    }
                }
            }
        }
        tot
    }
}

fn parse_input<T : From<u8>, R : BufRead>(reader : &mut R) -> io::Result<Vec<Vec<T>>> {
    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line?;
        grid.push(line.bytes().map(T::from).collect());
    }
    Ok(grid)
}

fn mill_iterate(inp : Vec<Vec<Cell>>) -> (u64, Vec<Vec<Cell>> ) {
    let mut out = new_mill(inp.height(), inp.width());
    let mut changed = 0;
    for row in 0..inp.height() {
        for col in 0..inp.width() {
            match inp.at(row, col) {
                Cell::Paper => {
                    if inp.count_around(row, col) < 4 {
                        changed += 1;
                        out.set(row, col, Cell::Empty);
                    } else {
                        out.set(row, col, Cell::Paper);
                    }
                },
                other => {
                    out.set(row, col, other);
                }
            }
        }
    }
    (changed, out)
}

fn part1<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    let grid = parse_input::<Cell, R>(reader)?;
    let (changed, _ ) = mill_iterate(grid);
    Ok(changed)
}

fn part2<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    let mut grid = parse_input::<Cell, R>(reader)?;
    let mut tot = 0u64;
    loop {
        let (changed, gridnew) = mill_iterate(grid);
        if changed == 0 {
            break;
        }
        tot += changed;
        grid = gridnew;
    }
    Ok(tot)
}

fn main() -> io::Result<()> {
    let name = env::args().nth(1).ok_or_else(|| aoc2025::ioerr( "no input file specified"))?;
    let mut file = File::open(&name)?;
    let mut reader = BufReader::new(&mut file);
    let p1 = part1(&mut reader)?;
    println!("part1: {}", p1);

    file.seek(SeekFrom::Start(0))?;

    let mut reader = BufReader::new(&mut file);
    let p2 = part2(&mut reader)?;
    println!("part2: {}", p2);
    Ok(())
}
