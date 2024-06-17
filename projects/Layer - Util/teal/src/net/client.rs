use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::net::handlers::MessageHandlers;
use crate::{Reader, Writer};

use super::server::Server;

#[async_trait]
pub trait Client {
    fn get_id(&self) -> i32;
    async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error>>;
    // TODO Client.send() + Client.broadcast()
    // async fn send<T: MessageIdentifiable + MessageFull>(
    //     &self,
    //     msg: T,
    // ) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    //     // let buf = serialize(&msg);
    //     // self.send_bytes(&buf).await
    // }
    // async fn broadcast<T: MessageIdentifiable + MessageFull>(&mut self, msg: T);
    async fn frame(&self, buf: &[u8]);
    async fn run(&self) -> Result<(), Box<dyn Error + Send>>;
}

#[derive(Clone)]
pub struct DefaultClient {
    pub id: i32,
    pub server: Option<Arc<Mutex<Server>>>,
    pub reader: Reader,
    pub writer: Writer,
    pub handlers: Arc<MessageHandlers>,
}

impl DefaultClient {
    pub fn new(
        socket: TcpStream,
        handlers: Arc<MessageHandlers>,
        server: Option<Arc<Mutex<Server>>>,
    ) -> Self {
        let (r, w) = socket.into_split();
        let reader = Arc::new(Mutex::new(r));
        let writer = Arc::new(Mutex::new(w));
        Self {
            // arc: None,
            id: 0,
            server,
            reader,
            writer,
            handlers,
        }
    }
    pub async fn new_connection(
        addr: &str,
        handlers: Arc<MessageHandlers>,
    ) -> Result<Self, Box<dyn Error>> {
        let socket = TcpStream::connect(addr).await?;
        Ok(Self::new(socket, handlers, None))
    }
}

#[async_trait]
impl Client for DefaultClient {
    fn get_id(&self) -> i32 {
        self.id
    }
    async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.writer.lock().await.write_all(buf).await?;
        return Ok(());
    }
    // async fn broadcast<T: MessageIdentifiable + MessageFull>(&mut self, msg: T) {
    //     self.server
    //         .as_ref()
    //         .unwrap()
    //         .lock()
    //         .await
    //         .broadcast(msg)
    //         .await
    // }
    async fn frame(&self, buf: &[u8]) {
        self.handlers
            .handle(&buf, self) //self.get_arc()) // self
            .await
            .expect("message handling error")
    }
    async fn run(&self) -> Result<(), Box<dyn Error + Send>> {
        let mut buf = vec![0; 4 * 1024];
        loop {
            let n = self
                .reader
                .lock()
                .await
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            println!("read {}", n);
            if n == 0 {
                println!("client stream terminated");
                break;
            }

            // total msg length, including the header (2 bytes for len + id)
            let msg_length = buf[0] as usize;

            // fragmentation
            if msg_length < n {
                let mut i = 0;
                let mut end = msg_length;
                while i < n {
                    self.frame(&buf[i..end]).await;
                    i = end;
                    end += buf[i] as usize;
                }
                // TODO problem:
                // the last message read could be incomplete
                // on check 'i < n', puis lit le header du packet, mais 'end' pourrait dépasser 'n'
            }
            // perfect packet size
            else if msg_length == n {
                self.frame(&buf[0..n]).await;
            }
            // defragmentation
            else if msg_length > n {
                // this might happen if we send huge packets. obviously also if we go over the buffer size of 4*1024
                panic!("message size is bigger than packet size received, need to read more");
            }
        }
        Ok(())
    }
}
