// compute_node.rs
use zmq::Context;
use std::thread;
use std::time::Duration;

fn main() {
    let context = Context::new();

    // Connect to the host's task sender to receive tasks
    let task_receiver = context.socket(zmq::PULL).expect("Failed to create socket");
    task_receiver.connect("tcp://rpi3b-0.local:5555").expect("Failed to connect to task receiver");

    // Connect to the host's result receiver to send results
    let result_sender = context.socket(zmq::PUSH).expect("Failed to create socket");
    result_sender.connect("tcp://rpi3b-0.local:5556").expect("Failed to connect to result sender");

    println!("Compute node is ready for tasks.");

    loop {
        // Receive task from the host
        let task = task_receiver.recv_string(0).expect("Failed to receive task").unwrap();
        println!("Received: {}", task);

        // Simulate processing
        thread::sleep(Duration::from_secs(2));
        let result = format!("{} completed", task);

        // Send result back to the host
        result_sender.send(&result, 0).expect("Failed to send result");
    }
}
