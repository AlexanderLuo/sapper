use std::net::SocketAddr;

use hyper::server::Request as HyperRequest;
use hyper::Method;
use hyper::HttpVersion;
use hyper::header::Headers;
use hyper::Body;
use typemap::TypeMap;

pub struct SapperRequest<B = Body> {
//    raw_req: &HyperRequest,
    raw_req: Box<HyperRequest<B>>,
    ext: TypeMap
} 

impl<B> SapperRequest<B> {
    pub fn new(req: Box<HyperRequest<B>>) -> SapperRequest<B> {

        SapperRequest {
            raw_req: req,
            ext: TypeMap::new()
        }
    }
    
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.raw_req.remote_addr()
    }
    
    pub fn method(&self) -> &Method {
        self.raw_req.method()
    }
    
    pub fn version(&self) -> HttpVersion {
        self.raw_req.version()
    }
    
    pub fn headers(&self) -> &Headers {
        self.raw_req.headers()
    }
    
    pub fn path(&self) -> &str {
        self.raw_req.path()
    }
    
    pub fn query(&self) -> Option<&str> {
        self.raw_req.query()
    }
    
    pub fn body_ref(&self) -> Option<&B> {
        self.raw_req.body_ref()
    }
    
    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }
    
    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}

impl SapperRequest<Body> {
    pub fn body(self) -> Body {
        self.raw_req.body()
    }
}


