mod events;
mod functs;
mod options;

#[rustfmt::skip]
const ENUM_NAMES_AND_DESCRIPTIONS: &[(&str, (&str, &str))] = &[
	("event_declaration",    ("Events",    self::events::EVENT_DECLARATION_DESCRIPTION)),
	("event_from_field",     ("from",      self::events::EVENT_FIELD_DESCRIPTION_FROM)),
	("event_type_field",     ("type",      self::events::EVENT_FIELD_DESCRIPTION_TYPE)),
	("event_call_field",     ("call",      self::events::EVENT_FIELD_DESCRIPTION_CALL)),
	("event_data_field",     ("data",      self::events::EVENT_FIELD_DESCRIPTION_DATA)),
	("function_declaration", ("Functions", self::functs::FUNCT_DECLARATION_DESCRIPTION)),
	("function_call_field",  ("call",      self::functs::FUNCT_FIELD_DESCRIPTION_CALL)),
	("function_args_field",  ("args",      self::functs::FUNCT_FIELD_DESCRIPTION_ARGS)),
	("function_rets_field",  ("rets",      self::functs::FUNCT_FIELD_DESCRIPTION_RETS)),
];

pub fn find_docs_enum<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let kinds: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (kind, (header, desc)) in ENUM_NAMES_AND_DESCRIPTIONS {
        if kinds.contains(&kind.to_string()) {
            return Some((header, desc));
        }
    }

    None
}

#[rustfmt::skip]
const OPTION_NAMES_AND_DESCRIPTIONS: &[(&str, (&str, &str))] = &[
	("server_output",               ("server_output",               self::options::OPTION_DESCRIPTION_SERVER_OUTPUT)),
	("client_output",               ("server_output",               self::options::OPTION_DESCRIPTION_CLIENT_OUTPUT)),
	("types_output",                ("types_output",                self::options::OPTION_DESCRIPTION_TYPES_OUTPUT)),
	("call_default",                ("call_default",                self::options::OPTION_DESCRIPTION_CALL_DEFAULT)),
	("remote_scope",                ("remote_scope",                self::options::OPTION_DESCRIPTION_REMOTE_SCOPE)),
	("remote_folder",               ("remote_folder",               self::options::OPTION_DESCRIPTION_REMOTE_FOLDER)),
	("casing",                      ("casing",                      self::options::OPTION_DESCRIPTION_CASING)),
	("write_checks",                ("write_checks",                self::options::OPTION_DESCRIPTION_WRITE_CHECKS)),
	("typescript",                  ("typescript",                  self::options::OPTION_DESCRIPTION_TYPESCRIPT)),
	("typescript_max_tuple_length", ("typescript_max_tuple_length", self::options::OPTION_DESCRIPTION_TYPESCRIPT_MAX_TUPLE_LENGTH)),
	("manual_event_loop",           ("manual_event_loop",           self::options::OPTION_DESCRIPTION_MANUAL_EVENT_LOOP)),
	("yield_type",                  ("yield_type",                  self::options::OPTION_DESCRIPTION_YIELD_TYPE)),
	("async_lib",                   ("async_lib",                   self::options::OPTION_DESCRIPTION_ASYNC_LIB)),
	("tooling",                     ("tooling",                     self::options::OPTION_DESCRIPTION_TOOLING)),
	("tooling_output",              ("tooling_output",              self::options::OPTION_DESCRIPTION_TOOLING_OUTPUT)),
	("tooling_show_internal_data",  ("tooling_show_internal_data",  self::options::OPTION_DESCRIPTION_TOOLING_SHOW_INTERNAL_DATA)),
	("disable_fire_all",            ("disable_fire_all",            self::options::OPTION_DESCRIPTION_DISABLE_FIRE_ALL)),
];

pub fn find_docs_option<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(|s| s.into()).collect();

    for (name, (header, desc)) in OPTION_NAMES_AND_DESCRIPTIONS {
        if names.contains(&name.to_string()) {
            return Some((header, desc));
        }
    }

    None
}
