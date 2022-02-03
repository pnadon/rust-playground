use std::{
    fs::File,
    io::{BufWriter, Write},
};

use futures_util::future::join_all;
use scraper::{Html, Selector};

use crate::ResultErr;

pub async fn scrape_suntimes() -> ResultErr<usize> {
    let header: Vec<String> = vec![
        "sunrise",
        "sunset",
        "twilight_end",
        "day_length",
        "solar_noon",
        "nautical_twilight_start",
        "nautical_twilight_end",
        "astronomical_twilight_start",
        "astronomical_twilight_end",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    
    let join = join_all((1..12).map(scrape_page_for_month))
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<String>>>();
    dbg!(&join);
    Ok(table_to_csv(&join, &header, "example2.txt")?)
}

async fn scrape_page_for_month(month: usize) -> Vec<Vec<String>> {
    let resp = reqwest::get(format!(
        "https://sunrise-sunset.org/ca/edmonton/2022/{}",
        month
    ))
    .await
    .unwrap();
    let html = Html::parse_document(&resp.text().await.unwrap());
    parse_times_for_month(&html)
}

fn parse_times_for_month(html: &Html) -> Vec<Vec<String>> {
    (1..=31)
        .map(|day| {
            (2..=10)
                .map(|col| parse_time_for_cell(html, day, col))
                .collect::<Vec<String>>()
        })
        .filter(|day| day.iter().all(|col| !col.is_empty()))
        .collect::<Vec<Vec<String>>>()
}

fn parse_time_for_cell(html: &Html, day: usize, col: usize) -> String {
    let selector = Selector::parse(&format!(
        "*[id=\"month\"] > tbody  > tr:nth-of-type({}) > td:nth-of-type({})",
        day + 2,
        col
    ))
    .unwrap();
    html.select(&selector)
        .next()
        .map(|input| input.text().next())
        .flatten()
        .unwrap_or_default()
        .to_owned()
}

fn table_to_csv(table: &[Vec<String>], header: &[String], fname: &str) -> ResultErr<usize> {
    if header.len() != table[0].len() {
        return Err(anyhow::anyhow!("header length must match table width").into());
    }
    let mut f = BufWriter::new(File::create(fname)?);
    let mut bytes_written = f.write((header.join(",") + "\n").as_bytes())?;
    for row in table {
        bytes_written += f.write((row.join(",") + "\n").as_bytes())?;
    }
    Ok(bytes_written)
}
