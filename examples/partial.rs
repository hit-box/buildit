use std::collections::HashMap;

use buildit::Builder;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Default)]
enum Proto {
    Http,
    #[default]
    Https,
}

#[derive(Builder, Debug)]
struct Source<'a, T> {
    url: &'a str,
    // valid
    #[builder(default)]
    // #[builder(default(expr = Proto::Http))]
    // #[builder(default = Proto::Http)]
    // invalid
    // #[builder(default = "42")]
    proto: Proto,
    // #[builder(default)]
    #[builder(default)]
    map: HashMap<u8, T>,
}

fn https_builder() -> impl SourceBuilderState + SourceBuilderGetProto {
    SourceBuilder::new().proto(Proto::Https)
}

fn main() {
    // let builder = https_builder();
    let builder = SourceBuilder::new();
    let source: Source<u8> = builder.url("test").build();
    assert_eq!(source.proto, Proto::Https);
    assert_eq!(source.url, "test");
    dbg!(&source);
}
