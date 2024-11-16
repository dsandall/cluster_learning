// host.rs
use zmq::{Context, Socket};
use std::time::Duration;

fn main() {
    let context = Context::new();

    // Set up the host socket for task distribution.
    let task_sender = context.socket(zmq::PUSH).expect("Failed to create socket");
    task_sender.bind("tcp://*:5555").expect("Failed to bind task sender");

    // Set up the host socket for receiving task completion messages.
    let result_receiver = context.socket(zmq::PULL).expect("Failed to create socket");
    result_receiver.bind("tcp://*:5556").expect("Failed to bind result receiver");

    println!("Host is ready to send tasks.");

    let tasks = vec!["Task 1", "Task 2", "Task 3", "Task 4"];

    // Send tasks to compute nodes
    for task in tasks.iter() {
        println!("Sending: {}", task);
        task_sender.send(task, 0).expect("Failed to send task");
    }

    // Wait for responses from compute nodes
    for _ in 0..tasks.len() {
        let result = result_receiver.recv_string(0).expect("Failed to receive result").unwrap();
        println!("Received result: {}", result);
    }

    println!("All tasks completed.");
}
