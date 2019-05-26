use crate::activitydesk::http::user_agent;
use elefren::http_send::HttpSend;
use elefren::Result;
use reqwest::{Client, Request, RequestBuilder, Response};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sender {}

impl HttpSend for Sender {
    fn send(&self, client: &Client, builder: RequestBuilder) -> Result<Response> {
        let request = builder.header("User-Agent", user_agent()).build()?;
        return self.execute(client, request);
    }
    fn execute(&self, client: &Client, request: Request) -> Result<Response> {
        return Ok(client.execute(request)?);
    }
}
