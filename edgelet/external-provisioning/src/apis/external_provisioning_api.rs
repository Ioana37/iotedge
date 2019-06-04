/*
 * IoT Edge External Provisioning Environment API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2019-04-10
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::borrow::Borrow;
use std::sync::Arc;

use futures;
use futures::{Future, Stream};
use hyper;
use serde_json;

use typed_headers::{self, http};

use super::{configuration, Error};

pub struct ExternalProvisioningApiClient<C: hyper::client::connect::Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ExternalProvisioningApiClient<C> {
    pub fn new(
        configuration: Arc<configuration::Configuration<C>>,
    ) -> ExternalProvisioningApiClient<C> {
        ExternalProvisioningApiClient { configuration }
    }
}

pub trait ExternalProvisioningApi: Send + Sync {
    fn get_device_provisioning_information(
        &self,
        api_version: &str,
    ) -> Box<
        dyn Future<Item = crate::models::DeviceProvisioningInfo, Error = Error<serde_json::Value>>
            + Send,
    >;
}

impl<C: hyper::client::connect::Connect> ExternalProvisioningApi
    for ExternalProvisioningApiClient<C>
where
    C: hyper::client::connect::Connect + 'static,
    <C as hyper::client::connect::Connect>::Transport: 'static,
    <C as hyper::client::connect::Connect>::Future: 'static,
{
    fn get_device_provisioning_information(
        &self,
        api_version: &str,
    ) -> Box<
        dyn Future<Item = crate::models::DeviceProvisioningInfo, Error = Error<serde_json::Value>>
            + Send,
    > {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::GET;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("api-version", &api_version.to_string())
            .finish();
        let uri_str = format!("/device/provisioninginformation?{}", query);

        let uri = (configuration.uri_composer)(&configuration.base_path, &uri_str);
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::builder();
        req.method(method).uri(uri.unwrap());
        if let Some(ref user_agent) = configuration.user_agent {
            req.header(http::header::USER_AGENT, &**user_agent);
        }

        let req = req
            .body(hyper::Body::empty())
            .expect("could not build hyper::Request");

        //        if let Some(ref sas_token) = configuration.sas_token {
        //            req.headers_mut().set(Authorization(sas_token.clone()));
        //        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let (http::response::Parts { status, .. }, body) = resp.into_parts();
                    body.concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(Error::from)
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<crate::models::DeviceProvisioningInfo, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
