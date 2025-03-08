//use regex::Regex;
//
//pub fn validate_commit_message(msg: &str) -> bool {
//    let types = ["feat", "fix", "docs", "style", "refactor", "test", "chore"];
//    let scopes = [
//        "core", "config", "deps", "scripts", "setup", "docs", "build", "ci", "testing", "refactor",
//    ];
//
//    let pattern = format!(
//        r"^(?P<type>{})(?:\((?P<scope>{})\))?: [A-Za-z][a-zA-Z0-9 ]+$",
//        types.join("|"),
//        scopes.join("|")
//    );
//
//    let re = Regex::new(&pattern).unwrap();
//    re.is_match(msg) && msg.len() <= 72
//}

// TODO: improve error handling
