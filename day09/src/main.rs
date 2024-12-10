use std::fs::read_to_string;

fn main() {
    let _file = read_to_string("input.txt").unwrap();
    // let input = r"2333133121414131402";
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

    let mut packed = unpacked.clone();
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

    let mut checksum = 0;
    for (i, e) in packed.iter().enumerate() {
        if *e == -1 {
            println!("exit at {}", i);
            break;
        }
        checksum += (*e as i64) * i as i64;
    }

    println!("sizes: {:?}, free_space: {:?}", sizes, spaces);
    println!("unpacked: {:?}", unpacked);
    println!("packed: {:?}", packed);
    println!("checksum: {:?}", checksum);
}
