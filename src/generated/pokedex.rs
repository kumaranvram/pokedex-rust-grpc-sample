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
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(enumeration = "PokemonType", repeated, tag = "3")]
    pub r#type: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PokemonResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, optional, tag = "2")]
    pub pokemon: ::std::option::Option<Pokemon>,
    #[prost(message, optional, tag = "3")]
    pub error: ::std::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PokemonsResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, repeated, tag = "2")]
    pub pokemons: ::std::vec::Vec<Pokemon>,
    #[prost(message, optional, tag = "3")]
    pub error: ::std::option::Option<Error>,
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
}
#[doc = r" Generated client implementations."]
pub mod poke_dex_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PokeDexServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PokeDexServiceClient<tonic::transport::Channel> {
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
    impl<T> PokeDexServiceClient<T>
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
            let path =
                http::uri::PathAndQuery::from_static("/pokedex.PokeDexService/GetPokemonByName");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pokemons_by_type(
            &mut self,
            request: impl tonic::IntoRequest<super::Query>,
        ) -> Result<tonic::Response<super::PokemonsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/pokedex.PokeDexService/GetPokemonsByType");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn make_pokedex_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::Pokemon>,
        ) -> Result<tonic::Response<super::PokemonResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/pokedex.PokeDexService/MakePokedexEntry");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PokeDexServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PokeDexServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PokeDexServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod poke_dex_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PokeDexServiceServer."]
    #[async_trait]
    pub trait PokeDexService: Send + Sync + 'static {
        async fn get_pokemon_by_name(
            &self,
            request: tonic::Request<super::Query>,
        ) -> Result<tonic::Response<super::PokemonResponse>, tonic::Status>;
        async fn get_pokemons_by_type(
            &self,
            request: tonic::Request<super::Query>,
        ) -> Result<tonic::Response<super::PokemonsResponse>, tonic::Status>;
        async fn make_pokedex_entry(
            &self,
            request: tonic::Request<super::Pokemon>,
        ) -> Result<tonic::Response<super::PokemonResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PokeDexServiceServer<T: PokeDexService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PokeDexService> PokeDexServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for PokeDexServiceServer<T>
    where
        T: PokeDexService,
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
                "/pokedex.PokeDexService/GetPokemonByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetPokemonByNameSvc<T: PokeDexService>(pub Arc<T>);
                    impl<T: PokeDexService> tonic::server::UnaryService<super::Query> for GetPokemonByNameSvc<T> {
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
                "/pokedex.PokeDexService/GetPokemonsByType" => {
                    #[allow(non_camel_case_types)]
                    struct GetPokemonsByTypeSvc<T: PokeDexService>(pub Arc<T>);
                    impl<T: PokeDexService> tonic::server::UnaryService<super::Query> for GetPokemonsByTypeSvc<T> {
                        type Response = super::PokemonsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Query>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_pokemons_by_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPokemonsByTypeSvc(inner);
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
                "/pokedex.PokeDexService/MakePokedexEntry" => {
                    #[allow(non_camel_case_types)]
                    struct MakePokedexEntrySvc<T: PokeDexService>(pub Arc<T>);
                    impl<T: PokeDexService> tonic::server::UnaryService<super::Pokemon> for MakePokedexEntrySvc<T> {
                        type Response = super::PokemonResponse;
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
    impl<T: PokeDexService> Clone for PokeDexServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PokeDexService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PokeDexService> tonic::transport::NamedService for PokeDexServiceServer<T> {
        const NAME: &'static str = "pokedex.PokeDexService";
    }
}
