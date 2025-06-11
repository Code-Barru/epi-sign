use crate::misc::{get_router, start_logger, start_server};

mod api;
pub mod misc;
mod schema;

#[tokio::main]
async fn main() {
    start_logger();
    start_server(get_router()).await;
}
