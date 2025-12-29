//! Example: Download Chart.js chart as data URI
//!
//! This example shows how to get a Chart.js chart as a base64-encoded data URI.
//! Useful for embedding directly in HTML or emails.
//! Run with: cargo run --example download_chart_as_data_uri --features async

use chartjs_image::ChartJSImage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_uri = ChartJSImage::new()
        .chart(r#"{
            "type": "pie",
            "data": {
                "labels": ["Hello", "World"],
                "datasets": [{
                    "data": [60, 40]
                }]
            }
        }"#)
        .width("100")
        .height("100")
        .to_data_uri()
        .await?;

    println!("Data URI length: {} characters", data_uri.len());
    println!("Data URI preview: {}...", &data_uri[..80.min(data_uri.len())]);

    // You can use this in HTML like:
    // <img src="{data_uri}" alt="Chart" />

    Ok(())
}
