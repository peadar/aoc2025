use std::env;
use std::fs::File;
use std::io::{self,BufReader, BufRead};
use std::str::FromStr;
use aoc2025;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
struct Range {
    start : u64,
    end : u64,
}

impl FromStr for Range {
    type Err = io::Error;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let (a,b) = s.split_once('-').ok_or_else(|| aoc2025::ioerr(&format!("invalid range {s}")))?;
        let start = a.trim().parse::<u64>().map_err(|e| aoc2025::ioerr(&format!("bad integer in range start {s}: {e}")))?;
        let end = b.trim()
            .parse::<u64>()
            .map_err(|e| aoc2025::ioerr(&format!("bad integer in range start {s}: {e}")))?;
        Ok(Range{start, end})
    }
}

struct Input {
    ranges : Vec<Range>,
    ingredients : Vec<u64>,
}

impl Input {
    fn from_reader<R : BufRead>(reader : R) -> io::Result<Self> {

        let mut ranges : Vec<Range> = Vec::with_capacity(200);
        let mut ingredients : Vec<u64> = Vec::with_capacity(1000);

        let mut lines = reader.lines();

        for line in &mut lines {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let range = line.parse::<Range>()?;
            ranges.push(range);
        }

        for line in &mut lines {
            let line = line?;
            let ingredient = line.trim().parse::<u64>().map_err(|_e| aoc2025::ioerr("bad int"))?;
            ingredients.push(ingredient);
        }

        // Canonicalize by sorting, then merging overlapping/adjacent ranges.
        ingredients.sort();
        ranges.sort_by_key(|x| x.start);
        let mut reduced_ranges = Vec::with_capacity(ranges.len());
        if let Some(&first) = ranges.first(){
            reduced_ranges.push(first);
            for r in ranges.iter().skip(1) {
                let earlier = reduced_ranges.last_mut().unwrap();
                if r.start <= earlier.end +1 {
                    earlier.end = earlier.end.max(r.end);
                } else {
                    reduced_ranges.push(*r)
                }
            }
        }
        Ok(Self{ranges:reduced_ranges, ingredients})
    }
}

fn part1(inp : &Input ) -> u64 {
    let mut ingi = inp.ingredients.iter();
    let mut rangei = inp.ranges.iter();
    let mut maybe_ingredient = ingi.next();
    let mut maybe_range = rangei.next();
    let mut tot = 0u64;

    while let (Some(range), Some(ingredient)) = (maybe_range, maybe_ingredient) {
        if range.end < *ingredient {
            maybe_range = rangei.next();
        } else {
            maybe_ingredient = ingi.next();
            if *ingredient >= range.start {
                tot += 1;
            }
        }
    }
    tot
}

fn part2(inp : &Input ) -> u64 {
    inp.ranges.iter().map(|range| range.end - range.start + 1).sum()
}

pub fn main() -> io::Result<()> {
    let name = env::args().nth(1).ok_or_else(|| aoc2025::ioerr( "no input file specified"))?;
    let mut file = File::open(&name)?;
    let input = Input::from_reader( BufReader::new(&mut file))?;

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}
