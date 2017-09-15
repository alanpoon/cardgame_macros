#[macro_use]
pub mod game_engine;
#[macro_use]
pub mod app_serde;
#[macro_use]
pub mod resources;
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
