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