use std::*;
use utils::prelude::*;

pub const EMPTY_HEADER: Header<'static> = Header {
    field: "",
    value: b"",
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    Ext(String),
}

#[derive(Debug)]
pub enum Version {
    V10,
    V11,
    Unknown,
}

#[derive(Debug)]
pub struct Header<'b> {
    field: &'b str,
    value: &'b [u8],
}

#[derive(Debug)]
pub enum ParseError {
    PartialData,
    ParseUtf8Err,
    ParseStatusCode,
    InvalidUri,
    InvalidVersion,
    InvalidHeaderField,
    InvalidHeaderValue,
    TooManyHeader,
}

impl From<core::str::Utf8Error> for ParseError {
    fn from(_: core::str::Utf8Error) -> Self {
        Self::ParseUtf8Err
    }
}

type Result<T> = core::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct HttpRequest<'buf, 'h> {
    method: Method,
    uri: &'buf str,
    version: Version,
    headers: &'h mut [Header<'buf>],
    header_count: usize,
}

impl<'buf, 'h> HttpRequest<'buf, 'h> {
    pub fn new(headers: &'h mut [Header<'buf>]) -> HttpRequest<'buf, 'h> {
        HttpRequest {
            method: Method::GET,
            uri: "",
            version: Version::Unknown,
            headers,
            header_count: 0,
        }
    }

    pub fn parse(&mut self, buffer: &'buf [u8]) -> Result<usize> {
        let mut bytes = BytesRef::from(buffer);

        self.method = parse_method(&mut bytes)?;

        // Nom whitespace
        if !bytes.consume(b' ').ok_or(ParseError::PartialData)? {
            return Err(ParseError::InvalidUri);
        }

        let uri = bytes.consume_until(b' ').ok_or(ParseError::PartialData)?;
        self.uri = if let Ok(v) = str::from_utf8(uri) {
            v
        } else {
            return Err(ParseError::ParseUtf8Err);
        };

        // Nom whitespace
        if !bytes.consume(b' ').ok_or(ParseError::PartialData)? {
            return Err(ParseError::InvalidUri);
        }

        self.version = parse_version(&mut bytes, true)?;

        // Nom \r\n
        if let Some(ok) = bytes.consume_str(b"\r\n") {
            if !ok {
                return Err(ParseError::InvalidHeaderField);
            }
        } else {
            return Err(ParseError::PartialData);
        }

        self.header_count = parse_headers(&mut bytes, &mut self.headers)?;

        Ok(bytes.offset())
    }

    pub fn header(&self, field: &str) -> Option<&'buf [u8]> {
        for i in 0..self.header_count {
            if self.headers[i].field.eq_ignore_ascii_case(field) {
                return Some(self.headers[i].value);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct HttpResponse<'buf, 'h> {
    version: Version,
    code: u16,
    status: &'buf str,
    headers: &'h mut [Header<'buf>],
    header_count: usize,
}

impl<'buf, 'h> HttpResponse<'buf, 'h> {
    pub fn new(headers: &'h mut [Header<'buf>]) -> HttpResponse<'buf, 'h> {
        HttpResponse {
            version: Version::Unknown,
            code: 0,
            status: "",
            headers,
            header_count: 0,
        }
    }

    pub fn parse(&mut self, buffer: &'buf [u8]) -> Result<usize> {
        let mut bytes = BytesRef::from(buffer);

        self.version = parse_version(&mut bytes, false)?;

        // Nom whitespace
        if !bytes.consume(b' ').ok_or(ParseError::PartialData)? {
            return Err(ParseError::InvalidUri);
        }

        let code = bytes.consume_until(b' ').ok_or(ParseError::PartialData)?;
        if let Ok(code) = str::from_utf8(code) {
            self.code = if let Ok(v) = str::parse::<u16>(code) {
                v
            } else {
                return Err(ParseError::ParseStatusCode);
            };
        } else {
            return Err(ParseError::ParseUtf8Err);
        }

        // Nom whitespace
        if !bytes.consume(b' ').ok_or(ParseError::PartialData)? {
            return Err(ParseError::InvalidUri);
        }

        if let Some(status) = bytes.consume_until(b'\r') {
            self.status = if let Ok(s) = str::from_utf8(status) {
                s
            } else {
                return Err(ParseError::ParseUtf8Err);
            }
        }

        // Nom \r\n
        if let Some(ok) = bytes.consume_str(b"\r\n") {
            if !ok {
                return Err(ParseError::InvalidHeaderField);
            }
        } else {
            return Err(ParseError::PartialData);
        }

        self.header_count = parse_headers(&mut bytes, &mut self.headers)?;

        Ok(bytes.offset())
    }

    pub fn header(&self, field: &str) -> Option<&'buf [u8]> {
        for i in 0..self.header_count {
            if self.headers[i].field.eq_ignore_ascii_case(field) {
                return Some(self.headers[i].value);
            }
        }

        None
    }
}

fn parse_method(buffer: &mut BytesRef) -> Result<Method> {
    match buffer.consume_until(b' ') {
        Some(b"GET") => Ok(Method::GET),
        Some(b"POST") => Ok(Method::POST),
        Some(b"HEAD") => Ok(Method::HEAD),
        Some(b"PUT") => Ok(Method::PUT),
        Some(b"DELETE") => Ok(Method::DELETE),
        Some(b"CONNECT") => Ok(Method::CONNECT),
        Some(b"OPTIONS") => Ok(Method::OPTIONS),
        Some(b"TRACE") => Ok(Method::TRACE),
        Some(b"PATCH") => Ok(Method::PATCH),
        Some(s) => Ok(Method::Ext(str::from_utf8(s)?.to_string())),
        None => Err(ParseError::PartialData),
    }
}

fn parse_version(buffer: &mut BytesRef, is_request: bool) -> Result<Version> {
    if let Some(ok) = buffer.consume_str(b"HTTP/") {
        if !ok {
            return Err(ParseError::InvalidVersion);
        }
    } else {
        return Err(ParseError::PartialData);
    }

    let s = if is_request {
        buffer.consume_until(b'\r')
    } else {
        buffer.consume_until(b' ')
    };

    match s {
        Some(b"1.0") => Ok(Version::V10),
        Some(b"1.1") => Ok(Version::V11),
        Some(_) => Err(ParseError::InvalidVersion),
        None => Err(ParseError::PartialData),
    }
}

fn parse_headers<'a, 'buf>(
    buffer: &'a mut BytesRef<'buf>,
    headers: &mut &mut [Header<'buf>],
) -> Result<usize> {
    let mut header_count = 0usize;
    let mut iter = headers.iter_mut();

    loop {
        if let Some(ok) = buffer.consume_str(b"\r\n") {
            if ok {
                break;
            }
        } else {
            return Err(ParseError::PartialData);
        }

        let field = buffer.consume_until(b':');
        if field.is_none() {
            return Err(ParseError::PartialData);
        }

        if let Some(ok) = buffer.consume_str(b": ") {
            if !ok {
                return Err(ParseError::InvalidHeaderField);
            }
        } else {
            return Err(ParseError::PartialData);
        }

        let field = field.unwrap();
        let field = if let Ok(v) = str::from_utf8(field) {
            v
        } else {
            return Err(ParseError::ParseUtf8Err);
        };

        let value = buffer.consume_until(b'\r');
        if value.is_none() {
            return Err(ParseError::PartialData);
        }

        if let Some(ok) = buffer.consume_str(b"\r\n") {
            if !ok {
                return Err(ParseError::InvalidHeaderValue);
            }
        } else {
            return Err(ParseError::PartialData);
        }

        let header = match iter.next() {
            Some(header) => header,
            None => return Err(ParseError::TooManyHeader),
        };

        header.field = field;
        header.value = value.unwrap();
        header_count += 1;
    }

    Ok(header_count)
}
