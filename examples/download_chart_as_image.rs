//! Example: Download and save Chart.js chart as image file
//!
//! This example shows how to download a Chart.js chart and save it to a file.
//! Run with: cargo run --example download_chart_as_image --features async

use chartjs_image::ChartJSImage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "chartjs_example.png";

    ChartJSImage::new()
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
        .to_file(path)
        .await?;

    println!("Chart saved to: {}", path);

    // Clean up
    std::fs::remove_file(path)?;
    println!("File cleaned up");

    Ok(())
}
