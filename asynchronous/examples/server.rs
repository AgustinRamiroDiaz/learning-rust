use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        println!("Client connected");
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 1000];
    socket.read(&mut buf).await.unwrap();

    println!("A client has sent '{}'", String::from_utf8_lossy(&buf));

    socket.write(b"HTTP/1.1 200 OK\r\n\r\n").await.unwrap();
    socket.flush().await.unwrap();
}
