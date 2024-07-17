// pub mod gen;
pub mod messages;

pub mod server {
    include!(concat!(env!("OUT_DIR"), "/server/mod.rs"));
}

pub mod models {
    include!(concat!(env!("OUT_DIR"), "/models/mod.rs"));
}

