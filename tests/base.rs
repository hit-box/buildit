use buildit::Builder;

#[derive(Builder)]
pub struct A<'a, T>
where
    T: Sized + 'a,
{
    pub u: &'a T,
}

#[test]
fn base_builder() {
    // let _a = A { u:  };
}
