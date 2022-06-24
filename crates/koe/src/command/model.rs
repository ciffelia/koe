#[derive(Debug, Clone)]
pub enum Command {
    Join,
    Leave,
    Skip,
    Voice,
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
