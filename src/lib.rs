#![feature(trace_macros)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;

mod data;
use data::*;

pub fn day1_1() {
    let mut data: Vec<char> = d_day1.chars().collect();
    let firstchar = data[0];
    // so that it gets grouped w/o making it wrap around
    data.push(firstchar);

    // leaves the number in place if the following one is the same
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
    // zip every character with its corresponding one, 
    // half the list later
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
        // in place of every row a tuple is left, 
        // that holds the row's "sum"
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

        // goes over every pair in a line, 
        // trying to find a divisible pair
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
                if count == 0 {break 'outer}
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

use std::collections::HashMap;
pub fn day3_2() {
    // each port underneath is actually on a Cartesian plane
    type XYV = (i64, i64, i64);
    let mut plane: HashMap<u32, XYV> = HashMap::new();
    plane.insert(1, (0, 0, 1));
    println!("Day 3, Part 2: {}", Port::find_first_gt(312051, &mut plane));
 
    fn neighbors(x1: i64, y1: i64, x: i64, y: i64) -> bool {
            if ((x1 - x).abs() > 1 || (y1 - y).abs() > 1) 
            && ((x1 - x).abs() + (y1 - y).abs()) > 1 {
                false
            } else { true }
    }

    struct Port {
        pub id: u32,
        pub x: i64,
        pub y: i64
    }

    impl Port {
        fn find_first_gt(id: u32, hmap: &mut HashMap<u32, XYV>) -> i64 {
            let mut pos: u32 = 1;
            // amm per move, count of moves, iters in the same direction
            let mut timer: (i64, i64, i64) = (1, 0, 1);
            let mut xy: (i64, i64) = (0, 0);

            'outer: loop {
                while timer.1 < timer.2 {
                    xy.0 += timer.0;
                    let mut value: i64 = 0;
                    // should only loop through its neighbors,
                    // the HM keys should be the x and y coords
                    // oh well
                    for (_, &(x, y, v)) in hmap.iter() {
                        if neighbors(xy.0, xy.1, x, y){
                            value += v;
                        }
                    }
                    if value > id as i64 {return value}
                    pos += 1;
                    hmap.insert(pos, (xy.0, xy.1, value));
                    timer.1 += 1;
                }
                timer.1 = 0;
                while timer.1 < timer.2 {
                    xy.1 += timer.0;
                    let mut value: i64 = 0;
                    for (_, &(x, y, v)) in hmap.iter() {
                        if neighbors(xy.0, xy.1, x, y) {
                            value += v;
                        }
                    }
                    if value > id as i64 {return value}
                    pos += 1;
                    hmap.insert(pos, (xy.0, xy.1, value));
                    timer.1 += 1;
                }
                timer.0 *= -1;
                timer.1 = 0;
                timer.2 += 1;
            }
        }
    }
}

pub fn day4_1() {
    let mut amm = 0;
    'lines: for line in d_day4.lines() {
        let mut prev = Vec::new();
        for pass in line.split_whitespace() {
            if prev.contains(&pass) {
                continue 'lines
            }
            prev.push(pass);
        }
        amm += 1;
    }
    println!("Day 4, Part 1: {}", amm);
}

pub fn day4_2() {
    let mut amm = 0;
    'lines: for line in d_day4.lines() {
        let mut prev = Vec::new();
        for pass in line.split_whitespace() {
            let mut pass = pass.chars().collect::<Vec<char>>();
            // comparing sorted strings
            pass.sort();
            pass.iter().collect::<String>();
            for val in &prev {
                if *val == pass {
                    continue 'lines
                }
            }
            prev.push(pass);
        }
        amm += 1;
    }
    println!("Day 4, Part 2: {}", amm);
}

pub fn day5_1() {
    let mut instructions: Vec<i32> = d_day5.lines().map(|v| {
        v.parse().unwrap()
    }).collect();

    let mut node = 0;
    let mut index_next: usize = 0;
    let mut steps = 0;

    loop {
        if let Some(x) = instructions.get_mut(index_next) {
            node = *x;
            *x += 1; 
        };

        // usize cannot be negative, u stands for unsigned
        if node.is_negative() {
            index_next -= node.abs() as usize
        } else {
            index_next += node as usize;
        }

        // if the next step would be out of bounds
        if instructions.len() < index_next - 1 {
            break;
        }
        // take a step
        steps += 1;
    }

    println!("Day 5, Part 1: {:?}", steps);
}

pub fn day5_2() {
    let mut instructions: Vec<i32> = d_day5.lines().map(|v| {
        v.parse().unwrap()
    }).collect();

    let mut node = 0;
    let mut index_next: usize = 0;
    let mut steps = 0;

    loop {
        if let Some(x) = instructions.get_mut(index_next) {
            node = *x;
            if *x < 3 {
                *x += 1; 
            } else {
                *x -= 1;
            }
        };

        if node.is_negative() {
            index_next -= node.abs() as usize
        } else {
            index_next += node as usize;
        }

        if instructions.len() < index_next - 1 {
            break;
        }
        steps += 1
    }

    println!("Day 5, Part 2: {:?}", steps);
}

pub fn day6_1() {
    let mut premuts: Vec<Vec<u32>> = Vec::new();
    let mut curr: Vec<u32> = d_day6.split_whitespace().map(|n| {
        n.parse().unwrap()
    }).collect();
    let mut amm = 0;
    // push the original state as the first premutation
    premuts.push(curr.clone());

    'redistribution: loop {
        let mut max = 0;
        // <0, 15>
        for num in 0..16 {
            if curr[num] > curr[max] {
                max = num;
            }
        }
        
        let mut value = 0;
        if let Some(x) = curr.get_mut(max) {
            value = x.clone();
            *x = 0;
        } else {
            unreachable!("Out of bounds on assigning value");
        }

        // actually redistribute the memory
        for index in (0..16).cycle().skip(max + 1) {
            curr[index] += 1;
            value -= 1;
            if value == 0 {
                break
            }
        }

        amm += 1;
        println!("{}: {:?}",amm, curr);
        if premuts.contains(&curr) {
            break 'redistribution
        }
        premuts.push(curr.clone());
    }

    println!("Day 6, Part 1: {}", amm);
}

pub fn day6_2() {
    let mut premuts: Vec<Vec<u32>> = Vec::new();
    let mut curr: Vec<u32> = d_day6.split_whitespace().map(|n| {
        n.parse().unwrap()
    }).collect();
    premuts.push(curr.clone());

    let mut clock = 0;
    let mut enc = false;

    'redistribution: loop {
        let mut max = 0;
        for num in 0..16 {
            if curr[num] > curr[max] {
                max = num;
            }
        }
        
        let mut value = 0;
        if let Some(x) = curr.get_mut(max) {
            value = x.clone();
            *x = 0;
        } else {
            unreachable!("Out of bounds on assigning value");
        }

        for index in (0..16).cycle().skip(max + 1) {
            curr[index] += 1;
            value -= 1;
            if value == 0 {
                break
            }
        }

        if enc {
            clock += 1;
        }

        if premuts.contains(&curr) {
            if enc {
                break 'redistribution
            } else {
                // loop through it again, 
                // this time count the steps from the start of the loop
                enc = true;
                premuts = Vec::new();
            }
        }
        premuts.push(curr.clone());
    }

    println!("Day 6, Part 2: {}", clock);
}

pub fn day7_1() {
    #[derive(Debug, Clone)]
    struct Prog {
        pub name: String,
        pub weight: u32,
        pub holds: Option<Vec<String>>,
        pub owned_by: Option<String>
    }

    let parse_p = |line: &str| {
        // extracts the name and the weigth
        let get_nw = |data: &mut Vec<String>| {
            let name = data[0].clone();
            data[1].remove(0);
            let len = data[1].len();
            data[1].remove(len - 1);
            let weight: u32 = data[1].parse().unwrap();

            (name, weight)
        };

        let mut data: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();
        let (name, weight) = get_nw(&mut data);

        // tests if there are childeren or not
        if data.len() == 2 {        
            Prog {
                name,
                weight,
                holds: None,
                owned_by: None
            }
        } else {
            let mut children = Vec::new();
            let child_amm = data.len() - 3;

            'children: for n in 0..child_amm {
                let mut buffer = String::new();
                for symbol in data[3 + n].chars() {
                    if symbol == ',' {
                        children.push(buffer);
                        continue 'children
                    }
                    buffer.push(symbol);
                }
                // last child has no comma
                children.push(buffer);
            }

            Prog {
                name,
                weight,
                holds: Some(children),
                owned_by: None
            }
        }
    };

    let mut dict: HashMap<String, Prog> = HashMap::new();

    // parse the input
    for line in d_day7.lines() {
        let prog = parse_p(line);
        //println!("{:?}", prog);
        dict.insert(prog.name.clone(), prog);
    }

    // if any key is not in the owned_by field of any other prog, then it is the root key/Prog
    let root = 'end: loop {
        'nextr: for k in dict.keys() {
            for v in dict.values() {
                if v.holds.is_some() {
                    if v.clone().holds.unwrap().contains(k) {
                        continue 'nextr
                    }
                }
            }
            break 'end k
        }
    };

    println!("Day 7, Part 1: {}", root);
}

pub fn day7_2() {
    #[derive(Debug, Clone)]
    struct Prog {
        pub name: String,
        pub weight: u32,
        pub holds: Option<Vec<String>>,
        pub owned_by: Option<String>
    }

    let parse_p = |line: &str| {
        // extracts the name and the weigth
        let get_nw = |data: &mut Vec<String>| {
            let name = data[0].clone();
            data[1].remove(0);
            let len = data[1].len();
            data[1].remove(len - 1);
            let weight: u32 = data[1].parse().unwrap();

            (name, weight)
        };

        let mut data: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();
        let (name, weight) = get_nw(&mut data);

        // tests if there are childeren or not
        if data.len() == 2 {        
            Prog {
                name,
                weight,
                holds: None,
                owned_by: None
            }
        } else {
            let mut children = Vec::new();
            let child_amm = data.len() - 3;

            'children: for n in 0..child_amm {
                let mut buffer = String::new();
                for symbol in data[3 + n].chars() {
                    if symbol == ',' {
                        children.push(buffer);
                        continue 'children
                    }
                    buffer.push(symbol);
                }
                // last child has no comma
                children.push(buffer);
            }

            Prog {
                name,
                weight,
                holds: Some(children),
                owned_by: None
            }
        }
    };

    let mut dict: HashMap<String, Prog> = HashMap::new();

    // parse the input
    for line in d_day7.lines() {
        let prog = parse_p(line);
        //println!("{:?}", prog);
        dict.insert(prog.name.clone(), prog);
    }

    fn rec_sum_of_branches(name: &String, dict: &HashMap<String, Prog>) -> u32 {
        let Prog { ref name, ref weight, holds: ref branch, ref owned_by } = *dict.get(name).unwrap();
        let mut sum: Vec<u32> = Vec::new();   
        if branch.is_some() {
            for key in branch.clone().unwrap().iter() { 
                sum.push(rec_sum_of_branches(key, dict));
            } 
        }
        sum.push(*weight);
        println!("{}'s branch: {:?}", name, sum);
        sum.iter().sum::<u32>()
    }

    // this solution is valid due to keen observation
    // proper one would have the result presented but oh well
    rec_sum_of_branches(&"gozhrsf".to_owned(), &dict); // -5
}

pub fn day8_1() {
    let mut registers: HashMap<String, i32> = HashMap::new();

    fn parse_l(line: String, registers: &mut HashMap<String, i32>) -> &mut HashMap<String, i32>{
        let query = line.split_whitespace().map(|d| d.to_owned()).collect::<Vec<String>>();
        
        // create entries for the registers, if not-existent
        registers.entry(query[0].clone()).or_insert(0);
        registers.entry(query[4].clone()).or_insert(0);

        let cond = query[6].parse::<i32>().unwrap();
        let amm = query[2].parse::<i32>().unwrap();

        match &*query[5] {
            ">" => {
                if *registers.get(&*query[4]).unwrap() > cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "<" => {
                if *registers.get(&*query[4]).unwrap() < cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            ">=" => {
                if *registers.get(&*query[4]).unwrap() >= cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "<=" => {
                if *registers.get(&*query[4]).unwrap() <= cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "==" => {
                if *registers.get(&*query[4]).unwrap() == cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "!=" => {
                if *registers.get(&*query[4]).unwrap() != cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
        registers
    };

    for line in d_day8.lines() {
        parse_l(line.to_owned(), &mut registers);
    }

    println!("Day 8, Part 1: {:?}", registers.iter().fold((String::new(), -999), |(name, max), (k, v)| { 
        if v > &max {
            (k.clone(), v.clone())
        } else {
            (name, max)
        }}));
}

pub fn day8_2() {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest = 0;

    fn parse_l<'a>(line: String, registers: &'a mut HashMap<String, i32>, highest: &mut i32) -> &'a mut HashMap<String, i32>{
        let query = line.split_whitespace().map(|d| d.to_owned()).collect::<Vec<String>>();

        // create entries for the registers, if not-existent
        registers.entry(query[0].clone()).or_insert(0);
        registers.entry(query[4].clone()).or_insert(0);
        
        let cond = query[6].parse::<i32>().unwrap();
        let amm = query[2].parse::<i32>().unwrap();
        
        match &*query[5] {
            ">" => {
                if *registers.get(&*query[4]).unwrap() > cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "<" => {
                if *registers.get(&*query[4]).unwrap() < cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            ">=" => {
                if *registers.get(&*query[4]).unwrap() >= cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "<=" => {
                if *registers.get(&*query[4]).unwrap() <= cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "==" => {
                if *registers.get(&*query[4]).unwrap() == cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            "!=" => {
                if *registers.get(&*query[4]).unwrap() != cond {
                    if query[1] == "dec" {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x -= amm;
                        }
                    } else {
                        if let Some(x) = registers.get_mut(&*query[0]) {
                            *x += amm;
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
        if let Some(x) = registers.get(&*query[0]) {
            if x > highest {
                *highest = *x;
            }
        }
        registers
    };

    for line in d_day8.lines() {
        parse_l(line.to_owned(), &mut registers, &mut highest);
    }

    println!("Day 8, Part 2: {}", highest);
}

pub fn day9_1() {
    #[derive(Debug)]
    struct Stream {
        pub data: Vec<char>
    }

    impl Stream {
        fn new(data: &str) -> Stream {
            let data = data.chars().collect::<Vec<char>>();
            Stream {
                data
            }
        }
        fn score(&self) -> u32 {
            let mut level = 1;
            let mut score = 0;
            let mut in_garbage = false;
            let mut iterator = self.data.iter();
            
            while let Some(x) = iterator.next() {
                match x {
                    &'<' => {
                        if !in_garbage {
                            in_garbage = true;
                        }
                    },
                    &'>' => {
                        in_garbage = false;
                    },
                    &'{' => {
                        if !in_garbage {
                            score += level;
                            level += 1;
                        }
                    },
                    &'}' => {
                        if !in_garbage {
                            level -= 1;
                        }
                    },
                    &'!' => {
                        // skip one
                        iterator.next();
                    },
                    _   => {
                        // null
                    }
                }
            }
            score
        } 
    }
    println!("Day 9, Part 1: {:?}", Stream::new(d_day9).score());
}

pub fn day9_2() {
    #[derive(Debug)]
    struct Stream {
        pub data: Vec<char>
    }

    impl Stream {
        fn new(data: &str) -> Stream {
            let data = data.chars().collect::<Vec<char>>();
            Stream {
                data
            }
        }
        fn scan(&self) -> (u32, u32) {
            let mut level = 1;
            let mut score = 0;
            let mut garbage = 0;
            let mut in_garbage = false;
            let mut iterator = self.data.iter();
            
            while let Some(x) = iterator.next() {
                match x {
                    &'<' => {
                        if !in_garbage {
                            in_garbage = true;
                        } else {
                            garbage += 1
                        }
                    },
                    &'>' => {
                        in_garbage = false;
                    },
                    &'{' => {
                        if !in_garbage {
                            score += level;
                            level += 1;
                        } else {
                            garbage += 1;
                        }
                    },
                    &'}' => {
                        if !in_garbage {
                            level -= 1;
                        } else {
                            garbage += 1;
                        }
                    },
                    &'!' => {
                        // skip one
                        iterator.next();
                    },
                    _   => {
                        if in_garbage {
                            garbage += 1;
                        }
                    }
                }
            }
            (score, garbage)
        } 
    }
    let (score, garbage) = Stream::new(d_day9).scan();
    println!("Day 9, Part 2: score:{}, in garbage:{}",score, garbage);
}

pub fn day10_1() {
    #[derive(Debug)]
    struct Knot {
        list: Vec<u32>,
        curr_pos: u32,
        skip_size: u32
    }
    impl Knot {
        fn new(list: Vec<u32>) -> Knot {
            Knot {
                list,
                curr_pos: 0,
                skip_size: 0,
            }
        }
        fn incr_pos(&mut self, chan: u32) {
            if self.list.len() as u32 <= self.curr_pos + chan + self.skip_size {
                self.curr_pos = self.curr_pos + chan + self.skip_size - self.list.len() as u32;
            } else {
                self.curr_pos += chan + self.skip_size;
            }
        }
        fn twist(&mut self, length: &u32) {
            // 0 and 1 change nothing in the list
            if *length != 0 && *length != 1 {
                let twisted: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip(self.curr_pos as usize)
                    .take(*length as usize).collect();
                let mut twisted: Vec<u32> = twisted.into_iter().rev().collect();

                let mut not_taken: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip((self.curr_pos + length) as usize)
                    .take(self.list.len() - *length as usize).collect();

                let mut temp: Vec<u32> = Vec::new();

                match (self.curr_pos + length).cmp(&(self.list.len() as u32)) {
                    Greater => {
                        let len = twisted.len();
                        let amm_pushed = self.curr_pos + length - self.list.len() as u32;
                        temp.append(
                            &mut twisted.clone().into_iter().skip(len - amm_pushed as usize).collect()
                        );
                        temp.append(&mut not_taken);
                        temp.append(
                            &mut twisted.into_iter().take(len - amm_pushed as usize).collect()
                        );
                    },
                    Equal => {
                        temp.append(&mut not_taken);
                        temp.append(&mut twisted);
                    },
                    Less => {
                        let len = not_taken.len();
                        temp.append(
                            &mut not_taken.clone().into_iter().skip(len - self.curr_pos as usize).collect()
                        );
                        temp.append(&mut twisted);
                        temp.append(
                            &mut not_taken.into_iter().take(len - self.curr_pos as usize).collect()
                        );
                    }
                } 
                self.list = temp
            }
        }
        fn hash(&mut self, lengths: Vec<u32>) {
            // reverses since it executes by popping them
            let mut lengths: Vec<u32> = lengths.into_iter().rev().collect();

            while let Some(x) = lengths.pop() {
                self.twist(&x);    
                self.incr_pos(x);
                self.skip_size += 1;
            };
        }
    }
    
    let mut knot = Knot::new((0..256).into_iter().collect::<Vec<u32>>());
    knot.hash(d_day10.split(',').map(|v| {
        v.parse::<u32>().unwrap()
    }).collect());

    println!("Day 10, Part 1: {}", knot.list[0] * knot.list[1]);
}

pub fn day10_2() {
    #[derive(Debug)]
    struct Knot {
        list: Vec<u32>,
        curr_pos: u32,
        skip_size: u32
    }
    impl Knot {
        fn new(list: Vec<u32>) -> Knot {
            Knot {
                list,
                curr_pos: 0,
                skip_size: 0,
            }
        }
        fn incr_pos(&mut self, chan: u32) {
            self.curr_pos = (self.curr_pos + chan + self.skip_size) % 256

        }
        fn dense_hash(&self) {
            let mut t: Vec<u32> = Vec::new();
            let mut list = self.list.clone();

            for _ in 0..16 {
                t.push(list.iter().take(16).fold(0, |acc, &n| {acc ^ n}));
                list = list.into_iter().skip(16).collect();
            }

            print!("Day 10, Part 2: ");
            for x in t.iter() {
                print!("{:x}", x);
            }
        }
        fn twist(&mut self, length: &u32) {
            // 0 and 1 change nothing in the list
            if *length != 0 && *length != 1 {
                let twisted: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip(self.curr_pos as usize)
                    .take(*length as usize).collect();
                let mut twisted: Vec<u32> = twisted.into_iter().rev().collect();

                let mut not_taken: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip((self.curr_pos + length) as usize)
                    .take(self.list.len() - *length as usize).collect();

                let mut temp: Vec<u32> = Vec::new();
                
                match (self.curr_pos + length).cmp(&(self.list.len() as u32)) {
                    Greater => {
                        let len = twisted.len();
                        let amm_pushed = self.curr_pos + length - self.list.len() as u32;
                        temp.append(
                            &mut twisted.clone().into_iter().skip(len - amm_pushed as usize).collect()
                        );
                        temp.append(&mut not_taken);
                        temp.append(
                            &mut twisted.into_iter().take(len - amm_pushed as usize).collect()
                        );
                    },
                    Equal => {
                        temp.append(&mut not_taken);
                        temp.append(&mut twisted);
                    },
                    Less => {
                        let len = not_taken.len();
                        temp.append(
                            &mut not_taken.clone().into_iter().skip(len - self.curr_pos as usize).collect()
                        );
                        temp.append(&mut twisted);
                        temp.append(
                            &mut not_taken.into_iter().take(len - self.curr_pos as usize).collect()
                        );
                    }
                } 
                self.list = temp
            }
        }
        fn hash(&mut self, lengths: Vec<u32>) {
            // reverses since it executes by popping them
            let lengths: Vec<u32> = lengths.into_iter().rev().collect();

            for _ in 0..64 {
                let mut temp = lengths.clone();
                while let Some(x) = temp.pop() {
                    self.twist(&x);    
                    self.incr_pos(x);
                    self.skip_size += 1;
                };
            }
            self.dense_hash();
        }
    }
    let mut lengths: Vec<u32> = d_day10.bytes().map(|v| v as u32).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let list: Vec<u32> = (0..256).into_iter().collect();

    let mut knot = Knot::new(list);
    knot.hash(lengths);
}

pub fn day11_1() {
    #[derive(Debug, Clone)]
    struct HexPath {
        n: u32,
        ne: u32,
        se: u32,
        s: u32,
        sw: u32,
        nw: u32,
    }

    impl HexPath {
        fn new(path: &'static str) -> HexPath {
            let mut n = 0;
            let mut ne = 0;
            let mut se = 0;
            let mut s = 0;
            let mut sw = 0;
            let mut nw = 0;
            let _: Vec<_> = path.split(',').map(|dir| {
                match dir {
                    "n"  => n += 1,
                    "ne" => ne += 1,
                    "se" => se += 1,
                    "s"  => s += 1,
                    "sw" => sw += 1,
                    "nw" => nw += 1,
                    _    => unreachable!()
                }
            }).collect();

            HexPath{ n, ne, se, s, sw, nw }
        }
        fn distance(&self) -> u32 {
            self.n + self.ne + self.se + self.s + self.sw + self.nw
        }
        fn weed(&mut self) {
            // weed out the step-backs
            if self.n > self.s {
                self.n -= self.s;
                self.s = 0;
            } else {
                self.s -= self.n;
                self.n = 0;
            }
            if self.ne > self.sw {
                self.ne -= self.sw;
                self.sw = 0;
            } else {
                self.sw -= self.ne;
                self.ne = 0;
            }
            if self.nw > self.se {
                self.nw -= self.se;
                self.se = 0;
            } else {
                self.se -= self.nw;
                self.nw = 0;
            }

            // weed out the unnecessary steps
            if self.ne > self.s {
                self.ne -= self.s; 
            } else {
                self.s -= self.ne;
            }
            if self.ne > self.nw {
                self.ne -= self.nw; 
            } else {
                self.nw -= self.ne;
            }
            if self.n > self.se {
                self.n -= self.se; 
            } else {
                self.se -= self.n;
            }
            if self.n > self.sw {
                self.n -= self.sw; 
            } else {
                self.sw -= self.n;
            } 
        }
    }

    let mut hp = HexPath::new(d_day11);
    hp.weed();
    println!("Day 11, Part 1: {:?}", hp.distance());
}

pub fn day11_2() {
    #[derive(Debug, Clone)]
    struct HexPathMax {
        n: u32,
        ne: u32,
        se: u32,
        s: u32,
        sw: u32,
        nw: u32,
        max_dist: u32
    }

    impl HexPathMax {
        fn distance(&self) -> u32 {
            self.n + self.ne + self.se + self.s + self.sw + self.nw
        }
        fn weed(&mut self) {
            // weed out the step-backs
            if self.n > self.s {
                self.n -= self.s;
                self.s = 0;
            } else {
                self.s -= self.n;
                self.n = 0;
            }
            if self.ne > self.sw {
                self.ne -= self.sw;
                self.sw = 0;
            } else {
                self.sw -= self.ne;
                self.ne = 0;
            }
            if self.nw > self.se {
                self.nw -= self.se;
                self.se = 0;
            } else {
                self.se -= self.nw;
                self.nw = 0;
            }

            // weed out the unnecessary steps
            if self.ne > self.s {
                self.ne -= self.s; 
            } else {
                self.s -= self.ne;
            }
            if self.ne > self.nw {
                self.ne -= self.nw; 
            } else {
                self.nw -= self.ne;
            }
            if self.n > self.se {
                self.n -= self.se; 
            } else {
                self.se -= self.n;
            }
            if self.n > self.sw {
                self.n -= self.sw; 
            } else {
                self.sw -= self.n;
            } 
        }
        fn new(path: &'static str) -> HexPathMax {
            let mut n = 0;
            let mut ne = 0;
            let mut se = 0;
            let mut s = 0;
            let mut sw = 0;
            let mut nw = 0;
            let mut max_dist = 0;
            let _: Vec<_> = path.split(',').map(|dir| {
                match dir {
                    "n"  => n += 1,
                    "ne" => ne += 1,
                    "se" => se += 1,
                    "s"  => s += 1,
                    "sw" => sw += 1,
                    "nw" => nw += 1,
                    _    => unreachable!()
                }

                // after every step it calculates the distance and if greater than max, assigns it
                let mut t = HexPathMax {n, ne, se, s, sw, nw, max_dist};
                t.weed();
                let d = t.distance();
                if d > max_dist {
                    max_dist = d;
                }

            }).collect();

            HexPathMax {n, ne, se, s, sw, nw, max_dist}
        }
    }

    let hp = HexPathMax::new(d_day11);
    println!("Day 11, Part 2: {:?}", hp.max_dist);
}

pub fn day12_1() {
    let mut pipe_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in d_day12.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();       
        let mut list = Vec::new();

        for x in 2..line.len() {
            list.push(line[x].chars().take_while(|c| c.is_digit(10))
                .collect::<String>().parse::<u32>().unwrap());
        }

        pipe_map.insert(line[0].parse().unwrap(), list);
    }

    fn mark_connected(num: &u32, hm: &HashMap<u32, Vec<u32>>, seen: &mut Vec<u32>) -> u32 {
        let list = hm.get(num).unwrap();
        let mut sum = 0;
        for v in list {  
            // I REALLY like how this debug print looks, and that is why i left it here
            println!("{} : {} <- {:?}", num, v, list);
            if !seen.contains(&v) {
                sum += 1;
                seen.push(*v);
                sum += mark_connected(v, &hm, seen);
            }
        }
        sum
    }
    let mut seen = Vec::new();

    println!("Day 12, Part 1: {}", mark_connected(&0, &pipe_map, &mut seen));
}

pub fn day12_2() {
    let mut pipe_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in d_day12.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();       
        let mut list = Vec::new();

        for x in 2..line.len() {
            list.push(line[x].chars().take_while(|c| c.is_digit(10))
                .collect::<String>().parse::<u32>().unwrap());
        }

        pipe_map.insert(line[0].parse().unwrap(), list);
    }

    fn mark_connected(num: &u32, hm: &HashMap<u32, Vec<u32>>, seen: &mut Vec<u32>) {
        let list = hm.get(num).unwrap();
        for v in list {
            // I REALLY like how this debug print looks, and that is why i left it here        
            println!("{} : {} <- {:?}", num, v, list);
            if !seen.contains(&v) {
                seen.push(*v);
                mark_connected(v, &hm, seen);
            }
        }
    }
    let mut seen = Vec::new();
    let mut gr_count = 0;
    for n in 0..2000 {
        if !seen.contains(&n) {
            mark_connected(&n, &pipe_map, &mut seen);
            gr_count += 1;
        }
    }
    

    println!("Day 12, Part 2: {}", gr_count);
}

pub fn day13_1() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Enterance {
        Caught(usize),
        Free
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Way {
        Up,
        Down
    }

    #[derive(Debug)]
    struct Layer {
        layer: usize, // if 0 -> empty
        pos: usize,
        way: Way
    }

    impl Layer {
        fn new(layer: usize) -> Layer {
            Layer { layer, pos: 0 , way: Way::Down}
        }
        
        fn tick(&mut self) {
            if self.layer != 0 {
                'lewp: loop {
                    match self.way {
                        Way::Down => {
                            if self.pos + 1 == self.layer {
                                self.way = Way::Up;
                                continue 'lewp
                            }
                            self.pos += 1;
                            break 'lewp
                        },
                        Way::Up   => {
                            if self.pos == 0 {
                                self.way = Way::Down;
                                continue 'lewp
                            }
                            self.pos -= 1;
                            break 'lewp
                        } 
                    }
                }
            }
        }

        fn enter(&self) -> Enterance {
            if self.pos == 0 && self.layer != 0 {
                Enterance::Caught(self.layer)
            } else {
                Enterance::Free
            }
        }
    }

    #[derive(Debug)]
    struct Firewall {
        firewall: HashMap<usize, Layer>
    }

    impl Firewall {
        fn new(data: &str) -> Firewall {
            let mut firewall = HashMap::new();

            for line in data.lines() {
                let mut line: Vec<&str> = line.split_whitespace().collect();
                let val: usize = line[1].parse().unwrap();
                let id: usize  = line[0].chars().take_while(|w| w.is_digit(10))
                    .collect::<String>().parse().unwrap();

                firewall.insert(id, Layer::new(val));
            }

            for n in 0..*firewall.keys().max().unwrap() {
                firewall.entry(n).or_insert(Layer::new(0));
            }

            Firewall { firewall }
        }

        fn traverse(&mut self) -> usize {
            let mut severity = 0;

            for at in 0..*self.firewall.keys().max().unwrap() + 1 {
                if let Some(l) = self.firewall.get_mut(&at) {

                    // if caught, increase severity
                    if let Enterance::Caught(depth) = l.enter() {
                        severity += at * depth;
                    }
                } else { unreachable!() }

                // move all pieces
                for layer in self.firewall.values_mut() {
                    layer.tick();
                }
            }

            severity
        }
    }

    let mut firewall = Firewall::new(d_day13);

    println!("Day 13, Part 1: {:?}", firewall.traverse());
}

pub fn day13_2() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Enterance {
        Caught(usize),
        Free
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum Way {
        Up,
        Down
    }

    #[derive(Debug, Clone)]
    struct Layer {
        layer: usize, // if 0 -> empty
        pos: usize,
        way: Way
    }

    impl Layer {
        fn new(layer: usize) -> Layer {
            Layer { layer, pos: 0 , way: Way::Down}
        }
        
        fn tick(&mut self) {
            if self.layer != 0 {
                'lewp: loop {
                    match self.way {
                        Way::Down => {
                            if self.pos + 1 == self.layer {
                                self.way = Way::Up;
                                continue 'lewp
                            }
                            self.pos += 1;
                            break 'lewp
                        },
                        Way::Up   => {
                            if self.pos == 0 {
                                self.way = Way::Down;
                                continue 'lewp
                            }
                            self.pos -= 1;
                            break 'lewp
                        } 
                    }
                }
            }
        }

        fn enter(&self) -> Enterance {
            if self.pos == 0 && self.layer != 0 {
                Enterance::Caught(self.layer)
            } else {
                Enterance::Free
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Firewall {
        firewall: HashMap<usize, Layer>
    }

    impl Firewall {
        fn new(data: &str) -> Firewall {
            let mut firewall = HashMap::new();

            for line in data.lines() {
                let mut line: Vec<&str> = line.split_whitespace().collect();
                let val: usize = line[1].parse().unwrap();
                let id: usize  = line[0].chars().take_while(|w| w.is_digit(10))
                    .collect::<String>().parse().unwrap();

                firewall.insert(id, Layer::new(val));
            }

            for n in 0..*firewall.keys().max().unwrap() {
                firewall.entry(n).or_insert(Layer::new(0));
            }

            Firewall { firewall }
        }

        fn move_time(&mut self) {
            for layer in self.firewall.values_mut() {
                layer.tick();
            }
        }

        fn traverse(&mut self) -> Enterance {         
            for at in 0..*self.firewall.keys().max().unwrap() + 1 {
                if let Some(l) = self.firewall.get_mut(&at) {

                    // if caught, return 1
                    if let Enterance::Caught(depth) = l.enter() {
                        return Enterance::Caught(at)
                    }
                } else { unreachable!() }

                self.move_time();
            }

            Enterance::Free
        }
    }

    let mut firewall = Firewall::new(d_day13);
    let mut delay = 0;

    loop {
        // takes a bit due to all the cloning involved
        firewall.move_time();
        let firewallc = firewall.clone();
        delay += 1;
        print!("Delay {}", delay);
        
        if let Enterance::Caught(at) = firewall.traverse() {
            println!(", Caught at: {}", at);
        } else { break }
        firewall = firewallc;
    }
    println!(", Finished");

    println!("Day 13, Part 2: {:?}", delay);
}

pub fn day14_1() {
    #[derive(Debug, Clone)]
    struct Knot {
        list: Vec<u32>,
        curr_pos: u32,
        skip_size: u32
    }
    impl Knot {
        fn new(list: Vec<u32>) -> Knot {
            Knot {
                list,
                curr_pos: 0,
                skip_size: 0,
            }
        }
        fn incr_pos(&mut self, chan: u32) {
            self.curr_pos = (self.curr_pos + chan + self.skip_size) % 256

        }
        fn dense_hash(&self) -> Vec<u32> {
            let mut t: Vec<u32> = Vec::new();
            let mut list = self.list.clone();

            for _ in 0..16 {
                t.push(list.iter().take(16).fold(0, |acc, &n| {acc ^ n}));
                list = list.into_iter().skip(16).collect();
            }
            t
        }
        fn twist(&mut self, length: &u32) {
            // 0 and 1 change nothing in the list
            if *length != 0 && *length != 1 {
                let twisted: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip(self.curr_pos as usize)
                    .take(*length as usize).collect();
                let mut twisted: Vec<u32> = twisted.into_iter().rev().collect();

                let mut not_taken: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip((self.curr_pos + length) as usize)
                    .take(self.list.len() - *length as usize).collect();

                let mut temp: Vec<u32> = Vec::new();
                
                match (self.curr_pos + length).cmp(&(self.list.len() as u32)) {
                    Greater => {
                        let len = twisted.len();
                        let amm_pushed = self.curr_pos + length - self.list.len() as u32;
                        temp.append(
                            &mut twisted.clone().into_iter().skip(len - amm_pushed as usize).collect()
                        );
                        temp.append(&mut not_taken);
                        temp.append(
                            &mut twisted.into_iter().take(len - amm_pushed as usize).collect()
                        );
                    },
                    Equal => {
                        temp.append(&mut not_taken);
                        temp.append(&mut twisted);
                    },
                    Less => {
                        let len = not_taken.len();
                        temp.append(
                            &mut not_taken.clone().into_iter().skip(len - self.curr_pos as usize).collect()
                        );
                        temp.append(&mut twisted);
                        temp.append(
                            &mut not_taken.into_iter().take(len - self.curr_pos as usize).collect()
                        );
                    }
                } 
                self.list = temp
            }
        }
        fn hash(&mut self, lengths: Vec<u32>) -> Vec<u32> {
            // reverses since it executes by popping them
            let lengths: Vec<u32> = lengths.into_iter().rev().collect();

            for _ in 0..64 {
                let mut temp = lengths.clone();
                while let Some(x) = temp.pop() {
                    self.twist(&x);    
                    self.incr_pos(x);
                    self.skip_size += 1;
                };
            }
            self.dense_hash()
        }
    }
    let mut entries: Vec<Vec<u32>> = Vec::new();

    for n in 0..128 {
        let mut temp: Vec<u32> = format!("{}-{}", d_day14, n).bytes().map(|v| v as u32).collect();
        temp.append(&mut vec![17, 31, 73, 47, 23]);
        entries.push(temp);
    }
    let list: Vec<u32> = (0..256).into_iter().collect();
    let mut hex_to_bin:HashMap<char, Vec<u32>> = HashMap::new();
    hex_to_bin.insert('0', vec![0,0,0,0]);
    hex_to_bin.insert('1', vec![0,0,0,1]);
    hex_to_bin.insert('2', vec![0,0,1,0]);
    hex_to_bin.insert('3', vec![0,0,1,1]);
    hex_to_bin.insert('4', vec![0,1,0,0]);
    hex_to_bin.insert('5', vec![0,1,0,1]);
    hex_to_bin.insert('6', vec![0,1,1,0]);
    hex_to_bin.insert('7', vec![0,1,1,1]);
    hex_to_bin.insert('8', vec![1,0,0,0]);
    hex_to_bin.insert('9', vec![1,0,0,1]);
    hex_to_bin.insert('a', vec![1,0,1,0]);
    hex_to_bin.insert('b', vec![1,0,1,1]);
    hex_to_bin.insert('c', vec![1,1,0,0]);
    hex_to_bin.insert('d', vec![1,1,0,1]);
    hex_to_bin.insert('e', vec![1,1,1,0]);
    hex_to_bin.insert('f', vec![1,1,1,1]);

    let knot = Knot::new(list);
    let mut sum = 0;
    while let Some(entry) = entries.pop() {
        let mut kknot = knot.clone();
        let mut as_hex = Vec::new();
        for x in kknot.hash(entry) {
            as_hex.push(format!("{:x}",x));
        }
        for v in as_hex {
            for c in v.chars() {
                if let Some(e) = hex_to_bin.get(&c) {
                    sum += e.iter().sum::<u32>();
                }
            }
        }  
    }
    println!("Day 14, Part 1: {}", sum);
}

use std::sync::Mutex;
pub fn day14_2() {
    #[derive(Debug, Clone)]
    struct Knot {
        list: Vec<u32>,
        curr_pos: u32,
        skip_size: u32
    }
    impl Knot {
        fn new(list: Vec<u32>) -> Knot {
            Knot {
                list,
                curr_pos: 0,
                skip_size: 0,
            }
        }
        fn incr_pos(&mut self, chan: u32) {
            self.curr_pos = (self.curr_pos + chan + self.skip_size) % 256

        }
        fn dense_hash(&self) -> Vec<u32> {
            let mut t: Vec<u32> = Vec::new();
            let mut list = self.list.clone();

            for _ in 0..16 {
                t.push(list.iter().take(16).fold(0, |acc, &n| {acc ^ n}));
                list = list.into_iter().skip(16).collect();
            }
            t
        }
        fn twist(&mut self, length: &u32) {
            // 0 and 1 change nothing in the list
            if *length != 0 && *length != 1 {
                let twisted: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip(self.curr_pos as usize)
                    .take(*length as usize).collect();
                let mut twisted: Vec<u32> = twisted.into_iter().rev().collect();

                let mut not_taken: Vec<u32> = self.list.clone().into_iter().cycle()
                    .skip((self.curr_pos + length) as usize)
                    .take(self.list.len() - *length as usize).collect();

                let mut temp: Vec<u32> = Vec::new();
                
                match (self.curr_pos + length).cmp(&(self.list.len() as u32)) {
                    Greater => {
                        let len = twisted.len();
                        let amm_pushed = self.curr_pos + length - self.list.len() as u32;
                        temp.append(
                            &mut twisted.clone().into_iter().skip(len - amm_pushed as usize).collect()
                        );
                        temp.append(&mut not_taken);
                        temp.append(
                            &mut twisted.into_iter().take(len - amm_pushed as usize).collect()
                        );
                    },
                    Equal => {
                        temp.append(&mut not_taken);
                        temp.append(&mut twisted);
                    },
                    Less => {
                        let len = not_taken.len();
                        temp.append(
                            &mut not_taken.clone().into_iter().skip(len - self.curr_pos as usize).collect()
                        );
                        temp.append(&mut twisted);
                        temp.append(
                            &mut not_taken.into_iter().take(len - self.curr_pos as usize).collect()
                        );
                    }
                } 
                self.list = temp
            }
        }
        fn hash(&mut self, lengths: Vec<u32>) -> Vec<u32> {
            // reverses since it executes by popping them
            let lengths: Vec<u32> = lengths.into_iter().rev().collect();

            for _ in 0..64 {
                let mut temp = lengths.clone();
                while let Some(x) = temp.pop() {
                    self.twist(&x);    
                    self.incr_pos(x);
                    self.skip_size += 1;
                };
            }
            self.dense_hash()
        }
    }
    let mut entries: Vec<Vec<u32>> = Vec::new();

    for n in 0..128 {
        let mut temp: Vec<u32> = format!("{}-{}", d_day14, n).bytes().map(|v| v as u32).collect();
        temp.append(&mut vec![17, 31, 73, 47, 23]);
        entries.push(temp);
    }
    let list: Vec<u32> = (0..256).into_iter().collect();
    let mut hex_to_bin:HashMap<char, Vec<u32>> = HashMap::new();
    hex_to_bin.insert('0', vec![0,0,0,0]);
    hex_to_bin.insert('1', vec![0,0,0,1]);
    hex_to_bin.insert('2', vec![0,0,1,0]);
    hex_to_bin.insert('3', vec![0,0,1,1]);
    hex_to_bin.insert('4', vec![0,1,0,0]);
    hex_to_bin.insert('5', vec![0,1,0,1]);
    hex_to_bin.insert('6', vec![0,1,1,0]);
    hex_to_bin.insert('7', vec![0,1,1,1]);
    hex_to_bin.insert('8', vec![1,0,0,0]);
    hex_to_bin.insert('9', vec![1,0,0,1]);
    hex_to_bin.insert('a', vec![1,0,1,0]);
    hex_to_bin.insert('b', vec![1,0,1,1]);
    hex_to_bin.insert('c', vec![1,1,0,0]);
    hex_to_bin.insert('d', vec![1,1,0,1]);
    hex_to_bin.insert('e', vec![1,1,1,0]);
    hex_to_bin.insert('f', vec![1,1,1,1]);

    type Point = (i32, i32);
    let mut plane: HashMap<Point, u32> = HashMap::new();

    let knot = Knot::new(list);
    let mut row = 0;
    while let Some(entry) = entries.pop() {
        let mut column = 0;
        let mut kknot = knot.clone();
        let mut as_hex = Vec::new();
        for x in kknot.hash(entry) {
            as_hex.push(format!("{:x}",x));
        }
        for v in as_hex {
            // if it had leading zeros
            // (it took me a whole day to figure it out)
            if v.len() != 2 {
                for _ in 0..4 {
                    plane.insert((row, column), 0); 
                    column += 1; 
                }
            }
            for c in v.chars() {
                if let Some(e) = hex_to_bin.get(&c) {
                    for booll in e {
                        plane.insert((row, column), *booll); 
                        column += 1;
                    }
                } else { unreachable!() }
            }
        } 
        row += 1;
    }

    let mut highest_align = 0;
    fn check_align(x: i32, y: i32, highest_align: &mut i32, plane: &mut HashMap<Point, u32>) {
        let mut targets = Vec::new();

        for x0 in -1..2i32 {
            for y0 in -1..2i32 {
                // no diagonals
                if (x0 + y0).abs() != 1 || x0 + x < 0 || y0 + y < 0 {
                    continue 
                }
                if let Some(point) = plane.get(&(x + x0, y + y0)) {
                    if *point == 1 {
                        if !excluded.lock().unwrap().contains(&[x+x0,y+y0]) {
                            targets.push((x+x0,y+y0));
                        }
                    }
                }
                excluded.lock().unwrap().push([x+x0,y+y0]);
            }
        }

        for point in targets {
            check_align(point.0, point.1, highest_align, plane);
        }
    };

    let mut groups = 0;
    lazy_static! {
        static ref excluded: Mutex<Vec<[i32; 2]>> = {
            Mutex::new(Vec::new())
        };
    }
    for x in 0..128 {
        'fory: for y in 0..128 {
            if !excluded.lock().unwrap().contains(&[x,y]) {
                excluded.lock().unwrap().push([x,y]);
                if let Some(point) = plane.get(&(x,y)) {
                    if *point == 0 {
                        continue 'fory //clarity
                    }
                }
                check_align(x, y, &mut highest_align, &mut plane);
                groups += 1;
            }
        }
    } 
    println!("Day 14, Part 2: {}", groups);
}

pub fn day15_1() {
    let a_fact = 16807;
    let b_fact = 48271;
    let guard = 2147483647u64;
    let mut judge_sum = 0;
    let mut state_a = d_day15_A;
    let mut state_b = d_day15_B;
    for _ in 0..40000000 {
        state_a = (state_a * a_fact) % guard;
        state_b = (state_b * b_fact) % guard;
        
        if state_a as u16 == state_b as u16 {
            judge_sum += 1;
        }
    }
    println!("Day 15, Part 1: {}", judge_sum);
}

pub fn day15_2() {
    let a_fact = 16807;
    let b_fact = 48271;
    let guard = 2147483647u64;
    let mut judge_sum = 0;
    let mut state_a = d_day15_A;
    let mut state_b = d_day15_B;
    for _ in 0..5_000_000 {
        state_a = state_a * a_fact % guard;
        while state_a % 4 != 0 {
            state_a = state_a * a_fact % guard;
        }
        state_b = state_b * b_fact % guard;
        while state_b % 8 != 0 {
            state_b = state_b * b_fact % guard;
        }
        
        if state_a as u16 == state_b as u16 {
            judge_sum += 1;
        }
    }
    println!("Day 15, Part 2: {}", judge_sum);
}

pub fn day16_1() {
    let mut programs = Vec::new();
    for c in 97..113u8 {
        programs.push(c as char);
    }

    fn spin(n: u8, programs: &mut Vec<char>) {
        for _ in 0..n {
            let t = programs.pop().unwrap();
            programs.insert(0, t);
        }
    }
    
    fn partner(prog1: char, prog2: char, programs: &mut Vec<char>) {
        let x = programs.iter().position(|&c| c == prog1).unwrap();
        let y = programs.iter().position(|&c| c == prog2).unwrap();
        programs.swap(x, y);
    } 

    for instr in d_day16.split(',') {
        let instr: Vec<char> = instr.chars().collect();
        match instr[0] {
            's' => spin(instr.iter().skip(1).collect::<String>()
                .parse().unwrap(), &mut programs),

            'x' => programs.swap(instr.iter()
                                .skip(1).take_while(|&c| c.is_digit(10))
                                .collect::<String>().parse().unwrap(),
                instr.iter().skip(1).skip_while(|&c| c.is_digit(10))
                    .skip(1).take_while(|&c| c.is_digit(10))
                    .collect::<String>().parse().unwrap()),

            'p' => partner(instr[1], instr[3], &mut programs),
            _ => unreachable!(),
        }
    }
    println!("Day 16, Part 1: {:?}", programs.iter().collect::<String>());
}

pub fn day16_2() {
    let mut programs = Vec::new();
    for c in 97..113u8 {
        programs.push(c as char);
    }

    fn spin(n: u8, programs: &mut Vec<char>) {
        for _ in 0..n {
            let t = programs.pop().unwrap();
            programs.insert(0, t);
        }
    }
    
    fn partner(prog1: char, prog2: char, programs: &mut Vec<char>) {
        let x = programs.iter().position(|&c| c == prog1).unwrap();
        let y = programs.iter().position(|&c| c == prog2).unwrap();
        programs.swap(x, y);
    } 
    let mut cache: HashMap<Vec<char>, Vec<char>> = HashMap::new();

    for n in 0..1_000_000_000 {
        if n % 5_000_000 == 0 {
            println!("Iteration {}/1_000_000_000", n);
        }
        if cache.keys().any(|ref k| **k == programs) {
            programs = cache.get(&programs).unwrap().clone();
            continue
        }
        let state = programs.clone();
        for instr in d_day16.split(',') {
            let instr: Vec<char> = instr.chars().collect();
            match instr[0] {
                's' => spin(instr.iter().skip(1).collect::<String>()
                    .parse().unwrap(), &mut programs),

                'x' => programs.swap(instr.iter()
                                    .skip(1).take_while(|&c| c.is_digit(10))
                                    .collect::<String>().parse().unwrap(),
                    instr.iter().skip(1).skip_while(|&c| c.is_digit(10))
                        .skip(1).take_while(|&c| c.is_digit(10))
                        .collect::<String>().parse().unwrap()),

                'p' => partner(instr[1], instr[3], &mut programs),
                _ => unreachable!(),
            }
        }
        cache.insert(state, programs.clone());
    }
    println!("Day 16, Part 2: {:?}", programs.iter().collect::<String>());
}

pub fn day17_1() {
    #[derive(Debug, Clone)]
    struct Spinlock {
        state: Vec<u32>,
        pos: usize,
        max: u32,
        hop_by: usize
    }
    impl Spinlock {
        fn new(hop_by: usize) -> Spinlock {
            Spinlock {
                state: vec![0],
                pos: 0,
                max: 0,
                hop_by
            }
        }

        fn spin(&mut self) {
            self.pos = (self.pos + self.hop_by) % self.state.len() + 1;
            self.max += 1;
            self.state.insert(self.pos, self.max);
        }
    }

    let mut spin_l = Spinlock::new(d_day17);
    for _ in 0..2017 {
        spin_l.spin();
    }

    println!("Day 17, Part 1: {}", spin_l.state[spin_l.pos + 1]);
}

pub fn day17_2() {
    #[derive(Debug, Clone)]
    struct Spinlock {
        state: Vec<u32>,
        pos: usize,
        max: u32,
        hop_by: usize
    }
    impl Spinlock {
        fn new(hop_by: usize) -> Spinlock {
            Spinlock {
                state: vec![0],
                pos: 0,
                max: 0,
                hop_by
            }
        }

        fn spin(&mut self, len: usize) {
            self.pos = (self.pos + self.hop_by) % len + 1;
            self.max += 1;
            if self.pos == 1 {
                self.state.insert(1, self.max);
            }
        }
    }

    let mut spin_l = Spinlock::new(d_day17);
    for n in 1..50_000_001 {
        spin_l.spin(n);
    }

    // 0 is always at 0'th
    println!("Day 17, Part 2: {}", spin_l.state[1]);
}