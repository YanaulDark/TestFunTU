use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn tokiofn() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // В цикле читаем данные из сокета и отправляем их обратно.
            loop {
                let n = match socket.read(&mut buf).await {
                    // сокет закрыт
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("не удалось прочитать из сокета; err = {:?}", e);
                        return;
                    }
                };

                // Отправляем данные обратно
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("не удалось записать в сокет; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
