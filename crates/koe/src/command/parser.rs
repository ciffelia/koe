use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};

use super::model::{Command, DictAddOption, DictRemoveOption};

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
    let option_dict = match cmd.data.options.first() {
        Some(option) => option,
        None => return Command::Unknown,
    };

    match option_dict.name.as_str() {
        "add" => {
            let option_word = match option_dict.options.first() {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let option_read_as = match option_dict.options.get(1) {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let word = match &option_word.resolved {
                Some(CommandDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };
            let read_as = match &option_read_as.resolved {
                Some(CommandDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };

            Command::DictAdd(DictAddOption {
                word: word.clone(),
                read_as: read_as.clone(),
            })
        }
        "remove" => {
            let option_word = match option_dict.options.first() {
                Some(x) => x,
                None => return Command::Unknown,
            };
            let word = match &option_word.resolved {
                Some(CommandDataOptionValue::String(x)) => x,
                _ => return Command::Unknown,
            };

            Command::DictRemove(DictRemoveOption { word: word.clone() })
        }
        "view" => Command::DictView,
        _ => Command::Unknown,
    }
}
