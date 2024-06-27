use std::fmt::Debug;

use buildit::Builder;

#[derive(Builder)]
pub struct Config<'a, T>
where
    T: Sized + Debug,
{
    #[builder(skip)]
    pub u: &'a T,
    pub log: bool,
}

fn main1() {
    let builder = Config::<u8>::builder();
    dbg!(&builder);
}

#[derive(Builder)]
pub struct Config2 {
    #[builder(skip)]
    pub u: String,
    pub port: u16,
}

fn main2() {
    let builder = Config2::builder();
    dbg!(&builder);
}

#[derive(Builder)]
pub struct Config3<T: Debug> {
    pub u: T,
    pub port: u16,
}

fn main3() {
    let builder = Config3::<u8>::builder();
    dbg!(&builder);
}

fn main() {
    main1();
    main2();
    main3();
}
