use serenity::model::application::{CommandInteraction, ResolvedOption, ResolvedValue};

use super::model::{Command, DictAddOption, DictRemoveOption};

pub fn parse(cmd: &CommandInteraction) -> Command {
    match cmd.data.name.as_str() {
        "join" | "kjoin" => Command::Join,
        "leave" | "kleave" => Command::Leave,
        "skip" | "kskip" => Command::Skip,
        "voice" => Command::Voice,
        "dict" => parse_dict(&cmd.data.options()),
        "help" => Command::Help,
        _ => Command::Unknown,
    }
}

fn parse_dict(options: &[ResolvedOption]) -> Command {
    let option_dict = match options.first() {
        Some(option) => option,
        None => return Command::Unknown,
    };

    match option_dict.name {
        "add" => {
            let ResolvedValue::SubCommand(suboptions) = &option_dict.value else {
                return Command::Unknown;
            };

            let [
                ResolvedOption {
                    name: "word",
                    value: ResolvedValue::String(word),
                    ..
                },
                ResolvedOption {
                    name: "read-as",
                    value: ResolvedValue::String(read_as),
                    ..
                },
            ] = &suboptions[..]
            else {
                return Command::Unknown;
            };

            Command::DictAdd(DictAddOption {
                word: word.to_string(),
                read_as: read_as.to_string(),
            })
        }
        "remove" => {
            let ResolvedValue::SubCommand(suboptions) = &option_dict.value else {
                return Command::Unknown;
            };

            let [
                ResolvedOption {
                    name: "word",
                    value: ResolvedValue::String(word),
                    ..
                },
            ] = &suboptions[..]
            else {
                return Command::Unknown;
            };

            Command::DictRemove(DictRemoveOption {
                word: word.to_string(),
            })
        }
        "view" => Command::DictView,
        _ => Command::Unknown,
    }
}
