use anyhow::Result;

mod consts;
mod fsf;
mod logic;
mod parser;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(parser::parse().await?)
}
