use bt_backend::SonyHeadphoneConnection;

#[tokio::main]
async fn main() {
    let connection = match SonyHeadphoneConnection::discover_and_connect().await {
        Ok(connection) => connection,
        Err(error) => panic!("Problem: {error:?}")
    };
}
