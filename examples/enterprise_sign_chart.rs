//! Example: Enterprise - Sign Chart.js chart URL with HMAC
//!
//! This example shows how to sign Chart.js chart URLs for enterprise accounts.
//! The signature ensures the chart URL cannot be tampered with.
//! Run with: cargo run --example enterprise_sign_chart

use chartjs_image::ChartJSImage;

fn main() {
    // Replace with your actual enterprise credentials
    let account_id = "YOUR_ACCOUNT_ID";
    let secret_key = "YOUR_SECRET_KEY";

    let url = ChartJSImage::builder()
        .secret(secret_key)
        .build()
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
        .icac(account_id)
        .to_url();

    println!("Signed Chart URL: {}", url);

    // The URL will contain an 'ichm' parameter with the HMAC signature
    assert!(url.contains("ichm="), "URL should contain HMAC signature");
    println!("✓ URL is signed with HMAC-SHA256");
}
