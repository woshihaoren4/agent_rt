use async_openai::types::ChatCompletionRequestMessage;
use std::collections::HashMap;

mod consts;
mod infra;
mod llm;
mod memory;
mod multi_agent;
mod single_agent;
mod tool;
pub mod short_long_memory;
pub mod pkg;

pub use consts::*;
pub use infra::*;
pub use llm::*;
pub use memory::*;
pub use multi_agent::*;
pub use single_agent::*;
pub use tool::*;
pub use short_long_memory::*;
pub use pkg::*;

pub trait EasyMemory: Send + Sync {
    fn load_context(&self, max: usize) -> anyhow::Result<Vec<ChatCompletionRequestMessage>>;
    fn recall_user_tag(&self) -> anyhow::Result<HashMap<String, String>>;
    fn add_session_log(&self, record: Vec<ChatCompletionRequestMessage>);
}

#[async_trait::async_trait]
pub trait Memory: Send + Sync{
    //加载上下文
    async fn load_context(&self,user:&str, max: usize) -> anyhow::Result<Vec<ChatCompletionRequestMessage>>;
    //追加会话日志，可以在上下文中获取到
    async fn add_session_log(&self,user:&str,record: Vec<ChatCompletionRequestMessage>);

    //拉取用户标签
    async fn get_user_tag(&self,user:&str,tag:&str) -> anyhow::Result<String>;
    //给用户贴标签
    async fn set_user_tage(&self,user:&str,kvs:HashMap<String,String>);

    //召回长期记忆
    async fn recall_summary(&self,user:&str,query: &str,n:usize)-> anyhow::Result<Vec<String>>;
    //将记忆进行总结
    async fn summary_history(&self,user:&str);
}




#[cfg(test)]
mod test {
    use async_openai::types::{ChatCompletionFunctionsArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionToolArgs, ChatCompletionToolType, CreateChatCompletionRequestArgs, FunctionObject, FunctionObjectArgs};
    use async_openai::Client;
    use serde_json::json;
    use crate::tool::ToolNode;

    #[tokio::test]
    async fn test_openai() {
        let chat_req = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("你是一个讲笑话助手")
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content("讲个笑话")
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();

        let client = Client::new();
        let resp = client.chat().create(chat_req).await.unwrap();
        for i in resp.choices {
            println!(
                "[{}] --->{}:{:?}",
                i.index, i.message.role, i.message.content
            );
        }
    }
}
