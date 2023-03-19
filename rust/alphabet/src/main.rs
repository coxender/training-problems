use std::io;

fn main() {
    println!("Please input a sentence: ");

    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut abc = [0; 26];

    let Some(Ok(sentence)) = lines.next() else {
        println!("What are you doing???");
        return;
    };

    for letter in sentence.chars() {
        match letter {
            'a'..='z' => {
                let index = letter as usize - 'a' as usize;
                abc[index] += 1;
            }
            'A'..='Z' => {
                let index = letter as usize - 'A' as usize;
                abc[index] += 1;
            }
            _ => {}
        }
    }

    for index in 0..abc.len() {
        let letter = (b'a' + index as u8) as char;
        let count = abc[index];
        if count != 0 {
            println!("{letter}: {count}");
        }
    }
}
