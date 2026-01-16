use url::{Url};
use http_traits::basic_authenticated_client::BasicAuthenticatedClient;
use http_traits::bearer_authenticated_client::BearerAuthenticatedClient;
use http_traits::client::Client;

pub trait ToBaseUrlClient<T: Client> {
	fn to_base_url_client(&'_ self, base_url: Url) -> BaseUrlClient<'_, T>;
}

impl <T: Client> ToBaseUrlClient<T> for T {
	fn to_base_url_client(&'_ self, base_url: Url) -> BaseUrlClient<'_, T> {
		BaseUrlClient {
			client: self,
			base_url
		}
	}
}


pub struct BaseUrlClient<'a, T: Client> {
	client: &'a T,
	base_url: Url
}

impl<'a, T: Client> http_traits::base_url_client::BaseUrlClient for BaseUrlClient<'a, T> {
	fn base_url(&self) -> &Url {
		&self.base_url
	}
}

impl<'a, T: Client> Client for BaseUrlClient<'a, T> {
	type Request = T::Request;
	type Response = T::Response;
	type Error = T::Error;

	async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
		self.client.execute(request).await
	}

	type Method = T::Method;
	type RequestBuilder = T::RequestBuilder;

	fn request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder {
		self.client.request(method, &url)
	}
}

impl<'a, T: BearerAuthenticatedClient> BearerAuthenticatedClient for BaseUrlClient<'a, T> {
	fn authentication_token(&self) -> &str {
		self.client.authentication_token()
	}
}

impl<'a, T: BasicAuthenticatedClient> BasicAuthenticatedClient for BaseUrlClient<'a, T> {
	fn username(&self) -> &str {
		self.client.username()
	}

	fn password(&self) -> Option<&str> {
		self.client.password()
	}
}