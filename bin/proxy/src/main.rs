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
            if let Err(err) = handle_connection(stream) {
                warn!("{:?}", err);
            }
        });
    }

    println!("Shutting down.");
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    HttpParse(http::http::ParseError),
    Utf8Error(std::str::Utf8Error),
    Timeout,
    Eof,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<http::http::ParseError> for Error {
    fn from(e: http::http::ParseError) -> Self {
        Self::HttpParse(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

fn handle_connection(mut host: TcpStream) -> Result<(), Error> {
    // TODO: Make this thread local to avoid re-allocation
    let mut _backing_vec1: Vec<u8> = vec![0; 8096];
    let mut _backing_vec2: Vec<u8> = vec![0; 8096];
    let mut req_headers = [EMPTY_HEADER; 32];
    let mut res_headers = [EMPTY_HEADER; 32];

    let req_buffer = _backing_vec1.as_mut_slice();
    let res_buffer = _backing_vec2.as_mut_slice();

    let mut request = HttpRequest::new(&mut req_headers);
    let mut response = HttpResponse::new(&mut res_headers);

    let _read_size = host.read(req_buffer)?;
    let req_header_size = request.parse(req_buffer)?;

    info!("Req:\n{}", str::from_utf8(&req_buffer[..req_header_size])?);

    let req_body_size = *request
        .header("content-length")
        .and_then(utils::ascii_parse::<usize>)
        .get_or_insert(0);

    let remote_host = str::from_utf8(request.header("host").unwrap())?;
    let mut remote = if remote_host.contains(':') {
        TcpStream::connect(remote_host)?
    } else {
        TcpStream::connect((remote_host, 80))?
    };

    remote.write_all(&req_buffer[..req_header_size + req_body_size])?;

    let _read_size = remote.read(res_buffer)?;
    let res_header_size = response.parse(res_buffer)?;

    info!(
        "Res:\n{}",
        str::from_utf8(&res_buffer[..res_header_size]).unwrap()
    );

    let res_body_size = *response
        .header("content-length")
        .and_then(utils::ascii_parse::<usize>)
        .get_or_insert(0);

    host.write_all(&res_buffer[..res_header_size + res_body_size])?;

    Ok(())
}
