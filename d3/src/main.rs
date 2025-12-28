use std::io;
use rand::Rng;
use std::collections::HashMap;
//celcius to farenheit
// fn main() {
//     let mut temp = String::new();
//     io::stdin()
//         .read_line(&mut temp)
//         .expect("Failed to read line");

//     let num: i32 = temp
//         .trim()
//         .parse()
//         .expect("Not a valid number");

//     let frh = convert(num);
//     println!("{}", frh);
// }

// fn convert(temp: i32) -> f32 {
//     (temp as f32 * 9.0) / 5.0 + 32.0
// }

//number guessing game
// fn main() {
//     let num : i32 = rand::thread_rng().gen_range(0..100);
//     let mut flag = false;
//     while !flag {
//         let mut inp = String::new();
//         io::stdin()
//             .read_line(&mut inp)
//             .expect("Failed to read line");
        
//         let rannum : i32 = inp
//             .trim()
//             .parse()
//             .expect("Not a valid number");

//         if rannum == num {
//             println!("you guessed it");
//             flag = true;
//         }
//         else if rannum > num {
//             println!("Number is big");
//         }
//         else {
//             println!("Number is small");
//         }
//     }


//     println!("{}",num);
// }

fn main() {
    let str = "HI I AM ABHINAV GARG";

    let mut map : HashMap<char, usize> = HashMap::new();

    for ch in str.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}