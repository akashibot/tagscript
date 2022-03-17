pub use crate::{Action, Adapter, Block, Context, Result};

pub fn helper_parse_if(if_string: String) -> Option<bool> {
    if if_string.contains("!=") {
        let spl = if_string.split("!=").collect::<Vec<&str>>();
        return Some(spl[0] != spl[1]);
    }
    if if_string.contains("==") {
        let spl = if_string.split("==").collect::<Vec<&str>>();
        return Some(spl[0] == spl[1]);
    }
    if if_string.contains("<") {
        let spl = if_string.split("<").collect::<Vec<&str>>();
        return Some(spl[0] < spl[1]);
    }
    if if_string.contains(">") {
        let spl = if_string.split(">").collect::<Vec<&str>>();
        return Some(spl[0] > spl[1]);
    }
    if if_string.contains("<=") {
        let spl = if_string.split("<=").collect::<Vec<&str>>();
        return Some(spl[0] <= spl[1]);
    }
    if if_string.contains(">=") {
        let spl = if_string.split(">=").collect::<Vec<&str>>();
        return Some(spl[0] >= spl[1]);
    }
    None
}

pub fn helper_split(split_string: String, easy: bool) -> Option<Vec<String>> {
    if split_string.contains("|") {
        return Some(split_string.split("|").map(|s| s.to_string()).collect());
    } else if easy && split_string.contains("~") {
        return Some(split_string.split("~").map(|s| s.to_string()).collect());
    } else if easy && split_string.contains(",") {
        return Some(split_string.split(",").map(|s| s.to_string()).collect());
    }

    None
}

pub fn helper_parse_list_if(if_string: String) -> Vec<Option<bool>> {
    match helper_split(if_string.clone(), false) {
        Some(list) => list.into_iter().map(|s| helper_parse_if(s)).collect(),
        None => vec![helper_parse_if(if_string)],
    }
}
