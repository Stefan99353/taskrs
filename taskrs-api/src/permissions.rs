use build_const::build_const;

build_const!("permissions");

pub const ALL_PERMISSIONS: &str =
    include_str!(concat!(env!("OUT_DIR"), concat!("/permissions.json")));
