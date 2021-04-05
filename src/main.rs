use loom::{sync::mpsc, thread};

fn main() {
    loom::model(|| {
        deadlock_example();
    })
}

fn deadlock_example() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        while let Ok(i) = receiver.recv() {
            println!("Got {}", i);
        }
    });

    sender.send(0_i32).expect("thread died");

    drop(sender);

    handle.join().expect("thread panicked");
}
