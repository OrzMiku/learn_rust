use std::sync::mpsc;
use std::thread;

fn main() {
    let raw_strings = vec![
        "!dlroW olleH",
        ".ukiMzrO m'I",
        "!tpircsepyt yrt oslA",
        ".tsur evol I",
    ];

    // mpsc::channel() creates a channel and returns a sender and a receiver
    let (tx, reverse_strings) = mpsc::channel();

    for raw_string in raw_strings {
        // Using clone() to duplicate the sender allows multiple threads to send messages
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let rev_string: String = raw_string.chars().rev().collect();
            // send() is used to transmit messages through the channel
            tx_clone.send(rev_string).unwrap();
        });
    }

    // All cloned senders (tx_clone) are automatically dropped when the threads end
    // The original sender needs to be explicitly dropped
    // The receiver will only know there are no more messages once the last sender is dropped
    drop(tx);

    // There can only be one receiver, which can be used as an iterator to process all received messages
    for reverse_string in reverse_strings {
        println!("{}", reverse_string);
    }
}
