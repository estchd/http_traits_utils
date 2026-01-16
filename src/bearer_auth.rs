use url::Url;
use http_traits::base_url_client::BaseUrlClient;
use http_traits::basic_authenticated_client::BasicAuthenticatedClient;
use http_traits::bearer_authenticated_client::BearerAuthenticatedClient;
use http_traits::client::Client;

pub trait ToBearerAuthClient<T: Client> {
	fn to_bearer_auth_client(&'_ self, token: &str) -> BearerAuthClient<'_, T>;
}

impl <T: Client> ToBearerAuthClient<T> for T {
	fn to_bearer_auth_client(&'_ self, token: &str) -> BearerAuthClient<'_, T> {
		BearerAuthClient {
			client: self,
			token: token.to_owned(),
		}
	}
}

pub struct BearerAuthClient<'a, T: Client> {
	client: &'a T,
	token: String,
}

impl<'a, T: Client> Client for BearerAuthClient<'a, T> {
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

impl<'a, T: Client> BearerAuthenticatedClient for BearerAuthClient<'a, T> {
	fn authentication_token(&self) -> &str {
		&self.token
	}
}

impl<'a, T: BaseUrlClient> BaseUrlClient for BearerAuthClient<'a, T> {
	fn base_url(&self) -> &Url {
		self.client.base_url()
	}
}

impl<'a, T: BasicAuthenticatedClient> BasicAuthenticatedClient for BearerAuthClient<'a, T> {
	fn username(&self) -> &str {
		self.client.username()
	}

	fn password(&self) -> Option<&str> {
		self.client.password()
	}
}