use serde::{Serialize, Deserialize};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Categories { 
    pub id: i64,
    pub category_type: String,
    pub category: String,
    pub user_id: i64,
    pub created_date: Option<PrimitiveDateTime>
}