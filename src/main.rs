use evo_agent_sdk::{AgentRunner, kernel_handlers::LearningHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AgentRunner::run(LearningHandler).await
}
