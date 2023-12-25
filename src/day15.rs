use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("15-full.txt");
    let mut boxes : Vec<LinkedHashMap<String, u32>> = (0..256).map(|_| LinkedHashMap::new()).collect_vec();
    content.split(',').for_each(|op| {
        if op.ends_with('-') {
            let label = String::from(&op[..op.len()-1]);
            boxes[hash(&label)].remove(&label);
        }
        else {
            let (label_str, num_str) = op.split_once('=').unwrap();
            let label = String::from(label_str);
            let num = num_str.parse::<u32>().unwrap();
            let map = &mut boxes[hash(&label)];
            let entry = map.get_mut(&label);
            if let Some(data) = entry {
                *data = num;
            }
            else {
                map.insert(label, num);
            }
        }
    });
    let answer : u64 = boxes.iter().enumerate().flat_map(|(box_num, lenses)| {
        lenses.values().enumerate().map(move |(slot, focal_length)| ((box_num+1)*(slot+1)) as u64 * (*focal_length as u64))
    }).sum();
    println!("{answer}");
}

fn hash(s: &String) -> usize {
    s.bytes().fold(0_u32, |acc, x| (acc + x as u32)*17%256) as usize
}