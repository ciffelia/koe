use serenity::builder::CreateEmbed;

#[derive(Debug, Clone)]
pub enum Command {
    Join,
    Leave,
    Skip,
    DictAdd(DictAddOption),
    DictRemove(DictRemoveOption),
    DictView,
    Help,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DictAddOption {
    pub word: String,
    pub read_as: String,
}

#[derive(Debug, Clone)]
pub struct DictRemoveOption {
    pub word: String,
}

#[derive(Debug, Clone)]
pub enum CommandResponse {
    Text(String),
    Embed(CreateEmbed),
}

impl<T> From<T> for CommandResponse
where
    T: ToString,
{
    fn from(value: T) -> Self {
        CommandResponse::Text(value.to_string())
    }
}
