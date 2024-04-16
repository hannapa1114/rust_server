use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // Result Enum
        let listner = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listner.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // 서버가 받을 수 있는 Result 버퍼 크기 설정
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Failed to connection: {}", e),
                    }
                }
                Err(e) => {
                    print!("Failed to connection :{}", e)
                }
            }
        }
    }
}
