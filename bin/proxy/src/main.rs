use http::parse::*;
use lazy_static::lazy_static;
use std::*;
use std::{io::prelude::*, net::*, time::*};
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
    let pool = ThreadPool::new(8);

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
    HttpParse(ParseError),
    Utf8Error(std::str::Utf8Error),
    Timeout,
    MissingHostHeader,
    Eof,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::HttpParse(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

lazy_static! {
    static ref BUFFER_POOL: ObjectPool<Vec<u8>> = ObjectPool::new();
}

fn handle_connection(mut host: TcpStream) -> Result<(), Error> {
    let mut maybe_remote: Option<TcpStream> = None;

    // host.set_read_timeout(Some(Duration::from_secs(1)))?;
    // host.set_write_timeout(Some(Duration::from_secs(1)))?;

    let mut buf1 = BUFFER_POOL.take();
    let mut buf2 = BUFFER_POOL.take();
    let req_buffer = buf1.as_mut_slice();
    let res_buffer = buf2.as_mut_slice();

    loop {
        let transaction_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let mut req_read_size = host.read(req_buffer)?;
        debug!(
            "[{}] Read {} bytes from host",
            transaction_id, req_read_size
        );

        let required_size = {
            let mut req_headers = [EMPTY_HEADER; 32];
            let mut request = HttpRequest::new(&mut req_headers);
            let req_header_size = request.parse(req_buffer)?;

            trace!(
                "[{}] Req:\n{}",
                transaction_id,
                str::from_utf8(&req_buffer[..req_header_size])?
            );

            trace!("[{}] Connecting to: {}", transaction_id, request.uri());

            let req_body_size = *request
                .header("content-length")
                .and_then(utils::ascii_parse::<usize>)
                .get_or_insert(0);

            trace!(
                "[{}] Host request content-length: {}",
                transaction_id,
                req_body_size
            );

            if maybe_remote.is_none() {
                let remote_host =
                    str::from_utf8(request.header("host").ok_or(Error::MissingHostHeader)?)?;

                let remote = if remote_host.contains(':') {
                    TcpStream::connect(remote_host)?
                } else {
                    TcpStream::connect((remote_host, 80))?
                };

                // remote.set_read_timeout(Some(Duration::from_secs(4)))?;
                // remote.set_write_timeout(Some(Duration::from_secs(4)))?;

                maybe_remote = Some(remote);
            }

            req_header_size + req_body_size
        };
        debug!(
            "[{}] Required sent bytes is {}",
            transaction_id, required_size
        );

        let remote = maybe_remote.as_mut().unwrap();

        let mut sent_size = 0;
        while sent_size < required_size {
            remote.write_all(&req_buffer[..req_read_size])?;
            sent_size += req_read_size;
            debug!(
                "[{}] Sent {} bytes to remote",
                transaction_id, req_read_size
            );

            if sent_size >= required_size {
                break;
            }

            req_read_size = host.read(req_buffer)?;
            debug!(
                "[{}] Read {} bytes from host",
                transaction_id, req_read_size
            );

            if req_read_size == 0 {
                return Err(Error::Eof);
            }
        }

        let mut res_read_size = remote.read(res_buffer)?;
        debug!(
            "[{}] Read {} bytes from remote",
            transaction_id, res_read_size
        );

        let required_size = {
            let mut res_headers = [EMPTY_HEADER; 32];
            let mut response = HttpResponse::new(&mut res_headers);
            let res_header_size = response.parse(res_buffer)?;

            trace!(
                "[{}] Res:\n{}",
                transaction_id,
                str::from_utf8(&res_buffer[..res_header_size])?
            );

            let res_body_size = *response
                .header("content-length")
                .and_then(utils::ascii_parse::<usize>)
                .get_or_insert(0);

            trace!(
                "[{}] Remote response content-length: {}",
                transaction_id,
                res_body_size
            );

            res_header_size + res_body_size
        };
        debug!(
            "[{}] Required receive bytes is {}",
            transaction_id, required_size
        );

        let mut sent_size = 0;
        while sent_size < required_size {
            host.write_all(&res_buffer[..res_read_size])?;
            sent_size += res_read_size;
            debug!("[{}] Sent {} bytes to host", transaction_id, res_read_size);

            if sent_size >= required_size {
                break;
            }

            res_read_size = remote.read(res_buffer)?;
            debug!(
                "[{}] Read {} bytes from remote",
                transaction_id, res_read_size
            );

            if res_read_size == 0 {
                return Err(Error::Eof);
            }
        }

        debug!("[{}] Transaction done!", transaction_id);
    }
}
