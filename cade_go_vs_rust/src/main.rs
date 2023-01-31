// created 01/30/23
// made because of https://www.youtube.com/watch?v=Mjk3FLlMhIw



#![allow(dead_code)]

#![feature(test)]

extern crate test;
use test::*;



const DATA_LENGTH: usize = 1024;
const WINDOW_LENGTH: usize = 30;
const WARMUP_COUNT: usize = 0;

/*

Test results: (ran many times, only showing the results with the lowest variation)

data_length: 1024, window_length: 5, warmup_count: 0
1: 130 ns  +/- 5
2:  17 ns  +/- 0
3: 166 ns  +/- 2

data_length: 1024, window_length: 10, warmup_count: 0
1: 201 ns  +/- 9
2: 100 ns  +/- 15
3: 169 ns  +/- 3

data_length: 1024, window_length: 20, warmup_count: 0  (about the same for data_length: 1,000,000)
1: 443 ns  +/- 13
2: 330 ns  +/- 8
3: 184 ns  +/- 2

data_length: 1024, window_length: 30, warmup_count: 0
1:   792 ns  +/- 19
2: 2,999 ns  +/- 102
3:   276 ns  +/- 4

 */



fn main() {

    for _ in 0..1000 {
        let mut data = vec!();
        for _ in 0..1024 {data.push(rand::random::<u8>());}
        assert!(find_uniques_2(&data, 10) == find_uniques_3(&data, 10));
    }

}





// basically Primeagen's version (I think Primeagen?) (modified to take the length as an arg and to not add the length to the output)
fn find_uniques_1 (data: &[u8], length: usize) -> usize {
    data.windows(length)
        .position(|window| {
            let mut array = vec!();
            array.reserve(length);
            let mut end_i = 0;
            for &byte in window {
                for i in 0..end_i {
                    if array[i] == byte {
                        return false;
                    }
                }
                array.push(byte);
                end_i += 1;
            }
            true
        })
        .unwrap()
}



fn find_uniques_2 (data: &[u8], length: usize) -> usize {
    data.windows(length)
        .position(|window| {
            for (i1, &byte) in window.iter().enumerate() {
                for i2 in 0..i1 {
                    if window[i2] == byte {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()
}



fn find_uniques_3 (data: &[u8], length: usize) -> usize {

    let mut byte_counts = [0; 256];
    let mut counts_over_1 = 0;

    // fill initial data
    for &byte in data.iter().take(length - 1) {
        let new_count = byte_counts[byte as usize] + 1;
        byte_counts[byte as usize] = new_count;
        if new_count == 2 {counts_over_1 += 1;}
    }

    // main calculations
    let mut start = 0;
    let mut end = length - 1;
    loop {
        
        // add new byte
        let end_byte = data[end];
        let new_count = byte_counts[end_byte as usize] + 1;
        byte_counts[end_byte as usize] = new_count;
        if new_count == 2 {counts_over_1 += 1;}

        // check if finished
        if counts_over_1 == 0 {return start;}

        // remove old byte
        let start_byte = data[start];
        let new_count = byte_counts[start_byte as usize] - 1;
        byte_counts[start_byte as usize] = new_count;
        if new_count == 1 {counts_over_1 -= 1;}

        start += 1;
        end += 1;

        if end == data.len() {break;}

    }

    panic!("no position found");
}



#[bench]
fn test_find_uniques_1 (bencher: &mut Bencher) {
    test_find_uniques(bencher, find_uniques_1);
}

#[bench]
fn test_find_uniques_2 (bencher: &mut Bencher) {
    test_find_uniques(bencher, find_uniques_2);
}

#[bench]
fn test_find_uniques_3 (bencher: &mut Bencher) {
    test_find_uniques(bencher, find_uniques_3);
}



fn test_find_uniques (bencher: &mut Bencher, function: fn(&[u8], usize) -> usize) {
    let mut data = [0; DATA_LENGTH];
    for i in 0..DATA_LENGTH {data[i] = rand::random::<u8>();}
    for _ in 0..WARMUP_COUNT {function(&data, WINDOW_LENGTH);}
    bencher.iter(|| test::black_box(function(&data, test::black_box(WINDOW_LENGTH))));
}
