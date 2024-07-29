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
use aws_sdk_mq::operation::create_broker::{builders::*, *};
use aws_sdk_mq::operation::create_configuration::{builders::*, *};
use aws_sdk_mq::operation::create_tags::{builders::*, *};
use aws_sdk_mq::operation::create_user::{builders::*, *};
use aws_sdk_mq::operation::delete_broker::{builders::*, *};
use aws_sdk_mq::operation::delete_tags::{builders::*, *};
use aws_sdk_mq::operation::delete_user::{builders::*, *};
use aws_sdk_mq::operation::describe_broker::{builders::*, *};
use aws_sdk_mq::operation::describe_broker_engine_types::{builders::*, *};
use aws_sdk_mq::operation::describe_broker_instance_options::{builders::*, *};
use aws_sdk_mq::operation::describe_configuration::{builders::*, *};
use aws_sdk_mq::operation::describe_configuration_revision::{builders::*, *};
use aws_sdk_mq::operation::describe_user::{builders::*, *};
use aws_sdk_mq::operation::list_brokers::{builders::*, *};
use aws_sdk_mq::operation::list_configuration_revisions::{builders::*, *};
use aws_sdk_mq::operation::list_configurations::{builders::*, *};
use aws_sdk_mq::operation::list_tags::{builders::*, *};
use aws_sdk_mq::operation::list_users::{builders::*, *};
use aws_sdk_mq::operation::promote::{builders::*, *};
use aws_sdk_mq::operation::reboot_broker::{builders::*, *};
use aws_sdk_mq::operation::update_broker::{builders::*, *};
use aws_sdk_mq::operation::update_configuration::{builders::*, *};
use aws_sdk_mq::operation::update_user::{builders::*, *};
use aws_sdk_mq::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_mq::Client;
use std::ops::Deref;

pub use aws_sdk_mq::*;

pub struct MQClientImpl(Client);
impl MQClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait MQClient {
    fn create_broker(&self, builder: CreateBrokerInputBuilder) -> impl Future<Output = Result<CreateBrokerOutput, SdkError<CreateBrokerError>>>;
    fn create_configuration(&self, builder: CreateConfigurationInputBuilder) -> impl Future<Output = Result<CreateConfigurationOutput, SdkError<CreateConfigurationError>>>;
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>>;
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>>;
    fn delete_broker(&self, builder: DeleteBrokerInputBuilder) -> impl Future<Output = Result<DeleteBrokerOutput, SdkError<DeleteBrokerError>>>;
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn describe_broker(&self, builder: DescribeBrokerInputBuilder) -> impl Future<Output = Result<DescribeBrokerOutput, SdkError<DescribeBrokerError>>>;
    fn describe_broker_engine_types(&self, builder: DescribeBrokerEngineTypesInputBuilder) -> impl Future<Output = Result<DescribeBrokerEngineTypesOutput, SdkError<DescribeBrokerEngineTypesError>>>;
    fn describe_broker_instance_options(&self, builder: DescribeBrokerInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeBrokerInstanceOptionsOutput, SdkError<DescribeBrokerInstanceOptionsError>>>;
    fn describe_configuration(&self, builder: DescribeConfigurationInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOutput, SdkError<DescribeConfigurationError>>>;
    fn describe_configuration_revision(&self, builder: DescribeConfigurationRevisionInputBuilder) -> impl Future<Output = Result<DescribeConfigurationRevisionOutput, SdkError<DescribeConfigurationRevisionError>>>;
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>>;
    fn list_brokers(&self, builder: ListBrokersInputBuilder) -> impl Future<Output = Result<ListBrokersOutput, SdkError<ListBrokersError>>>;
    fn list_configuration_revisions(&self, builder: ListConfigurationRevisionsInputBuilder) -> impl Future<Output = Result<ListConfigurationRevisionsOutput, SdkError<ListConfigurationRevisionsError>>>;
    fn list_configurations(&self, builder: ListConfigurationsInputBuilder) -> impl Future<Output = Result<ListConfigurationsOutput, SdkError<ListConfigurationsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>>;
    fn promote(&self, builder: PromoteInputBuilder) -> impl Future<Output = Result<PromoteOutput, SdkError<PromoteError>>>;
    fn reboot_broker(&self, builder: RebootBrokerInputBuilder) -> impl Future<Output = Result<RebootBrokerOutput, SdkError<RebootBrokerError>>>;
    fn update_broker(&self, builder: UpdateBrokerInputBuilder) -> impl Future<Output = Result<UpdateBrokerOutput, SdkError<UpdateBrokerError>>>;
    fn update_configuration(&self, builder: UpdateConfigurationInputBuilder) -> impl Future<Output = Result<UpdateConfigurationOutput, SdkError<UpdateConfigurationError>>>;
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>>;
}
impl MQClient for MQClientImpl {
    fn create_broker(&self, builder: CreateBrokerInputBuilder) -> impl Future<Output = Result<CreateBrokerOutput, SdkError<CreateBrokerError>>> {
        builder.send_with(&self.0)
    }
    fn create_configuration(&self, builder: CreateConfigurationInputBuilder) -> impl Future<Output = Result<CreateConfigurationOutput, SdkError<CreateConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        builder.send_with(&self.0)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_broker(&self, builder: DeleteBrokerInputBuilder) -> impl Future<Output = Result<DeleteBrokerOutput, SdkError<DeleteBrokerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn describe_broker(&self, builder: DescribeBrokerInputBuilder) -> impl Future<Output = Result<DescribeBrokerOutput, SdkError<DescribeBrokerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_broker_engine_types(&self, builder: DescribeBrokerEngineTypesInputBuilder) -> impl Future<Output = Result<DescribeBrokerEngineTypesOutput, SdkError<DescribeBrokerEngineTypesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_broker_instance_options(&self, builder: DescribeBrokerInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeBrokerInstanceOptionsOutput, SdkError<DescribeBrokerInstanceOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_configuration(&self, builder: DescribeConfigurationInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOutput, SdkError<DescribeConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_configuration_revision(&self, builder: DescribeConfigurationRevisionInputBuilder) -> impl Future<Output = Result<DescribeConfigurationRevisionOutput, SdkError<DescribeConfigurationRevisionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>> {
        builder.send_with(&self.0)
    }
    fn list_brokers(&self, builder: ListBrokersInputBuilder) -> impl Future<Output = Result<ListBrokersOutput, SdkError<ListBrokersError>>> {
        builder.send_with(&self.0)
    }
    fn list_configuration_revisions(&self, builder: ListConfigurationRevisionsInputBuilder) -> impl Future<Output = Result<ListConfigurationRevisionsOutput, SdkError<ListConfigurationRevisionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_configurations(&self, builder: ListConfigurationsInputBuilder) -> impl Future<Output = Result<ListConfigurationsOutput, SdkError<ListConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        builder.send_with(&self.0)
    }
    fn promote(&self, builder: PromoteInputBuilder) -> impl Future<Output = Result<PromoteOutput, SdkError<PromoteError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_broker(&self, builder: RebootBrokerInputBuilder) -> impl Future<Output = Result<RebootBrokerOutput, SdkError<RebootBrokerError>>> {
        builder.send_with(&self.0)
    }
    fn update_broker(&self, builder: UpdateBrokerInputBuilder) -> impl Future<Output = Result<UpdateBrokerOutput, SdkError<UpdateBrokerError>>> {
        builder.send_with(&self.0)
    }
    fn update_configuration(&self, builder: UpdateConfigurationInputBuilder) -> impl Future<Output = Result<UpdateConfigurationOutput, SdkError<UpdateConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> MQClient for T
where T: Deref,
      T::Target: MQClient {
    fn create_broker(&self, builder: CreateBrokerInputBuilder) -> impl Future<Output = Result<CreateBrokerOutput, SdkError<CreateBrokerError>>> {
        self.deref().create_broker(builder)
    }
    fn create_configuration(&self, builder: CreateConfigurationInputBuilder) -> impl Future<Output = Result<CreateConfigurationOutput, SdkError<CreateConfigurationError>>> {
        self.deref().create_configuration(builder)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        self.deref().create_tags(builder)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        self.deref().create_user(builder)
    }
    fn delete_broker(&self, builder: DeleteBrokerInputBuilder) -> impl Future<Output = Result<DeleteBrokerOutput, SdkError<DeleteBrokerError>>> {
        self.deref().delete_broker(builder)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        self.deref().delete_tags(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        self.deref().delete_user(builder)
    }
    fn describe_broker(&self, builder: DescribeBrokerInputBuilder) -> impl Future<Output = Result<DescribeBrokerOutput, SdkError<DescribeBrokerError>>> {
        self.deref().describe_broker(builder)
    }
    fn describe_broker_engine_types(&self, builder: DescribeBrokerEngineTypesInputBuilder) -> impl Future<Output = Result<DescribeBrokerEngineTypesOutput, SdkError<DescribeBrokerEngineTypesError>>> {
        self.deref().describe_broker_engine_types(builder)
    }
    fn describe_broker_instance_options(&self, builder: DescribeBrokerInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeBrokerInstanceOptionsOutput, SdkError<DescribeBrokerInstanceOptionsError>>> {
        self.deref().describe_broker_instance_options(builder)
    }
    fn describe_configuration(&self, builder: DescribeConfigurationInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOutput, SdkError<DescribeConfigurationError>>> {
        self.deref().describe_configuration(builder)
    }
    fn describe_configuration_revision(&self, builder: DescribeConfigurationRevisionInputBuilder) -> impl Future<Output = Result<DescribeConfigurationRevisionOutput, SdkError<DescribeConfigurationRevisionError>>> {
        self.deref().describe_configuration_revision(builder)
    }
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>> {
        self.deref().describe_user(builder)
    }
    fn list_brokers(&self, builder: ListBrokersInputBuilder) -> impl Future<Output = Result<ListBrokersOutput, SdkError<ListBrokersError>>> {
        self.deref().list_brokers(builder)
    }
    fn list_configuration_revisions(&self, builder: ListConfigurationRevisionsInputBuilder) -> impl Future<Output = Result<ListConfigurationRevisionsOutput, SdkError<ListConfigurationRevisionsError>>> {
        self.deref().list_configuration_revisions(builder)
    }
    fn list_configurations(&self, builder: ListConfigurationsInputBuilder) -> impl Future<Output = Result<ListConfigurationsOutput, SdkError<ListConfigurationsError>>> {
        self.deref().list_configurations(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        self.deref().list_tags(builder)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        self.deref().list_users(builder)
    }
    fn promote(&self, builder: PromoteInputBuilder) -> impl Future<Output = Result<PromoteOutput, SdkError<PromoteError>>> {
        self.deref().promote(builder)
    }
    fn reboot_broker(&self, builder: RebootBrokerInputBuilder) -> impl Future<Output = Result<RebootBrokerOutput, SdkError<RebootBrokerError>>> {
        self.deref().reboot_broker(builder)
    }
    fn update_broker(&self, builder: UpdateBrokerInputBuilder) -> impl Future<Output = Result<UpdateBrokerOutput, SdkError<UpdateBrokerError>>> {
        self.deref().update_broker(builder)
    }
    fn update_configuration(&self, builder: UpdateConfigurationInputBuilder) -> impl Future<Output = Result<UpdateConfigurationOutput, SdkError<UpdateConfigurationError>>> {
        self.deref().update_configuration(builder)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        self.deref().update_user(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edMQClient {}
    impl MQClient for edMQClient {
        async fn create_broker(&self, builder: CreateBrokerInputBuilder) -> Result<CreateBrokerOutput, SdkError<CreateBrokerError>>;
        async fn create_configuration(&self, builder: CreateConfigurationInputBuilder) -> Result<CreateConfigurationOutput, SdkError<CreateConfigurationError>>;
        async fn create_tags(&self, builder: CreateTagsInputBuilder) -> Result<CreateTagsOutput, SdkError<CreateTagsError>>;
        async fn create_user(&self, builder: CreateUserInputBuilder) -> Result<CreateUserOutput, SdkError<CreateUserError>>;
        async fn delete_broker(&self, builder: DeleteBrokerInputBuilder) -> Result<DeleteBrokerOutput, SdkError<DeleteBrokerError>>;
        async fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> Result<DeleteTagsOutput, SdkError<DeleteTagsError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn describe_broker(&self, builder: DescribeBrokerInputBuilder) -> Result<DescribeBrokerOutput, SdkError<DescribeBrokerError>>;
        async fn describe_broker_engine_types(&self, builder: DescribeBrokerEngineTypesInputBuilder) -> Result<DescribeBrokerEngineTypesOutput, SdkError<DescribeBrokerEngineTypesError>>;
        async fn describe_broker_instance_options(&self, builder: DescribeBrokerInstanceOptionsInputBuilder) -> Result<DescribeBrokerInstanceOptionsOutput, SdkError<DescribeBrokerInstanceOptionsError>>;
        async fn describe_configuration(&self, builder: DescribeConfigurationInputBuilder) -> Result<DescribeConfigurationOutput, SdkError<DescribeConfigurationError>>;
        async fn describe_configuration_revision(&self, builder: DescribeConfigurationRevisionInputBuilder) -> Result<DescribeConfigurationRevisionOutput, SdkError<DescribeConfigurationRevisionError>>;
        async fn describe_user(&self, builder: DescribeUserInputBuilder) -> Result<DescribeUserOutput, SdkError<DescribeUserError>>;
        async fn list_brokers(&self, builder: ListBrokersInputBuilder) -> Result<ListBrokersOutput, SdkError<ListBrokersError>>;
        async fn list_configuration_revisions(&self, builder: ListConfigurationRevisionsInputBuilder) -> Result<ListConfigurationRevisionsOutput, SdkError<ListConfigurationRevisionsError>>;
        async fn list_configurations(&self, builder: ListConfigurationsInputBuilder) -> Result<ListConfigurationsOutput, SdkError<ListConfigurationsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn list_users(&self, builder: ListUsersInputBuilder) -> Result<ListUsersOutput, SdkError<ListUsersError>>;
        async fn promote(&self, builder: PromoteInputBuilder) -> Result<PromoteOutput, SdkError<PromoteError>>;
        async fn reboot_broker(&self, builder: RebootBrokerInputBuilder) -> Result<RebootBrokerOutput, SdkError<RebootBrokerError>>;
        async fn update_broker(&self, builder: UpdateBrokerInputBuilder) -> Result<UpdateBrokerOutput, SdkError<UpdateBrokerError>>;
        async fn update_configuration(&self, builder: UpdateConfigurationInputBuilder) -> Result<UpdateConfigurationOutput, SdkError<UpdateConfigurationError>>;
        async fn update_user(&self, builder: UpdateUserInputBuilder) -> Result<UpdateUserOutput, SdkError<UpdateUserError>>;
    }
}
