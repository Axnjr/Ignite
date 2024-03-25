use std::env;

use crate::structs;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region, SdkConfig};
use aws_sdk_sqs::client::Client;
use socketioxide::extract::SocketRef;
use structs::{ PollingCounters, Message };

pub async fn broadcast_queue_messages(socket: SocketRef) {

    let sqs_client = Client::new(&aws_sdk_config().await);

    let mut counters = PollingCounters {
        null_counter: 0,
        full_counter: 5
    };

    let polling_factor : i32 = 3;
    let mut sleep_duration: u64 = 5;
    const SLEEP_DURATION_AT_START: u64 = 5;

    loop {

        let mes = sqs_client
            .receive_message()
            .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
            .set_max_number_of_messages(Some(5))
            .send()
            .await
            .expect("Error recieving messages from Message Queue ☠️😱")
        ;

        // no messages are being recived increase the null_counter
        if mes.messages().len() == 0 {
            counters.incre_null_counter();
        }
        // if all messages i.e messages.len() == max_number_of_messages are recived then increase the full_counter
        else if mes.messages().len() == 5 {
            counters.incre_full_counter();
            sleep_duration = SLEEP_DURATION_AT_START;
        }
        // reset all counter if messages are recived in between 0 - 5
        else if mes.messages().len() > 0 && mes.messages().len() < 5 {
            counters.reset();
        }

         // if null_counter is greater than 5 then increase the sleep_duration by 5 for making less requests to SQS
         if counters.null_counter > polling_factor {
            sleep_duration = sleep_duration * polling_factor as u64;

            println!("CHANGED AT LINE 49 sleep_duration: {}", sleep_duration);

            counters.reset_null_counter();
            tokio::time::sleep(std::time::Duration::from_secs(sleep_duration)).await;
            continue;
        }

        if counters.full_counter > polling_factor {
            sleep_duration = sleep_duration / polling_factor as u64;

            println!("Long Polling Duration increased: {}", sleep_duration);

            counters.reset_full_counter();
        }

        for message in mes.messages() {

            let mes = message.body().unwrap();
            let mes: Message = serde_json::from_str(mes).unwrap();

            println!("------------------------");
            println!("JSON MESSAGE: {:#?}", mes);
            println!("------------------------");

            let _ = socket
                .within(mes.key + "_" + &mes.group_id)
                .emit(mes.event_name, mes.message)
            ;

            let _ = sqs_client
                .delete_message()
                .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
                .receipt_handle(message.receipt_handle().unwrap())
                .send()
                .await
                .expect("Error deleting message from Message Queue ☠️😱")
            ;

            println!("Message Deleted from Message Queue ✅👍🏻");
        }

        if sleep_duration > 3599 {
            sleep_duration = SLEEP_DURATION_AT_START;
        }

        tokio::time::sleep(std::time::Duration::from_secs(sleep_duration)).await;
    }

}

async fn aws_sdk_config() -> SdkConfig {

    let provider = RegionProviderChain::first_try(env::var("REGION")
        .ok()
        .map(Region::new))
        .or_default_provider()
        .or_else(Region::new("ap-south-1"))
    ;

    aws_config::from_env().region(provider).load().await

    // println!("SDK CONFIG: {:#?}", t.region());
}