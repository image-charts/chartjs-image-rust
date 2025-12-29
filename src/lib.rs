#![allow(non_snake_case, dead_code)]

//! Render Chart.JS as Image (or URL of Image)
//!
//! Generate [Chart.JS charts](https://www.chartjs.org/docs/latest/) as image and embed them everywhere in emails, pdf reports, chat bots...!
//!
//! # Features
//!
//! - `async` (default): Async API using tokio and reqwest
//! - `blocking`: Blocking/synchronous API using reqwest blocking
//! - `full`: Both async and blocking APIs
//!
//! # Example
//!
//! ```rust
//! use chartjs_image::ChartJSImage;
//!
//! let url = ChartJSImage::new()
//!     .chart(r#"{"type":"bar","data":{"labels":["Q1","Q2"],"datasets":[{"data":[1,2]}]}}"#)
//!     .width("400")
//!     .height("300")
//!     .to_url();
//!
//! println!("{}", url);
//! ```

use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

/// Error type for ChartJSImage operations
#[derive(Error, Debug)]
#[error("{message}")]
pub struct ChartJSImageError {
    /// Error message
    pub message: String,
    /// Error code from Image-Charts API
    pub code: Option<String>,
    /// HTTP status code
    pub status_code: Option<u16>,
}

impl ChartJSImageError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
            status_code: None,
        }
    }

    fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    fn with_status(mut self, status: u16) -> Self {
        self.status_code = Some(status);
        self
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ValidationError {
    message: String,
}

/// Configuration for ChartJSImage client
#[derive(Debug, Clone)]
pub struct ChartJSImageConfig {
    /// Protocol (http or https)
    pub protocol: String,
    /// API host
    pub host: String,
    /// API port
    pub port: u16,
    /// API pathname
    pub pathname: String,
    /// Request timeout
    pub timeout: Duration,
    /// Enterprise secret key for signing
    pub secret: Option<String>,
    /// Custom user-agent string
    pub user_agent: Option<String>,
}

impl Default for ChartJSImageConfig {
    fn default() -> Self {
        Self {
            protocol: "https".to_string(),
            host: "image-charts.com".to_string(),
            port: 443,
            pathname: "/chart.js/2.8.0".to_string(),
            timeout: Duration::from_millis(5000),
            secret: None,
            user_agent: None,
        }
    }
}

/// Builder for Chart.JS to Image API requests
///
/// Use the fluent API to configure chart parameters, then call one of the
/// output methods (`to_url`, `to_buffer`, `to_file`, `to_data_uri`) to
/// generate the chart.
///
/// # Example
///
/// ```rust
/// use chartjs_image::ChartJSImage;
///
/// let chart = ChartJSImage::new()
///     .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2,3]}]}}"#)
///     .width("400")
///     .height("300")
///     .to_url();
/// ```
#[derive(Debug, Clone)]
pub struct ChartJSImage {
    config: ChartJSImageConfig,
    query: HashMap<String, String>,
}

impl Default for ChartJSImage {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartJSImage {
    /// Create a new ChartJSImage instance with default configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    ///
    /// let chart = ChartJSImage::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(ChartJSImageConfig::default())
    }

    /// Create a new ChartJSImage instance with custom configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use chartjs_image::{ ChartJSImage, ChartJSImageConfig };
    /// use std::time::Duration;
    ///
    /// let config = ChartJSImageConfig {
    ///     timeout: Duration::from_secs(10),
    ///     ..Default::default()
    /// };
    /// let chart = ChartJSImage::with_config(config);
    /// ```
    pub fn with_config(config: ChartJSImageConfig) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// Create a new ChartJSImage instance for Enterprise usage with a secret key
    ///
    /// # Example
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    ///
    /// let chart = ChartJSImage::with_secret("my-secret-key");
    /// ```
    pub fn with_secret(secret: impl Into<String>) -> Self {
        Self::with_config(ChartJSImageConfig {
            secret: Some(secret.into()),
            ..Default::default()
        })
    }

    /// Create a new ChartJSImage builder for advanced configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// use std::time::Duration;
    ///
    /// let chart = ChartJSImage::builder()
    ///     .secret("my-secret")
    ///     .timeout(Duration::from_secs(30))
    ///     .build();
    /// ```
    pub fn builder() -> ChartJSImageBuilder {
        ChartJSImageBuilder::default()
    }

    fn clone_with(&self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut new_instance = self.clone();
        new_instance.query.insert(key.into(), value.into());
        new_instance
    }

    
        /// Javascript/JSON definition of the chart. Use a Chart.js configuration object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().c("{type:'bar',data:{labels:['Q1','Q2','Q3','Q4'],datasets:[{label:'Users',data:[50,60,70,180]},{label:'Revenue',data:[100,200,300,400]}]}}");
    /// ```
    pub fn c(self, value: impl Into<String>) -> Self {
        self.clone_with("c", value)
    }
        /// Javascript/JSON definition of the chart. Use a Chart.js configuration object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().chart("{type:'bar',data:{labels:['Q1','Q2','Q3','Q4'],datasets:[{label:'Users',data:[50,60,70,180]},{label:'Revenue',data:[100,200,300,400]}]}}");
    /// ```
    pub fn chart(self, value: impl Into<String>) -> Self {
        self.clone_with("chart", value)
    }
        /// Width of the chart
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().width("400");
    /// ```
    ///
    /// Default: `"500"`
    pub fn width(self, value: impl Into<String>) -> Self {
        self.clone_with("width", value)
    }
        /// Height of the chart
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().height("300");
    /// ```
    ///
    /// Default: `"300"`
    pub fn height(self, value: impl Into<String>) -> Self {
        self.clone_with("height", value)
    }
        /// Background of the chart canvas. Accepts rgb (rgb(255,255,120)), colors (red), and url-encoded hex values (%23ff00ff). Abbreviated as "bkg"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().backgroundColor("black");
    /// ```
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().backgroundColor("rgb(255,255,120)");
    /// ```
    pub fn backgroundColor(self, value: impl Into<String>) -> Self {
        self.clone_with("backgroundColor", value)
    }
        /// Background of the chart canvas. Accepts rgb (rgb(255,255,120)), colors (red), and url-encoded hex values (%23ff00ff). Abbreviated as "bkg"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().bkg("black");
    /// ```
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().bkg("rgb(255,255,120)");
    /// ```
    pub fn bkg(self, value: impl Into<String>) -> Self {
        self.clone_with("bkg", value)
    }
        /// Encoding of your "chart" parameter. Accepted values are url and base64.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().encoding("url");
    /// ```
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().encoding("base64");
    /// ```
    ///
    /// Default: `"url"`
    pub fn encoding(self, value: impl Into<String>) -> Self {
        self.clone_with("encoding", value)
    }
        /// image-charts enterprise `account_id`
    ///
    /// [Reference documentation](https://documentation.image-charts.com/enterprise/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().icac("accountId");
    /// ```
    pub fn icac(self, value: impl Into<String>) -> Self {
        self.clone_with("icac", value)
    }
        /// HMAC-SHA256 signature required to activate paid features
    ///
    /// [Reference documentation](https://documentation.image-charts.com/enterprise/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    /// let chart = ChartJSImage::new().ichm("0785cf22a0381c2e0239e27c126de4181f501d117c2c81745611e9db928b0376");
    /// ```
    pub fn ichm(self, value: impl Into<String>) -> Self {
        self.clone_with("ichm", value)
    }
        /// Retina is a marketing term coined by Apple that refers to devices and monitors that have a resolution and pixel density so high — roughly 300 or more pixels per inch – that a person is unable to discern the individual pixels at a normal viewing distance.
    ///            In order to generate beautiful charts for these Retina displays, Image-Charts supports a retina mode that can be activated through the icretina=1 parameter
    ///
    /// [Reference documentation](https://documentation.image-charts.com/reference/retina/)
    pub fn icretina(self, value: impl Into<String>) -> Self {
        self.clone_with("icretina", value)
    }
    

    /// Get the full Image-Charts API URL (signed and encoded if necessary)
    ///
    /// This method returns the complete URL that can be used to fetch the chart image.
    /// If an enterprise account ID (`icac`) is set and a secret is configured,
    /// the URL will be automatically signed with HMAC-SHA256.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chartjs_image::ChartJSImage;
    ///
    /// let url = ChartJSImage::new()
    ///     .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///     .width("100")
    ///     .height("100")
    ///     .to_url();
    ///
    /// assert!(url.starts_with("https://image-charts.com/chart.js/2.8.0?"));
    /// ```
    pub fn to_url(&self) -> String {
        let mut pairs: Vec<(&String, &String)> = self.query.iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));

        let mut query_string = pairs
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        if self.query.contains_key("icac") {
            if let Some(ref secret) = self.config.secret {
                if !secret.is_empty() {
                    let signature = self.sign(&query_string, secret);
                    query_string.push_str(&format!("&ichm={}", signature));
                }
            }
        }

        // Only include port if it's not the default for the protocol
        let port_str = match (self.config.protocol.as_str(), self.config.port) {
            ("https", 443) | ("http", 80) => String::new(),
            (_, port) => format!(":{}", port),
        };

        format!(
            "{}://{}{}{}?{}",
            self.config.protocol,
            self.config.host,
            port_str,
            self.config.pathname,
            query_string
        )
    }

    fn sign(&self, data: &str, secret: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    fn get_mime_type(&self) -> &str {
        // ChartJSImage always outputs PNG (no GIF animation support)
        "image/png"
    }

    fn get_file_format(&self) -> &str {
        // ChartJSImage always outputs PNG (no GIF animation support)
        "png"
    }

    fn build_user_agent(&self) -> String {
        let default_ua = format!(
            "rust-chartjs_image/{}{}",
            env!("CARGO_PKG_VERSION"),
            self.query
                .get("icac")
                .map(|icac| format!(" ({})", icac))
                .unwrap_or_default()
        );
        self.config.user_agent.clone().unwrap_or(default_ua)
    }

    fn parse_error_response(
        status: u16,
        error_code: Option<String>,
        validation_header: Option<&str>,
    ) -> ChartJSImageError {
        let validation_message = validation_header
            .and_then(|v| serde_json::from_str::<Vec<ValidationError>>(v).ok())
            .map(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.message)
                    .collect::<Vec<_>>()
                    .join("\n")
            });

        let message = validation_message
            .or_else(|| error_code.clone())
            .unwrap_or_else(|| format!("HTTP {}", status));

        let mut err = ChartJSImageError::new(message).with_status(status);
        if let Some(code) = error_code {
            err = err.with_code(code);
        }
        err
    }
}

// Async implementation
#[cfg(feature = "async")]
impl ChartJSImage {
    /// Do an async request to Image-Charts API and return the image as bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let buffer = ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_buffer()
    ///         .await?;
    ///
    ///     println!("Image size: {} bytes", buffer.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_buffer(&self) -> Result<Vec<u8>, ChartJSImageError> {
        let client = reqwest::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| ChartJSImageError::new(e.to_string()))?;

        let response = client
            .get(self.to_url())
            .header("User-Agent", self.build_user_agent())
            .send()
            .await
            .map_err(|e| {
                let mut err = ChartJSImageError::new(e.to_string());
                if let Some(status) = e.status() {
                    err = err.with_status(status.as_u16());
                }
                err
            })?;

        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| ChartJSImageError::new(e.to_string()).with_status(status))
        } else {
            let error_code = response
                .headers()
                .get("x-ic-error-code")
                .and_then(|v| v.to_str().ok())
                .map(String::from);
            let validation_header = response
                .headers()
                .get("x-ic-error-validation")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            Err(Self::parse_error_response(
                status,
                error_code,
                validation_header.as_deref(),
            ))
        }
    }

    /// Do an async request and write the image to a file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_file("chart.png")
    ///         .await?;
    ///
    ///     println!("Chart saved to chart.png");
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), ChartJSImageError> {
        let buffer = self.to_buffer().await?;
        tokio::fs::write(path, buffer)
            .await
            .map_err(|e| ChartJSImageError::new(e.to_string()))
    }

    /// Do an async request and return a base64-encoded data URI
    ///
    /// The returned string can be used directly in HTML `<img>` tags or CSS.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let data_uri = ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_data_uri()
    ///         .await?;
    ///
    ///     println!("<img src=\"{}\" />", data_uri);
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_data_uri(&self) -> Result<String, ChartJSImageError> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let buffer = self.to_buffer().await?;
        let encoded = STANDARD.encode(&buffer);
        Ok(format!("data:{};base64,{}", self.get_mime_type(), encoded))
    }
}

// Blocking implementation
#[cfg(feature = "blocking")]
impl ChartJSImage {
    /// Do a blocking request to Image-Charts API and return the image as bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let buffer = ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_buffer_blocking()?;
    ///
    ///     println!("Image size: {} bytes", buffer.len());
    ///     Ok(())
    /// }
    /// ```
    pub fn to_buffer_blocking(&self) -> Result<Vec<u8>, ChartJSImageError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| ChartJSImageError::new(e.to_string()))?;

        let response = client
            .get(self.to_url())
            .header("User-Agent", self.build_user_agent())
            .send()
            .map_err(|e| {
                let mut err = ChartJSImageError::new(e.to_string());
                if let Some(status) = e.status() {
                    err = err.with_status(status.as_u16());
                }
                err
            })?;

        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            response
                .bytes()
                .map(|b| b.to_vec())
                .map_err(|e| ChartJSImageError::new(e.to_string()).with_status(status))
        } else {
            let error_code = response
                .headers()
                .get("x-ic-error-code")
                .and_then(|v| v.to_str().ok())
                .map(String::from);
            let validation_header = response
                .headers()
                .get("x-ic-error-validation")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            Err(Self::parse_error_response(
                status,
                error_code,
                validation_header.as_deref(),
            ))
        }
    }

    /// Do a blocking request and write the image to a file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_file_blocking("chart.png")?;
    ///
    ///     println!("Chart saved to chart.png");
    ///     Ok(())
    /// }
    /// ```
    pub fn to_file_blocking(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), ChartJSImageError> {
        let buffer = self.to_buffer_blocking()?;
        std::fs::write(path, buffer).map_err(|e| ChartJSImageError::new(e.to_string()))
    }

    /// Do a blocking request and return a base64-encoded data URI
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use chartjs_image::ChartJSImage;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let data_uri = ChartJSImage::new()
    ///         .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2]}]}}"#)
    ///         .width("100")
    ///         .height("100")
    ///         .to_data_uri_blocking()?;
    ///
    ///     println!("<img src=\"{}\" />", data_uri);
    ///     Ok(())
    /// }
    /// ```
    pub fn to_data_uri_blocking(&self) -> Result<String, ChartJSImageError> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let buffer = self.to_buffer_blocking()?;
        let encoded = STANDARD.encode(&buffer);
        Ok(format!("data:{};base64,{}", self.get_mime_type(), encoded))
    }
}

/// Builder for ChartJSImageConfig
///
/// Provides a fluent API to configure ChartJSImage instances.
///
/// # Example
///
/// ```rust
/// use chartjs_image::ChartJSImage;
/// use std::time::Duration;
///
/// let chart = ChartJSImage::builder()
///     .secret("my-secret-key")
///     .timeout(Duration::from_secs(30))
///     .host("custom.image-charts.com")
///     .build()
///     .chart(r#"{"type":"bar","data":{}}"#)
///     .width("400")
///     .height("300");
/// ```
#[derive(Debug, Default)]
pub struct ChartJSImageBuilder {
    protocol: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    pathname: Option<String>,
    timeout: Option<Duration>,
    secret: Option<String>,
    user_agent: Option<String>,
}

impl ChartJSImageBuilder {
    /// Set the protocol (http or https)
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    /// Set the API host
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Set the API port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the API pathname
    pub fn pathname(mut self, pathname: impl Into<String>) -> Self {
        self.pathname = Some(pathname.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the enterprise secret key for URL signing
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// Set a custom user-agent string
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the ChartJSImage instance
    pub fn build(self) -> ChartJSImage {
        let default = ChartJSImageConfig::default();
        ChartJSImage::with_config(ChartJSImageConfig {
            protocol: self.protocol.unwrap_or(default.protocol),
            host: self.host.unwrap_or(default.host),
            port: self.port.unwrap_or(default.port),
            pathname: self.pathname.unwrap_or(default.pathname),
            timeout: self.timeout.unwrap_or(default.timeout),
            secret: self.secret,
            user_agent: self.user_agent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_chartjs_image() -> ChartJSImage {
        match std::env::var("IMAGE_CHARTS_USER_AGENT") {
            Ok(ua) => ChartJSImage::builder().user_agent(ua).build(),
            Err(_) => ChartJSImage::new(),
        }
    }

    fn create_chartjs_image_with_secret(secret: &str) -> ChartJSImage {
        match std::env::var("IMAGE_CHARTS_USER_AGENT") {
            Ok(ua) => ChartJSImage::builder().secret(secret).user_agent(ua).build(),
            Err(_) => ChartJSImage::with_secret(secret),
        }
    }

    #[test]
    fn test_to_url_basic() {
        let url = ChartJSImage::new()
            .chart(r#"{"type":"pie"}"#)
            .width("100")
            .height("100")
            .to_url();
        assert!(url.contains("chart="));
        assert!(url.contains("width=100"));
        assert!(url.contains("height=100"));
    }

    #[test]
    fn test_to_url_includes_protocol_host() {
        let url = ChartJSImage::new().chart("{}").to_url();
        // Default port (443 for https) should not be included in the URL
        assert!(url.starts_with("https://image-charts.com/chart.js/2.8.0?"));
    }

    #[test]
    fn test_to_url_includes_custom_port() {
        let config = ChartJSImageConfig {
            port: 8080,
            ..Default::default()
        };
        let url = ChartJSImage::with_config(config).chart("{}").to_url();
        // Non-default port should be included in the URL
        assert!(url.starts_with("https://image-charts.com:8080/chart.js/2.8.0?"));
    }

    #[test]
    fn test_to_url_with_signature() {
        let url = ChartJSImage::with_secret("plop")
            .chart("{}")
            .width("100")
            .height("100")
            .icac("test_fixture")
            .to_url();
        assert!(url.contains("ichm="));
    }

    #[test]
    fn test_default_config() {
        let config = ChartJSImageConfig::default();
        assert_eq!(config.protocol, "https");
        assert_eq!(config.host, "image-charts.com");
        assert_eq!(config.port, 443);
        assert_eq!(config.pathname, "/chart.js/2.8.0");
        assert_eq!(config.timeout, Duration::from_millis(5000));
    }

    #[test]
    fn test_builder_pattern() {
        let chart = ChartJSImage::builder()
            .secret("test-secret")
            .timeout(Duration::from_secs(10))
            .host("custom.host.com")
            .build();

        assert_eq!(chart.config.host, "custom.host.com");
        assert_eq!(chart.config.timeout, Duration::from_secs(10));
        assert_eq!(chart.config.secret, Some("test-secret".to_string()));
    }

    #[test]
    fn test_get_mime_type_png() {
        // ChartJSImage always outputs PNG (no GIF animation support)
        let chart = ChartJSImage::new().chart("{}").width("100").height("100");
        assert_eq!(chart.get_mime_type(), "image/png");
    }

    #[cfg(feature = "blocking")]
    mod blocking_tests {
        use super::*;

        #[test]
        fn test_to_buffer_blocking_works() {
            std::thread::sleep(std::time::Duration::from_secs(3));

            let result = create_chartjs_image()
                .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2,3]}]}}"#)
                .width("100")
                .height("100")
                .to_buffer_blocking();
            assert!(result.is_ok());
            let buffer = result.unwrap();
            assert!(!buffer.is_empty());
        }

        #[test]
        fn test_to_data_uri_blocking_works() {
            std::thread::sleep(std::time::Duration::from_secs(3));

            let result = create_chartjs_image()
                .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2,3]}]}}"#)
                .width("100")
                .height("100")
                .to_data_uri_blocking();
            assert!(result.is_ok());
            let data_uri = result.unwrap();
            assert!(data_uri.starts_with("data:image/png;base64,"));
        }
    }

    #[cfg(feature = "async")]
    mod async_tests {
        use super::*;

        #[tokio::test]
        async fn test_to_buffer_async_works() {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let result = create_chartjs_image()
                .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2,3]}]}}"#)
                .width("100")
                .height("100")
                .to_buffer()
                .await;
            assert!(result.is_ok());
            let buffer = result.unwrap();
            assert!(!buffer.is_empty());
        }

        #[tokio::test]
        async fn test_to_data_uri_async_works() {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let result = create_chartjs_image()
                .chart(r#"{"type":"pie","data":{"datasets":[{"data":[1,2,3]}]}}"#)
                .width("100")
                .height("100")
                .to_data_uri()
                .await;
            assert!(result.is_ok());
            let data_uri = result.unwrap();
            assert!(data_uri.starts_with("data:image/png;base64,"));
        }
    }
}
