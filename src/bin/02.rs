use std::env;
use std::fs::File;
use std::io::{self,BufReader, BufRead, Seek, SeekFrom};
use std::str::FromStr;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Range {
    start : u64,
    end : u64,
}

fn ioerr(text : &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, text)
}

impl FromStr for Range {
    type Err = io::Error;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let (a,b) = s.split_once('-').ok_or_else(|| ioerr(&format!("invalid range {s}")))?;
        let start = a.trim().parse::<u64>().map_err(|e| ioerr(&format!("bad integer in range start {s}: {e}")))?;
        let end = b.trim()
            .parse::<u64>()
            .map_err(|e| ioerr(&format!("bad integer in range start {s}: {e}")))?;
        Ok(Range{start, end})
    }
}

fn parse_line<R: BufRead>(reader : &mut R) ->  io::Result<Vec<Range>> {
    let mut linebuf = String::new();
    let _linelen = reader.read_line(&mut linebuf)?;
    linebuf.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|txt| txt.trim().parse::<Range>())
        .collect()
}

fn part1<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    let ranges = parse_line( reader )?;
    let mut tot : u64 = 0;
    for range in ranges {
        let lowdigits = range.start.ilog10() + 1;
        let highdigits = range.end.ilog10() + 1;
        for i in lowdigits..=highdigits {
            if i % 2 != 0 {
                continue;
            }
            let low = if i == lowdigits { range.start } else { 10u64.pow(i-1) };
            let high = if i == highdigits { range.end } else { 10u64.pow(i) - 1};
            let digpow : u64 = 10u64.pow(i/2);

            let mut lowhigh : u64 = low / digpow;
            let lowlow : u64 = low %  digpow;
            let mut highhigh : u64 = high / digpow;
            let highlow : u64 = high %  digpow;
            if lowlow > lowhigh { lowhigh += 1 }
            if highlow < highhigh { highhigh -= 1 }
            for i in lowhigh..=highhigh { tot += i * digpow + i }

        }
    }
    Ok( tot )
}

fn topdigits( value : u64, digits : u32, repeatlen : u32 ) -> u64 {
    value / 10u64.pow(digits - repeatlen)
}

fn repeatdigits( repeated : u64, digits : u32, repeatlen : u32 ) -> u64 {
    let mut value : u64 = 0;
    for i in (0..digits).step_by(repeatlen as usize) {
        value += repeated * 10u64.pow(i);
    }
    value
}

fn part2<R : BufRead>(reader : &mut R) -> io::Result<u64> {
    let ranges = parse_line( reader )?;
    let mut tot : u64 = 0;
    for range in ranges {
        let lowdigits = range.start.ilog10() + 1;
        let highdigits = range.end.ilog10() + 1;

        // treat the problem as separate for each length of integer we care about in here.
        for digits in lowdigits..=highdigits {
            let low = if digits == lowdigits { range.start } else { 10u64.pow(digits-1) };
            let high = if digits == highdigits { range.end } else { 10u64.pow(digits)-1 };
            let mut already : HashSet<u64> = HashSet::new();
            // if I worked out how to ignore the duplicates beforehand, we could do away with the
            // innter loop below, and just calculate the number of invalid entries, rather than
            // testing every invalid entry to see if it was already seen.

            for digits_per_sequence in 1..=digits/2 {
                if digits % digits_per_sequence != 0 {
                    continue;
                }
                let mut lowseq = topdigits(low, digits, digits_per_sequence);
                let mut lowrep : u64;
                loop {
                    lowrep = repeatdigits(lowseq, digits, digits_per_sequence);
                    if lowrep >= low {
                        break;
                    }
                    lowseq += 1;
                }
                let mut highseq = topdigits(high, digits, digits_per_sequence);
                let mut highrep : u64;
                let inc = repeatdigits(1, digits, digits_per_sequence);
                loop {
                    highrep = repeatdigits(highseq, digits, digits_per_sequence);
                    if highrep <= high {
                        break;
                    }
                    highseq -= 1;
                }
                for val in (lowrep..=highrep).step_by(inc as usize) {
                    if already.insert(val) {
                        tot += val;
                    }
                }
            }

        }
    }
    Ok( tot )
}

pub fn main() -> io::Result<()> {
    let name = env::args().nth(1).ok_or_else(|| ioerr( "no input file specified"))?;
    let mut file = File::open(&name)?;
    let mut reader = BufReader::new(&mut file);
    let p1 = part1(&mut reader)?;
    println!("part1: {}", p1);

    file.seek(SeekFrom::Start(0))?;
    let mut reader = BufReader::new(&mut file);
    let start = Instant::now();
    let p2 = part2(&mut reader)?;
    let end = Instant::now();
    println!("part2: {} time taken (us) {}", p2, (end-start).as_micros());
    Ok(())
}
