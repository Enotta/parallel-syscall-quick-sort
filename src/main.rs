use std::io;
use rand::{rngs::ThreadRng, Rng};

fn pivot(rng: &mut ThreadRng, low: usize, high: usize) -> usize {
    rng.random_range(low..=high)
}

fn partition(vec: &mut Vec<i32>, rng: &mut ThreadRng, low: usize, high: usize) -> usize {
    let p = vec[pivot(rng, low, high)];

    let mut left = low.clone();
    let mut right = high.clone();

    loop {
        while vec[left] < p {
            left += 1;
        }
        while vec[right] > p { 
            right -= 1;
        }
        if left >= right {
             return right;
        }

        let temp = vec[left].clone();
        vec[left] = vec[right];
        vec[right] = temp;
        left += 1;
        left += 1;
    }
}

fn quick_sort(vec: &mut Vec<i32>, rng: &mut ThreadRng, low: usize, high: usize) {
    if low < high {
        let p = partition(vec, rng, low, high);
        quick_sort(vec, rng, low, p);
        quick_sort(vec, rng, p + 1, high);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut vec: Vec<i32> = input.split_whitespace()
                                 .map(|s| {s.parse().unwrap()})
                                 .collect();
    let high = vec.len() - 1;
    let mut rng = rand::rng();
    quick_sort(&mut vec, &mut rng, 0usize, high.clone());
    print!("{:?}", vec);
}
