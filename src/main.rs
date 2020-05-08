//use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {

    let client = redis::Client::open("redis://10.110.0.9:6379").unwrap();

    let connection_consumer = client.get_async_connection().await?;
    
    tokio::spawn(async move {
        let mut pubsub = connection_consumer.into_pubsub();
        
        pubsub.psubscribe("*").await;

        let mut pubsub_stream = pubsub.on_message();

        loop {
            use futures_util::StreamExt as _;

            let pubsub_msg = pubsub_stream.next().await.unwrap();

            let payload = pubsub_msg.get_payload::<String>();

            match payload {
                Ok(payload) => {
                    println!("payload = {}", payload);
                },
                Err(err) => {
                    println!("payload err {:?}", err);
                }
            };

            println!("channel {}", pubsub_msg.get_channel_name());
        }
    });

    tokio::time::delay_for(Duration::from_secs(2000)).await;

    Ok(())
}

