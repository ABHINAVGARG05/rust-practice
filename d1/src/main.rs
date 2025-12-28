fn main() {
    println!("Hello, world!");

    let sentence: String = String::from("Abhinav Garg");
    let first_word = get_first_word(sentence);

    //let mut x:i32 = 10;

    // for i in 0..1000 {
    //     x = x + 100;
    // }

    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans : String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;
}