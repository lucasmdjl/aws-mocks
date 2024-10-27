/*
 * aws-mocks - A mocking library for AWS.
 *
 * Copyright (C) 2024 Lucas M. de Jong Larrarte
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use aws_sdk_apigatewaymanagement::operation::delete_connection::{builders::*, *};
use aws_sdk_apigatewaymanagement::operation::get_connection::{builders::*, *};
use aws_sdk_apigatewaymanagement::operation::post_to_connection::{builders::*, *};
use aws_sdk_apigatewaymanagement::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_apigatewaymanagement::Client;
use std::ops::Deref;

pub use aws_sdk_apigatewaymanagement::*;

pub struct ApiGatewayManagementClientImpl(Client);
impl ApiGatewayManagementClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ApiGatewayManagementClient {
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> + Send;
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>> + Send;
    fn post_to_connection(&self, builder: PostToConnectionInputBuilder) -> impl Future<Output = Result<PostToConnectionOutput, SdkError<PostToConnectionError>>> + Send;
}
impl ApiGatewayManagementClient for ApiGatewayManagementClientImpl {
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn post_to_connection(&self, builder: PostToConnectionInputBuilder) -> impl Future<Output = Result<PostToConnectionOutput, SdkError<PostToConnectionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> ApiGatewayManagementClient for T
where T: Deref,
      T::Target: ApiGatewayManagementClient {
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        self.deref().delete_connection(builder)
    }
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>> {
        self.deref().get_connection(builder)
    }
    fn post_to_connection(&self, builder: PostToConnectionInputBuilder) -> impl Future<Output = Result<PostToConnectionOutput, SdkError<PostToConnectionError>>> {
        self.deref().post_to_connection(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edApiGatewayManagementClient {}
    impl ApiGatewayManagementClient for edApiGatewayManagementClient {
        async fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>;
        async fn get_connection(&self, builder: GetConnectionInputBuilder) -> Result<GetConnectionOutput, SdkError<GetConnectionError>>;
        async fn post_to_connection(&self, builder: PostToConnectionInputBuilder) -> Result<PostToConnectionOutput, SdkError<PostToConnectionError>>;
    }
}
