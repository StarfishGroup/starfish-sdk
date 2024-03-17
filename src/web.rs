use anyhow::Result;
use axum::{
	extract::{FromRequest, Request},
	http::StatusCode,
	response::{IntoResponse, Response},
	Json, RequestExt,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{
	borrow::Cow,
	fmt::{Debug, Display},
};

pub use axum::{
	routing::{get as WebGetRoute, post as WebPostRoute},
	Router as WebRouter,
};

#[derive(Debug)]
pub struct WebErrMsg(pub i64, pub Cow<'static, str>);

impl Display for WebErrMsg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "错误码({}), 消息({})", self.0, self.1)
	}
}

#[derive(Debug, Clone)]
pub enum WebCode {
	Service,
	RequestParam(String),
}

impl WebCode {
	pub fn to_err_msg(&self) -> WebErrMsg {
		let (code, msg) = match self {
			Self::Service => (1, Cow::from("服务异常")),
			Self::RequestParam(msg) => (2, Cow::from(format!("请求参数错误({})", msg))),
		};
		WebErrMsg(code, msg)
	}
}

pub struct WebError(pub anyhow::Error);

impl IntoResponse for WebError {
	fn into_response(self) -> Response {
		tracing::error!("{}", self.0.to_string());
		let code: i64;
		let msg: Cow<'static, str>;
		if let Some(err_msg) = self.0.downcast_ref::<WebErrMsg>() {
			code = err_msg.0;
			msg = err_msg.1.to_owned();
		} else {
			let err_msg = WebCode::Service.to_err_msg();
			code = err_msg.0;
			msg = err_msg.1.to_owned();
		}
		(
			StatusCode::OK,
			Json(json!({
				"code" : code,
				"msg" :msg,
				"data" : Value::Null,
			})),
		)
			.into_response()
	}
}

impl<E> From<E> for WebError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

pub type WebResult<T> = core::result::Result<T, WebError>;

pub struct WebJson<T>(pub T);

impl<T: serde::Serialize> IntoResponse for WebJson<T> {
	fn into_response(self) -> Response {
		(
			StatusCode::OK,
			Json(json!({
				"code":0,
				"msg":Value::Null,
				"data":self.0,
			})),
		)
			.into_response()
	}
}

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for WebJson<T>
where
	T: DeserializeOwned,
	S: Send + Sync,
	T: 'static,
{
	type Rejection = Response;

	async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
		match req.extract().await {
			Ok(Json(payload)) => return Ok(Self(payload)),
			Err(err) => {
				tracing::error!("{}", err.to_string());
				let err_msg = WebCode::RequestParam(err.to_string()).to_err_msg();
				Err((
					StatusCode::OK,
					Json(json!({
						"code":err_msg.0,
						"msg":err_msg.1,
						"data":Value::Null,
					})),
				)
					.into_response())
			},
		}
	}
}

pub async fn init_web<A: tokio::net::ToSocketAddrs>(bind: A, register: fn(WebRouter) -> WebRouter) -> Result<()> {
	let router = WebRouter::new();
	let router = register(router);
	let router = router.layer(tower_http::trace::TraceLayer::new_for_http().make_span_with(tracing::Span::current()));
	let listener = tokio::net::TcpListener::bind(bind).await?;
	axum::serve(listener, router).await?;
	Ok(())
}
