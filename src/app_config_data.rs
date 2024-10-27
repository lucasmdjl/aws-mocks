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
use aws_sdk_appconfigdata::operation::get_latest_configuration::{builders::*, *};
use aws_sdk_appconfigdata::operation::start_configuration_session::{builders::*, *};
use aws_sdk_appconfigdata::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appconfigdata::Client;
use std::ops::Deref;

pub use aws_sdk_appconfigdata::*;

pub struct AppConfigDataClientImpl(Client);
impl AppConfigDataClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppConfigDataClient {
    fn get_latest_configuration(&self, builder: GetLatestConfigurationInputBuilder) -> impl Future<Output = Result<GetLatestConfigurationOutput, SdkError<GetLatestConfigurationError>>> + Send;
    fn start_configuration_session(&self, builder: StartConfigurationSessionInputBuilder) -> impl Future<Output = Result<StartConfigurationSessionOutput, SdkError<StartConfigurationSessionError>>> + Send;
}
impl AppConfigDataClient for AppConfigDataClientImpl {
    fn get_latest_configuration(&self, builder: GetLatestConfigurationInputBuilder) -> impl Future<Output = Result<GetLatestConfigurationOutput, SdkError<GetLatestConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn start_configuration_session(&self, builder: StartConfigurationSessionInputBuilder) -> impl Future<Output = Result<StartConfigurationSessionOutput, SdkError<StartConfigurationSessionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppConfigDataClient for T
where T: Deref,
      T::Target: AppConfigDataClient {
    fn get_latest_configuration(&self, builder: GetLatestConfigurationInputBuilder) -> impl Future<Output = Result<GetLatestConfigurationOutput, SdkError<GetLatestConfigurationError>>> {
        self.deref().get_latest_configuration(builder)
    }
    fn start_configuration_session(&self, builder: StartConfigurationSessionInputBuilder) -> impl Future<Output = Result<StartConfigurationSessionOutput, SdkError<StartConfigurationSessionError>>> {
        self.deref().start_configuration_session(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppConfigDataClient {}
    impl AppConfigDataClient for edAppConfigDataClient {
        async fn get_latest_configuration(&self, builder: GetLatestConfigurationInputBuilder) -> Result<GetLatestConfigurationOutput, SdkError<GetLatestConfigurationError>>;
        async fn start_configuration_session(&self, builder: StartConfigurationSessionInputBuilder) -> Result<StartConfigurationSessionOutput, SdkError<StartConfigurationSessionError>>;
    }
}
