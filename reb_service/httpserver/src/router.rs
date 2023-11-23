use std::io::Write;
use http::httpreponse::HttpResponse;
use http::httprequest::{HttpRequest, Method, Resource};

pub struct Router;

impl Router{
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> (){
        match req.method {
            Method::Get => match &req.resource { Resource::Path(std) => {
                let route: Vec<&str> = std.split("/").collect();
                match route[1] { "api" => {
                    let resp: HttpResponse = WebServiceHandler::handler(&req);
                    let _ = resp.send_response(stream);
                }
                _ => {
                    let resp: HttpResponse = StaticPageHandeler::handler(&req);
                    let _ = resp.send_response(stream);
                }}
            } }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}