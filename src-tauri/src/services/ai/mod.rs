use std::sync::{OnceLock};
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use reqwest::Client;
use serde_json::{Value, json};
use tokio::sync::Mutex;

mod deepseek;
mod kimi;
const KIMI_CHAT: &str = "https://api.moonshot.cn/v1/chat/completions";
const DEEPSEEK_CHAT: &str = "https://api.deepseek.com/chat/completions";

pub static ONCE_AI:OnceLock<Mutex<Option<AI>>> = OnceLock::new();

#[allow(dead_code)]//field `online` is never read 
#[derive(Debug)]
pub struct AI {
    client: Client,
    pub supporter: String,
    pub key: String,
    pub model: String,
    pub online: bool,
}
impl AI {
    pub fn new(supporter: String, key: String, model: String, online: bool) -> Self {
        let client = reqwest::Client::new();
        AI {
            client,
            supporter,
            key,
            model,
            online,
        }
    }
    pub fn change(&mut self, supporter: String, key: String, model: String, online: bool) {
        self.supporter = supporter;
        self.key = key;
        self.model = model;
        self.online = online;
    }
    pub async fn analyse_paper(&self, paper_content: String, paper_id: i32) -> AppResult<String> {
        let url = match self.supporter.as_str() {
            "kimi" => KIMI_CHAT,
            "deepseek" => DEEPSEEK_CHAT,
            _ => panic!("Invalid supporter"),
        };
        let lang = "中文";
        let prompt = format!(
            "请你准确地根据提供给你的论文内容和联网搜索信息给出该论文的标题、作者、发表的刊物（会议或者期刊简称）、发表年份、并总结摘要和贡献点以及核心思想(即remark)。
            摘要、贡献点以及核心思想请总结为{}。
            必须把回答信息序列化为json字符串，key先插入一个id，值为{},然后分别是title，author，journal，year，abstract，contributions，remark，不要有任何额外信息。请务必准确，必要时请联网搜索。
            注意，年份为字符串，不要用数字。并且不要在json字符串中包含任何控制符（如换行符等）。",
            lang, paper_id
         );
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Content-Type", "application/json")
            .json(&json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "你是专业的artificial intelligence论文分析助手，你更擅长中文和英文的对话。你会为用户提供有帮助，准确的回答。在不确定时，你会联网搜索最新的信息。"},
                {"role": "system", "content": paper_content },
                {"role": "user", "content": prompt}
            ],
            "stream": false,
            "temperature": 0.1
        }))
            .send()
            .await?;
        let text = response
            .text()
            .await
            .map_err(|e| Tip(format!("获取响应的Text出错：{:#}", e)))?;
        let value: Value = serde_json::from_str(&text)?;
        // 提取 content 字段
        let content = value["choices"][0]["message"]["content"]
            .as_str()
            .ok_or(Tip(format!("content字段解析出错:{}", text)))?;
        Ok(content.into())
    }
}
