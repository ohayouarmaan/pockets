mod data_store;

#[derive(Debug)]
pub struct PocketConfig<'a> {
    pub write_to_disk: u32,
    pub write_location: &'a str,
    pub max_connections: u32
}

impl<'a> Default for PocketConfig<'a> {
    fn default() -> Self {
        Self {
            write_to_disk: 4 * 1000,
            write_location: "./data.Pocket",
            max_connections: 10
        }
    }
}

impl<'a> PocketConfig<'a> {
    fn new(write_to_disk: u32, write_location: Option<&'a str>, max_connections: Option<u32>) -> Self {
        Self {
            write_to_disk,
            write_location: write_location.unwrap_or("./data.Pocket"),
            max_connections: max_connections.unwrap_or(10)
        }
    }
}

#[derive(Debug)]
pub struct Pocket<'a> {
    pub ds: data_store::DataStore<'a>,
    pub config: PocketConfig<'a>
}

impl<'a> Default for Pocket<'a> {
    fn default() -> Self {
        Self {
            ds: data_store::DataStore::default(),
            config: PocketConfig::default()
        }
    }
}

