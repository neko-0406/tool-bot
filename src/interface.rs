use serde::{Deserialize, Serialize};

// Websocket接続用URL要求時のレスポンスオブジェクト
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayResponse {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u64,
    pub max_concurrency: u32,
}

// Websocket接続時のレスポンスオブジェクト
#[derive(Debug, Serialize, Deserialize)]
pub struct Opcode {
    pub t: Option<String>,
    pub s: Option<i32>,
    pub op: i32,
    pub d: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Op10 {
    pub heartbeat_interval: i32,
    pub _trace: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Op1 {
    pub op: u8,
    pub d: Option<i32>
}