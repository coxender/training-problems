use std::io::{self, Write};

fn main() {
    // needs to flush, because print does not auto-flush
    print!("Enter name: ");
    io::stdout().flush().expect("Error flushing.");

    // create a (rust always has) lazy iterator over stdin
    let mut lines = io::stdin().lines();
    // lines.next() needs to check Some() or None(). Ok() or Error() in that order.
    // notice enum variants are Pascal
    let Some(Ok(name)) = lines.next() else {
        eprintln!("Expected Input");
        return
    };
    println!("Hello, {name}");
}
