use serde::{Deserialize, Serialize, Serializer};
use std::fs;

fn main() -> anyhow::Result<()> {
    let file_name = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .unwrap_or("sample".to_string());
    let n = std::env::args_os()
        .nth(2)
        .and_then(|s| s.into_string().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let json_str = fs::read_to_string(format!("{}.json", file_name))?;
    let json: GeoData = serde_json::from_str(&json_str)?;

    let mut md_ctx = md5::Context::new();
    serde_json::to_writer(&mut md_ctx, &json)?;
    println!("{:x}", md_ctx.compute());

    let mut array = Vec::with_capacity(n);
    for _i in 0..n {
        let json: GeoData = serde_json::from_str(&json_str)?;
        array.push(json);
    }

    let mut md_ctx = md5::Context::new();
    serde_json::to_writer(&mut md_ctx, &array)?;
    println!("{:x}", md_ctx.compute());

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct GeoData {
    r#type: String,
    features: Vec<Feature>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Feature {
    r#type: String,
    properties: Properties,
    geometry: Geometry,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Properties {
    name: String,
}

#[derive(Deserialize, Debug, Default)]
struct MyF64(f64);

impl Serialize for MyF64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.fract() == 0.0 {
            serializer.serialize_i64(self.0 as i64)
        } else {
            serializer.serialize_f64(self.0)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Geometry {
    r#type: String,
    coordinates: Vec<Vec<[MyF64; 2]>>,
}
