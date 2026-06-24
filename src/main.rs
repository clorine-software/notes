use anyhow::Result;

mod lua;
mod data;
mod parser;
mod var;
mod idsb32c6;

#[tokio::main]
async fn main() -> Result<()> {
    parser::parse().await
}

