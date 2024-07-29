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
use aws_sdk_appintegrations::operation::create_application::{builders::*, *};
use aws_sdk_appintegrations::operation::create_data_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::create_event_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::delete_application::{builders::*, *};
use aws_sdk_appintegrations::operation::delete_data_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::delete_event_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::get_application::{builders::*, *};
use aws_sdk_appintegrations::operation::get_data_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::get_event_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::list_application_associations::{builders::*, *};
use aws_sdk_appintegrations::operation::list_applications::{builders::*, *};
use aws_sdk_appintegrations::operation::list_data_integration_associations::{builders::*, *};
use aws_sdk_appintegrations::operation::list_data_integrations::{builders::*, *};
use aws_sdk_appintegrations::operation::list_event_integration_associations::{builders::*, *};
use aws_sdk_appintegrations::operation::list_event_integrations::{builders::*, *};
use aws_sdk_appintegrations::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appintegrations::operation::tag_resource::{builders::*, *};
use aws_sdk_appintegrations::operation::untag_resource::{builders::*, *};
use aws_sdk_appintegrations::operation::update_application::{builders::*, *};
use aws_sdk_appintegrations::operation::update_data_integration::{builders::*, *};
use aws_sdk_appintegrations::operation::update_event_integration::{builders::*, *};
use aws_sdk_appintegrations::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appintegrations::Client;
use std::ops::Deref;

pub use aws_sdk_appintegrations::*;

pub struct AppIntegrationsClientImpl(Client);
impl AppIntegrationsClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppIntegrationsClient {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>>;
    fn create_data_integration(&self, builder: CreateDataIntegrationInputBuilder) -> impl Future<Output = Result<CreateDataIntegrationOutput, SdkError<CreateDataIntegrationError>>>;
    fn create_event_integration(&self, builder: CreateEventIntegrationInputBuilder) -> impl Future<Output = Result<CreateEventIntegrationOutput, SdkError<CreateEventIntegrationError>>>;
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>>;
    fn delete_data_integration(&self, builder: DeleteDataIntegrationInputBuilder) -> impl Future<Output = Result<DeleteDataIntegrationOutput, SdkError<DeleteDataIntegrationError>>>;
    fn delete_event_integration(&self, builder: DeleteEventIntegrationInputBuilder) -> impl Future<Output = Result<DeleteEventIntegrationOutput, SdkError<DeleteEventIntegrationError>>>;
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>>;
    fn get_data_integration(&self, builder: GetDataIntegrationInputBuilder) -> impl Future<Output = Result<GetDataIntegrationOutput, SdkError<GetDataIntegrationError>>>;
    fn get_event_integration(&self, builder: GetEventIntegrationInputBuilder) -> impl Future<Output = Result<GetEventIntegrationOutput, SdkError<GetEventIntegrationError>>>;
    fn list_application_associations(&self, builder: ListApplicationAssociationsInputBuilder) -> impl Future<Output = Result<ListApplicationAssociationsOutput, SdkError<ListApplicationAssociationsError>>>;
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>>;
    fn list_data_integration_associations(&self, builder: ListDataIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationAssociationsOutput, SdkError<ListDataIntegrationAssociationsError>>>;
    fn list_data_integrations(&self, builder: ListDataIntegrationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationsOutput, SdkError<ListDataIntegrationsError>>>;
    fn list_event_integration_associations(&self, builder: ListEventIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationAssociationsOutput, SdkError<ListEventIntegrationAssociationsError>>>;
    fn list_event_integrations(&self, builder: ListEventIntegrationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationsOutput, SdkError<ListEventIntegrationsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>>;
    fn update_data_integration(&self, builder: UpdateDataIntegrationInputBuilder) -> impl Future<Output = Result<UpdateDataIntegrationOutput, SdkError<UpdateDataIntegrationError>>>;
    fn update_event_integration(&self, builder: UpdateEventIntegrationInputBuilder) -> impl Future<Output = Result<UpdateEventIntegrationOutput, SdkError<UpdateEventIntegrationError>>>;
}
impl AppIntegrationsClient for AppIntegrationsClientImpl {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_integration(&self, builder: CreateDataIntegrationInputBuilder) -> impl Future<Output = Result<CreateDataIntegrationOutput, SdkError<CreateDataIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn create_event_integration(&self, builder: CreateEventIntegrationInputBuilder) -> impl Future<Output = Result<CreateEventIntegrationOutput, SdkError<CreateEventIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_integration(&self, builder: DeleteDataIntegrationInputBuilder) -> impl Future<Output = Result<DeleteDataIntegrationOutput, SdkError<DeleteDataIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_event_integration(&self, builder: DeleteEventIntegrationInputBuilder) -> impl Future<Output = Result<DeleteEventIntegrationOutput, SdkError<DeleteEventIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_integration(&self, builder: GetDataIntegrationInputBuilder) -> impl Future<Output = Result<GetDataIntegrationOutput, SdkError<GetDataIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn get_event_integration(&self, builder: GetEventIntegrationInputBuilder) -> impl Future<Output = Result<GetEventIntegrationOutput, SdkError<GetEventIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn list_application_associations(&self, builder: ListApplicationAssociationsInputBuilder) -> impl Future<Output = Result<ListApplicationAssociationsOutput, SdkError<ListApplicationAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_integration_associations(&self, builder: ListDataIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationAssociationsOutput, SdkError<ListDataIntegrationAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_integrations(&self, builder: ListDataIntegrationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationsOutput, SdkError<ListDataIntegrationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_event_integration_associations(&self, builder: ListEventIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationAssociationsOutput, SdkError<ListEventIntegrationAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_event_integrations(&self, builder: ListEventIntegrationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationsOutput, SdkError<ListEventIntegrationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_integration(&self, builder: UpdateDataIntegrationInputBuilder) -> impl Future<Output = Result<UpdateDataIntegrationOutput, SdkError<UpdateDataIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn update_event_integration(&self, builder: UpdateEventIntegrationInputBuilder) -> impl Future<Output = Result<UpdateEventIntegrationOutput, SdkError<UpdateEventIntegrationError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppIntegrationsClient for T
where T: Deref,
      T::Target: AppIntegrationsClient {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        self.deref().create_application(builder)
    }
    fn create_data_integration(&self, builder: CreateDataIntegrationInputBuilder) -> impl Future<Output = Result<CreateDataIntegrationOutput, SdkError<CreateDataIntegrationError>>> {
        self.deref().create_data_integration(builder)
    }
    fn create_event_integration(&self, builder: CreateEventIntegrationInputBuilder) -> impl Future<Output = Result<CreateEventIntegrationOutput, SdkError<CreateEventIntegrationError>>> {
        self.deref().create_event_integration(builder)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        self.deref().delete_application(builder)
    }
    fn delete_data_integration(&self, builder: DeleteDataIntegrationInputBuilder) -> impl Future<Output = Result<DeleteDataIntegrationOutput, SdkError<DeleteDataIntegrationError>>> {
        self.deref().delete_data_integration(builder)
    }
    fn delete_event_integration(&self, builder: DeleteEventIntegrationInputBuilder) -> impl Future<Output = Result<DeleteEventIntegrationOutput, SdkError<DeleteEventIntegrationError>>> {
        self.deref().delete_event_integration(builder)
    }
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>> {
        self.deref().get_application(builder)
    }
    fn get_data_integration(&self, builder: GetDataIntegrationInputBuilder) -> impl Future<Output = Result<GetDataIntegrationOutput, SdkError<GetDataIntegrationError>>> {
        self.deref().get_data_integration(builder)
    }
    fn get_event_integration(&self, builder: GetEventIntegrationInputBuilder) -> impl Future<Output = Result<GetEventIntegrationOutput, SdkError<GetEventIntegrationError>>> {
        self.deref().get_event_integration(builder)
    }
    fn list_application_associations(&self, builder: ListApplicationAssociationsInputBuilder) -> impl Future<Output = Result<ListApplicationAssociationsOutput, SdkError<ListApplicationAssociationsError>>> {
        self.deref().list_application_associations(builder)
    }
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>> {
        self.deref().list_applications(builder)
    }
    fn list_data_integration_associations(&self, builder: ListDataIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationAssociationsOutput, SdkError<ListDataIntegrationAssociationsError>>> {
        self.deref().list_data_integration_associations(builder)
    }
    fn list_data_integrations(&self, builder: ListDataIntegrationsInputBuilder) -> impl Future<Output = Result<ListDataIntegrationsOutput, SdkError<ListDataIntegrationsError>>> {
        self.deref().list_data_integrations(builder)
    }
    fn list_event_integration_associations(&self, builder: ListEventIntegrationAssociationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationAssociationsOutput, SdkError<ListEventIntegrationAssociationsError>>> {
        self.deref().list_event_integration_associations(builder)
    }
    fn list_event_integrations(&self, builder: ListEventIntegrationsInputBuilder) -> impl Future<Output = Result<ListEventIntegrationsOutput, SdkError<ListEventIntegrationsError>>> {
        self.deref().list_event_integrations(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        self.deref().update_application(builder)
    }
    fn update_data_integration(&self, builder: UpdateDataIntegrationInputBuilder) -> impl Future<Output = Result<UpdateDataIntegrationOutput, SdkError<UpdateDataIntegrationError>>> {
        self.deref().update_data_integration(builder)
    }
    fn update_event_integration(&self, builder: UpdateEventIntegrationInputBuilder) -> impl Future<Output = Result<UpdateEventIntegrationOutput, SdkError<UpdateEventIntegrationError>>> {
        self.deref().update_event_integration(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppIntegrationsClient {}
    impl AppIntegrationsClient for edAppIntegrationsClient {
        async fn create_application(&self, builder: CreateApplicationInputBuilder) -> Result<CreateApplicationOutput, SdkError<CreateApplicationError>>;
        async fn create_data_integration(&self, builder: CreateDataIntegrationInputBuilder) -> Result<CreateDataIntegrationOutput, SdkError<CreateDataIntegrationError>>;
        async fn create_event_integration(&self, builder: CreateEventIntegrationInputBuilder) -> Result<CreateEventIntegrationOutput, SdkError<CreateEventIntegrationError>>;
        async fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>;
        async fn delete_data_integration(&self, builder: DeleteDataIntegrationInputBuilder) -> Result<DeleteDataIntegrationOutput, SdkError<DeleteDataIntegrationError>>;
        async fn delete_event_integration(&self, builder: DeleteEventIntegrationInputBuilder) -> Result<DeleteEventIntegrationOutput, SdkError<DeleteEventIntegrationError>>;
        async fn get_application(&self, builder: GetApplicationInputBuilder) -> Result<GetApplicationOutput, SdkError<GetApplicationError>>;
        async fn get_data_integration(&self, builder: GetDataIntegrationInputBuilder) -> Result<GetDataIntegrationOutput, SdkError<GetDataIntegrationError>>;
        async fn get_event_integration(&self, builder: GetEventIntegrationInputBuilder) -> Result<GetEventIntegrationOutput, SdkError<GetEventIntegrationError>>;
        async fn list_application_associations(&self, builder: ListApplicationAssociationsInputBuilder) -> Result<ListApplicationAssociationsOutput, SdkError<ListApplicationAssociationsError>>;
        async fn list_applications(&self, builder: ListApplicationsInputBuilder) -> Result<ListApplicationsOutput, SdkError<ListApplicationsError>>;
        async fn list_data_integration_associations(&self, builder: ListDataIntegrationAssociationsInputBuilder) -> Result<ListDataIntegrationAssociationsOutput, SdkError<ListDataIntegrationAssociationsError>>;
        async fn list_data_integrations(&self, builder: ListDataIntegrationsInputBuilder) -> Result<ListDataIntegrationsOutput, SdkError<ListDataIntegrationsError>>;
        async fn list_event_integration_associations(&self, builder: ListEventIntegrationAssociationsInputBuilder) -> Result<ListEventIntegrationAssociationsOutput, SdkError<ListEventIntegrationAssociationsError>>;
        async fn list_event_integrations(&self, builder: ListEventIntegrationsInputBuilder) -> Result<ListEventIntegrationsOutput, SdkError<ListEventIntegrationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_application(&self, builder: UpdateApplicationInputBuilder) -> Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>;
        async fn update_data_integration(&self, builder: UpdateDataIntegrationInputBuilder) -> Result<UpdateDataIntegrationOutput, SdkError<UpdateDataIntegrationError>>;
        async fn update_event_integration(&self, builder: UpdateEventIntegrationInputBuilder) -> Result<UpdateEventIntegrationOutput, SdkError<UpdateEventIntegrationError>>;
    }
}
