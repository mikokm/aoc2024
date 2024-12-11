use std::fs::read_to_string;

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    let input = r"2333133121414131402";
    // let input = r"12345";
    let input = _file;
    // println!("{}", input);
    let digits: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();
    let (mut sizes, mut spaces) = (Vec::new(), Vec::new());
    let mut unpacked: Vec<i32> = Vec::new();
    let mut file_id = 0;
    for (i, e) in digits.iter().enumerate() {
        if i % 2 == 0 {
            sizes.push(*e);
            for _ in 0..(*e as usize) {
                unpacked.push(file_id);
            }

            file_id += 1;
        } else {
            for _ in 0..(*e as usize) {
                unpacked.push(-1);
            }
            spaces.push(e);
        }
    }

    println!("sizes: {:?}, spaces: {:?}", sizes, spaces);

    let mut packed = unpacked.clone();
    /* part 1
    for i in 0..packed.len() - 1 {
        // println!("packed[{:?}] = {:?}", i, packed[i]);
        if packed[i] == -1 {
            if let Some(idx) = packed.iter().rposition(|c| *c != -1) {
                if i >= idx {
                    break;
                }
                // println!("Next file {:?} at {:?}", packed[idx], idx);
                packed.swap(i, idx);
            }
        }
        // println!("packed: {:?}", packed);
    }
    */

    let next_free = |v: &Vec<i32>, start: usize| -> usize {
        let mut end = start;
        // println!("Checking for v[{}] ({})", start, v[start]);
        for i in start..=v.len() - 1 {
            end = i;

            if v[i] != -1 {
                break;
            }
        }

        // println!("end {} - start {} = {}", end, start, end - start);
        return end - start;
    };

    let write_next = |v: &mut Vec<i32>, start: usize, size: i32, id: i32| {
        for i in start..start + size as usize {
            // println!("v[{:?}] = {}", i, id);
            v[i] = id;
        }
    };

    let erase = |v: &mut Vec<i32>, id: usize| {
        for i in 0..=v.len() - 1 {
            if v[i] == id as i32 {
                // println!("v[{:?}] = {}", i, -1);
                v[i] = -1;
            }
        }
    };

    let files: Vec<(usize, i32)> = sizes.iter().map(|x| *x as i32).enumerate().rev().collect();
    println!("{:?}", files);

    // Yeah this will take ages.
    for file in files {
        // println!("Moving file {:?}", file);
        let file_start = packed.iter().position(|c| *c == file.0 as i32).unwrap();
        for i in 0..=unpacked.len() - 1 {
            let free = next_free(&packed, i) as i32;
            // println!("Free space {:?}", free);
            if free >= file.1 && file_start > i {
                erase(&mut packed, file.0);
                write_next(&mut packed, i, file.1, file.0 as i32);
                break;
            }
        }

        // println!("{:?}", packed);
    }

    let mut checksum = 0;
    for (i, e) in packed.iter().enumerate() {
        if *e == -1 {
            // println!("exit at {}", i);
            // break;
            continue;
        }
        checksum += (*e as i64) * i as i64;
    }

    // println!("sizes: {:?}, free_space: {:?}", sizes, spaces);
    println!("u: {:?}", unpacked);
    println!("p: {:?}", packed);
    println!("checksum: {:?}", checksum);
}
