mod options;

pub use self::options::completion as completion_for_options;

pub fn completion_trigger_characters() -> Vec<String> {
    let mut chars = vec![
        String::from("\""),
        String::from("'"),
        String::from("/"),
        String::from("@"),
        String::from("."),
        String::from("-"),
        String::from("_"),
    ];

    chars.sort();
    chars
}
