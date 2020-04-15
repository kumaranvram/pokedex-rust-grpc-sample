#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(string, tag = "1")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pokemon {
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(enumeration = "PokemonType", repeated, tag = "3")]
    pub r#type: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PokemonResponse {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(enumeration = "PokemonType", repeated, tag = "3")]
    pub pokemon_type: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PokedexEntryResponse {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PokemonType {
    Normal = 0,
    Fire = 1,
    Ground = 2,
    Water = 3,
    Grass = 4,
    Psychic = 5,
    Ghost = 6,
    Ice = 7,
    Steel = 8,
    Poison = 9,
    Flying = 10,
    Fighting = 11,
    Electric = 12,
}
#[doc = r" Generated client implementations."]
pub mod poke_dex_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PokeDexClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PokeDexClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PokeDexClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get_pokemon_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::Query>,
        ) -> Result<tonic::Response<super::PokemonResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pokedex.PokeDex/GetPokemonByName");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pokemons_by_type(
            &mut self,
            request: impl tonic::IntoRequest<super::Query>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::PokemonResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pokedex.PokeDex/GetPokemonsByType");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn make_pokedex_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::Pokemon>,
        ) -> Result<tonic::Response<super::PokedexEntryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pokedex.PokeDex/MakePokedexEntry");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PokeDexClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PokeDexClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PokeDexClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod poke_dex_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PokeDexServer."]
    #[async_trait]
    pub trait PokeDex: Send + Sync + 'static {
        async fn get_pokemon_by_name(
            &self,
            request: tonic::Request<super::Query>,
        ) -> Result<tonic::Response<super::PokemonResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the GetPokemonsByType method."]
        type GetPokemonsByTypeStream: Stream<Item = Result<super::PokemonResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_pokemons_by_type(
            &self,
            request: tonic::Request<super::Query>,
        ) -> Result<tonic::Response<Self::GetPokemonsByTypeStream>, tonic::Status>;
        async fn make_pokedex_entry(
            &self,
            request: tonic::Request<super::Pokemon>,
        ) -> Result<tonic::Response<super::PokedexEntryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PokeDexServer<T: PokeDex> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PokeDex> PokeDexServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PokeDexServer<T>
    where
        T: PokeDex,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pokedex.PokeDex/GetPokemonByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetPokemonByNameSvc<T: PokeDex>(pub Arc<T>);
                    impl<T: PokeDex> tonic::server::UnaryService<super::Query> for GetPokemonByNameSvc<T> {
                        type Response = super::PokemonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Query>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_pokemon_by_name(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPokemonByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pokedex.PokeDex/GetPokemonsByType" => {
                    #[allow(non_camel_case_types)]
                    struct GetPokemonsByTypeSvc<T: PokeDex>(pub Arc<T>);
                    impl<T: PokeDex> tonic::server::ServerStreamingService<super::Query> for GetPokemonsByTypeSvc<T> {
                        type Response = super::PokemonResponse;
                        type ResponseStream = T::GetPokemonsByTypeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Query>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_pokemons_by_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetPokemonsByTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pokedex.PokeDex/MakePokedexEntry" => {
                    #[allow(non_camel_case_types)]
                    struct MakePokedexEntrySvc<T: PokeDex>(pub Arc<T>);
                    impl<T: PokeDex> tonic::server::UnaryService<super::Pokemon> for MakePokedexEntrySvc<T> {
                        type Response = super::PokedexEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Pokemon>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.make_pokedex_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MakePokedexEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PokeDex> Clone for PokeDexServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PokeDex> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PokeDex> tonic::transport::NamedService for PokeDexServer<T> {
        const NAME: &'static str = "pokedex.PokeDex";
    }
}
