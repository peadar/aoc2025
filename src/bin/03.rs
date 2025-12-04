use std::env;
use std::fs::File;
use std::io::{self,BufReader, BufRead, Seek, SeekFrom};
use aoc2025;

fn solve_line(sz : usize, bs : &[u8]) -> u64 {
    let mut digits : Vec<u8> = Vec::new();
    let mut discards = bs.len() - sz;
    for c in bs {
        let i = c - '0' as u8;
        while discards > 0 && digits.last().is_some_and(|&last| last < i)  {
            digits.pop();
            discards -= 1;
        }
        digits.push(i);
    }
    digits.truncate(sz);
    digits.into_iter().fold(0, |acc, digit| acc * 10u64 + digit as u64)
}

fn solve<R : BufRead>(sz : usize, reader : &mut R) -> io::Result<u64> {
    Ok( reader.lines().map(|l| solve_line(sz, l.unwrap().as_bytes())).sum())
}

fn part1<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    solve( 2, reader )
}

fn part2<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    solve(12, reader)
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
