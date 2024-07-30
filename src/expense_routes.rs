use std::ops::{Sub, Add};

use chrono::{DateTime, Utc, Datelike};
use entity::expense_view::ExpenseView as ExpenseView;
use rocket_db_pools::{sqlx, Connection};
use serde::Serialize;
use time::{PrimitiveDateTime, OffsetDateTime};

use crate::user_routes::AuthenticatedUser;
use crate::db::Logs;

#[post("/save_expense?<name>&<value>&<category_id>&<date>")]
pub async fn save_expense(mut db: Connection<Logs>, name: &str, value: f64, category_id: &str, date: &str, user: AuthenticatedUser) -> String {
    let now = OffsetDateTime::now_utc(); //.to_offset(offset!(-3))

    sqlx::query!("INSERT INTO expenses
        (name, value, category_id, date, user_id, created_date)
        VALUES(?, ?, ?, ?, ?, ?)",
        name, value, category_id, date, user.user_id, PrimitiveDateTime::new(now.date(), now.time()))
        .execute(db.as_mut()).await.unwrap();

    "ok".to_string()
}

#[post("/edit_expense?<id>&<name>&<value>&<category_id>&<date>")]
pub async fn edit_expense(mut db: Connection<Logs>,id: i64, name: Option<&str>, value: Option<f64>, category_id: Option<i64>, date: Option<&str>, user: AuthenticatedUser) -> String {

    if name.is_some() && value.is_some() && category_id.is_some() && date.is_some() {
        sqlx::query!("UPDATE expenses SET name = ?, value = ?, category_id = ?, `date` = ? WHERE id = ? and user_id = ?",
            name, value, category_id, date, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if name.is_some() {
        sqlx::query!("UPDATE expenses SET name = ? WHERE id = ? and user_id = ?",
            name, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if value.is_some() {
        sqlx::query!("UPDATE expenses SET value = ? WHERE id = ? and user_id = ?",
            value, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if category_id.is_some() {
        sqlx::query!("UPDATE expenses SET category_id = ? WHERE id = ? and user_id = ?",
            category_id, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if date.is_some() {
        sqlx::query!("UPDATE expenses SET date = ? WHERE id = ? and user_id = ?",
            date, id, user.user_id).execute(db.as_mut()).await.unwrap();
    }

    "ok".to_string()
}

#[get("/get_expense?<id>")]
pub async fn get_expense(mut db: Connection<Logs>, id: i64, user: AuthenticatedUser) -> String {
    let stream = sqlx::query_as!(ExpenseView,
        "SELECT * FROM expenses_view WHERE user_id = ? AND id = ?",
        user.user_id, id
    )
    .fetch_one(db.as_mut())
    .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses?<name>&<value1>&<value2>&<category>")]
pub async fn search_expenses(mut db: Connection<Logs>, name: &str, category: Option<&str>, value1: Option<&str>, value2: Option<&str>, user: AuthenticatedUser) -> String {

    let mut stream: Vec<ExpenseView> = vec![];
    if name == "category" {
        if category == Some("Indefinido") {
            stream = sqlx::query_as!(ExpenseView,
                "SELECT * FROM expenses_view WHERE `date` between ? and ? and user_id = ? ORDER BY date DESC, id DESC",
                value1,
                value2,
                user.user_id
            )
            .fetch_all(db.as_mut())
            .await.unwrap();
        } else {
            stream = sqlx::query_as!(ExpenseView,
                "SELECT * FROM expenses_view WHERE `date` between ? and ? and category = ? and user_id = ? ORDER BY date DESC, id DESC",
                value1,
                value2,
                category,
                user.user_id
            )
            .fetch_all(db.as_mut())
            .await.unwrap();
        }
    } else if name == "currentMonth" {
        stream = sqlx::query_as!(ExpenseView, "SELECT * FROM expenses_view WHERE user_id = ? AND MONTH(`date`) = MONTH(now()) AND YEAR(`date`) = YEAR(now()) ORDER BY date DESC, id DESC", user.user_id)
        .fetch_all(db.as_mut())
        .await.unwrap();
    } else if name == "lastExpenses" {
        stream = sqlx::query_as!(ExpenseView, "SELECT * FROM expenses_view WHERE user_id = ? ORDER BY date desc,created_date desc LIMIT 100", user.user_id)
        .fetch_all(db.as_mut())
        .await.unwrap();
    } else if name == "lastAddedExpenses" {
        stream = sqlx::query_as!(ExpenseView, "SELECT * FROM expenses_view WHERE user_id = ? ORDER BY created_date desc LIMIT 100", user.user_id)
        .fetch_all(db.as_mut())
        .await.unwrap();
    }

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses_category?<value1>&<value2>")]
pub async fn search_expenses_category(mut db: Connection<Logs>, user: AuthenticatedUser, value1: &str, value2: &str) -> String {
    #[derive(Serialize, Debug, Clone)]
    struct Record {category: Option<String>, sum: Option<f64>, month: Option<i64>}

    #[derive(Serialize, Debug, Clone)]
    struct ExpensesCategory {category: Option<String>, months: Vec<MonthExpenses>}
    #[derive(Serialize, Debug, Clone)]
    struct MonthExpenses{sum: Option<f64>, month: Option<i64>, year: i64}
    #[derive(Serialize, Debug, Clone)]
    struct Return{categories: Vec<ExpensesCategory>, months: Vec<Months>}
    #[derive(Serialize, Debug, Clone)]
    struct Months{month: i64, year: i64, sum: f64}

    let stream = sqlx::query_as!(Record, "SELECT sum(value) as sum, category, month(`date`) as month
        FROM expenses_view WHERE `date` between ? and ? and user_id = ? group by category,month(`date`) order by 3,2",
        value1, value2, user.user_id)
        .fetch_all(db.as_mut())
        .await.unwrap();

    let mut expenses: Vec<ExpensesCategory> = vec![];
    let mut months: Vec<Months> = vec![];
    
    for s in stream.clone().into_iter() {
        let mut utc: DateTime<Utc> = Utc::now().sub(chrono::Months::new(3));

        let expense = expenses.iter_mut().find(|e| e.category == s.category);
        if expense.is_none() {
            let mut e = ExpensesCategory{ category: s.category, months: Vec::new()};
            for _ in 0..3 {
                utc = utc.add(chrono::Months::new(1));
                e.months.push(MonthExpenses{sum: Some(0.0), month: Some(utc.month() as i64), year: utc.year() as i64});
                if months.len() < 3 {
                    months.push(Months{month: utc.month() as i64, year: utc.year() as i64, sum: 0.0});
                }
            }
            
            expenses.push(e);
        }
    }

    for s in stream.into_iter() {
        let expense = expenses.iter_mut().find(|e| e.category == s.category);
        if expense.is_some() {
            let expense_month = expense.unwrap().months.iter_mut().find(|m| m.month == s.month);

            expense_month.unwrap().sum = s.sum;
        }

        let month = months.iter_mut().find(|m| m.month == s.month.unwrap());
        if month.is_some() {
            month.unwrap().sum += s.sum.unwrap();
        }
    }

    let retorno = Return{categories: expenses, months};

    serde_json::to_string(&retorno).unwrap()
}

#[post("/delete_expense?<id>")]
pub async fn delete_expense(mut db: Connection<Logs>,id: i64, user: AuthenticatedUser) -> String {

    sqlx::query!("DELETE from expenses WHERE id = ? and user_id = ?",
        id, user.user_id).execute(db.as_mut()).await.unwrap();

    "ok".to_string()
}

#[get("/get_balance?<months>")]
pub async fn get_balance(mut db: Connection<Logs>, months: u32, user: AuthenticatedUser) -> String {
    #[derive(Serialize, Debug, Clone)]
    struct Balance {value: Option<f64>, month: Option<i64>, year: Option<i64>, balance_type: String}

    let stream = sqlx::query_as!(Balance,
        "SELECT sum(value) as value, month(`date`) as month, year(`date`) as year, 'incomes' as balance_type
        FROM incomes 
        where user_id = ?
        group by 2,3
        UNION all
        SELECT sum(value) as value, month(`date`) as month, year(`date`) as year, 'expenses' as balance_type
        FROM expenses 
        where user_id = ?
        group by 2,3
        order by 3 desc, 2 desc;",
        user.user_id, user.user_id
    )
    .fetch_all(db.as_mut())
    .await.unwrap();

    let mut utc: DateTime<Utc> = Utc::now().sub(chrono::Months::new(months-1));

    let mut expenses: Vec<Balance> = Vec::new();
    let mut incomes: Vec<Balance> = Vec::new();
    let mut balance: Vec<Balance> = Vec::new();
    let mut labels: Vec<String> = Vec::new();

    for _ in 0..months {
        let expense = stream.iter().find(|s| 
            s.month.unwrap() == utc.month() as i64 && 
            s.year.unwrap() == utc.year() as i64 &&
            s.balance_type == "expenses"
        );
        
        if let Some(e) = expense {
            expenses.push(e.clone());
        } else {
            expenses.push(Balance { value: Some(0.0), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "expenses".to_string() })
        }

        let income = stream.iter().find(|s| 
            s.month.unwrap() == utc.month() as i64 && 
            s.year.unwrap() == utc.year() as i64 &&
            s.balance_type == "incomes"
        );
        if let Some(i) = income {
            incomes.push(i.clone());
        } else {
            incomes.push(Balance { value: Some(0.0), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "incomes".to_string() })
        }

        if let (Some(e), Some(i)) = (expense, income) {
            balance.push(Balance { value: Some(i.value.unwrap() - e.value.unwrap()), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "balance".to_string() })
        } else if let Some(e) = expense {
            balance.push(Balance { value: Some(-(e.value.unwrap())), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "balance".to_string() })
        } else if let Some(i) = income {
            balance.push(Balance { value: Some(i.value.unwrap()), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "balance".to_string() })
        } else {
            balance.push(Balance { value: Some(0.0), month: Some(utc.month() as i64), year: Some(utc.year() as i64), balance_type: "balance".to_string() })
        }

        labels.push(format!("{}-{}", utc.month(), utc.year()));

        utc = utc.add(chrono::Months::new(1));
    }

    #[derive(Serialize, Debug, Clone)]
    struct Return{expenses: Vec<Balance>, incomes: Vec<Balance>, balance: Vec<Balance>, labels: Vec<String>}

    let retorno = Return{ expenses, incomes, balance, labels};

    serde_json::to_string(&retorno).unwrap()

    // "ok".to_string()
}