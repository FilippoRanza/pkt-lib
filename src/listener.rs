use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

type Handler = tokio::task::JoinHandle<std::io::Result<()>>;
type Receiver<T> = mpsc::Receiver<RecvInfo<T>>;

pub struct ListenerController<T> {
    send: mpsc::Sender<()>,
    recv: Receiver<T>,
    handler: Handler,
}

impl<T> ListenerController<T> {
    fn new(send: mpsc::Sender<()>, recv: Receiver<T>, handler: Handler) -> Self {
        Self {
            send,
            recv,
            handler,
        }
    }

    pub fn stop(self) -> Handler {
        self.send.send(()).unwrap();
        self.handler
    }

    pub fn try_recv(&self) -> Result<RecvInfo<T>, mpsc::TryRecvError> {
        self.recv.try_recv()
    }
}

pub struct RecvInfo<T> {
    pub data: T,
    pub addr: SocketAddr,
}

pub fn create_tcp_listener<F, T, K>(ip: IpAddr, port: u16, f: &'static F) -> ListenerController<K>
where
    F: Fn(&T) -> K + Sync,
    T: Default + AsMut<[u8]> + Send,
    K: Send + Sync + 'static,
{
    let (info_trans, info_recv) = mpsc::channel();
    let (control_trans, control_recv) = mpsc::channel();
    let addr = SocketAddr::new(ip, port);
    let handle = start_tcp_listener(addr, f, info_trans, control_recv);

    ListenerController::new(control_trans, info_recv, handle)
}

fn start_tcp_listener<F, T, K>(
    addr: SocketAddr,
    f: &'static F,
    send: mpsc::Sender<RecvInfo<K>>,
    recv: mpsc::Receiver<()>,
) -> Handler
where
    F: Fn(&T) -> K + Sync,
    T: Default + AsMut<[u8]> + Send,
    K: Send + Sync + 'static,
{
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await?;
        while should_continue(&recv) {
            let accept = listener.accept().await?;
            let clone_send = send.clone();
            tokio::spawn(async move {
                let (sock, addr) = accept;
                handle_connetion(sock, addr, f, clone_send).await;
            });
        }

        Ok(())
    })
}

async fn handle_connetion<F, T, K>(
    mut sock: TcpStream,
    addr: SocketAddr,
    f: &'static F,
    send: mpsc::Sender<RecvInfo<K>>,
) where
    F: Fn(&T) -> K + Sync,
    T: Default + AsMut<[u8]> + Send,
    K: Send + Sync + 'static,
{
    let mut buff = T::default();

    let res = sock.read(buff.as_mut()).await;
    match res {
        Ok(n) if n > 0 => {
            let data = f(&buff);
            let info = RecvInfo { data, addr };
            send.send(info).unwrap();
        }
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    };
}

fn should_continue(recv: &mpsc::Receiver<()>) -> bool {
    let tmp = recv.try_recv();
    matches! {tmp, Err(mpsc::TryRecvError::Empty)}
}
