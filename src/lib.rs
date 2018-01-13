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

pub fn day3_1() {
    // each port underneath is actually on a Cartesian plane
    let port = Port::new(312051);

    println!("Day 3, part 1: {} {:?}", port.distance(), (port.x, port.y));

    struct Port {
        pub id: u32,
        pub x: i64,
        pub y: i64
    }

    impl Port {
        fn distance(&self) -> i64 {
            self.x.abs() + self.y.abs()
        }
        fn new(id: u32) -> Port {
            let mut count = id.clone() - 1;
            // amm per move, count of moves, iters in the same direction
            let mut timer: (i64, i64, i64) = (1, 0, 1);
            let mut xy: (i64, i64) = (0, 0);

            'outer: loop {
                while timer.1 < timer.2 {
                    xy.0 += timer.0;
                    count -= 1;
                    if count == 0 {break 'outer}
                    timer.1 += 1;
                }
                timer.1 = 0;
                while timer.1 < timer.2 {
                    xy.1 += timer.0;
                    count -= 1;
                    if count == 0 {break 'outer}
                    timer.1 += 1;
                }
                timer.0 *= -1;
                timer.1 = 0;
                timer.2 += 1;
            }
            Port {
                id, x: xy.0, y: xy.1
            }
        }
    }
}