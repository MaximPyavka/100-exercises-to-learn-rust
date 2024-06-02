use httparse;

use serde::{Deserialize};
use serde_json;

pub fn parse_request_body<'de, T: Deserialize<'de>>(request_content: &'de [u8]) -> T {
    serde_json::from_slice(request_content).unwrap()
}

#[derive(Debug)]
pub struct Request<'header, 'buf> {
    pub request_meta: httparse::Request<'header, 'buf>,
    pub request_body: &'buf [u8],
}

pub fn parse_request<'buf, 'headersbuf: 'buf>(
    request_buf: &'buf Vec<u8>,
    headers_buf: &'headersbuf mut [httparse::Header<'headersbuf>; 16],
) -> Request<'headersbuf, 'buf> {
    let mut req: httparse::Request = httparse::Request::new(headers_buf);
    let res = req.parse(request_buf as &[u8]).unwrap();
    if res.is_complete() {
        match req.path {
            Some(_) => {
                Request {
                    request_meta: req,
                    request_body: &request_buf[res.unwrap()..],
                }
            }
            None => {
                todo!();
            }
        }
    } else {
        todo!("parse more");
    }
}