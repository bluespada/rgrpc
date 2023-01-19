
pub mod coin {
    use tonic::{Request, Response, Result, Status};

    use self::coin_server::Coin;

    tonic::include_proto!("coin");

    #[derive(Debug, Default)]
    pub struct Coiner{}

    #[tonic::async_trait]
    impl Coin for Coiner {
        async fn supply(&self, _request: Request<SupplyRequest>) -> Result<Response<SupplyResponse>, Status> {
            let reply = SupplyResponse{
                total_supply: 24000000
            };
            Ok(Response::new(reply))
        }

        async fn transaction(&self, _request: Request<ReciveRequest>) -> Result<Response<ReciveResponse>, Status> {
            let reply = ReciveResponse {
                tx_success: true,
                tx_from: 1,
                tx_id: 1,
                tx_to: 2,
                tx_total: 200,
            };
            Ok(Response::new(reply))
        }
    }
}
