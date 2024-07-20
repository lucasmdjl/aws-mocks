/*
 * aws_mock - A mocking library for AWS.
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
use aws_sdk_amp::operation::create_alert_manager_definition::{builders::*, *};
use aws_sdk_amp::operation::create_logging_configuration::{builders::*, *};
use aws_sdk_amp::operation::create_rule_groups_namespace::{builders::*, *};
use aws_sdk_amp::operation::create_scraper::{builders::*, *};
use aws_sdk_amp::operation::create_workspace::{builders::*, *};
use aws_sdk_amp::operation::delete_alert_manager_definition::{builders::*, *};
use aws_sdk_amp::operation::delete_logging_configuration::{builders::*, *};
use aws_sdk_amp::operation::delete_rule_groups_namespace::{builders::*, *};
use aws_sdk_amp::operation::delete_scraper::{builders::*, *};
use aws_sdk_amp::operation::delete_workspace::{builders::*, *};
use aws_sdk_amp::operation::describe_alert_manager_definition::{builders::*, *};
use aws_sdk_amp::operation::describe_logging_configuration::{builders::*, *};
use aws_sdk_amp::operation::describe_rule_groups_namespace::{builders::*, *};
use aws_sdk_amp::operation::describe_scraper::{builders::*, *};
use aws_sdk_amp::operation::describe_workspace::{builders::*, *};
use aws_sdk_amp::operation::get_default_scraper_configuration::{builders::*, *};
use aws_sdk_amp::operation::list_rule_groups_namespaces::{builders::*, *};
use aws_sdk_amp::operation::list_scrapers::{builders::*, *};
use aws_sdk_amp::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_amp::operation::list_workspaces::{builders::*, *};
use aws_sdk_amp::operation::put_alert_manager_definition::{builders::*, *};
use aws_sdk_amp::operation::put_rule_groups_namespace::{builders::*, *};
use aws_sdk_amp::operation::tag_resource::{builders::*, *};
use aws_sdk_amp::operation::untag_resource::{builders::*, *};
use aws_sdk_amp::operation::update_logging_configuration::{builders::*, *};
use aws_sdk_amp::operation::update_workspace_alias::{builders::*, *};
use aws_sdk_amp::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_amp::Client;

pub use aws_sdk_amp::*;

pub struct AMPClientImpl(Client);
impl AMPClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AMPClient {
    fn create_alert_manager_definition(&self, builder: CreateAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<CreateAlertManagerDefinitionOutput, SdkError<CreateAlertManagerDefinitionError>>>;
    fn create_logging_configuration(&self, builder: CreateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<CreateLoggingConfigurationOutput, SdkError<CreateLoggingConfigurationError>>>;
    fn create_rule_groups_namespace(&self, builder: CreateRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<CreateRuleGroupsNamespaceOutput, SdkError<CreateRuleGroupsNamespaceError>>>;
    fn create_scraper(&self, builder: CreateScraperInputBuilder) -> impl Future<Output = Result<CreateScraperOutput, SdkError<CreateScraperError>>>;
    fn create_workspace(&self, builder: CreateWorkspaceInputBuilder) -> impl Future<Output = Result<CreateWorkspaceOutput, SdkError<CreateWorkspaceError>>>;
    fn delete_alert_manager_definition(&self, builder: DeleteAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DeleteAlertManagerDefinitionOutput, SdkError<DeleteAlertManagerDefinitionError>>>;
    fn delete_logging_configuration(&self, builder: DeleteLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteLoggingConfigurationOutput, SdkError<DeleteLoggingConfigurationError>>>;
    fn delete_rule_groups_namespace(&self, builder: DeleteRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DeleteRuleGroupsNamespaceOutput, SdkError<DeleteRuleGroupsNamespaceError>>>;
    fn delete_scraper(&self, builder: DeleteScraperInputBuilder) -> impl Future<Output = Result<DeleteScraperOutput, SdkError<DeleteScraperError>>>;
    fn delete_workspace(&self, builder: DeleteWorkspaceInputBuilder) -> impl Future<Output = Result<DeleteWorkspaceOutput, SdkError<DeleteWorkspaceError>>>;
    fn describe_alert_manager_definition(&self, builder: DescribeAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAlertManagerDefinitionOutput, SdkError<DescribeAlertManagerDefinitionError>>>;
    fn describe_logging_configuration(&self, builder: DescribeLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLoggingConfigurationOutput, SdkError<DescribeLoggingConfigurationError>>>;
    fn describe_rule_groups_namespace(&self, builder: DescribeRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DescribeRuleGroupsNamespaceOutput, SdkError<DescribeRuleGroupsNamespaceError>>>;
    fn describe_scraper(&self, builder: DescribeScraperInputBuilder) -> impl Future<Output = Result<DescribeScraperOutput, SdkError<DescribeScraperError>>>;
    fn describe_workspace(&self, builder: DescribeWorkspaceInputBuilder) -> impl Future<Output = Result<DescribeWorkspaceOutput, SdkError<DescribeWorkspaceError>>>;
    fn get_default_scraper_configuration(&self, builder: GetDefaultScraperConfigurationInputBuilder) -> impl Future<Output = Result<GetDefaultScraperConfigurationOutput, SdkError<GetDefaultScraperConfigurationError>>>;
    fn list_rule_groups_namespaces(&self, builder: ListRuleGroupsNamespacesInputBuilder) -> impl Future<Output = Result<ListRuleGroupsNamespacesOutput, SdkError<ListRuleGroupsNamespacesError>>>;
    fn list_scrapers(&self, builder: ListScrapersInputBuilder) -> impl Future<Output = Result<ListScrapersOutput, SdkError<ListScrapersError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_workspaces(&self, builder: ListWorkspacesInputBuilder) -> impl Future<Output = Result<ListWorkspacesOutput, SdkError<ListWorkspacesError>>>;
    fn put_alert_manager_definition(&self, builder: PutAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<PutAlertManagerDefinitionOutput, SdkError<PutAlertManagerDefinitionError>>>;
    fn put_rule_groups_namespace(&self, builder: PutRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<PutRuleGroupsNamespaceOutput, SdkError<PutRuleGroupsNamespaceError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_logging_configuration(&self, builder: UpdateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateLoggingConfigurationOutput, SdkError<UpdateLoggingConfigurationError>>>;
    fn update_workspace_alias(&self, builder: UpdateWorkspaceAliasInputBuilder) -> impl Future<Output = Result<UpdateWorkspaceAliasOutput, SdkError<UpdateWorkspaceAliasError>>>;
}
impl AMPClient for AMPClientImpl {
    fn create_alert_manager_definition(&self, builder: CreateAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<CreateAlertManagerDefinitionOutput, SdkError<CreateAlertManagerDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_logging_configuration(&self, builder: CreateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<CreateLoggingConfigurationOutput, SdkError<CreateLoggingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_rule_groups_namespace(&self, builder: CreateRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<CreateRuleGroupsNamespaceOutput, SdkError<CreateRuleGroupsNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_scraper(&self, builder: CreateScraperInputBuilder) -> impl Future<Output = Result<CreateScraperOutput, SdkError<CreateScraperError>>> {
        builder.send_with(&self.0)
    }
    fn create_workspace(&self, builder: CreateWorkspaceInputBuilder) -> impl Future<Output = Result<CreateWorkspaceOutput, SdkError<CreateWorkspaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_alert_manager_definition(&self, builder: DeleteAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DeleteAlertManagerDefinitionOutput, SdkError<DeleteAlertManagerDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_logging_configuration(&self, builder: DeleteLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteLoggingConfigurationOutput, SdkError<DeleteLoggingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_rule_groups_namespace(&self, builder: DeleteRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DeleteRuleGroupsNamespaceOutput, SdkError<DeleteRuleGroupsNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_scraper(&self, builder: DeleteScraperInputBuilder) -> impl Future<Output = Result<DeleteScraperOutput, SdkError<DeleteScraperError>>> {
        builder.send_with(&self.0)
    }
    fn delete_workspace(&self, builder: DeleteWorkspaceInputBuilder) -> impl Future<Output = Result<DeleteWorkspaceOutput, SdkError<DeleteWorkspaceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_alert_manager_definition(&self, builder: DescribeAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAlertManagerDefinitionOutput, SdkError<DescribeAlertManagerDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_logging_configuration(&self, builder: DescribeLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLoggingConfigurationOutput, SdkError<DescribeLoggingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_rule_groups_namespace(&self, builder: DescribeRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DescribeRuleGroupsNamespaceOutput, SdkError<DescribeRuleGroupsNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_scraper(&self, builder: DescribeScraperInputBuilder) -> impl Future<Output = Result<DescribeScraperOutput, SdkError<DescribeScraperError>>> {
        builder.send_with(&self.0)
    }
    fn describe_workspace(&self, builder: DescribeWorkspaceInputBuilder) -> impl Future<Output = Result<DescribeWorkspaceOutput, SdkError<DescribeWorkspaceError>>> {
        builder.send_with(&self.0)
    }
    fn get_default_scraper_configuration(&self, builder: GetDefaultScraperConfigurationInputBuilder) -> impl Future<Output = Result<GetDefaultScraperConfigurationOutput, SdkError<GetDefaultScraperConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn list_rule_groups_namespaces(&self, builder: ListRuleGroupsNamespacesInputBuilder) -> impl Future<Output = Result<ListRuleGroupsNamespacesOutput, SdkError<ListRuleGroupsNamespacesError>>> {
        builder.send_with(&self.0)
    }
    fn list_scrapers(&self, builder: ListScrapersInputBuilder) -> impl Future<Output = Result<ListScrapersOutput, SdkError<ListScrapersError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_workspaces(&self, builder: ListWorkspacesInputBuilder) -> impl Future<Output = Result<ListWorkspacesOutput, SdkError<ListWorkspacesError>>> {
        builder.send_with(&self.0)
    }
    fn put_alert_manager_definition(&self, builder: PutAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<PutAlertManagerDefinitionOutput, SdkError<PutAlertManagerDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn put_rule_groups_namespace(&self, builder: PutRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<PutRuleGroupsNamespaceOutput, SdkError<PutRuleGroupsNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_logging_configuration(&self, builder: UpdateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateLoggingConfigurationOutput, SdkError<UpdateLoggingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_workspace_alias(&self, builder: UpdateWorkspaceAliasInputBuilder) -> impl Future<Output = Result<UpdateWorkspaceAliasOutput, SdkError<UpdateWorkspaceAliasError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: AMPClient> AMPClient for &T {
    fn create_alert_manager_definition(&self, builder: CreateAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<CreateAlertManagerDefinitionOutput, SdkError<CreateAlertManagerDefinitionError>>> {
        (*self).create_alert_manager_definition(builder)
    }
    fn create_logging_configuration(&self, builder: CreateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<CreateLoggingConfigurationOutput, SdkError<CreateLoggingConfigurationError>>> {
        (*self).create_logging_configuration(builder)
    }
    fn create_rule_groups_namespace(&self, builder: CreateRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<CreateRuleGroupsNamespaceOutput, SdkError<CreateRuleGroupsNamespaceError>>> {
        (*self).create_rule_groups_namespace(builder)
    }
    fn create_scraper(&self, builder: CreateScraperInputBuilder) -> impl Future<Output = Result<CreateScraperOutput, SdkError<CreateScraperError>>> {
        (*self).create_scraper(builder)
    }
    fn create_workspace(&self, builder: CreateWorkspaceInputBuilder) -> impl Future<Output = Result<CreateWorkspaceOutput, SdkError<CreateWorkspaceError>>> {
        (*self).create_workspace(builder)
    }
    fn delete_alert_manager_definition(&self, builder: DeleteAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DeleteAlertManagerDefinitionOutput, SdkError<DeleteAlertManagerDefinitionError>>> {
        (*self).delete_alert_manager_definition(builder)
    }
    fn delete_logging_configuration(&self, builder: DeleteLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteLoggingConfigurationOutput, SdkError<DeleteLoggingConfigurationError>>> {
        (*self).delete_logging_configuration(builder)
    }
    fn delete_rule_groups_namespace(&self, builder: DeleteRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DeleteRuleGroupsNamespaceOutput, SdkError<DeleteRuleGroupsNamespaceError>>> {
        (*self).delete_rule_groups_namespace(builder)
    }
    fn delete_scraper(&self, builder: DeleteScraperInputBuilder) -> impl Future<Output = Result<DeleteScraperOutput, SdkError<DeleteScraperError>>> {
        (*self).delete_scraper(builder)
    }
    fn delete_workspace(&self, builder: DeleteWorkspaceInputBuilder) -> impl Future<Output = Result<DeleteWorkspaceOutput, SdkError<DeleteWorkspaceError>>> {
        (*self).delete_workspace(builder)
    }
    fn describe_alert_manager_definition(&self, builder: DescribeAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAlertManagerDefinitionOutput, SdkError<DescribeAlertManagerDefinitionError>>> {
        (*self).describe_alert_manager_definition(builder)
    }
    fn describe_logging_configuration(&self, builder: DescribeLoggingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLoggingConfigurationOutput, SdkError<DescribeLoggingConfigurationError>>> {
        (*self).describe_logging_configuration(builder)
    }
    fn describe_rule_groups_namespace(&self, builder: DescribeRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<DescribeRuleGroupsNamespaceOutput, SdkError<DescribeRuleGroupsNamespaceError>>> {
        (*self).describe_rule_groups_namespace(builder)
    }
    fn describe_scraper(&self, builder: DescribeScraperInputBuilder) -> impl Future<Output = Result<DescribeScraperOutput, SdkError<DescribeScraperError>>> {
        (*self).describe_scraper(builder)
    }
    fn describe_workspace(&self, builder: DescribeWorkspaceInputBuilder) -> impl Future<Output = Result<DescribeWorkspaceOutput, SdkError<DescribeWorkspaceError>>> {
        (*self).describe_workspace(builder)
    }
    fn get_default_scraper_configuration(&self, builder: GetDefaultScraperConfigurationInputBuilder) -> impl Future<Output = Result<GetDefaultScraperConfigurationOutput, SdkError<GetDefaultScraperConfigurationError>>> {
        (*self).get_default_scraper_configuration(builder)
    }
    fn list_rule_groups_namespaces(&self, builder: ListRuleGroupsNamespacesInputBuilder) -> impl Future<Output = Result<ListRuleGroupsNamespacesOutput, SdkError<ListRuleGroupsNamespacesError>>> {
        (*self).list_rule_groups_namespaces(builder)
    }
    fn list_scrapers(&self, builder: ListScrapersInputBuilder) -> impl Future<Output = Result<ListScrapersOutput, SdkError<ListScrapersError>>> {
        (*self).list_scrapers(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn list_workspaces(&self, builder: ListWorkspacesInputBuilder) -> impl Future<Output = Result<ListWorkspacesOutput, SdkError<ListWorkspacesError>>> {
        (*self).list_workspaces(builder)
    }
    fn put_alert_manager_definition(&self, builder: PutAlertManagerDefinitionInputBuilder) -> impl Future<Output = Result<PutAlertManagerDefinitionOutput, SdkError<PutAlertManagerDefinitionError>>> {
        (*self).put_alert_manager_definition(builder)
    }
    fn put_rule_groups_namespace(&self, builder: PutRuleGroupsNamespaceInputBuilder) -> impl Future<Output = Result<PutRuleGroupsNamespaceOutput, SdkError<PutRuleGroupsNamespaceError>>> {
        (*self).put_rule_groups_namespace(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_logging_configuration(&self, builder: UpdateLoggingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateLoggingConfigurationOutput, SdkError<UpdateLoggingConfigurationError>>> {
        (*self).update_logging_configuration(builder)
    }
    fn update_workspace_alias(&self, builder: UpdateWorkspaceAliasInputBuilder) -> impl Future<Output = Result<UpdateWorkspaceAliasOutput, SdkError<UpdateWorkspaceAliasError>>> {
        (*self).update_workspace_alias(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAMPClient {}
    impl AMPClient for edAMPClient {
        async fn create_alert_manager_definition(&self, builder: CreateAlertManagerDefinitionInputBuilder) -> Result<CreateAlertManagerDefinitionOutput, SdkError<CreateAlertManagerDefinitionError>>;
        async fn create_logging_configuration(&self, builder: CreateLoggingConfigurationInputBuilder) -> Result<CreateLoggingConfigurationOutput, SdkError<CreateLoggingConfigurationError>>;
        async fn create_rule_groups_namespace(&self, builder: CreateRuleGroupsNamespaceInputBuilder) -> Result<CreateRuleGroupsNamespaceOutput, SdkError<CreateRuleGroupsNamespaceError>>;
        async fn create_scraper(&self, builder: CreateScraperInputBuilder) -> Result<CreateScraperOutput, SdkError<CreateScraperError>>;
        async fn create_workspace(&self, builder: CreateWorkspaceInputBuilder) -> Result<CreateWorkspaceOutput, SdkError<CreateWorkspaceError>>;
        async fn delete_alert_manager_definition(&self, builder: DeleteAlertManagerDefinitionInputBuilder) -> Result<DeleteAlertManagerDefinitionOutput, SdkError<DeleteAlertManagerDefinitionError>>;
        async fn delete_logging_configuration(&self, builder: DeleteLoggingConfigurationInputBuilder) -> Result<DeleteLoggingConfigurationOutput, SdkError<DeleteLoggingConfigurationError>>;
        async fn delete_rule_groups_namespace(&self, builder: DeleteRuleGroupsNamespaceInputBuilder) -> Result<DeleteRuleGroupsNamespaceOutput, SdkError<DeleteRuleGroupsNamespaceError>>;
        async fn delete_scraper(&self, builder: DeleteScraperInputBuilder) -> Result<DeleteScraperOutput, SdkError<DeleteScraperError>>;
        async fn delete_workspace(&self, builder: DeleteWorkspaceInputBuilder) -> Result<DeleteWorkspaceOutput, SdkError<DeleteWorkspaceError>>;
        async fn describe_alert_manager_definition(&self, builder: DescribeAlertManagerDefinitionInputBuilder) -> Result<DescribeAlertManagerDefinitionOutput, SdkError<DescribeAlertManagerDefinitionError>>;
        async fn describe_logging_configuration(&self, builder: DescribeLoggingConfigurationInputBuilder) -> Result<DescribeLoggingConfigurationOutput, SdkError<DescribeLoggingConfigurationError>>;
        async fn describe_rule_groups_namespace(&self, builder: DescribeRuleGroupsNamespaceInputBuilder) -> Result<DescribeRuleGroupsNamespaceOutput, SdkError<DescribeRuleGroupsNamespaceError>>;
        async fn describe_scraper(&self, builder: DescribeScraperInputBuilder) -> Result<DescribeScraperOutput, SdkError<DescribeScraperError>>;
        async fn describe_workspace(&self, builder: DescribeWorkspaceInputBuilder) -> Result<DescribeWorkspaceOutput, SdkError<DescribeWorkspaceError>>;
        async fn get_default_scraper_configuration(&self, builder: GetDefaultScraperConfigurationInputBuilder) -> Result<GetDefaultScraperConfigurationOutput, SdkError<GetDefaultScraperConfigurationError>>;
        async fn list_rule_groups_namespaces(&self, builder: ListRuleGroupsNamespacesInputBuilder) -> Result<ListRuleGroupsNamespacesOutput, SdkError<ListRuleGroupsNamespacesError>>;
        async fn list_scrapers(&self, builder: ListScrapersInputBuilder) -> Result<ListScrapersOutput, SdkError<ListScrapersError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_workspaces(&self, builder: ListWorkspacesInputBuilder) -> Result<ListWorkspacesOutput, SdkError<ListWorkspacesError>>;
        async fn put_alert_manager_definition(&self, builder: PutAlertManagerDefinitionInputBuilder) -> Result<PutAlertManagerDefinitionOutput, SdkError<PutAlertManagerDefinitionError>>;
        async fn put_rule_groups_namespace(&self, builder: PutRuleGroupsNamespaceInputBuilder) -> Result<PutRuleGroupsNamespaceOutput, SdkError<PutRuleGroupsNamespaceError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_logging_configuration(&self, builder: UpdateLoggingConfigurationInputBuilder) -> Result<UpdateLoggingConfigurationOutput, SdkError<UpdateLoggingConfigurationError>>;
        async fn update_workspace_alias(&self, builder: UpdateWorkspaceAliasInputBuilder) -> Result<UpdateWorkspaceAliasOutput, SdkError<UpdateWorkspaceAliasError>>;
    }
}
