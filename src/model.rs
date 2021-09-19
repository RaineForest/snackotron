use quaint::single::Quaint;

pub struct Asset<'a> {
        upc: u64,
        count: u32,
        unit: &'a str
}

impl Asset<'_> {
        pub async fn register(&self) -> Result<(), quaint::error::Error> {
                let conn = quaint::single::Quaint::new("file://snackotron.db").await?;
                conn.insert(Insert::single_into("assets")
                        .value("upc", self.upc.to_string())
                        .value("count", self.count.to_string())
                        .value("unit", self.unit)
                        .build()).await?
        }
}
