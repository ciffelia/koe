use super::model::{Command, DictAddOption, DictRemoveOption};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};

pub fn parse(cmd: &ApplicationCommandInteraction) -> Command {
    match cmd.data.name.as_str() {
        "join" | "kjoin" => Command::Join,
        "leave" | "kleave" => Command::Leave,
        "skip" | "kskip" => Command::Skip,
        "voice" => Command::Voice,
        "dict" => parse_dict(cmd),
        "help" => Command::Help,
        _ => Command::Unknown,
    }
}

fn parse_dict(cmd: &ApplicationCommandInteraction) -> Command {
    let option_dict = match cmd.data.options.get(0) {
        Some(option) => option,
        None => return Command::Unknown,
    };

    match option_dict.name.as_str() {
        "add" => {
            let option_word = match option_dict.options.get(0) {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let option_read_as = match option_dict.options.get(1) {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let word = match &option_word.resolved {
                Some(ApplicationCommandInteractionDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };
            let read_as = match &option_read_as.resolved {
                Some(ApplicationCommandInteractionDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };

            Command::DictAdd(DictAddOption {
                word: word.clone(),
                read_as: read_as.clone(),
            })
        }
        "remove" => {
            let option_word = match option_dict.options.get(0) {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let word = match &option_word.resolved {
                Some(ApplicationCommandInteractionDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };

            Command::DictRemove(DictRemoveOption { word: word.clone() })
        }
        "view" => Command::DictView,
        _ => Command::Unknown,
    }
}
