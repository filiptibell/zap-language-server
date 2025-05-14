mod definitions;

use definitions::PRIMITIVE_DEFINITIONS;

use self::definitions::{
    INSTANCE_CLASS_NAMES, KEYWORD_DEFINITIONS, OPTION_DEFINITIONS, PROPERTY_DEFINITIONS,
    VARIANT_DEFINITIONS,
};

pub fn get_option_names() -> impl Iterator<Item = &'static str> {
    OPTION_DEFINITIONS.iter().map(|(name, _, _)| *name)
}

pub fn get_primitive_names() -> impl Iterator<Item = &'static str> {
    PRIMITIVE_DEFINITIONS.iter().map(|(name, _, _)| *name)
}

pub fn get_instance_class_names() -> impl Iterator<Item = &'static str> {
    INSTANCE_CLASS_NAMES.iter().copied()
}

pub fn find_keyword<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(Into::into).collect();

    for (name, header, desc) in KEYWORD_DEFINITIONS {
        if names.contains(&(*name).to_string()) {
            return Some((header, desc));
        }
    }

    None
}

pub fn find_property<I, S>(it: I) -> Option<(&'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(Into::into).collect();

    for (name, header, desc) in PROPERTY_DEFINITIONS {
        if names.contains(&(*name).to_string()) {
            return Some((header, desc));
        }
    }

    None
}

pub fn find_option<I, S>(it: I) -> Option<(&'static str, &'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(Into::into).collect();

    for (name, typ, desc) in OPTION_DEFINITIONS {
        if names.contains(&(*name).to_string()) {
            return Some((name, typ, desc));
        }
    }

    None
}

pub fn find_primitive<I, S>(it: I) -> Option<(&'static str, &'static str, &'static str)>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(Into::into).collect();

    for (name, header, desc) in PRIMITIVE_DEFINITIONS {
        if names.contains(&(*name).to_string()) {
            return Some((name, header, desc));
        }
    }

    None
}

pub fn find_variants<I, S>(it: I) -> Option<(bool, &'static [&'static str])>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let names: Vec<String> = it.into_iter().map(Into::into).collect();

    for (name, is_option, desc) in VARIANT_DEFINITIONS {
        if names.contains(&(*name).to_string()) {
            return Some((*is_option, desc));
        }
    }

    None
}
