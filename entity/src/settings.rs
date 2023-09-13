use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Settings { 
    pub user_id: i64,
    pub budget: Option<f64>,
}