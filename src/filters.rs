use rocket_dyn_templates::tera::{Value, Error, Result};
use std::collections::HashMap;

use chrono::{NaiveDateTime, DateTime, Utc};

pub fn tofixed(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let precision = match args.get("precision") {
        Some(Value::Number(n)) => n.as_u64().unwrap_or(2) as usize,
        _ => 2,
    };

    match value {
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::String(format!("{:.precision$}", f, precision = precision)))
            } else if let Some(i) = n.as_i64() {
                Ok(Value::String(format!("{:.precision$}", i as f64, precision = precision)))
            } else {
                Err(Error::msg("Valor numérico inválido"))
            }
        }
        Value::String(s) => {
            if let Ok(f) = s.parse::<f64>() {
                Ok(Value::String(format!("{:.precision$}", f, precision = precision)))
            } else {
                Err(Error::msg("String não pode ser convertida para número"))
            }
        }
        _ => Err(Error::msg("Filtro tofixed só funciona com números ou strings numéricas"))
    }
}

pub fn format_date(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let data_str = value.as_str().ok_or_else(|| Error::msg("Valor não é uma string"))?;
    
    let input_format = "%Y-%m-%d %H:%M:%S%.f";
    let output_format = "%d/%m/%Y %H:%M:%S%.f";

    match NaiveDateTime::parse_from_str(data_str, input_format) {
        Ok(naive_dt) => {
            let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
            let formatted = dt_utc.format(output_format).to_string();
            Ok(Value::String(formatted))
        }
        Err(_) => {
            Err(Error::msg(format!("Não foi possível parsear a data: {}", data_str)))
        }
    }
}
