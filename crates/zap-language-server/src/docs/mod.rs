mod events;
mod functs;
mod keywords;
mod options;

#[rustfmt::skip]
const KEYWORD_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event",  "Events",    self::keywords::KEYWORD_EVENT_DESCRIPTION),
	("funct",  "Functions", self::keywords::KEYWORD_FUNCT_DESCRIPTION),
	("struct", "Structs",   self::keywords::KEYWORD_STRUCT_DESCRIPTION),
	("enum",   "Enums",     self::keywords::KEYWORD_ENUM_DESCRIPTION),
	("map",    "Maps",      self::keywords::KEYWORD_MAP_DESCRIPTION),
	("set",    "Sets",      self::keywords::KEYWORD_SET_DESCRIPTION),
];

pub fn find_keyword<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, header, desc) in KEYWORD_DEFINITIONS {
        if names.contains(&name.to_string()) {
            return Some((header, desc));
        }
    }

    None
}

#[rustfmt::skip]
const PROPERTY_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event_from_field",     "from",      self::events::EVENT_FIELD_DESCRIPTION_FROM),
	("event_type_field",     "type",      self::events::EVENT_FIELD_DESCRIPTION_TYPE),
	("event_call_field",     "call",      self::events::EVENT_FIELD_DESCRIPTION_CALL),
	("event_data_field",     "data",      self::events::EVENT_FIELD_DESCRIPTION_DATA),
	("function_call_field",  "call",      self::functs::FUNCT_FIELD_DESCRIPTION_CALL),
	("function_args_field",  "args",      self::functs::FUNCT_FIELD_DESCRIPTION_ARGS),
	("function_rets_field",  "rets",      self::functs::FUNCT_FIELD_DESCRIPTION_RETS),
];

pub fn find_property<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, header, desc) in PROPERTY_DEFINITIONS {
        if names.contains(&name.to_string()) {
            return Some((header, desc));
        }
    }

    None
}

#[rustfmt::skip]
const OPTION_DEFINITIONS: &[(&str, &str, &str)] = &[
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

pub fn find_option<I, S>(it: I) -> Option<(&'static str, &'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, typ, desc) in OPTION_DEFINITIONS {
        if names.contains(&name.to_string()) {
            return Some((name, typ, desc));
        }
    }

    None
}

pub fn get_option_names() -> impl Iterator<Item = &'static str> {
    OPTION_DEFINITIONS.iter().map(|(name, _, _)| *name)
}

#[rustfmt::skip]
const VARIANT_DEFINITIONS: &[(&str, bool, &[&str])] = &[
	("event_from_field",     false, &["Server", "Client"]),
	("event_type_field",     false, &["Reliable", "Unreliable"]),
	("event_call_field",     false, &["ManyAsync", "ManySync", "SingleAsync", "SingleSync", "Polling"]),
	("function_call_field",  false, &["Async", "Sync"]),
	("call_default",         true,  &["ManyAsync", "ManySync", "SingleAsync", "SingleSync", "Polling"]),
	("casing",               true,  &["camelCase", "PascalCase", "snake_case"]),
	("yield_type",           true,  &["yield", "future", "promise"]),
];

pub fn find_variants<I, S>(it: I) -> Option<(bool, &'static [&'static str])>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, is_option, desc) in VARIANT_DEFINITIONS {
        if names.contains(&name.to_string()) {
            return Some((*is_option, desc));
        }
    }

    None
}
