use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // Destructure the queue to move its parts into threads separately
    let Queue {
        first_half,
        second_half,
    } = q;

    let tx_clone = tx.clone();
    thread::spawn(move || {
        for val in first_half {
            println!("Sending {val:?}");
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in second_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // Optional: Run to see the output manually
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();

    send_tx(queue, tx);

    for val in rx {
        println!("Received {val:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}