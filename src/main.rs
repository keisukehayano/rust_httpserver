// Actix-webは、actix-web-actorsクレートでWebSocketをサポートします。
// リクエストのペイロードをweb :: Payloadでws :: Messageのストリームに変換してから、
// ストリームコンビネータを使用して実際のメッセージを処理することは可能ですが、
// httpアクターとのWebSocket通信を処理する方が簡単です。

// 以下は、単純なWebSocketエコーサーバーの例です。

use actix::{ Actor, StreamHandler };
use actix_web::{ web, App, Error, HttpResponse, HttpRequest, HttpServer };
use actix_web_actors::ws;

// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

// Handler for we::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let test = &req;
    println!("test: {:?}", test);
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}