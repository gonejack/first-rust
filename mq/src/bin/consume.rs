use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and commited as consumed to ensure
/// only once delivery.
fn main() {
    env_logger::init();

    let broker = vec![
        "192.168.11.10:9093".to_string(),
        "192.168.11.10:9094".to_string(),
        "192.168.11.10:9095".to_string(),
    ];
    let topic = "test_topic".to_string();
    let group = "test_group".to_string();

    if let Err(e) = consume_messages(&group, &topic, &broker) {
        println!("Failed consuming messages: {}", e);
    }

    println!("abc {}", group)
}

fn consume_messages(group: &String, topic: &String, brokers: &Vec<String>) -> Result<(), KafkaError> {
    let mut consumer = Consumer::from_hosts(brokers.to_owned())
        .with_topic(topic.to_owned())
        .with_group(group.to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = consumer.poll()?;

        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}#{}#{}: {:?}", ms.topic(), ms.partition(), m.offset, String::from_utf8_lossy(m.value));
            }
            let _ = consumer.consume_messageset(ms);
        }

        consumer.commit_consumed()?;
    }
}
