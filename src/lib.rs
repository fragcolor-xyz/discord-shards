use ctor::ctor;
use shards::core::{init, registerShard};

mod shards_impl;

#[ctor]
fn register_shards() {
    use shards_impl::bot::BotShard;

    init();
    registerShard::<BotShard>();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
