

// HttpServerタイプは、HTTPリクエストの処理を担当します。

// HttpServerは、アプリケーションファクトリをパラメータとして受け入れ、
// アプリケーションファクトリには送信と同期の境界が必要です。
// 詳細については、マルチスレッドのセクションをご覧ください。

// 特定のソケットアドレスにバインドするには、bind（）を使用する必要があり、複数回呼び出すことができます。
//  sslソケットをバインドするには、bind_openssl（）またはbind_rustls（）を使用する必要があります。 
// HTTPサーバーを実行するには、HttpServer :: run（）メソッドを使用します。

use actix_web::{ web, App, HttpResponse, HttpServer };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// run（）メソッドは、サーバータイプのインスタンスを返します。
// サーバータイプのメソッドは、HTTPサーバーの管理に使用できます

// pause（）-着信接続の受け入れを一時停止します
// resume（）-着信接続の受け入れを再開します
// stop（）-着信接続処理を停止し、すべてのワーカーを停止して終了します

