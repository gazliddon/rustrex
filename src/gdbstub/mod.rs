use std::net::{TcpListener, TcpStream};

struct GdbStub {
    port : u16,
    host : String,
    listener : TcpListener
}

pub impl GdbStub {

    fn new(host : &String, port : u16) -> Self {
        let connect_str = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&connect_str).unwrap();

        GdbStub {
            port: port,
            host:  host,
            listener: listener,
        }
    }
}
