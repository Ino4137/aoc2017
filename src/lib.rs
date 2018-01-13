#![allow(non_upper_case_globals)]
//#![feature(slice_patterns)]

mod data;
use data::*;

pub fn day1_1() {
    let mut data: Vec<char> = d_day1.chars().collect();
    let firstchar = data[0];
    data.push(firstchar);

    let tot: u32 = data.windows(2).map(|window| {
        let (uno, dos) = (window[0], window[1]);
        if uno == dos {
            uno.to_digit(10).unwrap()
        } else {
            0
        }
    }).sum();

    println!("Day 1, part 1: {}", tot);
}

pub fn day1_2() {
    let tot: u32 = d_day1.chars().zip(
        d_day1.chars().cycle().skip(
            d_day1.chars().count() / 2
    )).map(|(uno, dos)| {
        if uno == dos {
            uno.to_digit(10).unwrap()
        } else {
            0
        }}).sum();

    println!("Day 1, part 2: {}", tot);
}

use std::cmp::Ordering::*;
pub fn day2_1() {
    let checksum: u32 = d_day2.split(|c| c == '\n').map(
        |row| row.split(char::is_whitespace).fold((9999, 0, 0), |(min, max, sum), num| {
            let n: u32 = num.parse().unwrap();

            match (n.cmp(&min), n.cmp(&max)) {
                (Less, Greater) => (n, n, 0),
                (Less, _)       => (n, max, max-n),
                (_, Greater)    => (min, n, n-min),
                (_, _)          => (min, max, sum)
            }           
    }).2).sum();

    println!("Day 2, part 1: {}", checksum);
}

pub fn day2_2() {
    let mut checksum: u32 = 0;
    'forlines: for line in d_day2.lines() {
        let mut line: Vec<u32> = line.split_whitespace().map(|v| v.parse().unwrap()).collect();

        line.sort();
        line.reverse();

        let mut position = 0;
        while position < line.len() {
            let mut small_position = position + 1;
            while small_position < line.len(){

                if line[position] % line[small_position] == 0 {
                    checksum += line[position] / line[small_position];
                    continue 'forlines
                }
                small_position += 1;
            }
            position += 1; 
        }
        unreachable!("Couldn't find a divisible pair in: {:?}", line); 
    }
    println!("Day 2, part 2: {}", checksum);
}