use http::parse::{HttpRequest, HttpResponse, EMPTY_HEADER};

use std::{env, marker::PhantomData};
use std::{fs, net::Ipv4Addr};
use utils::prelude::*;

fn main() {
    utils::init_logger().unwrap();

    // let mut pool = ThreadPool::new(2);
    // pool.execute(|| {
    //     panic!("AAAAAAAA");
    // });

    // thread::sleep(Duration::from_secs(1));
    // pool.respawn_worker_if_needed();

    // pool.execute(|| {
    //     thread::sleep(Duration::from_secs(4));
    //     error!("Hi! 2");
    // });

    // pool.execute(|| {
    //     thread::sleep(Duration::from_secs(4));
    //     warn!("Hi! 3");
    // });

    // pool.execute(|| {
    //     thread::sleep(Duration::from_secs(4));
    //     info!("Hi! 4");
    //     debug!("Hi! 4");
    //     trace!("Hi! 4");
    // });

    let mut headers = [EMPTY_HEADER; 16];
    {
        let mut req = HttpRequest::new(&mut headers);
        let r = req.parse(
            b"GET /hello.html HTTP/1.1\r\nHost: example.com\r\nAccept-Encoding: gzip, deflate\r\nContent-Length: 10\r\n\r\nHell"
        );

        println!("{:?}\n{:#?}", r, req);
    }
    {
        let mut res = HttpResponse::new(&mut headers);
        let r = res.parse(
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 10\r\nContent-Type: text/html\r\n\r\n",
        );
        println!("{:?}\n{:#?}", r, res);
    }
}
