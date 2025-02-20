use std::io::{stdin, Write};
use std::path::Path;
use std::{fs, fs::File};
use std::time;
use std::error::Error;

use rand::{rngs::ThreadRng, Rng};

fn pivot(rng: &mut ThreadRng, low: usize, high: usize) -> usize {
    rng.random_range(low..=high)
}

fn partition(vec: &mut Vec<i32>, rng: &mut ThreadRng, low: usize, high: usize) -> usize {
    let p = vec[pivot(rng, low, high)];

    let mut left = low;
    let mut right = high;

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

        vec.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn quick_sort(vec: &mut Vec<i32>, rng: &mut ThreadRng, low: usize, high: usize) -> () {
    if low < high {
        let p = partition(vec, rng, low, high);
        quick_sort(vec, rng, low, p);
        quick_sort(vec, rng, p + 1, high);
    }
}

fn gen_file(path: &Path, rng: &mut ThreadRng, amount: usize) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    let mut buffer = String::new();
    for _ in 0..amount {
        buffer.push_str(&rng.random::<i32>().to_string());
        buffer.push_str(" ");
    }

    file.write_all(buffer.as_bytes())?;

    Ok(())
}

fn validate(vec: &Vec<i32>) -> bool {
    for index in 1..vec.len() {
        if vec[index] < vec[index - 1] {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let amount = 100usize;
    let input_path = Path::new("data/input.txt");

    println!("Нужно ли генерировать новый вход (Y - Yes, n - no): ");
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    match answer.as_str().trim() {
       "Y" => {
            gen_file(input_path, &mut rng, amount)?;
        }
        "n" => { }
        _ => { }
    }

    let input = fs::read_to_string(input_path)?;
    let mut vec: Vec<i32> = input.split_whitespace()
                                 .map(|s| {s.parse().unwrap()})
                                 .collect();
    let high = vec.len() - 1;

    let mut now = time::Instant::now();
    quick_sort(&mut vec, &mut rng, 0usize, high);
    let elapsed_custom = now.elapsed();
    match validate(&vec) {
        true => {
            println!("На сортировку {} элементов кастомной сортировкой потрачено {} секунд", amount, elapsed_custom.as_secs_f32());
        }
        false => {
            println!("Кастомная сортировка не справилась с задачей!")
        }
    }
    

    vec.clear();
    vec = input.split_whitespace()
               .map(|s| {s.parse().unwrap()})
               .collect();

    now = time::Instant::now();
    vec.sort();
    let elapsed_built_in = now.elapsed();
    println!("На сортировку {} элементов встроенной сортировкой потрачено {} секунд", amount, elapsed_built_in.as_secs_f32());

    Ok(())
}
