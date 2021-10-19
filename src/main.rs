//! Test graceful shutdowns with tonic.

use tokio::net::TcpListener;

mod proto {
    tonic::include_proto!("dummy");
}

#[derive(Clone, Default)]
struct DummyImpl;

#[tonic::async_trait]
impl proto::dummy_server::Dummy for DummyImpl {
    async fn dummy(&self, _: tonic::Request<proto::DummyRequest>) -> Result<tonic::Response<proto::DummyResponse>, tonic::Status> {
        println!("request");
        Ok(tonic::Response::new(proto::DummyResponse::default()))
    }
}


#[tokio::main]
async fn main() {
    let std_listener = listenfd::ListenFd::from_env()
        .take_tcp_listener(0)
        .expect("Failed to get TCP listener at index 0.")
        .expect("No TCP listener at index 0.");


    // IMPORTANT! not setting the tcp listener to nonblocking mode will not
    // allow the server to shut down gracefully
    std_listener.set_nonblocking(true).expect("Failed to set non-blocking mode");

    let listener = TcpListener::from_std(std_listener)
        .expect("Failed to convert standard tcp listener to a tokio tcp listener");
    let stream = tokio_stream::wrappers::TcpListenerStream::new(listener);
    tonic::transport::Server::builder()
        .add_service(proto::dummy_server::DummyServer::new(DummyImpl::default()))
        .serve_with_incoming_shutdown(stream, async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("received stop signal");
        })
        .await
        .expect("Failed to run grpc server");
}
