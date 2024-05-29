use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx0, rx) = mpsc::channel();

    let tx1 = tx0.clone();
    transmit_in_one_thread(tx1, String::from("tx-1-"), 800);
    transmit_in_one_thread(tx0, String::from("tx-0-"), 200);

    for received in rx {
        println!("Got: {}", received);
    }
}

fn transmit_in_one_thread(tx: mpsc::Sender<String>, msg_prefix: String, interval_millis: u64) {
    thread::spawn(move || {
        let vals = vec![
            format!("{}-hi", msg_prefix),
            format!("{}-from", msg_prefix),
            format!("{}-a", msg_prefix),
            format!("{}-thread", msg_prefix),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(interval_millis));
        }
    });
}
