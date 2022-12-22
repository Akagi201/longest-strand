use std::fs;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    // Get the list of file names that match the pattern "sample.*" in the "samples" directory
    let dir = "samples";
    let files: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().starts_with("sample."))
        .map(|entry| entry.path())
        .map(|path| path.to_str().unwrap().to_string())
        .collect();
    println!("Dealing with samples: {files:?}");

    // Read the contents of each file into memory
    let mut contents = Vec::new();
    for file in &files {
        let file_contents = fs::read(file).expect("Error reading file");
        contents.push(file_contents);
    }

    let start = Instant::now();
    // Initialize variables to track the longest strand
    let longest_strand = Arc::new(Mutex::new(0_usize));
    let file_names = Arc::new(Mutex::new(Vec::new()));
    let offsets = Arc::new(Mutex::new(Vec::new()));

    // Compare each file to every other file in parallel
    files.par_iter().enumerate().for_each(|(i, _)| {
        files[i + 1..]
            .par_iter()
            .enumerate()
            .for_each(|(j, file_j)| {
                let file_i = &files[i];
                let lcs = lcs(&contents[i], &contents[j]);

                // Update the longest strand if a longer one is found
                if lcs.len() > *longest_strand.lock().unwrap() {
                    *longest_strand.lock().unwrap() = lcs.len();
                    file_names.lock().unwrap().clear();
                    file_names
                        .lock()
                        .unwrap()
                        .extend_from_slice(&[file_i, file_j]);
                    offsets.lock().unwrap().clear();
                    offsets.lock().unwrap().extend_from_slice(&[
                        contents[i].iter().position(|&b| b == lcs[0]).unwrap(),
                        contents[j].iter().position(|&b| b == lcs[0]).unwrap(),
                    ]);
                }
            });
    });
    let duration = start.elapsed();

    // Output the results
    println!("Longest strand: {} bytes", *longest_strand.lock().unwrap());
    println!("File names: {:?}", *file_names.lock().unwrap());
    println!("Offsets: {:?}", *offsets.lock().unwrap());
    println!("duration: {duration:?}");
}

fn lcs(x: &[u8], y: &[u8]) -> Vec<u8> {
    // Initialize the LCS table
    let m = x.len();
    let n = y.len();
    let mut l = vec![vec![0; n + 1]; m + 1];

    // Fill in the LCS table
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                l[i][j] = 0;
            } else if x[i - 1] == y[j - 1] {
                l[i][j] = l[i - 1][j - 1] + 1;
            } else {
                l[i][j] = std::cmp::max(l[i - 1][j], l[i][j - 1]);
            }
        }
    }

    // Backtrack to find the LCS
    let mut i = m;
    let mut j = n;
    let mut index = l[i][j];
    let mut lcs = Vec::with_capacity(index as usize);
    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            lcs.push(x[i - 1]);
            i -= 1;
            j -= 1;
            index -= 1;
        } else if l[i - 1][j] > l[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    lcs
}
