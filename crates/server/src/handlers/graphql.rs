use std::str::FromStr;

use async_graphql::http::GraphiQLSource;
use async_graphql::ServerError;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::Extension;

use crate::graphql::schema::GraphQLSchema;
use crate::services::auth::Token;

pub async fn schema(
    Extension(schema): Extension<GraphQLSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(maybe_token) = headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(Token::from_str).ok())
    {
        match maybe_token {
            Ok(token) => req = req.data(token),
            Err(err) => {
                return GraphQLResponse::from(async_graphql::Response::from_errors(vec![
                    ServerError::new(err.to_string(), None),
                ]));
            }
        }
    }

    schema.execute(req).await.into()
}

pub async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
