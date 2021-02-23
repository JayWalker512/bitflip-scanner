#[macro_use]
extern crate clap;

use std::vec::*;
use std::convert::TryInto;
use std::{thread, time};
use clap::App;

fn main() {
    let matches = App::new("Bitflip Scanner")
        .args_from_usage("[scale] 'Number of Gibibytes to allocate for bitflip detection. Defaults to 1.'")
        .get_matches();

    let scale = value_t!(matches, "scale", u32).unwrap_or(1) as usize;
    let num_bytes: usize = scale*1024*1024*1024;
    let mut memory: Vec<u8> = Vec::with_capacity(num_bytes);

    // Populate the memory in a predictable way
    println!("Populating memory with {} bytes...", num_bytes);
    for i in 0..num_bytes {
        memory.push(get_value_at_index(i))
    }

    println!("Scanning for bitflips...");
    loop {
        thread::sleep(time::Duration::from_millis(60000));
        let count = check_memory(&mut memory);
        if count > 0 {
            println!("Found {} bit flips.", count); // TODO include a timestamp with this output
        }
    }
}

fn get_value_at_index(index: usize) -> u8 {
    (index % 256).try_into().unwrap()
}

fn check_memory(mem: &mut Vec<u8>) -> u64 {
    let mut count: u64 = 0;
    for i in 0..mem.capacity() {
        if mem[i] != get_value_at_index(i) {
            count = count + 1;
            mem[i] = get_value_at_index(i);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_at_index() {
        assert_eq!(0, get_value_at_index(0));
        assert_eq!(0, get_value_at_index(256));
        assert_eq!(1, get_value_at_index(257));
    }

    #[test]
    fn test_check_memory_finds_errors() {
        let mut mem: Vec<u8> = vec![0,1,4];
        assert_eq!(1, check_memory(&mut mem));

        // The error is fixed in the previous pass
        assert_eq!(0, check_memory(&mut mem));
    }
}
