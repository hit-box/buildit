use buildit::Builder;

trait Backend {}
#[derive(Debug)]
struct Mem {}
impl Backend for Mem {}

#[allow(dead_code)]
#[derive(Builder, Debug)]
struct Cache<B>
where
    B: Backend,
{
    backend: B,
    enabled: bool,
    enabled1: bool,
}

fn main() {
    let mem = Mem {};
    let cache = CacheBuilder::new()
        .backend(mem)
        .enabled1(false)
        .enabled(true)
        .build();
    dbg!(cache);
}
