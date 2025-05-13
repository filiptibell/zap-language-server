use std::sync::LazyLock;

mod events;
mod functs;
mod keywords;
mod options;
mod primitives;

pub const INSTANCE_CLASS_FILE: &str = include_str!("./classes.txt");
pub const INSTANCE_CLASS_NAMES: LazyLock<&'static [&'static str]> = LazyLock::new(|| {
    let mut names = Vec::new();
    for line in INSTANCE_CLASS_FILE.lines() {
        let name = line.trim();
        if !name.is_empty() {
            names.push(name);
        }
    }
    names.leak()
});

#[rustfmt::skip]
pub const KEYWORD_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event",  "Events",    self::keywords::KEYWORD_EVENT_DESCRIPTION),
	("funct",  "Functions", self::keywords::KEYWORD_FUNCT_DESCRIPTION),
	("type",   "Types",     self::keywords::KEYWORD_TYPE_DESCRIPTION),
	("struct", "Structs",   self::keywords::KEYWORD_STRUCT_DESCRIPTION),
	("enum",   "Enums",     self::keywords::KEYWORD_ENUM_DESCRIPTION),
	("map",    "Maps",      self::keywords::KEYWORD_MAP_DESCRIPTION),
	("set",    "Sets",      self::keywords::KEYWORD_SET_DESCRIPTION),
];

#[rustfmt::skip]
pub const PROPERTY_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event_from_field",     "from", self::events::EVENT_FIELD_DESCRIPTION_FROM),
	("event_type_field",     "type", self::events::EVENT_FIELD_DESCRIPTION_TYPE),
	("event_call_field",     "call", self::events::EVENT_FIELD_DESCRIPTION_CALL),
	("event_data_field",     "data", self::events::EVENT_FIELD_DESCRIPTION_DATA),
	("function_call_field",  "call", self::functs::FUNCT_FIELD_DESCRIPTION_CALL),
	("function_args_field",  "args", self::functs::FUNCT_FIELD_DESCRIPTION_ARGS),
	("function_rets_field",  "rets", self::functs::FUNCT_FIELD_DESCRIPTION_RETS),
];

#[rustfmt::skip]
pub const OPTION_DEFINITIONS: &[(&str, &str, &str)] = &[
	("server_output",               "path",    self::options::OPTION_DESCRIPTION_SERVER_OUTPUT),
	("client_output",               "path",    self::options::OPTION_DESCRIPTION_CLIENT_OUTPUT),
	("types_output",                "path",    self::options::OPTION_DESCRIPTION_TYPES_OUTPUT),
	("call_default",                "variant", self::options::OPTION_DESCRIPTION_CALL_DEFAULT),
	("remote_scope",                "string",  self::options::OPTION_DESCRIPTION_REMOTE_SCOPE),
	("remote_folder",               "string",  self::options::OPTION_DESCRIPTION_REMOTE_FOLDER),
	("casing",                      "variant", self::options::OPTION_DESCRIPTION_CASING),
	("write_checks",                "boolean", self::options::OPTION_DESCRIPTION_WRITE_CHECKS),
	("typescript",                  "boolean", self::options::OPTION_DESCRIPTION_TYPESCRIPT),
	("typescript_max_tuple_length", "number",  self::options::OPTION_DESCRIPTION_TYPESCRIPT_MAX_TUPLE_LENGTH),
	("manual_event_loop",           "boolean", self::options::OPTION_DESCRIPTION_MANUAL_EVENT_LOOP),
	("yield_type",                  "variant", self::options::OPTION_DESCRIPTION_YIELD_TYPE),
	("async_lib",                   "string",  self::options::OPTION_DESCRIPTION_ASYNC_LIB),
	("tooling",                     "boolean", self::options::OPTION_DESCRIPTION_TOOLING),
	("tooling_output",              "path",    self::options::OPTION_DESCRIPTION_TOOLING_OUTPUT),
	("tooling_show_internal_data",  "boolean", self::options::OPTION_DESCRIPTION_TOOLING_SHOW_INTERNAL_DATA),
	("disable_fire_all",            "boolean", self::options::OPTION_DESCRIPTION_DISABLE_FIRE_ALL),
];

#[rustfmt::skip]
pub const VARIANT_DEFINITIONS: &[(&str, bool, &[&str])] = &[
	// Variants for events and functions are plain idents
	("event_from_field",    false, &["Server", "Client"]),
	("event_type_field",    false, &["Reliable", "Unreliable"]),
	("event_call_field",    false, &["ManyAsync", "ManySync", "SingleAsync", "SingleSync", "Polling"]),
	("function_call_field", false, &["Async", "Sync"]),
	// Options variants need to be enclosed in a string,
	// unlike above, so we give them a special bool flag
	("call_default", true,  &["ManyAsync", "ManySync", "SingleAsync", "SingleSync", "Polling"]),
	("casing",       true,  &["camelCase", "PascalCase", "snake_case"]),
	("yield_type",   true,  &["yield", "future", "promise"]),
];

#[rustfmt::skip]
pub const PRIMITIVE_DEFINITIONS: &[(&str, &str, &str)] = &[
	("boolean",        "Booleans",               self::primitives::PRIMITIVE_DESCRIPTION_BOOLEAN),
	("string",         "Strings",                self::primitives::PRIMITIVE_DESCRIPTION_STRING),
	("f64",            "Floating Point Numbers", self::primitives::PRIMITIVE_DESCRIPTION_FLOAT),
	("f32",            "Floating Point Numbers", self::primitives::PRIMITIVE_DESCRIPTION_FLOAT),
	("u8",             "Unsigned Integers",      self::primitives::PRIMITIVE_DESCRIPTION_UNSIGNED),
    ("u16",            "Unsigned Integers",      self::primitives::PRIMITIVE_DESCRIPTION_UNSIGNED),
    ("u32",            "Unsigned Integers",      self::primitives::PRIMITIVE_DESCRIPTION_UNSIGNED),
    ("i8",             "Signed Integers",        self::primitives::PRIMITIVE_DESCRIPTION_SIGNED),
    ("i16",            "Signed Integers",        self::primitives::PRIMITIVE_DESCRIPTION_SIGNED),
    ("i32",            "Signed Integers",        self::primitives::PRIMITIVE_DESCRIPTION_SIGNED),
    ("CFrame",         "CFrames",                self::primitives::PRIMITIVE_DESCRIPTION_CFRAME),
    ("AlignedCFrame",  "CFrames",                self::primitives::PRIMITIVE_DESCRIPTION_CFRAME),
    ("Vector2",        "Vectors",                self::primitives::PRIMITIVE_DESCRIPTION_VECTOR),
    ("Vector3",        "Vectors",                self::primitives::PRIMITIVE_DESCRIPTION_VECTOR),
    ("DateTime",       "DateTimes",              self::primitives::PRIMITIVE_DESCRIPTION_DATETIME),
    ("DateTimeMillis", "DateTimes",              self::primitives::PRIMITIVE_DESCRIPTION_DATETIME),
    ("BrickColor",     "Colors",                 self::primitives::PRIMITIVE_DESCRIPTION_COLORS),
    ("Color3",         "Colors",                 self::primitives::PRIMITIVE_DESCRIPTION_COLORS),
    ("Instance",       "Instances",              self::primitives::PRIMITIVE_DESCRIPTION_INSTANCE),
];
