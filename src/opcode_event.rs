use std::time::Duration;
use rand::RngExt;
use crate::interface::{Op10, Opcode};

pub async fn opcode10_event(op10_data: &Opcode) -> anyhow::Result<()> {
    let op10 = serde_json::from_value::<Op10>(op10_data.d.clone())?;

    // 最初の待ち時間を計算
    let jitter = rand::rng().random_range(0.0..=1.0);
    let wait_time = (op10.heartbeat_interval as f64) * jitter;

    // 待ち時間分スレットを待機
    tokio::time::sleep(Duration::from_millis(wait_time as u64)).await;

    // 以降はこのインターバルで送信
    let mut interval = tokio::time::interval(Duration::from_millis(op10.heartbeat_interval as u64));

    // インターバルでループ開始
    loop {
        interval.tick().await;
    }
    Ok(())
}