use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::interface::{Op1, Op10, Opcode};
use rand::RngExt;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use crate::bot::Bot;

impl Bot {
    pub async fn opcode10_event(&self, op10_data: &Opcode, sender: Sender<Message>) -> anyhow::Result<()> {
        let op10 = serde_json::from_value::<Op10>(op10_data.d.clone())?;

        // 最初の待ち時間を計算
        let jitter = rand::rng().random_range(0.0..=1.0);
        let wait_time = (op10.heartbeat_interval as f64) * jitter;

        let sequential_num = Arc::clone(&self.sequential_num);

        // 別スレット
        tokio::task::spawn(async move {
            // 待ち時間分スレットを待機
            tokio::time::sleep(Duration::from_millis(wait_time as u64)).await;

            // 以降はこのインターバルで送信
            let mut interval = tokio::time::interval(Duration::from_millis(op10.heartbeat_interval as u64));

            // インターバルでループ開始
            loop {
                interval.tick().await;
                let sequential_num = {
                    let s = sequential_num.load(Ordering::Relaxed);
                    if s == -1 {
                        None
                    } else {
                        Some(s)
                    }
                };
                let op1 = Op1 {op: 1, d: sequential_num};
                match serde_json::to_string(&op1) {
                    Ok(json) => {
                        println!("op1: {:#?}", &json);
                        let _ = sender.send(Message::Text(Utf8Bytes::from(json))).await;
                    },
                    Err(error) => {
                        println!("{:#?}", error.to_string());
                        break;
                    }
                }
            }
        });
        Ok(())
    }
}
