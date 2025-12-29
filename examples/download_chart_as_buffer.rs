//! Example: Download Chart.js chart as binary buffer
//!
//! This example shows how to download a Chart.js chart image as bytes.
//! Run with: cargo run --example download_chart_as_buffer --features async

use chartjs_image::ChartJSImage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer = ChartJSImage::new()
        .chart(r#"{
            "type": "pie",
            "data": {
                "labels": ["Hello", "World"],
                "datasets": [{
                    "data": [60, 40]
                }]
            }
        }"#)
        .width("700")
        .height("300")
        .to_buffer()
        .await?;

    println!("Downloaded {} bytes", buffer.len());
    println!("First 20 bytes (PNG header): {:?}", &buffer[..20.min(buffer.len())]);

    Ok(())
}
