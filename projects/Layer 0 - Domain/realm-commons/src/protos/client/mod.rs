// pub mod gen;
pub mod messages;

pub mod client {
    include!(concat!(env!("OUT_DIR"), "/client/mod.rs"));
}

pub mod models {
    include!(concat!(env!("OUT_DIR"), "/models/mod.rs"));
}

