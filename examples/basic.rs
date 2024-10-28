use std::{fmt::Debug, marker::PhantomData};

use buildit::Builder;

// TODO: field with multiple generics and lifetimes
#[derive(Builder, Debug)]
pub struct Config<'a, 'b, T, K, V>
where
    T: Sized + Debug,
{
    // #[builder(skip)]
    pub u: &'a T,
    pub log: LogRecord<'a, 'b, K, V>,
}

#[derive(Debug)]
pub struct LogRecord<'a, 'b, K, V> {
    _key: K,
    _v: V,
    _k: PhantomData<&'a K>,
    _vv: PhantomData<&'b V>,
}

fn main1() {
    // let builder = Config::<u8, &'static str, ()>::builder()
    let builder = ConfigBuilder::new()
        .u(&42)
        .log(LogRecord {
            _key: 42,
            _v: "test",
            _k: Default::default(),
            _vv: Default::default(),
        })
        .build();
    dbg!(&builder);
}

#[derive(Builder, Debug)]
pub struct Config2 {
    pub u: String,
    pub port: u16,
}

fn main2() {
    let builder = Config2::builder().port(42).u("test".to_owned()).build();
    dbg!(&builder);
}

#[derive(Builder, Debug)]
pub struct Config3<T: Debug> {
    pub u: T,
    pub port: u16,
}

fn main3() {
    let builder = Config3::<u8>::builder().u(42).port(10).build();
    dbg!(&builder);
}

fn main() {
    main1();
    main2();
    main3();
}
