// https://github.com/rust-lang/rfcs/pull/2522
#![feature(type_ascription)]

use failure::Error;
use itertools::{process_results, FoldWhile, Itertools};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Error> {
    println!("day 1, part 1 = {}", solve_day1_part1()?);
    println!("day 1, part 2 = {}", solve_day1_part2()?);

    Ok(())
}

// https://adventofcode.com/2018/day/1
fn solve_day1_part1() -> Result<i32, Error> {
    let input = fs::read_to_string("input/day-1")?;
    input.lines().try_fold(0, |frequency, shift| {
        // type ascription is needed here as [i32 + ?] is ambiguous
        // turbo-fish could be used instead (as in [parse::<i32>])
        Ok(frequency + (shift.parse()?: i32))
    })
}

// https://adventofcode.com/2018/day/1#part2
fn solve_day1_part2() -> Result<i32, Error> {
    let input = fs::read_to_string("input/day-1")?;

    let mut seen = HashMap::new();
    seen.insert(0, 1);

    Ok(process_results(
        input.lines().cycle().map(str::parse),
        |mut iter| {
            iter.fold_while(0, |frequency, shift: i32| {
                let frequency = frequency + shift;
                let cnt = *seen.entry(frequency).and_modify(|e| *e += 1).or_insert(1);

                if cnt == 2 {
                    FoldWhile::Done(frequency)
                } else {
                    FoldWhile::Continue(frequency)
                }
            })
            .into_inner()
        },
    )?)
}
