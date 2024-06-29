use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;

use serde_json::json;

use entity::{stocks::Stocks, investments::Investment};
use crate::user_routes::AuthenticatedUser;
use crate::db::Logs;

#[get("/add_investment?<id>&<stock>")]
pub async fn add_investment(mut db: Connection<Logs>, _user: AuthenticatedUser, id: Option<i64>, stock: Option<String>) -> Template {
    if id.is_some() {
        let investment = sqlx::query_as!(Investment,
            "SELECT id, stock, name, quantity, user_id
            FROM investments where id = ?",
            id.unwrap()
        )
        .fetch_one(db.as_mut())
        .await.unwrap();

        return Template::render("pages/investment/save_investment", json!({"investment": investment}))
    } else if stock.is_some() {
        let stock = sqlx::query_as!(Stocks,
            "SELECT id, stock, name, `close`, `change`, volume, market_cap, logo, sector, `type`
            FROM stocks
            where stock = ?",
            stock
        )
        .fetch_one(db.as_mut())
        .await.unwrap();

        return Template::render("pages/investment/save_investment", json!({"investment": stock}))
    }

    Template::render("pages/investment/search_investment", json!({}))
}

#[get("/search_investment?<stock>")]
pub async fn search_investment(mut db: Connection<Logs>, _user: AuthenticatedUser, stock: String) -> Template {
    let stocks = sqlx::query_as!(Stocks,
                "SELECT id, stock, name, `close`, `change`, volume, market_cap, logo, sector, `type`
                FROM stocks
                where stock like ? or name like ?
                LIMIT 15",
                format!("%{}%", stock), format!("%{}%", stock)
            )
            .fetch_all(db.as_mut())
            .await.unwrap();

    Template::render("pages/investment/list_stocks", json!({"stocks": stocks}))
}

#[get("/get_investment?<id>&<stock>")]
pub async fn get_investment(mut db: Connection<Logs>, _user: AuthenticatedUser, id: Option<i64>, stock: String) -> Template {
    if id.is_some() {
        let investment = sqlx::query_as!(Investment,
            "SELECT id, stock, name, quantity, user_id
            FROM investments where id = ?",
            id.unwrap()
        )
        .fetch_one(db.as_mut())
        .await.unwrap();

        return Template::render("pages/investment/save_investment.html", json!({"investment": investment}))
    }

    let stock = sqlx::query_as!(Stocks,
            "SELECT id, stock, name, `close`, `change`, volume, market_cap, logo, sector, `type`
            FROM stocks
            where stock = ?",
            stock
        )
        .fetch_one(db.as_mut())
        .await.unwrap();

    Template::render("pages/investment/save_investment.html", json!({"investment": stock}))
}

#[post("/save_investment?<id>&<stock>&<quantity>")]
pub async fn save_investment(mut db: Connection<Logs>, id: Option<i64>, user: AuthenticatedUser, stock: String, quantity: i64) -> String {
    let stock = sqlx::query_as!(Stocks,
            "SELECT id, stock, name, `close`, `change`, volume, market_cap, logo, sector, `type`
            FROM stocks
            where stock = ?",
            stock
        )
        .fetch_one(db.as_mut())
        .await.unwrap();

    if id.is_some() {
        println!("TODO update investment {}", id.unwrap());
        // TODO update investment
    } else {
        sqlx::query!("INSERT INTO investments
            (stock, name, quantity, user_id)
            VALUES(?, ?, ?, ?)",
            stock.stock, stock.name, quantity, user.user_id)
            .execute(db.as_mut()).await.unwrap();
    }

    "ok".to_string()
}