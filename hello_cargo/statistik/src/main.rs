use std::collections::HashMap;

fn main() {
    //Vektor
    let mut vector: Vec<i32> = vec![1,2,3,23,24,5443,24,34];

    //Methoden aufrufen
    println!("Mittelwert: {}", mittelwert (&vector));
    println!("Median: {}", median(&mut vector));
    println!("Modus: {:?}", modus (&vector));
}

fn mittelwert(numbers: &Vec<i32>) -> f32 {
    //berechnet Mittelwert und gibt ihn zurück
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32

}

fn median(numbers: &mut Vec<i32>) -> i32 {

    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mittelwert (&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }

}

fn modus (numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in numbers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()

}
