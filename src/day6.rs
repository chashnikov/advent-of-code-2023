use std::fs;
use std::path::Path;

pub fn day6() {
    let content = fs::read_to_string(Path::new("6-full.txt")).expect("input must exist");
    let mut times : Vec<u64> = Vec::new();
    let mut distances : Vec<u64> = Vec::new();
    content.lines().for_each(|line| {
        if line.starts_with("Time:") {
            times = Vec::from([line[5..].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap()]);
        }
        else if line.starts_with("Distance:") {
            distances = Vec::from([line[9..].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap()])
        }
    });
    let answer : u64 = times.iter().zip(distances.iter()).map(|(time, distance)| {
        let d2 = time * time - distance * 4;
        if d2 <= 0 {
            0
        }
        else {
            let s2 : f64 = (d2 as f64).sqrt();
            let s2_int = s2.round() as u64;
            if s2_int * s2_int == d2 && (time + s2_int) % 2 == 0 {
                s2_int - 1
            }
            else {
                let f_time = *time as f64;
                (((f_time + s2)/ 2_f64).floor() as u64) - (((f_time - s2)/ 2_f64).ceil() as u64) + 1
            }
        }
    }).product();
    /*
      time, distance:
      t: [0, time] -> t*(time-t) > distance
      t^2 - t*time + distance < 0
      (t-time/2)^2 + distance - time^2/4 < 0
      abs(t-time/2) < sqrt(time^2/4 - distance)
      t in (time/2-sqrt, time/2+sqrt)
     */
    println!("{answer}")
}