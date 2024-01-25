use std::sync::mpsc;
use std::thread;

enum Message {
    Greet(String),
    Quit,
}

fn actor(rx: mpsc::Receiver<Message>) {
    for msg in rx {
        match msg {
            Message::Greet(name) => println!("Hello, {name}"),
            Message::Quit => break,
        }
    }
}

fn main() {
    // create a channel for message passing
    let (tx, rx) = mpsc::channel();

    // spawn a thread for the actor
    let actor_handle = thread::spawn(move || {
        actor(rx);
    });

    tx.send(Message::Greet("Alice".into())).unwrap();
    tx.send(Message::Greet("Bob".into())).unwrap();
    tx.send(Message::Quit).unwrap();

    actor_handle.join().unwrap();
}
