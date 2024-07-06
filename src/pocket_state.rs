mod data_store;

use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

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
    pub config: PocketConfig<'a>,
    server: Option<TcpListener>,
    connection_count: Arc<Mutex<u32>>,
    connections: Vec<Arc<Mutex<TcpStream>>>
}

impl<'a> Default for Pocket<'a> {
    fn default() -> Self {
        Self {
            ds: data_store::DataStore::default(),
            config: PocketConfig::default(),
            server: None,
            connection_count: Arc::new(Mutex::new(0)),
            connections: vec![]
        }
    }
}

impl<'a> Pocket<'a> {
    async fn handle_connection(s: Arc<Mutex<TcpStream>>, connection_count: Arc<Mutex<u32>>) {
        {
            let mut x = connection_count.lock().unwrap();
            *x += 1;
        }
        println!("{:?}", connection_count);
    }

    pub async fn listen(&mut self, listen_addr: &'a str, port: u32) {
        self.connect(listen_addr, &port).await;
        if let Some(server) = &self.server {
            loop {
                let (socket, _) = server.accept().await.unwrap();
                let cc = self.connection_count.clone();
                let s = Arc::new(Mutex::new(socket));
                self.connections.push(s.clone());
                tokio::spawn(async move {
                        Self::handle_connection(s.clone(), cc).await;
                });
            }
        }
    }

    async fn connect(&mut self, listen_addr: &'a str, port: &u32) {
        let server: TcpListener = TcpListener::bind(format!("{}:{}", listen_addr, port))
                                    .await
                                    .unwrap_or_else(|e| panic!("Unable to bind to the specified address: {}", e));
        self.server = Some(server);
    }
}

