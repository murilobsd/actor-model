// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
// 
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
// 
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
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
