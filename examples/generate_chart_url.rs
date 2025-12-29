//! Example: Generate a Chart.js chart URL
//!
//! This example shows how to generate a Chart.js chart URL without making any HTTP request.
//! Run with: cargo run --example generate_chart_url

use chartjs_image::ChartJSImage;

fn main() {
    // Create a simple pie chart URL
    let url = ChartJSImage::new()
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
        .to_url();

    println!("Chart URL: {}", url);

    // Create a bar chart
    let bar_url = ChartJSImage::new()
        .chart(r#"{
            "type": "bar",
            "data": {
                "labels": ["2014", "2015", "2016", "2017", "2018"],
                "datasets": [{
                    "label": "Income",
                    "data": [30010, -30000, 50000, 80000, 20000]
                }]
            }
        }"#)
        .width("700")
        .height("300")
        .to_url();

    println!("Bar Chart URL: {}", bar_url);
}
