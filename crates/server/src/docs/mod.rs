mod events;
mod functs;
mod options;

#[rustfmt::skip]
const ENUM_DEFINITIONS: &[(&str, &str, &str)] = &[
	("event_declaration",    "Events",    self::events::EVENT_DECLARATION_DESCRIPTION),
	("event_from_field",     "from",      self::events::EVENT_FIELD_DESCRIPTION_FROM),
	("event_type_field",     "type",      self::events::EVENT_FIELD_DESCRIPTION_TYPE),
	("event_call_field",     "call",      self::events::EVENT_FIELD_DESCRIPTION_CALL),
	("event_data_field",     "data",      self::events::EVENT_FIELD_DESCRIPTION_DATA),
	("function_declaration", "Functions", self::functs::FUNCT_DECLARATION_DESCRIPTION),
	("function_call_field",  "call",      self::functs::FUNCT_FIELD_DESCRIPTION_CALL),
	("function_args_field",  "args",      self::functs::FUNCT_FIELD_DESCRIPTION_ARGS),
	("function_rets_field",  "rets",      self::functs::FUNCT_FIELD_DESCRIPTION_RETS),
];

pub fn find_enum_docs<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let kinds: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (kind, header, desc) in ENUM_DEFINITIONS {
        if kinds.contains(&kind.to_string()) {
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
	("call_default",                "string",  self::options::OPTION_DESCRIPTION_CALL_DEFAULT),
	("remote_scope",                "string",  self::options::OPTION_DESCRIPTION_REMOTE_SCOPE),
	("remote_folder",               "string",  self::options::OPTION_DESCRIPTION_REMOTE_FOLDER),
	("casing",                      "string",  self::options::OPTION_DESCRIPTION_CASING),
	("write_checks",                "boolean", self::options::OPTION_DESCRIPTION_WRITE_CHECKS),
	("typescript",                  "boolean", self::options::OPTION_DESCRIPTION_TYPESCRIPT),
	("typescript_max_tuple_length", "number",  self::options::OPTION_DESCRIPTION_TYPESCRIPT_MAX_TUPLE_LENGTH),
	("manual_event_loop",           "boolean", self::options::OPTION_DESCRIPTION_MANUAL_EVENT_LOOP),
	("yield_type",                  "string",  self::options::OPTION_DESCRIPTION_YIELD_TYPE),
	("async_lib",                   "string",  self::options::OPTION_DESCRIPTION_ASYNC_LIB),
	("tooling",                     "boolean", self::options::OPTION_DESCRIPTION_TOOLING),
	("tooling_output",              "path",    self::options::OPTION_DESCRIPTION_TOOLING_OUTPUT),
	("tooling_show_internal_data",  "boolean", self::options::OPTION_DESCRIPTION_TOOLING_SHOW_INTERNAL_DATA),
	("disable_fire_all",            "boolean", self::options::OPTION_DESCRIPTION_DISABLE_FIRE_ALL),
];

pub fn find_option_docs<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, _, desc) in OPTION_DEFINITIONS {
        if names.contains(&name.to_string()) {
            return Some((name, desc));
        }
    }

    None
}

pub fn get_option_names() -> impl Iterator<Item = &'static str> {
    OPTION_DEFINITIONS.iter().map(|(name, _, _)| *name)
}
