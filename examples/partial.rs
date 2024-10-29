use buildit::Builder;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug)]
enum Proto {
    Http,
    Https,
}

#[derive(Builder)]
struct Source<'a> {
    url: &'a str,
    proto: Proto,
}

fn https_builder() -> impl SourceBuilderState + SourceBuilderGetProto {
    SourceBuilder::new().proto(Proto::Https)
}

fn main() {
    let builder = https_builder();
    let source = builder.url("test").build();
    assert_eq!(source.proto, Proto::Https);
    assert_eq!(source.url, "test");
}
