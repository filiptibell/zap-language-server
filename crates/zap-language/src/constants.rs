use std::sync::LazyLock;

use zap_documentation as docs;

pub static INSTANCE_CLASS_NAMES: LazyLock<&'static [&'static str]> = LazyLock::new(|| {
    let mut names = Vec::new();
    for line in docs::generated::INSTANCE_CLASSES.lines() {
        let name = line.trim();
        if !name.is_empty() {
            names.push(name);
        }
    }
    names.leak()
});

#[rustfmt::skip]
pub const KEYWORD_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event",     "Events",     docs::keywords::EVENT),
	("funct",     "Functions",  docs::keywords::FUNCT),
	("type",      "Types",      docs::keywords::TYPE),
	("struct",    "Structs",    docs::keywords::STRUCT),
	("enum",      "Enums",      docs::keywords::ENUM),
	("map",       "Maps",       docs::keywords::MAP),
	("set",       "Sets",       docs::keywords::SET),
	("namespace", "Namespaces", docs::keywords::NAMESPACE),
];

#[rustfmt::skip]
pub const PROPERTY_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event_from_field",     "from", docs::events::FROM),
	("event_type_field",     "type", docs::events::TYPE),
	("event_call_field",     "call", docs::events::CALL),
	("event_data_field",     "data", docs::events::DATA),
	("function_call_field",  "call", docs::functs::CALL),
	("function_args_field",  "args", docs::functs::ARGS),
	("function_rets_field",  "rets", docs::functs::RETS),
];

#[rustfmt::skip]
pub const OPTION_DEFINITIONS: &[(&str, &str, &str)] = &[
	("server_output",               "path",    docs::options::SERVER_OUTPUT),
	("client_output",               "path",    docs::options::CLIENT_OUTPUT),
	("types_output",                "path",    docs::options::TYPES_OUTPUT),
	("call_default",                "variant", docs::options::CALL_DEFAULT),
	("remote_scope",                "string",  docs::options::REMOTE_SCOPE),
	("remote_folder",               "string",  docs::options::REMOTE_FOLDER),
	("casing",                      "variant", docs::options::CASING),
	("write_checks",                "boolean", docs::options::WRITE_CHECKS),
	("typescript",                  "boolean", docs::options::TYPESCRIPT),
	("typescript_max_tuple_length", "number",  docs::options::TYPESCRIPT_MAX_TUPLE_LENGTH),
	("manual_event_loop",           "boolean", docs::options::MANUAL_EVENT_LOOP),
	("yield_type",                  "variant", docs::options::YIELD_TYPE),
	("async_lib",                   "string",  docs::options::ASYNC_LIB),
	("tooling",                     "boolean", docs::options::TOOLING),
	("tooling_output",              "path",    docs::options::TOOLING_OUTPUT),
	("tooling_show_internal_data",  "boolean", docs::options::TOOLING_SHOW_INTERNAL_DATA),
	("disable_fire_all",            "boolean", docs::options::DISABLE_FIRE_ALL),
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
	("boolean",        "Booleans",               docs::primitives::BOOLEAN),
	("string",         "Strings",                docs::primitives::STRING),
	("f64",            "Floating Point Numbers", docs::primitives::FLOAT),
	("f32",            "Floating Point Numbers", docs::primitives::FLOAT),
	("u8",             "Unsigned Integers",      docs::primitives::INT_UNSIGNED),
    ("u16",            "Unsigned Integers",      docs::primitives::INT_UNSIGNED),
    ("u32",            "Unsigned Integers",      docs::primitives::INT_UNSIGNED),
    ("i8",             "Signed Integers",        docs::primitives::INT_SIGNED),
    ("i16",            "Signed Integers",        docs::primitives::INT_SIGNED),
    ("i32",            "Signed Integers",        docs::primitives::INT_SIGNED),
    ("CFrame",         "CFrames",                docs::primitives::CFRAME),
    ("AlignedCFrame",  "CFrames",                docs::primitives::CFRAME),
    ("Vector2",        "Vectors",                docs::primitives::VECTORS),
    ("Vector3",        "Vectors",                docs::primitives::VECTORS),
    ("DateTime",       "DateTimes",              docs::primitives::DATETIME),
    ("DateTimeMillis", "DateTimes",              docs::primitives::DATETIME),
    ("BrickColor",     "Colors",                 docs::primitives::COLORS),
    ("Color3",         "Colors",                 docs::primitives::COLORS),
    ("Instance",       "Instances",              docs::primitives::INSTANCE),
];
