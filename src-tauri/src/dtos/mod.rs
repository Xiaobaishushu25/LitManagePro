use serde::{Deserialize, Serialize};

pub mod doc;
pub mod tag;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub id: i32, //为了以后多个进度条做准备
    pub msg: String,
    pub now: f64,
    pub max: f64,
}
impl Progress {
    pub fn new(id: i32, msg: String, now: f64, max: f64) -> Self {
        Self {
            id,
            msg,
            now,
            max,
        }
    }
}
