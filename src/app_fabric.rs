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
use aws_sdk_appfabric::operation::batch_get_user_access_tasks::{builders::*, *};
use aws_sdk_appfabric::operation::connect_app_authorization::{builders::*, *};
use aws_sdk_appfabric::operation::create_app_authorization::{builders::*, *};
use aws_sdk_appfabric::operation::create_app_bundle::{builders::*, *};
use aws_sdk_appfabric::operation::create_ingestion::{builders::*, *};
use aws_sdk_appfabric::operation::create_ingestion_destination::{builders::*, *};
use aws_sdk_appfabric::operation::delete_app_authorization::{builders::*, *};
use aws_sdk_appfabric::operation::delete_app_bundle::{builders::*, *};
use aws_sdk_appfabric::operation::delete_ingestion::{builders::*, *};
use aws_sdk_appfabric::operation::delete_ingestion_destination::{builders::*, *};
use aws_sdk_appfabric::operation::get_app_authorization::{builders::*, *};
use aws_sdk_appfabric::operation::get_app_bundle::{builders::*, *};
use aws_sdk_appfabric::operation::get_ingestion::{builders::*, *};
use aws_sdk_appfabric::operation::get_ingestion_destination::{builders::*, *};
use aws_sdk_appfabric::operation::list_app_authorizations::{builders::*, *};
use aws_sdk_appfabric::operation::list_app_bundles::{builders::*, *};
use aws_sdk_appfabric::operation::list_ingestion_destinations::{builders::*, *};
use aws_sdk_appfabric::operation::list_ingestions::{builders::*, *};
use aws_sdk_appfabric::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appfabric::operation::start_ingestion::{builders::*, *};
use aws_sdk_appfabric::operation::start_user_access_tasks::{builders::*, *};
use aws_sdk_appfabric::operation::stop_ingestion::{builders::*, *};
use aws_sdk_appfabric::operation::tag_resource::{builders::*, *};
use aws_sdk_appfabric::operation::untag_resource::{builders::*, *};
use aws_sdk_appfabric::operation::update_app_authorization::{builders::*, *};
use aws_sdk_appfabric::operation::update_ingestion_destination::{builders::*, *};
use aws_sdk_appfabric::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appfabric::Client;
use std::ops::Deref;

pub use aws_sdk_appfabric::*;

pub struct AppFabricClientImpl(Client);
impl AppFabricClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppFabricClient {
    fn batch_get_user_access_tasks(&self, builder: BatchGetUserAccessTasksInputBuilder) -> impl Future<Output = Result<BatchGetUserAccessTasksOutput, SdkError<BatchGetUserAccessTasksError>>>;
    fn connect_app_authorization(&self, builder: ConnectAppAuthorizationInputBuilder) -> impl Future<Output = Result<ConnectAppAuthorizationOutput, SdkError<ConnectAppAuthorizationError>>>;
    fn create_app_authorization(&self, builder: CreateAppAuthorizationInputBuilder) -> impl Future<Output = Result<CreateAppAuthorizationOutput, SdkError<CreateAppAuthorizationError>>>;
    fn create_app_bundle(&self, builder: CreateAppBundleInputBuilder) -> impl Future<Output = Result<CreateAppBundleOutput, SdkError<CreateAppBundleError>>>;
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>>;
    fn create_ingestion_destination(&self, builder: CreateIngestionDestinationInputBuilder) -> impl Future<Output = Result<CreateIngestionDestinationOutput, SdkError<CreateIngestionDestinationError>>>;
    fn delete_app_authorization(&self, builder: DeleteAppAuthorizationInputBuilder) -> impl Future<Output = Result<DeleteAppAuthorizationOutput, SdkError<DeleteAppAuthorizationError>>>;
    fn delete_app_bundle(&self, builder: DeleteAppBundleInputBuilder) -> impl Future<Output = Result<DeleteAppBundleOutput, SdkError<DeleteAppBundleError>>>;
    fn delete_ingestion(&self, builder: DeleteIngestionInputBuilder) -> impl Future<Output = Result<DeleteIngestionOutput, SdkError<DeleteIngestionError>>>;
    fn delete_ingestion_destination(&self, builder: DeleteIngestionDestinationInputBuilder) -> impl Future<Output = Result<DeleteIngestionDestinationOutput, SdkError<DeleteIngestionDestinationError>>>;
    fn get_app_authorization(&self, builder: GetAppAuthorizationInputBuilder) -> impl Future<Output = Result<GetAppAuthorizationOutput, SdkError<GetAppAuthorizationError>>>;
    fn get_app_bundle(&self, builder: GetAppBundleInputBuilder) -> impl Future<Output = Result<GetAppBundleOutput, SdkError<GetAppBundleError>>>;
    fn get_ingestion(&self, builder: GetIngestionInputBuilder) -> impl Future<Output = Result<GetIngestionOutput, SdkError<GetIngestionError>>>;
    fn get_ingestion_destination(&self, builder: GetIngestionDestinationInputBuilder) -> impl Future<Output = Result<GetIngestionDestinationOutput, SdkError<GetIngestionDestinationError>>>;
    fn list_app_authorizations(&self, builder: ListAppAuthorizationsInputBuilder) -> impl Future<Output = Result<ListAppAuthorizationsOutput, SdkError<ListAppAuthorizationsError>>>;
    fn list_app_bundles(&self, builder: ListAppBundlesInputBuilder) -> impl Future<Output = Result<ListAppBundlesOutput, SdkError<ListAppBundlesError>>>;
    fn list_ingestion_destinations(&self, builder: ListIngestionDestinationsInputBuilder) -> impl Future<Output = Result<ListIngestionDestinationsOutput, SdkError<ListIngestionDestinationsError>>>;
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn start_ingestion(&self, builder: StartIngestionInputBuilder) -> impl Future<Output = Result<StartIngestionOutput, SdkError<StartIngestionError>>>;
    fn start_user_access_tasks(&self, builder: StartUserAccessTasksInputBuilder) -> impl Future<Output = Result<StartUserAccessTasksOutput, SdkError<StartUserAccessTasksError>>>;
    fn stop_ingestion(&self, builder: StopIngestionInputBuilder) -> impl Future<Output = Result<StopIngestionOutput, SdkError<StopIngestionError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_app_authorization(&self, builder: UpdateAppAuthorizationInputBuilder) -> impl Future<Output = Result<UpdateAppAuthorizationOutput, SdkError<UpdateAppAuthorizationError>>>;
    fn update_ingestion_destination(&self, builder: UpdateIngestionDestinationInputBuilder) -> impl Future<Output = Result<UpdateIngestionDestinationOutput, SdkError<UpdateIngestionDestinationError>>>;
}
impl AppFabricClient for AppFabricClientImpl {
    fn batch_get_user_access_tasks(&self, builder: BatchGetUserAccessTasksInputBuilder) -> impl Future<Output = Result<BatchGetUserAccessTasksOutput, SdkError<BatchGetUserAccessTasksError>>> {
        builder.send_with(&self.0)
    }
    fn connect_app_authorization(&self, builder: ConnectAppAuthorizationInputBuilder) -> impl Future<Output = Result<ConnectAppAuthorizationOutput, SdkError<ConnectAppAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_authorization(&self, builder: CreateAppAuthorizationInputBuilder) -> impl Future<Output = Result<CreateAppAuthorizationOutput, SdkError<CreateAppAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_bundle(&self, builder: CreateAppBundleInputBuilder) -> impl Future<Output = Result<CreateAppBundleOutput, SdkError<CreateAppBundleError>>> {
        builder.send_with(&self.0)
    }
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn create_ingestion_destination(&self, builder: CreateIngestionDestinationInputBuilder) -> impl Future<Output = Result<CreateIngestionDestinationOutput, SdkError<CreateIngestionDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app_authorization(&self, builder: DeleteAppAuthorizationInputBuilder) -> impl Future<Output = Result<DeleteAppAuthorizationOutput, SdkError<DeleteAppAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app_bundle(&self, builder: DeleteAppBundleInputBuilder) -> impl Future<Output = Result<DeleteAppBundleOutput, SdkError<DeleteAppBundleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ingestion(&self, builder: DeleteIngestionInputBuilder) -> impl Future<Output = Result<DeleteIngestionOutput, SdkError<DeleteIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ingestion_destination(&self, builder: DeleteIngestionDestinationInputBuilder) -> impl Future<Output = Result<DeleteIngestionDestinationOutput, SdkError<DeleteIngestionDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn get_app_authorization(&self, builder: GetAppAuthorizationInputBuilder) -> impl Future<Output = Result<GetAppAuthorizationOutput, SdkError<GetAppAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn get_app_bundle(&self, builder: GetAppBundleInputBuilder) -> impl Future<Output = Result<GetAppBundleOutput, SdkError<GetAppBundleError>>> {
        builder.send_with(&self.0)
    }
    fn get_ingestion(&self, builder: GetIngestionInputBuilder) -> impl Future<Output = Result<GetIngestionOutput, SdkError<GetIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn get_ingestion_destination(&self, builder: GetIngestionDestinationInputBuilder) -> impl Future<Output = Result<GetIngestionDestinationOutput, SdkError<GetIngestionDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn list_app_authorizations(&self, builder: ListAppAuthorizationsInputBuilder) -> impl Future<Output = Result<ListAppAuthorizationsOutput, SdkError<ListAppAuthorizationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_app_bundles(&self, builder: ListAppBundlesInputBuilder) -> impl Future<Output = Result<ListAppBundlesOutput, SdkError<ListAppBundlesError>>> {
        builder.send_with(&self.0)
    }
    fn list_ingestion_destinations(&self, builder: ListIngestionDestinationsInputBuilder) -> impl Future<Output = Result<ListIngestionDestinationsOutput, SdkError<ListIngestionDestinationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_ingestion(&self, builder: StartIngestionInputBuilder) -> impl Future<Output = Result<StartIngestionOutput, SdkError<StartIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn start_user_access_tasks(&self, builder: StartUserAccessTasksInputBuilder) -> impl Future<Output = Result<StartUserAccessTasksOutput, SdkError<StartUserAccessTasksError>>> {
        builder.send_with(&self.0)
    }
    fn stop_ingestion(&self, builder: StopIngestionInputBuilder) -> impl Future<Output = Result<StopIngestionOutput, SdkError<StopIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_app_authorization(&self, builder: UpdateAppAuthorizationInputBuilder) -> impl Future<Output = Result<UpdateAppAuthorizationOutput, SdkError<UpdateAppAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn update_ingestion_destination(&self, builder: UpdateIngestionDestinationInputBuilder) -> impl Future<Output = Result<UpdateIngestionDestinationOutput, SdkError<UpdateIngestionDestinationError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppFabricClient for T
where T: Deref,
      T::Target: AppFabricClient {
    fn batch_get_user_access_tasks(&self, builder: BatchGetUserAccessTasksInputBuilder) -> impl Future<Output = Result<BatchGetUserAccessTasksOutput, SdkError<BatchGetUserAccessTasksError>>> {
        self.deref().batch_get_user_access_tasks(builder)
    }
    fn connect_app_authorization(&self, builder: ConnectAppAuthorizationInputBuilder) -> impl Future<Output = Result<ConnectAppAuthorizationOutput, SdkError<ConnectAppAuthorizationError>>> {
        self.deref().connect_app_authorization(builder)
    }
    fn create_app_authorization(&self, builder: CreateAppAuthorizationInputBuilder) -> impl Future<Output = Result<CreateAppAuthorizationOutput, SdkError<CreateAppAuthorizationError>>> {
        self.deref().create_app_authorization(builder)
    }
    fn create_app_bundle(&self, builder: CreateAppBundleInputBuilder) -> impl Future<Output = Result<CreateAppBundleOutput, SdkError<CreateAppBundleError>>> {
        self.deref().create_app_bundle(builder)
    }
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>> {
        self.deref().create_ingestion(builder)
    }
    fn create_ingestion_destination(&self, builder: CreateIngestionDestinationInputBuilder) -> impl Future<Output = Result<CreateIngestionDestinationOutput, SdkError<CreateIngestionDestinationError>>> {
        self.deref().create_ingestion_destination(builder)
    }
    fn delete_app_authorization(&self, builder: DeleteAppAuthorizationInputBuilder) -> impl Future<Output = Result<DeleteAppAuthorizationOutput, SdkError<DeleteAppAuthorizationError>>> {
        self.deref().delete_app_authorization(builder)
    }
    fn delete_app_bundle(&self, builder: DeleteAppBundleInputBuilder) -> impl Future<Output = Result<DeleteAppBundleOutput, SdkError<DeleteAppBundleError>>> {
        self.deref().delete_app_bundle(builder)
    }
    fn delete_ingestion(&self, builder: DeleteIngestionInputBuilder) -> impl Future<Output = Result<DeleteIngestionOutput, SdkError<DeleteIngestionError>>> {
        self.deref().delete_ingestion(builder)
    }
    fn delete_ingestion_destination(&self, builder: DeleteIngestionDestinationInputBuilder) -> impl Future<Output = Result<DeleteIngestionDestinationOutput, SdkError<DeleteIngestionDestinationError>>> {
        self.deref().delete_ingestion_destination(builder)
    }
    fn get_app_authorization(&self, builder: GetAppAuthorizationInputBuilder) -> impl Future<Output = Result<GetAppAuthorizationOutput, SdkError<GetAppAuthorizationError>>> {
        self.deref().get_app_authorization(builder)
    }
    fn get_app_bundle(&self, builder: GetAppBundleInputBuilder) -> impl Future<Output = Result<GetAppBundleOutput, SdkError<GetAppBundleError>>> {
        self.deref().get_app_bundle(builder)
    }
    fn get_ingestion(&self, builder: GetIngestionInputBuilder) -> impl Future<Output = Result<GetIngestionOutput, SdkError<GetIngestionError>>> {
        self.deref().get_ingestion(builder)
    }
    fn get_ingestion_destination(&self, builder: GetIngestionDestinationInputBuilder) -> impl Future<Output = Result<GetIngestionDestinationOutput, SdkError<GetIngestionDestinationError>>> {
        self.deref().get_ingestion_destination(builder)
    }
    fn list_app_authorizations(&self, builder: ListAppAuthorizationsInputBuilder) -> impl Future<Output = Result<ListAppAuthorizationsOutput, SdkError<ListAppAuthorizationsError>>> {
        self.deref().list_app_authorizations(builder)
    }
    fn list_app_bundles(&self, builder: ListAppBundlesInputBuilder) -> impl Future<Output = Result<ListAppBundlesOutput, SdkError<ListAppBundlesError>>> {
        self.deref().list_app_bundles(builder)
    }
    fn list_ingestion_destinations(&self, builder: ListIngestionDestinationsInputBuilder) -> impl Future<Output = Result<ListIngestionDestinationsOutput, SdkError<ListIngestionDestinationsError>>> {
        self.deref().list_ingestion_destinations(builder)
    }
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>> {
        self.deref().list_ingestions(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn start_ingestion(&self, builder: StartIngestionInputBuilder) -> impl Future<Output = Result<StartIngestionOutput, SdkError<StartIngestionError>>> {
        self.deref().start_ingestion(builder)
    }
    fn start_user_access_tasks(&self, builder: StartUserAccessTasksInputBuilder) -> impl Future<Output = Result<StartUserAccessTasksOutput, SdkError<StartUserAccessTasksError>>> {
        self.deref().start_user_access_tasks(builder)
    }
    fn stop_ingestion(&self, builder: StopIngestionInputBuilder) -> impl Future<Output = Result<StopIngestionOutput, SdkError<StopIngestionError>>> {
        self.deref().stop_ingestion(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_app_authorization(&self, builder: UpdateAppAuthorizationInputBuilder) -> impl Future<Output = Result<UpdateAppAuthorizationOutput, SdkError<UpdateAppAuthorizationError>>> {
        self.deref().update_app_authorization(builder)
    }
    fn update_ingestion_destination(&self, builder: UpdateIngestionDestinationInputBuilder) -> impl Future<Output = Result<UpdateIngestionDestinationOutput, SdkError<UpdateIngestionDestinationError>>> {
        self.deref().update_ingestion_destination(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppFabricClient {}
    impl AppFabricClient for edAppFabricClient {
        async fn batch_get_user_access_tasks(&self, builder: BatchGetUserAccessTasksInputBuilder) -> Result<BatchGetUserAccessTasksOutput, SdkError<BatchGetUserAccessTasksError>>;
        async fn connect_app_authorization(&self, builder: ConnectAppAuthorizationInputBuilder) -> Result<ConnectAppAuthorizationOutput, SdkError<ConnectAppAuthorizationError>>;
        async fn create_app_authorization(&self, builder: CreateAppAuthorizationInputBuilder) -> Result<CreateAppAuthorizationOutput, SdkError<CreateAppAuthorizationError>>;
        async fn create_app_bundle(&self, builder: CreateAppBundleInputBuilder) -> Result<CreateAppBundleOutput, SdkError<CreateAppBundleError>>;
        async fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> Result<CreateIngestionOutput, SdkError<CreateIngestionError>>;
        async fn create_ingestion_destination(&self, builder: CreateIngestionDestinationInputBuilder) -> Result<CreateIngestionDestinationOutput, SdkError<CreateIngestionDestinationError>>;
        async fn delete_app_authorization(&self, builder: DeleteAppAuthorizationInputBuilder) -> Result<DeleteAppAuthorizationOutput, SdkError<DeleteAppAuthorizationError>>;
        async fn delete_app_bundle(&self, builder: DeleteAppBundleInputBuilder) -> Result<DeleteAppBundleOutput, SdkError<DeleteAppBundleError>>;
        async fn delete_ingestion(&self, builder: DeleteIngestionInputBuilder) -> Result<DeleteIngestionOutput, SdkError<DeleteIngestionError>>;
        async fn delete_ingestion_destination(&self, builder: DeleteIngestionDestinationInputBuilder) -> Result<DeleteIngestionDestinationOutput, SdkError<DeleteIngestionDestinationError>>;
        async fn get_app_authorization(&self, builder: GetAppAuthorizationInputBuilder) -> Result<GetAppAuthorizationOutput, SdkError<GetAppAuthorizationError>>;
        async fn get_app_bundle(&self, builder: GetAppBundleInputBuilder) -> Result<GetAppBundleOutput, SdkError<GetAppBundleError>>;
        async fn get_ingestion(&self, builder: GetIngestionInputBuilder) -> Result<GetIngestionOutput, SdkError<GetIngestionError>>;
        async fn get_ingestion_destination(&self, builder: GetIngestionDestinationInputBuilder) -> Result<GetIngestionDestinationOutput, SdkError<GetIngestionDestinationError>>;
        async fn list_app_authorizations(&self, builder: ListAppAuthorizationsInputBuilder) -> Result<ListAppAuthorizationsOutput, SdkError<ListAppAuthorizationsError>>;
        async fn list_app_bundles(&self, builder: ListAppBundlesInputBuilder) -> Result<ListAppBundlesOutput, SdkError<ListAppBundlesError>>;
        async fn list_ingestion_destinations(&self, builder: ListIngestionDestinationsInputBuilder) -> Result<ListIngestionDestinationsOutput, SdkError<ListIngestionDestinationsError>>;
        async fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> Result<ListIngestionsOutput, SdkError<ListIngestionsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn start_ingestion(&self, builder: StartIngestionInputBuilder) -> Result<StartIngestionOutput, SdkError<StartIngestionError>>;
        async fn start_user_access_tasks(&self, builder: StartUserAccessTasksInputBuilder) -> Result<StartUserAccessTasksOutput, SdkError<StartUserAccessTasksError>>;
        async fn stop_ingestion(&self, builder: StopIngestionInputBuilder) -> Result<StopIngestionOutput, SdkError<StopIngestionError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_app_authorization(&self, builder: UpdateAppAuthorizationInputBuilder) -> Result<UpdateAppAuthorizationOutput, SdkError<UpdateAppAuthorizationError>>;
        async fn update_ingestion_destination(&self, builder: UpdateIngestionDestinationInputBuilder) -> Result<UpdateIngestionDestinationOutput, SdkError<UpdateIngestionDestinationError>>;
    }
}
