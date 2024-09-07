use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use xlsxwriter::Workbook;

#[derive(Serialize, Deserialize)]
struct Book {
    title: String,
    price: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;
    let mut books = Vec::new();

    let workbook = Workbook::new("books.xlsx")?;
    let mut sheet = workbook.add_worksheet(None)?;

    sheet.write_string(0, 0, "書名", None)?;
    sheet.write_string(0, 1, "價格", None)?;

    let mut row = 1;

    let total_pages = get_total_pages(&client).await?;
    println!("總頁數: {}", total_pages);

    for page in 1..=total_pages {
        let url = if page == 1 {
            "https://books.toscrape.com".to_string()
        } else {
            format!("https://books.toscrape.com/catalogue/page-{}.html", page)
        };

        println!("正在爬取頁面: {}", url);

        let response = client.get(&url).send().await?;
        println!("狀態: {}", response.status());

        if !response.status().is_success() {
            println!("狀態碼: {}", response.status());
            continue;
        }

        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let book_selector = Selector::parse("article.product_pod")?;
        let title_selector = Selector::parse("h3 a")?;
        let price_selector = Selector::parse("div.product_price .price_color")?;

        let mut book_count = 0;

        for book in document.select(&book_selector) {
            book_count += 1;
            let title_element = book.select(&title_selector).next().unwrap();
            let title = title_element
                .value()
                .attr("title")
                .ok_or("找不到 Title 屬性")?;
            let price = book
                .select(&price_selector)
                .next()
                .ok_or("找不到 Price 元素")?
                .text()
                .collect::<String>();

            sheet.write_string(row, 0, title, None)?;
            sheet.write_string(row, 1, &price, None)?;

            books.push(Book {
                title: title.to_string(),
                price,
            });

            // println!("書名: {}", title);
            // println!("價格: {}", price);
            // println!("---");

            row += 1;
        }

        println!("在第 {} 頁找到 {} 本書", page, book_count);
    }

    let file = File::create("books.json")?;
    serde_json::to_writer_pretty(file, &books)?;

    println!("資料已存到 books.json");

    Ok(())
}

async fn get_total_pages(client: &reqwest::Client) -> Result<u32, Box<dyn Error>> {
    let url = "https://books.toscrape.com/index.html";
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    let pager_selector = Selector::parse("ul.pager li.current")?;
    let pager_text = document
        .select(&pager_selector)
        .next()
        .ok_or("無法找到分頁資料")?
        .text()
        .collect::<String>();

    let total_pages = pager_text
        .split_whitespace()
        .last()
        .ok_or("無法取得總頁數")?
        .parse::<u32>()?;

    Ok(total_pages)
}
