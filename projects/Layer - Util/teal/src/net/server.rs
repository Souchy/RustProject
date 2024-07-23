use std::{error::Error, sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};
use tracing::error;

use crate::net::client::{Client, DefaultClient};
use crate::net::handlers::MessageHandlers;

use super::message::{serialize, MessageIdentifiable};

#[derive(Default)]
pub struct Server {
    clients: Vec<Arc<DefaultClient>>,
}

impl Server {
    pub async fn run(addr: String, handlers: Arc<MessageHandlers>) -> Result<(), Box<dyn Error>> {
        // Next up we create a TCP listener which will listen for incoming
        // connections. This TCP listener is bound to the address we determined
        // above and must be associated with an event loop.
        let listener = TcpListener::bind(&addr).await?;
        println!("Listening on: {}", addr);

        let server = Self {
            clients: Vec::new(),
        };
        let server_ptr = Arc::new(Mutex::new(server));

        loop {
            // Asynchronously wait for an inbound socket.
            let (socket, _addr) = listener.accept().await?;

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
            // let server_ref = Arc::new(self);

            let client: DefaultClient =
                DefaultClient::new(socket, handlers.clone(), server_ptr.clone());

            let client_runner = Arc::new(client);
            server_ptr.lock().await.clients.push(client_runner.clone());

            tokio::spawn(async move {
                if let Err(err) = client_runner.run().await {
                    error!(cause = ?err, "client connection error");
                }
            });
        }
    }

    pub async fn broadcast<T: MessageIdentifiable>(&self, msg: T) -> Result<(), Box<dyn Error>> {
        let buf = serialize(&msg);
        self.broadcast_bytes(&buf).await
    }

    pub async fn broadcast_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        for c in &self.clients {
            _ = c.send_bytes(&buf).await;
        }
        Ok(())
    }
}
