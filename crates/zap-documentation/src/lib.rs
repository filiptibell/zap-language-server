macro_rules! file_constants {
    ($($name:ident => $path:literal),* $(,)?) => {
        $(
            pub const $name: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path));
        )*
    };
}

pub mod events {
    file_constants! {
        CALL => "/docs/events/call.md",
        DATA => "/docs/events/data.md",
        FROM => "/docs/events/from.md",
        TYPE => "/docs/events/type.md",
    }
}

pub mod functs {
    file_constants! {
        ARGS => "/docs/functs/args.md",
        CALL => "/docs/functs/call.md",
        RETS => "/docs/functs/rets.md",
    }
}

pub mod generated {
    file_constants! {
        INSTANCE_CLASSES => "/docs/generated/instance_classes.txt",
    }
}

pub mod keywords {
    file_constants! {
        ENUM      => "/docs/keywords/enum.md",
        EVENT     => "/docs/keywords/event.md",
        FUNCT     => "/docs/keywords/funct.md",
        MAP       => "/docs/keywords/map.md",
        NAMESPACE => "/docs/keywords/namespace.md",
        SET       => "/docs/keywords/set.md",
        STRUCT    => "/docs/keywords/struct.md",
        TYPE      => "/docs/keywords/type.md",
    }
}

pub mod options {
    file_constants! {
        ASYNC_LIB                   => "/docs/options/async_lib.md",
        CALL_DEFAULT                => "/docs/options/call_default.md",
        CASING                      => "/docs/options/casing.md",
        CLIENT_OUTPUT               => "/docs/options/client_output.md",
        DISABLE_FIRE_ALL            => "/docs/options/disable_fire_all.md",
        MANUAL_EVENT_LOOP           => "/docs/options/manual_event_loop.md",
        REMOTE_FOLDER               => "/docs/options/remote_folder.md",
        REMOTE_SCOPE                => "/docs/options/remote_scope.md",
        SERVER_OUTPUT               => "/docs/options/server_output.md",
        TOOLING                     => "/docs/options/tooling.md",
        TOOLING_OUTPUT              => "/docs/options/tooling_output.md",
        TOOLING_SHOW_INTERNAL_DATA  => "/docs/options/tooling_show_internal_data.md",
        TYPES_OUTPUT                => "/docs/options/types_output.md",
        TYPESCRIPT                  => "/docs/options/typescript.md",
        TYPESCRIPT_MAX_TUPLE_LENGTH => "/docs/options/typescript_max_tuple_length.md",
        WRITE_CHECKS                => "/docs/options/write_checks.md",
        YIELD_TYPE                  => "/docs/options/yield_type.md",
    }
}

pub mod primitives {
    file_constants! {
        BOOLEAN      => "/docs/primitives/boolean.md",
        CFRAME       => "/docs/primitives/cframe.md",
        COLORS       => "/docs/primitives/colors.md",
        DATETIME     => "/docs/primitives/datetime.md",
        FLOAT        => "/docs/primitives/float.md",
        INSTANCE     => "/docs/primitives/instance.md",
        INT_SIGNED   => "/docs/primitives/int_signed.md",
        INT_UNSIGNED => "/docs/primitives/int_unsigned.md",
        STRING       => "/docs/primitives/string.md",
        VECTORS      => "/docs/primitives/vectors.md",
    }
}
