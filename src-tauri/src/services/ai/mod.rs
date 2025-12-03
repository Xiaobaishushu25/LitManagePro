use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::config::{AiConfig};
use reqwest::Client;
use serde_json::{Value, json};
use std::fmt;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use crate::dtos::tag::TagAndGroups;

mod deepseek;
mod kimi;
const KIMI_CHAT: &str = "https://api.moonshot.cn/v1/chat/completions";
const DEEPSEEK_CHAT: &str = "https://api.deepseek.com/chat/completions";

pub static ONCE_AI: OnceLock<Mutex<Option<AI>>> = OnceLock::new();

#[allow(dead_code)] //field `online` is never read
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
    pub fn get_ai_from_ai_config(ai_config: &AiConfig) -> AppResult<AI> {
        let (supporter, key, model, online) = Self::parse_ai_config(ai_config)?;
        let ai = AI::new(supporter.into(), key.to_string(), model.to_string(), online);
        Ok(ai)
    }
    pub fn update_by_ai_config(&mut self, ai_config: &AiConfig) -> AppResult<()> {
        let (supporter, key, model, online) = Self::parse_ai_config(ai_config)?;
        self.supporter = supporter.into();
        self.key = key.into();
        self.model = model.into();
        self.online = online;
        Ok(())
    }
    fn parse_ai_config(ai_config: &AiConfig) -> AppResult<(&String, &String, &String, bool)> {
        let default_ai = &ai_config.default_ai;
        let key = ai_config
            .keys
            .get(default_ai)
            .ok_or(Tip(format!("未找到{}对应的key", default_ai)))?;
        let model = ai_config
            .default_model
            .get(default_ai)
            .ok_or(Tip(format!("未找到{}对应的默认模型", default_ai)))?;
        Ok((default_ai, key, model, ai_config.online))
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
            必须把回答信息序列化为json字符串，key先插入一个id，值为{},然后分别是title，author，journal，year，abstract，contributions，remark，不要有任何额外信息。请务必准确，如果提供给你的内容中找不到需要的数据时请联网搜索。
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
    pub async fn analyse_tags(&self, paper_content: String, paper_id: i32,tag_and_groups: Vec<TagAndGroups>) -> AppResult<String> {
        let url = match self.supporter.as_str() {
            "kimi" => KIMI_CHAT,
            "deepseek" => DEEPSEEK_CHAT,
            _ => panic!("Invalid supporter"),
        };
        let prompt = format!(
            "请你准确地根据提供给你的论文内容、该论文的期刊会议信息、联网搜索信息和标签组以及每组内的标签，选出所有与论文内容符合的标签,组内若有多个同时符合的标签可多选。
            必须把回答信息序列化为json字符串，key先插入一个id，值为{},然后分别是你选出的标签，标签之间用逗号隔开，不要有任何额外信息。请务必准确，如果提供给你的内容中找不到需要的数据时请联网搜索。
            注意，不要在json字符串中包含任何控制符（如换行符等）。",
            paper_id
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
                {"role": "system", "content": tag_and_groups },
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
        println!("{}", content);
        Ok(content.into())
    }
}
impl fmt::Display for AI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 在这里指定要显示的字段，忽略 client 字段
        write!(
            f,
            "AI {{ supporter: {}, key: {}, model: {}, online: {} }}",
            self.supporter, self.key, self.model, self.online
        )
    }
}
