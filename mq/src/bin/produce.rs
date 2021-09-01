use std::thread::sleep;
use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main() {
    env_logger::init();

    let broker = vec![
        "192.168.11.10:9093".to_string(),
        "192.168.11.10:9094".to_string(),
        "192.168.11.10:9095".to_string(),
    ];
    let topic = "test_topic";
    if let Err(e) = produce_message(topic, broker) {
        println!("Failed producing messages: {}", e);
    }
}

fn produce_message(topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    for n in 1..100 {
        let msg = format!("消息: {}", n).to_string();

        producer.send(&Record::from_value(topic, msg.clone()))?;

        println!("sent {}", msg.clone());

        sleep(Duration::from_secs(1))
    }

    println!("Sent");

    Ok(())
}
