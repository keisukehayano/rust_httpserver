// Actixは、キープアライブ接続で要求を待機できます。

// 存続接続の動作は、サーバー設定によって定義されます。

// 75、Some（75）、KeepAlive :: Timeout（75）-75秒のキープアライブタイマーを有効にします。
// NoneまたはKeepAlive :: Disabled-キープアライブを無効にします。
// KeepAlive :: Tcp（75）-SO_KEEPALIVEソケットオプションを使用します。

use actix_web::{ http, web, App, HttpResponse, HttpServer, HttpRequest };

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    let one = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(75);            // <- Set keep-alive to 75 seconds

    // let _two = HttpServer::new(|| {
    //     App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    // })
    // .keep_alive(); // <- Use `SO_KEEPALIVE` socket option.

    let three = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(None);
    

  one.bind("127.0.0.1:8080")?.run().await
}

// 上記の最初のオプションが選択されている場合、キープアライブ状態は応答の接続タイプに基づいて計算されます。
// デフォルトでは、HttpResponse :: connection_typeは定義されていません。
// その場合、キープアライブはリクエストのHTTPバージョンによって定義されます。

// keep alive is off for HTTP/1.0 and is on for HTTP/1.1 and HTTP/2.0.

// 接続タイプは、HttpResponseBuilder :: connection_type（）メソッドを使用して変更できます。

//async fn index(req: HttpRequest) -> HttpResponse {
//    HttpResponse::Ok()
//        .connection_type(http::ConnectionType::Close) // <- Close connection
//        .force_close() // <- Alternative method
//        .finish()
//}