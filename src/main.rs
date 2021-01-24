// 次の例は、HTTPサーバーを別のスレッドで起動する方法を示しています。

use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8080")?
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();

    // 新しい接続の受け入れを一時停止します
    srv.pause().await;

    // 新しい接続の受け入れを再開します
    srv.resume().await;

    // サーバーを停止します
    srv.stop(true).await;

}