use iron::mime::Mime;
use iron::headers::ContentType;
use iron::{AfterMiddleware, Request, Response, IronResult, Handler, Chain};

struct TextPlainUtf8;
impl AfterMiddleware for TextPlainUtf8 {
    fn after(&self, _: &mut Request, mut resp: Response) -> IronResult<Response> {
        let text_plain: Mime = "text/plain".parse().unwrap();
        // ContentType::plaintext() create "Content-Type: text/plain; charset=utf-8" header
        if resp.headers.get::<ContentType>() == Some(&ContentType(text_plain)) {
            resp.headers.set(ContentType::plaintext());
        }
        Ok(resp)
    }
}

pub fn chainer<H: Handler>(handler: H) -> Chain {
    let mut handler = Chain::new(handler);
    handler.link_after(TextPlainUtf8);
    handler
}
