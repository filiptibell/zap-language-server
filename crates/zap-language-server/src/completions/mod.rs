mod instances;
mod keywords;
mod options;
mod types;

pub use self::instances::completion as completion_for_instances;
pub use self::keywords::completion as completion_for_keywords;
pub use self::options::completion as completion_for_options;
pub use self::types::completion as completion_for_types;

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
