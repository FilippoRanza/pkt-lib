use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc;

use tokio::net::UdpSocket;


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

pub fn create_udp_listener<F, T, K>(ip: IpAddr, port: u16, f: &'static F) -> ListenerController<K>
where
    F: Fn(&T) -> K + Sync,
    T: Default + AsMut<[u8]> + Send,
    K: Send + Sync + 'static,
{
    let (info_trans, info_recv) = mpsc::channel();
    let (control_trans, control_recv) = mpsc::channel();
    let addr = SocketAddr::new(ip, port);
    let handle = start_udp_listener(addr, f, info_trans, control_recv);

    ListenerController::new(control_trans, info_recv, handle)
}

fn start_udp_listener<F, T, K>(
    addr: SocketAddr,
    f: &'static F,
    send: mpsc::Sender<RecvInfo<K>>,
    recv: mpsc::Receiver<()>,
) -> Handler
where
    F: Fn(&T) -> K + Sync,
    T:  Default + AsMut<[u8]> + Send,
    K: Send + Sync + 'static,
{
    tokio::spawn( async move {
        let sock = UdpSocket::bind(addr).await?;
        let mut buff = T::default();
        while should_continue(&recv) {
            let (amt, addr) = sock.recv_from(buff.as_mut()).await?;
            if amt > 0 {
                let data = f(&buff);
                let info = RecvInfo { data, addr };
                send.send(info).unwrap();
            }
        }

        Ok(())
    })
}

fn should_continue(recv: &mpsc::Receiver<()>) -> bool {
    let tmp = recv.try_recv();
    matches! {tmp, Err(mpsc::TryRecvError::Empty)}
}
