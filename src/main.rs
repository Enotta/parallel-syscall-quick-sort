use std::io::{stdin, Write};
use std::{fs, fs::File};
use std::time;
use std::error::Error;

use rand::{rngs::ThreadRng, Rng};

fn pivot(rng: &mut ThreadRng, low: usize, high: usize) -> usize {
    return rng.random_range(low..=high);
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

        let temp = vec[left];
        vec[left] = vec[right];
        vec[right] = temp;
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

fn gen_file(path: String, rng: &mut ThreadRng, amount: usize) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    let mut buffer = String::new();
    for _ in 0..amount {
        buffer.push_str(&rng.random::<i32>().to_string());
        buffer.push_str(" ");
    }

    file.write_all(buffer.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let amount = 10000000usize;
    let input_path = "data/input.txt".to_string();

    println!("Нужно ли генерировать новый вход (Y - Yes, n - no): ");
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    match answer.as_str() {
       "Y" => {
            gen_file(input_path.clone(), &mut rng, amount)?;
        }
        "n" => { }
        _ => { }
    }

    let input = fs::read_to_string(input_path.clone())?;
    let mut vec: Vec<i32> = input.split_whitespace()
                                 .map(|s| {s.parse().unwrap()})
                                 .collect();
    let high = vec.len() - 1;

    let mut now = time::Instant::now();
    quick_sort(&mut vec, &mut rng, 0usize, high);
    let elapsed_custom = now.elapsed();
    println!("На сортировку {} элементов кастомной сортировкой потрачено {} секунд", amount, elapsed_custom.as_secs_f32());

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
