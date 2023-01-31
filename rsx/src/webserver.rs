use actix_service::{IntoServiceFactory, Service, ServiceFactory};
use actix_web::{dev::*, error::Error, Result, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_http::{body::MessageBody, Request, Response};
use std::{net::ToSocketAddrs, fmt};
use actix_server::Server;

/// A wrapper around [`actix_web::HttpServer`](https://docs.rs/actix-web/latest/actix_web/struct.HttpServer.html) with useful utilities
///
/// Implements: [`Debug`](`core::fmt::Debug`), [`Clone`](`core::clone::Clone`)
///
/// Example Usage:
/// ```rust,no_run
/// use rsx::*;
/// 
/// #[rsx::main]
/// async fn main() -> std::io::Result<()> {
///     WebServer::new(||{
///         App::new()
///         .service(index)
///     })
///     .https("127.0.0.1:8080")?
///     .run().await
/// }
/// ```
pub struct WebServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig>,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody
{
    server: actix_web::HttpServer<F, I, S, B>,
    factory: F,
}

impl<F, I, S, B> WebServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig> + 'static,
    S::Error: Into<Error> + 'static,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>> + 'static,
    <S::Service as Service<Request>>::Future: 'static,
    S::Service: 'static,
    B: MessageBody + 'static,
{

    /// Create a new [`WebServer`] with the given factory
    /// 
    /// Example:
    /// ```rust,no_run
    /// use actix_web::App;
    /// use rsx::*;
    /// 
    /// #[rsx::main]
    /// async fn main() -> std::io::Result<()> {
    ///     WebServer::new(||{
    ///         App::new()
    ///         .service(index)
    ///     })
    ///     .https("127.0.0.1:8080")?
    ///     .run().await
    /// }
    /// ```
    pub fn new(f: F) -> Self {
        let f2 = f.clone();
        Self {
            server: actix_web::HttpServer::new(f),
            factory: f2
        }
    }

    /// Bind to a socket address and start listening for incoming connections
    /// 
    /// Uses Https, requires `key.pem` and `cert.pem` in crate root
    pub fn https<Addr: ToSocketAddrs>(mut self, addr: Addr) -> Result<Self, std::io::Error> {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
        let server = self.server.bind_openssl(addr, builder)?;
        self.server = server;
        Ok(self)
    }

    /// Reveals internal server
    #[doc(hidden)]
    #[allow(clippy::needless_return)]
    pub unsafe fn get_server(&mut self) -> &mut actix_web::HttpServer<F, I, S, B> {
        return &mut self.server;
    }
}

impl<F, I, S, B> WebServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig>,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    S::Service: 'static,
    B: MessageBody,
{
    /// Runs the webserver
    /// 
    /// Example:
    /// ```rust,no_run
    /// #[rsx::main]
    /// async fn main() -> std::io::Result<()> {
    ///     WebServer::new(||{
    ///         App::new()
    ///         .service(index)
    ///     })
    ///     .https("127.0.0.1:8080")?
    ///     .run().await
    /// }
    /// ```
    pub fn run(self) -> Server {
        self.server.run()
    }
}

impl<F, I, S, B> fmt::Debug for WebServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig>,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebServer")
            .field("server", &"actix_web::HttpServer")
            .finish()
    }
}

impl<F, I, S, B> core::clone::Clone for WebServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig> + 'static,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody + 'static
{
    fn clone(&self) -> Self {
        let f = self.factory.clone();
        Self::new(f)
    }
}

/// A simple Wrapper around [`Result`](`actix_web::Result<actix_web::HttpResponse>`)
pub type HttpResult<E = Error> = Result<HttpResponse, E>;
