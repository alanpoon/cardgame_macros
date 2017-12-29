#[macro_use]
pub mod game_engine;
#[macro_use]
pub mod codec_serde;
#[macro_use]
pub mod resources;
#[macro_use]
pub mod resources_iter;
pub mod hello;

#[cfg(test)]
mod tests {
    use hello as t;
    #[test]
    fn it_works() {
        test_path!{};
    }
}
//cargo test -- --nocapture
