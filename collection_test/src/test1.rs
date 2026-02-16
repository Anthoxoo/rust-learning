/* 1 - Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map
will be helpful here) of the list. */

use std::collections::HashMap;

fn swap_number_vector_i32(list: &mut Vec<i32>, i: usize, j: usize) {
    // Gotta be careful with indexes, must be usize
    let tmp: i32 = list[i];
    list[i] = list[j];
    list[j] = tmp;
}
fn most_occurence_value(list: &Vec<i32>) -> i32 {
    /*the lenght of list must be > 0 */
    let mut map = HashMap::new();
    for i in 0..list.len() {
        let count = map.entry(list[i]).or_insert(0);
        *count += 1;
    }

    let mut max_value = map
        .get(&(list[0]))
        .expect("No value has been found at the 0 index of the list vector");
    let mut max_key = &list[0];
    for (key, value) in &map {
        if &value > &max_value {
            max_value = &value;
            max_key = key;
        }
    }
    return *max_key;
}

fn selection_sort_i32(list: &mut Vec<i32>) {
    for i in 0..list.len() {
        let mut min: usize = i;
        for j in i + 1..list.len() {
            if list[min] > list[j] {
                min = j;
            }
        }
        swap_number_vector_i32(list, min, i);
    }
}

pub fn answer(mut list: Vec<i32>) -> (i32, i32) {
    selection_sort_i32(&mut list);
    let list_lenght: usize = list.len();
    let median: i32 = list[list_lenght / 2];

    let mode: i32 = most_occurence_value(&list);
    return (median, mode);
}
