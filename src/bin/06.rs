use std::env;
use std::fs::File;
use std::io::{self,BufReader, BufRead, Seek, SeekFrom};
use aoc2025;

struct Solution {
    products : Vec<u64>,
    sums : Vec<u64>
}

struct Solution2 {
    products : Vec<u64>,
    sums : Vec<u64>
}


impl Solution {
    fn new () -> Self {
        Self {
            products: Vec::new(),
            sums: Vec::new(),
        }
    }
    fn feed (&mut self, input : Vec<u64> ) {
        if self.products.len() < input.len() {
            self.products.resize(input.len(), 1);
            self.sums.resize(input.len(), 0)
        }
        for (idx, val) in input.iter().enumerate() {
            self.products[idx] *= val;
            self.sums[idx] += val;
        }
    }
    fn accumulate (&mut self, input : Vec<&str> ) -> u64 {
        let mut tot = 0u64;
        for (idx, val) in input.iter().enumerate() {
            tot += match *val {
              "+" => self.sums[idx],
              "*" => self.products[idx],
            _ => panic!("oh noes"),

            }
        }
        tot
    }
}

fn part1<R : BufRead>(reader : &mut R) -> Result<u64, Box<dyn std::error::Error>> {
    let mut soln = Solution::new();
    for line in reader.lines() {
        let line = line?;
        let first = line.chars().next().unwrap();
        if first ==  '+' || first == '*' {
            let out = line.split_whitespace().collect();
            return Ok(soln.accumulate(out));
        } else {
            let ints : Vec<u64> = line.split_whitespace().map(|s|  s.parse::<u64>()).collect::<Result<_,_>>()?;
            soln.feed( ints );
        }
    }
    Ok(0)
}

fn part2<R : BufRead>(reader : &mut R) -> Result<u64, Box<dyn std::error::Error>> {
    let mut lines : Vec<Vec<u8>> = reader.lines().map(|line| line.and_then(|l| Ok(l.into_bytes()))).collect::<Result<_,_>>()?;
    let opers = lines.pop().unwrap();
    let mut i = 0;

    let mut _result : Vec<u64> = Vec::new();
    let mut tot = 0u64;

    loop { // over each sum
        if i == opers.len() {
            break;
        }
        let op = opers[i];
        assert!(op  == '+' as u8 || op == '*' as u8);

        let (mut accum, op) : (u64, fn(u64, u64) -> u64) = if op == b'+' {
            (0u64, |s,v| s+v as u64)
        } else {
            (1u64, |s,v| s*v as u64)
        };

        loop { // over each column/number
            let mut number = 0;
            for line in lines.iter() { // over each digit in the number
                match line[i] {
                    b' ' => {},
                    digit => {
                        number *= 10;
                        number += (digit - b'0') as u64;
                    }
                }
            }
            i += 1;
            if i == opers.len() || opers[i] == b' ' {
                accum = op(accum, number);
            }
            if i == opers.len() || opers[i] != ' ' as u8 {
                break;
            }
        }
        tot += accum;
    }
    Ok(tot)
}

fn main() -> io::Result<()> {
    let name = env::args().nth(1).ok_or_else(|| aoc2025::ioerr( "no input file specified"))?;
    let mut file = File::open(&name)?;

    println!("part1: {}", part1(&mut BufReader::new(&mut file)).unwrap());
    file.seek(SeekFrom::Start(0));
    println!("part1: {}", part2(&mut BufReader::new(&mut file)).unwrap());
    Ok(())
}
