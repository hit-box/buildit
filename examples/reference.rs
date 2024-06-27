use builder_a::{ABuild, ABuilderSetHost, ABuilderSetLogging, ABuilderSetPort};

#[derive(Debug)]
pub struct A {
    pub logging: bool,
    pub port: u16,
    pub host: String,
}

mod builder_a {
    use super::A;

    impl A {
        pub fn builder() -> ABuilderInitial {
            ABuilderInitial {}
        }
    }

    // initial
    pub trait ABuilderState {}

    #[derive(Debug)]
    pub struct ABuilderInitial {}

    impl ABuilderState for ABuilderInitial {}

    // logging
    #[derive(Debug)]
    pub struct ABuilderLoggingSet<I: ABuilderState> {
        logging: bool,
        inner: I,
    }

    impl<I: ABuilderState> ABuilderState for ABuilderLoggingSet<I> {}

    pub trait ABuilderSetLogging: Sized + ABuilderState {
        fn logging(self, logging: bool) -> ABuilderLoggingSet<Self> {
            ABuilderLoggingSet {
                logging,
                inner: self,
            }
        }
    }

    impl<T: ABuilderState> ABuilderSetLogging for T {}

    pub trait ABuilderGetLogging {
        fn get_logging(&self) -> bool;
    }

    impl<I: ABuilderState> ABuilderGetLogging for ABuilderLoggingSet<I> {
        fn get_logging(&self) -> bool {
            self.logging
        }
    }

    impl<T: ABuilderState + ABuilderGetPort> ABuilderGetPort for ABuilderLoggingSet<T> {
        fn get_port(&mut self) -> u16 {
            self.inner.get_port()
        }
    }

    impl<T: ABuilderState + ABuilderGetHost> ABuilderGetHost for ABuilderLoggingSet<T> {
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

    impl<T: ABuilderState + ABuilderGetLogging> ABuilderGetLogging for ABuilderPortSet<T> {
        fn get_logging(&self) -> bool {
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

    pub trait ABuilderGetHost {
        fn get_host(&mut self) -> String;
    }

    impl<I: ABuilderState> ABuilderGetHost for ABuilderHostSet<I> {
        fn get_host(&mut self) -> String {
            self.host.take().expect("@TODO")
        }
    }

    impl<T: ABuilderState + ABuilderGetLogging> ABuilderGetLogging for ABuilderHostSet<T> {
        fn get_logging(&self) -> bool {
            self.inner.get_logging()
        }
    }

    impl<T: ABuilderState + ABuilderGetPort> ABuilderGetPort for ABuilderHostSet<T> {
        fn get_port(&mut self) -> u16 {
            self.inner.get_port()
        }
    }

    // builder finalize
    pub trait ABuild: ABuilderState {
        fn build(self) -> A;
    }

    impl<T: ABuilderState + ABuilderGetLogging + ABuilderGetPort + ABuilderGetHost> ABuild for T {
        fn build(mut self) -> A {
            A {
                logging: self.get_logging(),
                port: self.get_port(),
                host: self.get_host(),
            }
        }
    }
}

fn main() {
    let state = A::builder()
        .logging(true)
        .port(8080)
        .host("localhost".to_owned())
        .port(443)
        .logging(false);
    dbg!(&state);
    let a = state.build();
    dbg!(&a);
}
