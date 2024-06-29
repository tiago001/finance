use serde::{Serialize, Deserialize};
use sqlx::types::BigDecimal;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Stocks {
    pub id: Option<i64>,
    pub stock: String,
    pub name: String,
    pub close: BigDecimal,
    pub change: Option<BigDecimal>,
    pub volume: i64,
    pub market_cap: Option<BigDecimal>,
    pub logo: String,
    pub sector: Option<String>,
    pub r#type: String
}