// sslサーバーには、rustlsとopensslの2つの機能があります。 
// rustls機能はrustls統合用であり、opensslはopenssl用です。

// SSLサーバを作成する場合は、以下をCargo.tomlファイルに追記
// [dependencies]
// actix-web = { version = "3", features = ["openssl"] }
// openssl = { version = "0.10" }

use actix_web::{ get, App, HttpRequest, HttpServer, Responder };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };


#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcom!!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load ssl keys
    // テスト用の自己署名一時証明書を作成するには：
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}

// 注：HTTP /2.0プロトコルにはtlsalpnが必要です。
// 現時点では、opensslのみがalpnをサポートしています。
// 完全な例については、examples / opensslを確認してください。

// key.pemおよびcert.pemを作成するには、コマンドを使用します。あなた自身の主題を記入してください

// 以下をターミナルで実行
// $ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
//   -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"

// パスワードを削除するには、nopass.pemをkey.pemにコピーします
// $ openssl rsa -in key.pem -out nopass.pem