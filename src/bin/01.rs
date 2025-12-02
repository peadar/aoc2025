use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader,SeekFrom, Seek, BufRead};

fn parse_line(l : &str) ->  Option<i32> {
    let (dirstr, rest ) = l.split_at(1);
    let multiplier : i32 = match dirstr {
        "L" => -1,
        "R" => 1,
        _ => return None,
    };
    let value : i32 = rest.trim().parse().ok()?;
    Some(value * multiplier)
}

fn lines<'a, R : BufRead + 'a>(reader : R) -> impl Iterator<Item=i32> + 'a {
    reader.lines().filter_map(|l| { let l = l.ok()?; parse_line(&l)})
}

fn part1<R : BufRead>(start : i32, reader : R) -> Result<u32, io::Error> {
    let mut dial = start;
    let mut count = 0;
    for amount in lines(reader) {
        dial += amount;
        if dial % 100 == 0 {
            count = count + 1
        }
    }
    Ok( count )
}

fn part2<R : BufRead>(start : i32, reader : R) -> io::Result<u32> {
    let mut dial = start;
    let mut count : u32 = 0;

    for amount in lines(reader) {
        let prevdial = dial;
        dial += amount;
        let off : i32 = if amount < 0 { -1 } else { 0 };
        count += ((prevdial + off).div_euclid( 100 ) - (dial + off).div_euclid(100)).abs() as u32;
    };
    Ok( count )
}

pub fn main() -> io::Result<()> {
    let name = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::Other, "no input file specified"))?;
    let mut file = File::open(&name)?;
    let p1 = part1(50, BufReader::new(&mut file))?;
    println!("part1: {}", p1);
    file.seek(SeekFrom::Start(0))?;
    let p2 =  part2(50, BufReader::new(&mut file))?;
    println!("part2: {}", p2);
    Ok(())
}
