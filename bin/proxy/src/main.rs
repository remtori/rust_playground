use http::http::{HttpRequest, HttpResponse, EMPTY_HEADER};
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::*;
use utils::prelude::*;

fn main() {
    utils::init_logger().unwrap();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let port = if args.len() >= 2 {
        u16::from_str_radix(&args[0], 10).expect("Invalid port number")
    } else {
        7676
    };

    let addr = (Ipv4Addr::new(127, 0, 0, 1), port);
    println!("Proxy server started on: {}:{}", addr.0.to_string(), addr.1);

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            let _ = handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut host: TcpStream) -> Result<(), std::io::Error> {
    // TODO: Make this thread local to avoid re-allocation
    let mut _backing_vec1: Vec<u8> = vec![0; 8096];
    let mut _backing_vec2: Vec<u8> = vec![0; 8096];
    let mut req_headers = [EMPTY_HEADER; 32];
    let mut res_headers = [EMPTY_HEADER; 32];

    let mut remote = None;

    // loop {
    let mut req_buffer = _backing_vec1.as_mut_slice();
    let mut res_buffer = _backing_vec2.as_mut_slice();

    let mut request = HttpRequest::new(&mut req_headers);
    let mut response = HttpResponse::new(&mut res_headers);

    let read_size = host.read(&mut req_buffer)?;
    let req_size = match request.parse(req_buffer) {
        Ok(v) => v,
        Err(e) => {
            info!("{:?}", e);
            return Ok(());
        }
    };
    info!("Req:\n{}", str::from_utf8(&req_buffer[..req_size]).unwrap());

    let req_body_size = {
        if let Some(s) = request.header("content-length") {
            if let Ok(Ok(size)) = str::from_utf8(s).map(|s| s.parse::<usize>()) {
                size
            } else {
                0
            }
        } else {
            0
        }
    };

    let mut remote = remote.get_or_insert_with(|| {
        let remote_host = str::from_utf8(request.header("host").unwrap()).unwrap();
        if remote_host.contains(':') {
            TcpStream::connect(remote_host).unwrap()
        } else {
            TcpStream::connect((remote_host, 80)).unwrap()
        }
    });
    remote.write_all(&req_buffer[..req_size + req_body_size])?;

    let read_size = remote.read(&mut res_buffer)?;
    let res_size = match response.parse(res_buffer) {
        Ok(v) => v,
        Err(e) => {
            info!("{:?}", e);
            return Ok(());
        }
    };
    info!("Res:\n{}", str::from_utf8(&res_buffer[..res_size]).unwrap());

    let res_body_size = {
        if let Some(s) = response.header("content-length") {
            if let Ok(Ok(size)) = str::from_utf8(s).map(|s| s.parse::<usize>()) {
                size
            } else {
                0
            }
        } else {
            0
        }
    };

    host.write_all(&res_buffer[..res_size + res_body_size])?;
    // }

    Ok(())
}
