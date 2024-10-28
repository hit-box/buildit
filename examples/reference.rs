use builder_a::{ABuilderInitial, ABuilderSetHost, ABuilderSetPort, ABuilderState};

#[derive(Debug)]
pub struct A<T> {
    pub logging: Option<T>,
    pub port: u16,
    pub host: String,
}

mod builder_a {
    use super::A;

    impl<T> A<T> {
        pub fn builder() -> ABuilderInitial {
            ABuilderInitial {}
        }
    }

    // initial
    pub trait ABuilderState {
        fn build<T>(mut self) -> A<T>
        where
            Self: ABuilderGetLogging<T> + ABuilderGetPort + ABuilderGetHost + Sized,
        {
            A {
                logging: self.get_logging(),
                port: self.get_port(),
                host: self.get_host(),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct ABuilderInitial {}

    impl ABuilderState for ABuilderInitial {}

    // logging
    #[derive(Debug)]
    pub struct ABuilderLoggingSet<T, I: ABuilderState> {
        logging: Option<Option<T>>,
        inner: I,
    }

    impl<T, I: ABuilderState> ABuilderState for ABuilderLoggingSet<T, I> {}

    pub trait ABuilderSetLogging: Sized + ABuilderState {
        fn logging<T>(self, logging: Option<T>) -> ABuilderLoggingSet<T, Self> {
            ABuilderLoggingSet {
                logging: Some(logging),
                inner: self,
            }
        }
    }

    impl<T: ABuilderState> ABuilderSetLogging for T {}

    #[diagnostic::on_unimplemented(
        message = "LOGGING My Message for `SourceBuilder` is not implemented for `{Self}`",
        label = "My Label",
        note = "Note 1",
        note = "Note 2"
    )]
    pub trait ABuilderGetLogging<T> {
        fn get_logging(&mut self) -> Option<T>;
    }

    impl<T, I: ABuilderState> ABuilderGetLogging<T> for ABuilderLoggingSet<T, I> {
        fn get_logging(&mut self) -> Option<T> {
            self.logging.take().expect("@TODO")
        }
    }

    impl<T, I: ABuilderState + ABuilderGetPort> ABuilderGetPort for ABuilderLoggingSet<T, I> {
        fn get_port(&mut self) -> u16 {
            self.inner.get_port()
        }
    }

    impl<T, I: ABuilderState + ABuilderGetHost> ABuilderGetHost for ABuilderLoggingSet<T, I> {
        fn get_host(&mut self) -> String {
            self.inner.get_host()
        }
    }

    // port
    #[derive(Debug)]
    pub struct ABuilderPortSet<I: ABuilderState> {
        port: Option<u16>,
        inner: I,
    }

    impl<I: ABuilderState> ABuilderState for ABuilderPortSet<I> {}

    pub trait ABuilderSetPort: Sized + ABuilderState {
        fn port(self, port: u16) -> ABuilderPortSet<Self> {
            ABuilderPortSet {
                port: Some(port),
                inner: self,
            }
        }
    }

    impl<T: ABuilderState> ABuilderSetPort for T {}

    pub trait ABuilderGetPort {
        fn get_port(&mut self) -> u16;
    }

    impl<I: ABuilderState> ABuilderGetPort for ABuilderPortSet<I> {
        fn get_port(&mut self) -> u16 {
            self.port.take().expect("@TODO")
        }
    }

    impl<T, I: ABuilderState + ABuilderGetLogging<T>> ABuilderGetLogging<T> for ABuilderPortSet<I> {
        fn get_logging(&mut self) -> Option<T> {
            self.inner.get_logging()
        }
    }

    impl<T: ABuilderState + ABuilderGetHost> ABuilderGetHost for ABuilderPortSet<T> {
        fn get_host(&mut self) -> String {
            self.inner.get_host()
        }
    }

    // host
    #[derive(Debug)]
    pub struct ABuilderHostSet<I: ABuilderState> {
        host: Option<String>,
        inner: I,
    }

    impl<I: ABuilderState> ABuilderState for ABuilderHostSet<I> {}

    pub trait ABuilderSetHost: Sized + ABuilderState {
        fn host(self, host: String) -> ABuilderHostSet<Self> {
            ABuilderHostSet {
                host: Some(host),
                inner: self,
            }
        }
    }

    impl<T: ABuilderState> ABuilderSetHost for T {}

    #[diagnostic::on_unimplemented(
        message = "HOST My Message for `SourceBuilder` is not implemented for `{Self}`",
        label = "My Label",
        note = "Note 1",
        note = "Note 2"
    )]
    pub trait ABuilderGetHost {
        fn get_host(&mut self) -> String;
    }

    impl<I: ABuilderState> ABuilderGetHost for ABuilderHostSet<I> {
        fn get_host(&mut self) -> String {
            self.host.take().expect("@TODO")
        }
    }

    impl<T, I: ABuilderState + ABuilderGetLogging<T>> ABuilderGetLogging<T> for ABuilderHostSet<I> {
        fn get_logging(&mut self) -> Option<T> {
            self.inner.get_logging()
        }
    }

    impl<T: ABuilderState + ABuilderGetPort> ABuilderGetPort for ABuilderHostSet<T> {
        fn get_port(&mut self) -> u16 {
            self.inner.get_port()
        }
    }

    // builder finalize
    // pub trait ABuild: ABuilderState {
    //     fn build<T>(self) -> A<T>;
    // }
    //
    // impl<I: ABuilderState + ABuilderGetLogging + ABuilderGetPort + ABuilderGetHost> ABuild for I {
    //     fn build<T>(mut self) -> A<T> {
    //         A {
    //             logging: self.get_logging(),
    //             port: self.get_port(),
    //             host: self.get_host(),
    //         }
    //     }
    // }
}

fn main() {
    // let state = A::builder()
    let state = ABuilderInitial::default()
        // .logging(Some(true))
        .port(8080)
        .host("localhost".to_owned())
        .port(443);
    // .logging::<bool>(None);
    let a = state.build();
    dbg!(a);
}
