use url::Url;
use http_traits::base_url_client::BaseUrlClient;
use http_traits::basic_authenticated_client::BasicAuthenticatedClient;
use http_traits::bearer_authenticated_client::BearerAuthenticatedClient;
use http_traits::client::Client;

pub trait ToBasicAuthClient<T: Client> {
	fn to_basic_auth_client(&'_ self, username: &str, password: Option<&str>) -> BasicAuthClient<'_, T>;
}

impl <T: Client> ToBasicAuthClient<T> for T {
	fn to_basic_auth_client(&'_ self, username: &str, password: Option<&str>) -> BasicAuthClient<'_, T> {
		BasicAuthClient {
			client: self,
			username: username.to_owned(),
			password: password.map(String::from),
		}
	}
}

#[derive(Clone, Debug)]
pub struct BasicAuthClient<'a, T: Client> {
	client: &'a T,
	username: String,
	password: Option<String>,
}

impl<'a, T: Client> Client for BasicAuthClient<'a, T> {
	type Request = T::Request;
	type Response = T::Response;
	type Error = T::Error;

	async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
		self.client.execute(request).await
	}

	type Method = T::Method;
	type RequestBuilder = T::RequestBuilder;

	fn request(&self, method: Self::Method, url: &url::Url) -> Self::RequestBuilder {
		self.client.request(method, url)
	}
}

impl<'a, T: Client> BasicAuthenticatedClient for BasicAuthClient<'a, T> {
	fn username(&self) -> &str {
		&self.username
	}

	fn password(&self) -> Option<&str> {
		self.password.as_deref()
	}
}

impl<'a, T: BaseUrlClient> BaseUrlClient for BasicAuthClient<'a, T> {
	fn base_url(&self) -> &Url {
		self.client.base_url()
	}
}

impl<'a, T: BearerAuthenticatedClient> BearerAuthenticatedClient for BasicAuthClient<'a, T> {
	fn authentication_token(&self) -> &str {
		self.client.authentication_token()
	}
}