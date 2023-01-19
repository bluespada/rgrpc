
mod protocol;


// all database data will transfer from grpc server
// so server for client just handle http to render the UI
use protocol::coin::{Coiner};
use protocol::coin::coin_server::{CoinServer};
use actix_web::{HttpServer, App, web, HttpResponse};

async fn grpc_server() -> std::io::Result<()> {
    let addr = "[::1]:8001".parse().unwrap();
    let _ = tonic::transport::Server::builder()
        .add_service(CoinServer::new(Coiner::default()))
        .serve(addr).await;
    Ok(())
}

fn main() {
    // should create main pool to approach 2 process server client run at sametime.
    // 1. Server for UI
    // 2. Server for gRPC Server
    
    // create main pool from actix web rt
    let main_pool = actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("rgrpc_pool")
            .build()
            .unwrap()
    });
    main_pool.block_on(async move {
        
        let http_server = HttpServer::new(move || {
            App::new()
            .route("/", web::get().to(|| { HttpResponse::Ok() }))
        }).bind(("127.0.0.1", 8000))
        .expect("failed to start http server")
        .run();

        let grpc_server = grpc_server();
        let _ = futures::future::try_join(http_server, grpc_server).await;
    });
}
