#![feature(trace_macros)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(overflowing_literals)]

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