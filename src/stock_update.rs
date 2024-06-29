use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use time::{PrimitiveDateTime, OffsetDateTime};

use entity::stocks::Stocks;

#[derive(Serialize, Deserialize, Debug)]
struct BrApi {
    indexes: Vec<Indexes>,
    stocks: Vec<Stocks>,
    #[serde(alias = "availableSectors")]
    available_sectors: Vec<String>,
    #[serde(alias = "availableStockType")]
    available_stock_type: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Indexes {
    stock: String,
    name: String
}

pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    let now = OffsetDateTime::now_utc();

    println!("{}", PrimitiveDateTime::new(now.date(), now.time()));

    // TODO improve connection
    let conn = MySqlPoolOptions::new()
        // .max_connections(5)
        .connect(std::env::var_os("DATABASE_URL").unwrap().to_str().unwrap()).await?;

    let resp = reqwest::get("https://brapi.dev/api/quote/list")
    // let resp = reqwest::get("http://localhost/api.json")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let indexes: Vec<Stocks> = serde_json::from_value(resp.get("stocks").unwrap().clone())?;
    
    for data in indexes.iter() {
        let stock: Option<Stocks> = match sqlx::query_as!(Stocks,
            "SELECT * FROM stocks WHERE stock = ?",
            data.stock
        )
        .fetch_one(&conn)
        .await {
            Ok(result) => Some(result),
            Err(..) => None
        };

        if let Some(s) = stock {
            sqlx::query!("UPDATE stocks
                SET stock=?, name=?, `close`=?, `change`=?, volume=?, market_cap=?, logo=?, sector=?, `type`=?
                WHERE id=?;",
                data.stock, data.name, data.close, data.change, data.volume, data.market_cap, data.logo, data.sector, data.r#type, s.id)
                .execute(&conn).await.unwrap();
        } else {
            sqlx::query!("INSERT INTO stocks
                (stock, name, `close`, `change`, volume, market_cap, logo, sector, `type`)
                VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?);",
                data.stock, data.name, data.close, data.change, data.volume, data.market_cap, data.logo, data.sector, data.r#type)
                .execute(&conn).await.unwrap();
        }
    }
    
    Ok(())
}