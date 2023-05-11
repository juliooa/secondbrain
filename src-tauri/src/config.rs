use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub authentication: Authentication,
    pub model: Model,
    pub inference: Inference,
    pub commands: HashMap<String, Command>,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            authentication: Authentication {
                discord_token: None,
            },
            model: Model {
                path: "models/7B/ggml-alpaca-q4_0.bin".into(),
                context_token_length: 2048,
                architecture: llm::ModelArchitecture::Llama.to_string(),
                prefer_mmap: true,
            },
            inference: Inference {
                thread_count: 8,
                batch_size: 8,
                discord_message_update_interval_ms: 250,
                replace_newlines: true,
                show_prompt_template: true,
            },
            commands: HashMap::from_iter([
                (
                    "hallucinate".into(),
                    Command {
                        enabled: false,
                        description: "Hallucinates some text.".into(),
                        prompt: "{{PROMPT}}".into(),
                    },
                ),
                (
                    "alpaca".into(),
                    Command {
                        enabled: false,
                        description: "Responds to the provided instruction.".into(),
                        prompt: indoc::indoc! {
                            "Below is an instruction that describes a task. Write a response that appropriately completes the request.

                            ### Instruction:
                            
                            {{PROMPT}}
                            
                            ### Response:
                            
                            "
                        }.into(),
                    },
                ),
            ]),
        }
    }
}
impl Configuration {
    const FILENAME: &str = "config.toml";

    pub fn load() -> anyhow::Result<Self> {
        let config = if let Ok(file) = std::fs::read_to_string(Self::FILENAME) {
            toml::from_str(&file).context("failed to load config")?
        } else {
            let config = Self::default();
            config.save()?;
            config
        };

        Ok(config)
    }

    fn save(&self) -> anyhow::Result<()> {
        Ok(std::fs::write(
            Self::FILENAME,
            toml::to_string_pretty(self)?,
        )?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authentication {
    pub discord_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub path: PathBuf,
    pub context_token_length: usize,
    pub architecture: String,
    pub prefer_mmap: bool,
}
impl Model {
    pub fn architecture(&self) -> Option<llm::ModelArchitecture> {
        self.architecture.parse().ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inference {
    /// The number of threads to use
    pub thread_count: usize,
    /// When the prompt is sent to the model, it will be batched. This
    /// controls the size of that batch. Larger values will result in
    /// faster inference, but will use more memory.
    pub batch_size: usize,
    /// Low values will result in you getting throttled by Discord
    pub discord_message_update_interval_ms: u64,
    /// Whether or not to replace '\n' with newlines
    pub replace_newlines: bool,
    /// Whether or not to show the entire prompt template, or just
    /// what the user specified
    pub show_prompt_template: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub enabled: bool,
    pub description: String,
    pub prompt: String,
}
