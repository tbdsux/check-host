use std::error::Error;
use tabled::builder::Builder;

use checkhost;

pub fn check_http(host: &str, nodes: u8) -> Result<Builder, Box<dyn Error>> {
    let check_result = checkhost::check_http(host, nodes)?;

    let mut builder = Builder::default();

    let table_headers: Vec<String> = vec![
        "Location".to_string(),
        "Time".to_string(),
        "Result".to_string(),
        "Code".to_string(),
        "IP Address".to_string(),
    ];
    builder.set_columns(table_headers);

    for (key, value) in check_result {
        match value {
            Some(value) => {
                let x = &value[0];
                let mut it = vec![key];

                let time = &x[1];
                let result = &x[2];
                let code = &x[3];
                let ip_address = &x[4];

                it.push(format!(
                    "{:.2}s",
                    (time.as_f64().unwrap_or(0.0) * 100.0).round() / 100.0
                ));
                it.push(result.to_string());
                it.push(code.to_string());
                it.push(ip_address.to_string());

                builder.add_record(it);
            }
            None => {
                builder.add_record(vec![
                    key,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ]);
            }
        }
    }

    Ok(builder)
}
