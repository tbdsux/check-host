use std::error::Error;
use tabled::builder::Builder;

use checkhost;

pub fn check_udp(host: &str, nodes: u8, wait: u8) -> Result<Builder, Box<dyn Error>> {
    let check_result = checkhost::check_udp(host, nodes, wait)?;

    let mut builder = Builder::default();
    let table_headers = vec!["Location", "Result", "IP Address"];
    builder.set_columns(table_headers);

    for (key, value) in check_result {
        match value {
            Some(value) => {
                let mut it = vec![key];

                let res = &value[0];

                match res {
                    checkhost::UdpCheckResponse::Ok {
                        address,
                        timeout: _,
                    } => {
                        // Info: Successful connect to host.

                        it.push("Open or filtered".to_string());
                        it.push(address.to_string());
                    }
                    checkhost::UdpCheckResponse::Err { error } => {
                        // Info: Connection timed out.

                        it.push(error.to_string());
                        it.push("".to_string());
                    }
                }

                builder.add_record(it);
            }
            None => {
                builder.add_record(vec![key, "".to_string(), "".to_string()]);
            }
        }
    }

    Ok(builder)
}

pub fn check_tcp(host: &str, nodes: u8, wait: u8) -> Result<Builder, Box<dyn Error>> {
    let check_result = checkhost::check_tcp(host, nodes, wait)?;

    let mut builder = Builder::default();
    let table_headers = vec!["Location", "Result", "Time", "IP Address"];
    builder.set_columns(table_headers);

    for (key, value) in check_result {
        match value {
            Some(value) => {
                let mut it = vec![key];

                let res = &value[0];

                match res {
                    checkhost::TcpCheckResponse::Ok { address, time } => {
                        // Info: Successful connect to host.

                        it.push("Connected".to_string());
                        it.push(format!("{} s", (time * 1000.0).round() / 1000.0));
                        it.push(address.to_string());
                    }
                    checkhost::TcpCheckResponse::Err { error } => {
                        // Info: Connection timed out.

                        it.push(error.to_string());
                        it.push("".to_string());
                        it.push("".to_string());
                    }
                }

                builder.add_record(it);
            }
            None => {
                builder.add_record(vec![key, "".to_string(), "".to_string(), "".to_string()]);
            }
        }
    }

    Ok(builder)
}

pub fn check_ping(host: &str, nodes: u8, wait: u8) -> Result<Builder, Box<dyn Error>> {
    let check_result = checkhost::check_ping(host, nodes, wait)?;

    let mut builder = Builder::default();

    let table_headers = vec!["Location", "Result", "rtt min / avg / max", "IP Address"];
    builder.set_columns(table_headers);

    for (key, value) in check_result {
        match value {
            Some(value) => {
                let mut it = vec![key];

                let res = &value[0];

                let first_val = res[0][0].as_null();
                match first_val {
                    Some(_first_val) => {
                        // Info: node was unable to resolve the domain.
                        it.push("Unknown host".to_string());
                        it.push("".to_string());
                        it.push("".to_string());

                        builder.add_record(it);
                        continue;
                    }
                    None => {}
                }

                let ip_address = res[0][2].as_str().unwrap();
                let mut ok_result = 0;
                let mut rtt_min = 0.0;
                let mut rtt_total = 0.0;
                let mut rtt_max = 0.0;

                for i in res {
                    let r = i[0].as_str().unwrap();
                    let time = &i[1].as_f64().unwrap();

                    if r == "OK" {
                        ok_result += 1;
                    }

                    rtt_total += time;

                    if time > &rtt_max {
                        rtt_max = *time;
                    }

                    if rtt_min == 0.0 {
                        rtt_min = *time;
                    }
                    if time < &rtt_min {
                        rtt_min = *time;
                    }
                }

                // result
                it.push(format!("{}/4", ok_result));

                // rtt
                it.push(format!(
                    "{} / {} / {} ms",
                    (rtt_min * 100000.0).round() / 100.0,
                    ((rtt_total / 4.0) * 100000.0).round() / 100.0,
                    (rtt_max * 100000.0).round() / 100.0
                ));

                // ip_address
                it.push(ip_address.to_string());

                // add record
                builder.add_record(it);
            }
            None => {
                // Info: node is still performing the check
                builder.add_record(vec![key, "".to_string(), "".to_string(), "".to_string()]);
            }
        }
    }

    Ok(builder)
}

pub fn check_http(host: &str, nodes: u8, wait: u8) -> Result<Builder, Box<dyn Error>> {
    let check_result = checkhost::check_http(host, nodes, wait)?;

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
