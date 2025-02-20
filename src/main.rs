use std::io::{stdin, Write};
use std::path::Path;
use std::{fs, fs::File};
use std::time;
use std::error::Error;

use rand::{rngs::ThreadRng, Rng};
use rayon;

fn pivot(low: usize, high: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(low..=high)
}

fn partition(vec: &mut [i32]) -> usize {
    let mut left = 0usize;
    let mut right = vec.len()-1;

    let p = vec[pivot(left, right)];

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
    }
}

fn quick_sort(vec: &mut [i32]) -> () {
    if vec.len() > 1 {
        let p = partition(vec);
        let (low, high) = vec.split_at_mut(p);
        rayon::join(|| quick_sort(low), || quick_sort(high));
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

fn validate(vec: &Vec<i32>) -> Result<(), usize> {
    for index in 1..vec.len() {
        if vec[index] < vec[index - 1] {
            return Err(index);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let amount = 10000usize;
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

    let mut now = time::Instant::now();
    quick_sort(&mut vec);
    let elapsed_custom = now.elapsed();
    match validate(&vec) {
        Ok(()) => {
            println!("На сортировку {} элементов кастомной сортировкой потрачено {} секунд", amount, elapsed_custom.as_secs_f32());
        }
        Err(i) => {
            println!("Кастомная сортировка не справилась с задачей! Значение под индексом {} ({}), меньше, чем предыдущее {}", i, vec[i], vec[i-1]);
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
