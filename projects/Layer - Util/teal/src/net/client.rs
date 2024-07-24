use std::any::{Any, TypeId};
use std::error::Error;
use std::sync::Arc;
use std::fmt::Debug;

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::net::handlers::MessageHandlers;
use crate::{ Reader, Writer};
use crate::{HEADER_LEN, LEN_LEN};

use super::server::Server;

#[async_trait]
pub trait Client : Send + Sync + Debug + 'static {
    fn get_id_ref(&self) -> Arc<Mutex<String>>;
    fn get_id_sync(&self) -> String;
    async fn set_id(&self, id: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    // fn set_id(&self, id: String);
    fn get_server(&self) ->  Arc<Mutex<Server>>;

    async fn run(&self) -> Result<(), Box<dyn Error + Send>>;
    async fn frame(&self, buf: &[u8]);
    async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>>;
    // These make the object unsafe.
    // async fn send<T: MessageIdentifiable>(
    //     &self,
    //     msg: &T,
    // ) -> Result<(), Box<dyn Error>> {
    //     let buf = serialize(msg);
    //     self.send_bytes(&buf).await
    // }
    // async fn broadcast<T: MessageIdentifiable>(
    //     &self,
    //     msg: &T,
    // ) -> Result<(), Box<dyn Error>> {
    //     let buf = serialize(msg);
    //     self.get_server().lock().await.broadcast(msg).await;
    // }
    // async fn broadcast_bytes(
    //     &self,
    //     buf: &[u8],
    // ) -> Result<(), Box<dyn Error>> {
    //     // let buf = serialize(msg);
    //     self.get_server().ok_or(Errors::Missing("".to_string(), "".to_string()))?.lock().await.broadcast_bytes(&buf).await
    // }
}

/**
 * Downcast functions sourced from rust-protobuf
 */
impl dyn Client {
    /// Downcast `Box<dyn Client>` to specific client type.
    ///
    /// ```
    /// # use teal::net::client::Client;
    /// # fn foo<T: Client>(client: Box<dyn Client>) {
    /// let c: Box<dyn Client> = client;
    /// let dc: Box<T> = <dyn Client>::downcast_box(c).unwrap();
    /// # }
    /// ```
    pub fn downcast_box<T: Any>(
        self: Box<dyn Client>,
    ) -> std::result::Result<Box<T>, Box<dyn Client>> {
        if Any::type_id(&*self) == TypeId::of::<T>() {
            unsafe {
                let raw: *mut dyn Client = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }

    /// Downcast `&dyn Client` to specific client type.
    ///
    /// ```
    /// # use teal::net::client::Client;
    /// # fn foo<T: Client>(client: &dyn Client) {
    /// let c: &dyn Client = client;
    /// let dc: &T = <dyn Client>::downcast_ref(c).unwrap();
    /// # }
    /// ```
    pub fn downcast_ref<'a, M: Client + 'a>(&'a self) -> Option<&'a M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&*(self as *const dyn Client as *const M)) }
        } else {
            None
        }
    }

    /// Downcast `&mut dyn Client` to specific client type.
    ///
    /// ```
    /// # use teal::net::client::Client;
    /// # fn foo<T: Client>(client: &mut dyn Client) {
    /// let c: &mut dyn Client = client;
    /// let dc: &mut T = <dyn Client>::downcast_mut(c).unwrap();
    /// # }
    /// ```
    pub fn downcast_mut<'a, M: Client + 'a>(&'a mut self) -> Option<&'a mut M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&mut *(self as *mut dyn Client as *mut M)) }
        } else {
            None
        }
    }
}


#[derive(Clone, Debug)]
pub struct DefaultClient {
    pub id: Arc<Mutex<String>>,
    pub server: Arc<Mutex<Server>>,
    pub reader: Reader,
    pub writer: Writer,
    pub handlers: Arc<MessageHandlers>,
}

impl DefaultClient {
    pub fn new(
        socket: TcpStream,
        handlers: Arc<MessageHandlers>,
        server: Arc<Mutex<Server>>,
    ) -> Self {
        let (r, w) = socket.into_split();
        let reader = Arc::new(Mutex::new(r));
        let writer = Arc::new(Mutex::new(w));
        Self {
            // arc: None,
            id: Arc::new(Mutex::new(String::default())),
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
        Ok(Self::new(socket, handlers, Arc::new(Mutex::new(Server::default()))))
    }

}

#[async_trait]
impl Client for DefaultClient {
    fn get_id_ref(&self) -> Arc<Mutex<String>> {
        self.id.clone()
    }
    fn get_id_sync(&self) -> String {
        self.id.blocking_lock().clone()
    }
    async fn set_id(&self, id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        *self.id.lock().await = id;
        Ok(())
    }
    fn get_server(&self) -> Arc<Mutex<Server>> {
        self.server.clone()
    }
    async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = self.writer.lock().await.write_all(buf).await;
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

            // println!("read {}", n);
            if n == 0 {
                println!("client stream terminated");
                break;
            }

            if n < HEADER_LEN {
                continue;
            }
            let mut dst = [0u8; LEN_LEN];
            dst.clone_from_slice(&buf[0..LEN_LEN]);
            // total msg length, including the header (2 bytes for id + 8 bytes for length)
            let mut msg_length = usize::from_be_bytes(dst);

            // fragmentation
            if msg_length < n {
                let mut start = 0;
                let mut end = msg_length;
                while start < n && msg_length >= HEADER_LEN {
                    // println!("frame {} to {}", start, end);
                    self.frame(&buf[start..end]).await;
                    // read next frame
                    start = end;
                    dst.clone_from_slice(&buf[start..start + LEN_LEN]);
                    msg_length = usize::from_be_bytes(dst);
                    end += msg_length;
                }
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


/*
async fn frame(client: Arc<DefaultClient>, buf: &[u8]) {
    client.handlers
        .handle(&buf, client.as_ref()) 
        .await
        .expect("message handling error")
}
async fn run(client: Arc<DefaultClient>) -> Result<(), Box<dyn Error + Send>> {
    let mut buf = vec![0; 4 * 1024];
    loop {
        let n = client
            .reader
            .lock()
            .await
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        // println!("read {}", n);
        if n == 0 {
            println!("client stream terminated");
            break;
        }

        if n < HEADER_LEN {
            continue;
        }
        let mut dst = [0u8; LEN_LEN];
        dst.clone_from_slice(&buf[0..LEN_LEN]);
        // total msg length, including the header (2 bytes for id + 8 bytes for length)
        let mut msg_length = usize::from_be_bytes(dst);

        // fragmentation
        if msg_length < n {
            let mut start = 0;
            let mut end = msg_length;
            while start < n && msg_length >= HEADER_LEN {
                // println!("frame {} to {}", start, end);
                client.frame(&buf[start..end]).await;
                // read next frame
                start = end;
                dst.clone_from_slice(&buf[start..start + LEN_LEN]);
                msg_length = usize::from_be_bytes(dst);
                end += msg_length;
            }
        }
        // perfect packet size
        else if msg_length == n {
            client.frame(&buf[0..n]).await;
        }
        // defragmentation
        else if msg_length > n {
            // this might happen if we send huge packets. obviously also if we go over the buffer size of 4*1024
            panic!("message size is bigger than packet size received, need to read more");
        }
    }
    Ok(())
}
*/
