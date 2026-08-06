use chrono::{DateTime, NaiveDateTime, Utc};
use tera::{Error, Kwargs, State, Value};

pub fn tofixed(value: &Value, kwargs: Kwargs, _state: &State) -> Result<Value, Error> {
    let precision = kwargs.get::<usize>("precision").unwrap_or(Some(2)).unwrap_or(2);

    if let Some(f) = value.as_f64() {
        Ok(Value::from(format!("{:.precision$}", f, precision = precision)))
    } else if let Some(s) = value.as_str() {
        if let Ok(f) = s.parse::<f64>() {
            Ok(Value::from(format!("{:.precision$}", f, precision = precision)))
        } else {
            Err(Error::message("String não pode ser convertida para número"))
        }
    } else {
        Err(Error::message("Filtro tofixed só funciona com números ou strings numéricas"))
    }
}

pub fn format_date(value: &Value, _kwargs: Kwargs, _state: &State) -> Result<Value, Error> {
    let data_str = value.as_str().ok_or_else(|| Error::message("Valor não é uma string"))?;

    let input_format = "%Y-%m-%d %H:%M:%S%.f";
    let output_format = "%d/%m/%Y %H:%M:%S%.f";

    if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(data_str, "%Y-%m-%d") {
        return Ok(Value::from(naive_date.format("%d/%m/%Y").to_string()));
    }

    match NaiveDateTime::parse_from_str(data_str, input_format) {
        Ok(naive_dt) => {
            let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
            let formatted = dt_utc.format(output_format).to_string();
            Ok(Value::from(formatted))
        }
        Err(_) => Err(Error::message(format!(
            "Não foi possível parsear a data: {}",
            data_str
        ))),
    }
}
