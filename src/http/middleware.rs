pub mod authorization {
    use axum::{
        extract::{Request, State},
        http::StatusCode,
        middleware::Next,
        response::Response,
    };
    use axum_extra::{
        extract::TypedHeader,
        headers::{Authorization, authorization::Bearer},
    };

    use crate::{result_trace::ResultTrace as _, state::AppState};

    pub async fn authorize_middleware(
        State(state): State<AppState>,
        TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let token_str = auth_header.token();

        let claims = state
            .jwt_codec()
            .decode(token_str)
            .trace_warn()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        request.extensions_mut().insert(claims);

        Ok(next.run(request).await)
    }
}
