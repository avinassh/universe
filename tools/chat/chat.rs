#[macro_use]
extern crate flags;
use irc::IrcServer;

fn main() {
    let grpc_port = define_flag!("grpc_port", 6668, "The gRPC port to bind to");
    parse_flags!(grpc_port);

    let handler = chat_service::ChatServiceHandler::new();

    let mut server = grpc::ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    server.http.set_port(grpc_port.value());
    server.http.set_cpu_pool_threads(2);
    server.add_service(chat_grpc_rust::ChatServiceServer::new_service_def(
        handler.clone(),
    ));
    let _server = server.build().unwrap();

    let listener = std::net::TcpListener::bind("127.0.0.1:6667").unwrap();
    for stream in listener.incoming() {
        let h = handler.clone();
        std::thread::spawn(move || {
            let server = IrcServer::new(h);
            server.handle_client(stream.unwrap());
        });
    }

    loop {
        std::thread::park();
    }
}
