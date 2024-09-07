# Books to Scrape Web Scraper

這個專案是一個用 Rust 編寫的網路爬蟲教學專案，目標是從 [Books to Scrape](https://books.toscrape.com/) 網站抓取書籍資訊。

## 專案簡介

本專案展示了如何使用 Rust 進行網路爬蟲，包括處理分頁、取得資料、以及將資料匯出為 Excel 和 JSON 格式。

### 主要功能

- 自動爬取所有頁面的書籍資訊
- 取得每本書的標題和價格
- 將資料匯出為 Excel 檔案 (.xlsx)
- 將資料匯出為 JSON 檔案
- 錯誤處理和異常情況管理

## 安裝

1. 確保已安裝 Rust 和 Cargo。如果沒有，請從 [rustup.rs](https://rustup.rs/) 安裝。

2. clone repo 到本機

3. 安裝 dependencies：
   ```
   cargo build
   ```

## 使用方法

執行以下指令來進行爬蟲：

```
cargo run
```

執行後，會在根目錄產生兩個文件：
- `books.xlsx`：包含所有書籍資訊的 Excel 文件
- `books.json`：包含所有書籍資訊的 JSON 文件

## Tech

本專案使用了以下 crates：
- `reqwest`：發送 HTTP 請求
- `tokio`：非同步處理
- `scraper`：解析 HTML 和取得資料
- `serde` 和 `serde_json`：處理 JSON
- `xlsxwriter`：建立 Excel 文件

## 目標

通過這個專案，可以做到：
- Rust 中的非同步處理
- 使用 reqwest 發送 HTTP 請求
- HTML 解析和資料取得
- 錯誤處理和結果處理
- 文件 I/O 操作
- JSON 和 Excel 文件的產生

## 注意事項

這個專案僅供教育目的使用。在使用網路爬蟲時，請確保遵守目標網站的使用條款和爬蟲禮儀。
