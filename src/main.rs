use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // using 50 byte buffer
    let mut data = [0 as u8; 50];
    // 从stream读取数据，并写入mut data
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything! 向client返回data
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // 创建一个listener实例，listen to 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7878");
    // 读stream
    for stream in listener.incoming() {
        match stream {
            // stream返回ok
            Ok(stream) => {
                // 返回tcp链接的remote peer的socket地址
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 生成一个新线程处理stream. move表示ownership给这个线程
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            // stream报错处理
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
