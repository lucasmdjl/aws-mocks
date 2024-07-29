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
use aws_sdk_appstream::operation::associate_app_block_builder_app_block::{builders::*, *};
use aws_sdk_appstream::operation::associate_application_fleet::{builders::*, *};
use aws_sdk_appstream::operation::associate_application_to_entitlement::{builders::*, *};
use aws_sdk_appstream::operation::associate_fleet::{builders::*, *};
use aws_sdk_appstream::operation::batch_associate_user_stack::{builders::*, *};
use aws_sdk_appstream::operation::batch_disassociate_user_stack::{builders::*, *};
use aws_sdk_appstream::operation::copy_image::{builders::*, *};
use aws_sdk_appstream::operation::create_app_block::{builders::*, *};
use aws_sdk_appstream::operation::create_app_block_builder::{builders::*, *};
use aws_sdk_appstream::operation::create_app_block_builder_streaming_url::{builders::*, *};
use aws_sdk_appstream::operation::create_application::{builders::*, *};
use aws_sdk_appstream::operation::create_directory_config::{builders::*, *};
use aws_sdk_appstream::operation::create_entitlement::{builders::*, *};
use aws_sdk_appstream::operation::create_fleet::{builders::*, *};
use aws_sdk_appstream::operation::create_image_builder::{builders::*, *};
use aws_sdk_appstream::operation::create_image_builder_streaming_url::{builders::*, *};
use aws_sdk_appstream::operation::create_stack::{builders::*, *};
use aws_sdk_appstream::operation::create_streaming_url::{builders::*, *};
use aws_sdk_appstream::operation::create_updated_image::{builders::*, *};
use aws_sdk_appstream::operation::create_usage_report_subscription::{builders::*, *};
use aws_sdk_appstream::operation::create_user::{builders::*, *};
use aws_sdk_appstream::operation::delete_app_block::{builders::*, *};
use aws_sdk_appstream::operation::delete_app_block_builder::{builders::*, *};
use aws_sdk_appstream::operation::delete_application::{builders::*, *};
use aws_sdk_appstream::operation::delete_directory_config::{builders::*, *};
use aws_sdk_appstream::operation::delete_entitlement::{builders::*, *};
use aws_sdk_appstream::operation::delete_fleet::{builders::*, *};
use aws_sdk_appstream::operation::delete_image::{builders::*, *};
use aws_sdk_appstream::operation::delete_image_builder::{builders::*, *};
use aws_sdk_appstream::operation::delete_image_permissions::{builders::*, *};
use aws_sdk_appstream::operation::delete_stack::{builders::*, *};
use aws_sdk_appstream::operation::delete_usage_report_subscription::{builders::*, *};
use aws_sdk_appstream::operation::delete_user::{builders::*, *};
use aws_sdk_appstream::operation::describe_app_block_builder_app_block_associations::{builders::*, *};
use aws_sdk_appstream::operation::describe_app_block_builders::{builders::*, *};
use aws_sdk_appstream::operation::describe_app_blocks::{builders::*, *};
use aws_sdk_appstream::operation::describe_application_fleet_associations::{builders::*, *};
use aws_sdk_appstream::operation::describe_applications::{builders::*, *};
use aws_sdk_appstream::operation::describe_directory_configs::{builders::*, *};
use aws_sdk_appstream::operation::describe_entitlements::{builders::*, *};
use aws_sdk_appstream::operation::describe_fleets::{builders::*, *};
use aws_sdk_appstream::operation::describe_image_builders::{builders::*, *};
use aws_sdk_appstream::operation::describe_image_permissions::{builders::*, *};
use aws_sdk_appstream::operation::describe_images::{builders::*, *};
use aws_sdk_appstream::operation::describe_sessions::{builders::*, *};
use aws_sdk_appstream::operation::describe_stacks::{builders::*, *};
use aws_sdk_appstream::operation::describe_usage_report_subscriptions::{builders::*, *};
use aws_sdk_appstream::operation::describe_user_stack_associations::{builders::*, *};
use aws_sdk_appstream::operation::describe_users::{builders::*, *};
use aws_sdk_appstream::operation::disable_user::{builders::*, *};
use aws_sdk_appstream::operation::disassociate_app_block_builder_app_block::{builders::*, *};
use aws_sdk_appstream::operation::disassociate_application_fleet::{builders::*, *};
use aws_sdk_appstream::operation::disassociate_application_from_entitlement::{builders::*, *};
use aws_sdk_appstream::operation::disassociate_fleet::{builders::*, *};
use aws_sdk_appstream::operation::enable_user::{builders::*, *};
use aws_sdk_appstream::operation::expire_session::{builders::*, *};
use aws_sdk_appstream::operation::list_associated_fleets::{builders::*, *};
use aws_sdk_appstream::operation::list_associated_stacks::{builders::*, *};
use aws_sdk_appstream::operation::list_entitled_applications::{builders::*, *};
use aws_sdk_appstream::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appstream::operation::start_app_block_builder::{builders::*, *};
use aws_sdk_appstream::operation::start_fleet::{builders::*, *};
use aws_sdk_appstream::operation::start_image_builder::{builders::*, *};
use aws_sdk_appstream::operation::stop_app_block_builder::{builders::*, *};
use aws_sdk_appstream::operation::stop_fleet::{builders::*, *};
use aws_sdk_appstream::operation::stop_image_builder::{builders::*, *};
use aws_sdk_appstream::operation::tag_resource::{builders::*, *};
use aws_sdk_appstream::operation::untag_resource::{builders::*, *};
use aws_sdk_appstream::operation::update_app_block_builder::{builders::*, *};
use aws_sdk_appstream::operation::update_application::{builders::*, *};
use aws_sdk_appstream::operation::update_directory_config::{builders::*, *};
use aws_sdk_appstream::operation::update_entitlement::{builders::*, *};
use aws_sdk_appstream::operation::update_fleet::{builders::*, *};
use aws_sdk_appstream::operation::update_image_permissions::{builders::*, *};
use aws_sdk_appstream::operation::update_stack::{builders::*, *};
use aws_sdk_appstream::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appstream::Client;
use std::ops::Deref;

pub use aws_sdk_appstream::*;

pub struct AppStreamClientImpl(Client);
impl AppStreamClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppStreamClient {
    fn associate_app_block_builder_app_block(&self, builder: AssociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<AssociateAppBlockBuilderAppBlockOutput, SdkError<AssociateAppBlockBuilderAppBlockError>>>;
    fn associate_application_fleet(&self, builder: AssociateApplicationFleetInputBuilder) -> impl Future<Output = Result<AssociateApplicationFleetOutput, SdkError<AssociateApplicationFleetError>>>;
    fn associate_application_to_entitlement(&self, builder: AssociateApplicationToEntitlementInputBuilder) -> impl Future<Output = Result<AssociateApplicationToEntitlementOutput, SdkError<AssociateApplicationToEntitlementError>>>;
    fn associate_fleet(&self, builder: AssociateFleetInputBuilder) -> impl Future<Output = Result<AssociateFleetOutput, SdkError<AssociateFleetError>>>;
    fn batch_associate_user_stack(&self, builder: BatchAssociateUserStackInputBuilder) -> impl Future<Output = Result<BatchAssociateUserStackOutput, SdkError<BatchAssociateUserStackError>>>;
    fn batch_disassociate_user_stack(&self, builder: BatchDisassociateUserStackInputBuilder) -> impl Future<Output = Result<BatchDisassociateUserStackOutput, SdkError<BatchDisassociateUserStackError>>>;
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>>;
    fn create_app_block(&self, builder: CreateAppBlockInputBuilder) -> impl Future<Output = Result<CreateAppBlockOutput, SdkError<CreateAppBlockError>>>;
    fn create_app_block_builder(&self, builder: CreateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderOutput, SdkError<CreateAppBlockBuilderError>>>;
    fn create_app_block_builder_streaming_url(&self, builder: CreateAppBlockBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderStreamingUrlOutput, SdkError<CreateAppBlockBuilderStreamingURLError>>>;
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>>;
    fn create_directory_config(&self, builder: CreateDirectoryConfigInputBuilder) -> impl Future<Output = Result<CreateDirectoryConfigOutput, SdkError<CreateDirectoryConfigError>>>;
    fn create_entitlement(&self, builder: CreateEntitlementInputBuilder) -> impl Future<Output = Result<CreateEntitlementOutput, SdkError<CreateEntitlementError>>>;
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>>;
    fn create_image_builder(&self, builder: CreateImageBuilderInputBuilder) -> impl Future<Output = Result<CreateImageBuilderOutput, SdkError<CreateImageBuilderError>>>;
    fn create_image_builder_streaming_url(&self, builder: CreateImageBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateImageBuilderStreamingUrlOutput, SdkError<CreateImageBuilderStreamingURLError>>>;
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>>;
    fn create_streaming_url(&self, builder: CreateStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateStreamingUrlOutput, SdkError<CreateStreamingURLError>>>;
    fn create_updated_image(&self, builder: CreateUpdatedImageInputBuilder) -> impl Future<Output = Result<CreateUpdatedImageOutput, SdkError<CreateUpdatedImageError>>>;
    fn create_usage_report_subscription(&self, builder: CreateUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<CreateUsageReportSubscriptionOutput, SdkError<CreateUsageReportSubscriptionError>>>;
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>>;
    fn delete_app_block(&self, builder: DeleteAppBlockInputBuilder) -> impl Future<Output = Result<DeleteAppBlockOutput, SdkError<DeleteAppBlockError>>>;
    fn delete_app_block_builder(&self, builder: DeleteAppBlockBuilderInputBuilder) -> impl Future<Output = Result<DeleteAppBlockBuilderOutput, SdkError<DeleteAppBlockBuilderError>>>;
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>>;
    fn delete_directory_config(&self, builder: DeleteDirectoryConfigInputBuilder) -> impl Future<Output = Result<DeleteDirectoryConfigOutput, SdkError<DeleteDirectoryConfigError>>>;
    fn delete_entitlement(&self, builder: DeleteEntitlementInputBuilder) -> impl Future<Output = Result<DeleteEntitlementOutput, SdkError<DeleteEntitlementError>>>;
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>>;
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>>;
    fn delete_image_builder(&self, builder: DeleteImageBuilderInputBuilder) -> impl Future<Output = Result<DeleteImageBuilderOutput, SdkError<DeleteImageBuilderError>>>;
    fn delete_image_permissions(&self, builder: DeleteImagePermissionsInputBuilder) -> impl Future<Output = Result<DeleteImagePermissionsOutput, SdkError<DeleteImagePermissionsError>>>;
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>>;
    fn delete_usage_report_subscription(&self, builder: DeleteUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteUsageReportSubscriptionOutput, SdkError<DeleteUsageReportSubscriptionError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn describe_app_block_builder_app_block_associations(&self, builder: DescribeAppBlockBuilderAppBlockAssociationsInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuilderAppBlockAssociationsOutput, SdkError<DescribeAppBlockBuilderAppBlockAssociationsError>>>;
    fn describe_app_block_builders(&self, builder: DescribeAppBlockBuildersInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuildersOutput, SdkError<DescribeAppBlockBuildersError>>>;
    fn describe_app_blocks(&self, builder: DescribeAppBlocksInputBuilder) -> impl Future<Output = Result<DescribeAppBlocksOutput, SdkError<DescribeAppBlocksError>>>;
    fn describe_application_fleet_associations(&self, builder: DescribeApplicationFleetAssociationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationFleetAssociationsOutput, SdkError<DescribeApplicationFleetAssociationsError>>>;
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>>;
    fn describe_directory_configs(&self, builder: DescribeDirectoryConfigsInputBuilder) -> impl Future<Output = Result<DescribeDirectoryConfigsOutput, SdkError<DescribeDirectoryConfigsError>>>;
    fn describe_entitlements(&self, builder: DescribeEntitlementsInputBuilder) -> impl Future<Output = Result<DescribeEntitlementsOutput, SdkError<DescribeEntitlementsError>>>;
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>>;
    fn describe_image_builders(&self, builder: DescribeImageBuildersInputBuilder) -> impl Future<Output = Result<DescribeImageBuildersOutput, SdkError<DescribeImageBuildersError>>>;
    fn describe_image_permissions(&self, builder: DescribeImagePermissionsInputBuilder) -> impl Future<Output = Result<DescribeImagePermissionsOutput, SdkError<DescribeImagePermissionsError>>>;
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>>;
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>>;
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>>;
    fn describe_usage_report_subscriptions(&self, builder: DescribeUsageReportSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeUsageReportSubscriptionsOutput, SdkError<DescribeUsageReportSubscriptionsError>>>;
    fn describe_user_stack_associations(&self, builder: DescribeUserStackAssociationsInputBuilder) -> impl Future<Output = Result<DescribeUserStackAssociationsOutput, SdkError<DescribeUserStackAssociationsError>>>;
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>>;
    fn disable_user(&self, builder: DisableUserInputBuilder) -> impl Future<Output = Result<DisableUserOutput, SdkError<DisableUserError>>>;
    fn disassociate_app_block_builder_app_block(&self, builder: DisassociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<DisassociateAppBlockBuilderAppBlockOutput, SdkError<DisassociateAppBlockBuilderAppBlockError>>>;
    fn disassociate_application_fleet(&self, builder: DisassociateApplicationFleetInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFleetOutput, SdkError<DisassociateApplicationFleetError>>>;
    fn disassociate_application_from_entitlement(&self, builder: DisassociateApplicationFromEntitlementInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFromEntitlementOutput, SdkError<DisassociateApplicationFromEntitlementError>>>;
    fn disassociate_fleet(&self, builder: DisassociateFleetInputBuilder) -> impl Future<Output = Result<DisassociateFleetOutput, SdkError<DisassociateFleetError>>>;
    fn enable_user(&self, builder: EnableUserInputBuilder) -> impl Future<Output = Result<EnableUserOutput, SdkError<EnableUserError>>>;
    fn expire_session(&self, builder: ExpireSessionInputBuilder) -> impl Future<Output = Result<ExpireSessionOutput, SdkError<ExpireSessionError>>>;
    fn list_associated_fleets(&self, builder: ListAssociatedFleetsInputBuilder) -> impl Future<Output = Result<ListAssociatedFleetsOutput, SdkError<ListAssociatedFleetsError>>>;
    fn list_associated_stacks(&self, builder: ListAssociatedStacksInputBuilder) -> impl Future<Output = Result<ListAssociatedStacksOutput, SdkError<ListAssociatedStacksError>>>;
    fn list_entitled_applications(&self, builder: ListEntitledApplicationsInputBuilder) -> impl Future<Output = Result<ListEntitledApplicationsOutput, SdkError<ListEntitledApplicationsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn start_app_block_builder(&self, builder: StartAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StartAppBlockBuilderOutput, SdkError<StartAppBlockBuilderError>>>;
    fn start_fleet(&self, builder: StartFleetInputBuilder) -> impl Future<Output = Result<StartFleetOutput, SdkError<StartFleetError>>>;
    fn start_image_builder(&self, builder: StartImageBuilderInputBuilder) -> impl Future<Output = Result<StartImageBuilderOutput, SdkError<StartImageBuilderError>>>;
    fn stop_app_block_builder(&self, builder: StopAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StopAppBlockBuilderOutput, SdkError<StopAppBlockBuilderError>>>;
    fn stop_fleet(&self, builder: StopFleetInputBuilder) -> impl Future<Output = Result<StopFleetOutput, SdkError<StopFleetError>>>;
    fn stop_image_builder(&self, builder: StopImageBuilderInputBuilder) -> impl Future<Output = Result<StopImageBuilderOutput, SdkError<StopImageBuilderError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_app_block_builder(&self, builder: UpdateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<UpdateAppBlockBuilderOutput, SdkError<UpdateAppBlockBuilderError>>>;
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>>;
    fn update_directory_config(&self, builder: UpdateDirectoryConfigInputBuilder) -> impl Future<Output = Result<UpdateDirectoryConfigOutput, SdkError<UpdateDirectoryConfigError>>>;
    fn update_entitlement(&self, builder: UpdateEntitlementInputBuilder) -> impl Future<Output = Result<UpdateEntitlementOutput, SdkError<UpdateEntitlementError>>>;
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>>;
    fn update_image_permissions(&self, builder: UpdateImagePermissionsInputBuilder) -> impl Future<Output = Result<UpdateImagePermissionsOutput, SdkError<UpdateImagePermissionsError>>>;
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>>;
}
impl AppStreamClient for AppStreamClientImpl {
    fn associate_app_block_builder_app_block(&self, builder: AssociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<AssociateAppBlockBuilderAppBlockOutput, SdkError<AssociateAppBlockBuilderAppBlockError>>> {
        builder.send_with(&self.0)
    }
    fn associate_application_fleet(&self, builder: AssociateApplicationFleetInputBuilder) -> impl Future<Output = Result<AssociateApplicationFleetOutput, SdkError<AssociateApplicationFleetError>>> {
        builder.send_with(&self.0)
    }
    fn associate_application_to_entitlement(&self, builder: AssociateApplicationToEntitlementInputBuilder) -> impl Future<Output = Result<AssociateApplicationToEntitlementOutput, SdkError<AssociateApplicationToEntitlementError>>> {
        builder.send_with(&self.0)
    }
    fn associate_fleet(&self, builder: AssociateFleetInputBuilder) -> impl Future<Output = Result<AssociateFleetOutput, SdkError<AssociateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn batch_associate_user_stack(&self, builder: BatchAssociateUserStackInputBuilder) -> impl Future<Output = Result<BatchAssociateUserStackOutput, SdkError<BatchAssociateUserStackError>>> {
        builder.send_with(&self.0)
    }
    fn batch_disassociate_user_stack(&self, builder: BatchDisassociateUserStackInputBuilder) -> impl Future<Output = Result<BatchDisassociateUserStackOutput, SdkError<BatchDisassociateUserStackError>>> {
        builder.send_with(&self.0)
    }
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_block(&self, builder: CreateAppBlockInputBuilder) -> impl Future<Output = Result<CreateAppBlockOutput, SdkError<CreateAppBlockError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_block_builder(&self, builder: CreateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderOutput, SdkError<CreateAppBlockBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_block_builder_streaming_url(&self, builder: CreateAppBlockBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderStreamingUrlOutput, SdkError<CreateAppBlockBuilderStreamingURLError>>> {
        builder.send_with(&self.0)
    }
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_directory_config(&self, builder: CreateDirectoryConfigInputBuilder) -> impl Future<Output = Result<CreateDirectoryConfigOutput, SdkError<CreateDirectoryConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_entitlement(&self, builder: CreateEntitlementInputBuilder) -> impl Future<Output = Result<CreateEntitlementOutput, SdkError<CreateEntitlementError>>> {
        builder.send_with(&self.0)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn create_image_builder(&self, builder: CreateImageBuilderInputBuilder) -> impl Future<Output = Result<CreateImageBuilderOutput, SdkError<CreateImageBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn create_image_builder_streaming_url(&self, builder: CreateImageBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateImageBuilderStreamingUrlOutput, SdkError<CreateImageBuilderStreamingURLError>>> {
        builder.send_with(&self.0)
    }
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>> {
        builder.send_with(&self.0)
    }
    fn create_streaming_url(&self, builder: CreateStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateStreamingUrlOutput, SdkError<CreateStreamingURLError>>> {
        builder.send_with(&self.0)
    }
    fn create_updated_image(&self, builder: CreateUpdatedImageInputBuilder) -> impl Future<Output = Result<CreateUpdatedImageOutput, SdkError<CreateUpdatedImageError>>> {
        builder.send_with(&self.0)
    }
    fn create_usage_report_subscription(&self, builder: CreateUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<CreateUsageReportSubscriptionOutput, SdkError<CreateUsageReportSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app_block(&self, builder: DeleteAppBlockInputBuilder) -> impl Future<Output = Result<DeleteAppBlockOutput, SdkError<DeleteAppBlockError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app_block_builder(&self, builder: DeleteAppBlockBuilderInputBuilder) -> impl Future<Output = Result<DeleteAppBlockBuilderOutput, SdkError<DeleteAppBlockBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_directory_config(&self, builder: DeleteDirectoryConfigInputBuilder) -> impl Future<Output = Result<DeleteDirectoryConfigOutput, SdkError<DeleteDirectoryConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_entitlement(&self, builder: DeleteEntitlementInputBuilder) -> impl Future<Output = Result<DeleteEntitlementOutput, SdkError<DeleteEntitlementError>>> {
        builder.send_with(&self.0)
    }
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_image_builder(&self, builder: DeleteImageBuilderInputBuilder) -> impl Future<Output = Result<DeleteImageBuilderOutput, SdkError<DeleteImageBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_image_permissions(&self, builder: DeleteImagePermissionsInputBuilder) -> impl Future<Output = Result<DeleteImagePermissionsOutput, SdkError<DeleteImagePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>> {
        builder.send_with(&self.0)
    }
    fn delete_usage_report_subscription(&self, builder: DeleteUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteUsageReportSubscriptionOutput, SdkError<DeleteUsageReportSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn describe_app_block_builder_app_block_associations(&self, builder: DescribeAppBlockBuilderAppBlockAssociationsInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuilderAppBlockAssociationsOutput, SdkError<DescribeAppBlockBuilderAppBlockAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_app_block_builders(&self, builder: DescribeAppBlockBuildersInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuildersOutput, SdkError<DescribeAppBlockBuildersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_app_blocks(&self, builder: DescribeAppBlocksInputBuilder) -> impl Future<Output = Result<DescribeAppBlocksOutput, SdkError<DescribeAppBlocksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_application_fleet_associations(&self, builder: DescribeApplicationFleetAssociationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationFleetAssociationsOutput, SdkError<DescribeApplicationFleetAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_directory_configs(&self, builder: DescribeDirectoryConfigsInputBuilder) -> impl Future<Output = Result<DescribeDirectoryConfigsOutput, SdkError<DescribeDirectoryConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_entitlements(&self, builder: DescribeEntitlementsInputBuilder) -> impl Future<Output = Result<DescribeEntitlementsOutput, SdkError<DescribeEntitlementsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_image_builders(&self, builder: DescribeImageBuildersInputBuilder) -> impl Future<Output = Result<DescribeImageBuildersOutput, SdkError<DescribeImageBuildersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_image_permissions(&self, builder: DescribeImagePermissionsInputBuilder) -> impl Future<Output = Result<DescribeImagePermissionsOutput, SdkError<DescribeImagePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_usage_report_subscriptions(&self, builder: DescribeUsageReportSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeUsageReportSubscriptionsOutput, SdkError<DescribeUsageReportSubscriptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_stack_associations(&self, builder: DescribeUserStackAssociationsInputBuilder) -> impl Future<Output = Result<DescribeUserStackAssociationsOutput, SdkError<DescribeUserStackAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>> {
        builder.send_with(&self.0)
    }
    fn disable_user(&self, builder: DisableUserInputBuilder) -> impl Future<Output = Result<DisableUserOutput, SdkError<DisableUserError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_app_block_builder_app_block(&self, builder: DisassociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<DisassociateAppBlockBuilderAppBlockOutput, SdkError<DisassociateAppBlockBuilderAppBlockError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_application_fleet(&self, builder: DisassociateApplicationFleetInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFleetOutput, SdkError<DisassociateApplicationFleetError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_application_from_entitlement(&self, builder: DisassociateApplicationFromEntitlementInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFromEntitlementOutput, SdkError<DisassociateApplicationFromEntitlementError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_fleet(&self, builder: DisassociateFleetInputBuilder) -> impl Future<Output = Result<DisassociateFleetOutput, SdkError<DisassociateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn enable_user(&self, builder: EnableUserInputBuilder) -> impl Future<Output = Result<EnableUserOutput, SdkError<EnableUserError>>> {
        builder.send_with(&self.0)
    }
    fn expire_session(&self, builder: ExpireSessionInputBuilder) -> impl Future<Output = Result<ExpireSessionOutput, SdkError<ExpireSessionError>>> {
        builder.send_with(&self.0)
    }
    fn list_associated_fleets(&self, builder: ListAssociatedFleetsInputBuilder) -> impl Future<Output = Result<ListAssociatedFleetsOutput, SdkError<ListAssociatedFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_associated_stacks(&self, builder: ListAssociatedStacksInputBuilder) -> impl Future<Output = Result<ListAssociatedStacksOutput, SdkError<ListAssociatedStacksError>>> {
        builder.send_with(&self.0)
    }
    fn list_entitled_applications(&self, builder: ListEntitledApplicationsInputBuilder) -> impl Future<Output = Result<ListEntitledApplicationsOutput, SdkError<ListEntitledApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_app_block_builder(&self, builder: StartAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StartAppBlockBuilderOutput, SdkError<StartAppBlockBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn start_fleet(&self, builder: StartFleetInputBuilder) -> impl Future<Output = Result<StartFleetOutput, SdkError<StartFleetError>>> {
        builder.send_with(&self.0)
    }
    fn start_image_builder(&self, builder: StartImageBuilderInputBuilder) -> impl Future<Output = Result<StartImageBuilderOutput, SdkError<StartImageBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn stop_app_block_builder(&self, builder: StopAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StopAppBlockBuilderOutput, SdkError<StopAppBlockBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn stop_fleet(&self, builder: StopFleetInputBuilder) -> impl Future<Output = Result<StopFleetOutput, SdkError<StopFleetError>>> {
        builder.send_with(&self.0)
    }
    fn stop_image_builder(&self, builder: StopImageBuilderInputBuilder) -> impl Future<Output = Result<StopImageBuilderOutput, SdkError<StopImageBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_app_block_builder(&self, builder: UpdateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<UpdateAppBlockBuilderOutput, SdkError<UpdateAppBlockBuilderError>>> {
        builder.send_with(&self.0)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn update_directory_config(&self, builder: UpdateDirectoryConfigInputBuilder) -> impl Future<Output = Result<UpdateDirectoryConfigOutput, SdkError<UpdateDirectoryConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_entitlement(&self, builder: UpdateEntitlementInputBuilder) -> impl Future<Output = Result<UpdateEntitlementOutput, SdkError<UpdateEntitlementError>>> {
        builder.send_with(&self.0)
    }
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn update_image_permissions(&self, builder: UpdateImagePermissionsInputBuilder) -> impl Future<Output = Result<UpdateImagePermissionsOutput, SdkError<UpdateImagePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppStreamClient for T
where T: Deref,
      T::Target: AppStreamClient {
    fn associate_app_block_builder_app_block(&self, builder: AssociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<AssociateAppBlockBuilderAppBlockOutput, SdkError<AssociateAppBlockBuilderAppBlockError>>> {
        self.deref().associate_app_block_builder_app_block(builder)
    }
    fn associate_application_fleet(&self, builder: AssociateApplicationFleetInputBuilder) -> impl Future<Output = Result<AssociateApplicationFleetOutput, SdkError<AssociateApplicationFleetError>>> {
        self.deref().associate_application_fleet(builder)
    }
    fn associate_application_to_entitlement(&self, builder: AssociateApplicationToEntitlementInputBuilder) -> impl Future<Output = Result<AssociateApplicationToEntitlementOutput, SdkError<AssociateApplicationToEntitlementError>>> {
        self.deref().associate_application_to_entitlement(builder)
    }
    fn associate_fleet(&self, builder: AssociateFleetInputBuilder) -> impl Future<Output = Result<AssociateFleetOutput, SdkError<AssociateFleetError>>> {
        self.deref().associate_fleet(builder)
    }
    fn batch_associate_user_stack(&self, builder: BatchAssociateUserStackInputBuilder) -> impl Future<Output = Result<BatchAssociateUserStackOutput, SdkError<BatchAssociateUserStackError>>> {
        self.deref().batch_associate_user_stack(builder)
    }
    fn batch_disassociate_user_stack(&self, builder: BatchDisassociateUserStackInputBuilder) -> impl Future<Output = Result<BatchDisassociateUserStackOutput, SdkError<BatchDisassociateUserStackError>>> {
        self.deref().batch_disassociate_user_stack(builder)
    }
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>> {
        self.deref().copy_image(builder)
    }
    fn create_app_block(&self, builder: CreateAppBlockInputBuilder) -> impl Future<Output = Result<CreateAppBlockOutput, SdkError<CreateAppBlockError>>> {
        self.deref().create_app_block(builder)
    }
    fn create_app_block_builder(&self, builder: CreateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderOutput, SdkError<CreateAppBlockBuilderError>>> {
        self.deref().create_app_block_builder(builder)
    }
    fn create_app_block_builder_streaming_url(&self, builder: CreateAppBlockBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateAppBlockBuilderStreamingUrlOutput, SdkError<CreateAppBlockBuilderStreamingURLError>>> {
        self.deref().create_app_block_builder_streaming_url(builder)
    }
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        self.deref().create_application(builder)
    }
    fn create_directory_config(&self, builder: CreateDirectoryConfigInputBuilder) -> impl Future<Output = Result<CreateDirectoryConfigOutput, SdkError<CreateDirectoryConfigError>>> {
        self.deref().create_directory_config(builder)
    }
    fn create_entitlement(&self, builder: CreateEntitlementInputBuilder) -> impl Future<Output = Result<CreateEntitlementOutput, SdkError<CreateEntitlementError>>> {
        self.deref().create_entitlement(builder)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        self.deref().create_fleet(builder)
    }
    fn create_image_builder(&self, builder: CreateImageBuilderInputBuilder) -> impl Future<Output = Result<CreateImageBuilderOutput, SdkError<CreateImageBuilderError>>> {
        self.deref().create_image_builder(builder)
    }
    fn create_image_builder_streaming_url(&self, builder: CreateImageBuilderStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateImageBuilderStreamingUrlOutput, SdkError<CreateImageBuilderStreamingURLError>>> {
        self.deref().create_image_builder_streaming_url(builder)
    }
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>> {
        self.deref().create_stack(builder)
    }
    fn create_streaming_url(&self, builder: CreateStreamingUrlInputBuilder) -> impl Future<Output = Result<CreateStreamingUrlOutput, SdkError<CreateStreamingURLError>>> {
        self.deref().create_streaming_url(builder)
    }
    fn create_updated_image(&self, builder: CreateUpdatedImageInputBuilder) -> impl Future<Output = Result<CreateUpdatedImageOutput, SdkError<CreateUpdatedImageError>>> {
        self.deref().create_updated_image(builder)
    }
    fn create_usage_report_subscription(&self, builder: CreateUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<CreateUsageReportSubscriptionOutput, SdkError<CreateUsageReportSubscriptionError>>> {
        self.deref().create_usage_report_subscription(builder)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        self.deref().create_user(builder)
    }
    fn delete_app_block(&self, builder: DeleteAppBlockInputBuilder) -> impl Future<Output = Result<DeleteAppBlockOutput, SdkError<DeleteAppBlockError>>> {
        self.deref().delete_app_block(builder)
    }
    fn delete_app_block_builder(&self, builder: DeleteAppBlockBuilderInputBuilder) -> impl Future<Output = Result<DeleteAppBlockBuilderOutput, SdkError<DeleteAppBlockBuilderError>>> {
        self.deref().delete_app_block_builder(builder)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        self.deref().delete_application(builder)
    }
    fn delete_directory_config(&self, builder: DeleteDirectoryConfigInputBuilder) -> impl Future<Output = Result<DeleteDirectoryConfigOutput, SdkError<DeleteDirectoryConfigError>>> {
        self.deref().delete_directory_config(builder)
    }
    fn delete_entitlement(&self, builder: DeleteEntitlementInputBuilder) -> impl Future<Output = Result<DeleteEntitlementOutput, SdkError<DeleteEntitlementError>>> {
        self.deref().delete_entitlement(builder)
    }
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>> {
        self.deref().delete_fleet(builder)
    }
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>> {
        self.deref().delete_image(builder)
    }
    fn delete_image_builder(&self, builder: DeleteImageBuilderInputBuilder) -> impl Future<Output = Result<DeleteImageBuilderOutput, SdkError<DeleteImageBuilderError>>> {
        self.deref().delete_image_builder(builder)
    }
    fn delete_image_permissions(&self, builder: DeleteImagePermissionsInputBuilder) -> impl Future<Output = Result<DeleteImagePermissionsOutput, SdkError<DeleteImagePermissionsError>>> {
        self.deref().delete_image_permissions(builder)
    }
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>> {
        self.deref().delete_stack(builder)
    }
    fn delete_usage_report_subscription(&self, builder: DeleteUsageReportSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteUsageReportSubscriptionOutput, SdkError<DeleteUsageReportSubscriptionError>>> {
        self.deref().delete_usage_report_subscription(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        self.deref().delete_user(builder)
    }
    fn describe_app_block_builder_app_block_associations(&self, builder: DescribeAppBlockBuilderAppBlockAssociationsInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuilderAppBlockAssociationsOutput, SdkError<DescribeAppBlockBuilderAppBlockAssociationsError>>> {
        self.deref().describe_app_block_builder_app_block_associations(builder)
    }
    fn describe_app_block_builders(&self, builder: DescribeAppBlockBuildersInputBuilder) -> impl Future<Output = Result<DescribeAppBlockBuildersOutput, SdkError<DescribeAppBlockBuildersError>>> {
        self.deref().describe_app_block_builders(builder)
    }
    fn describe_app_blocks(&self, builder: DescribeAppBlocksInputBuilder) -> impl Future<Output = Result<DescribeAppBlocksOutput, SdkError<DescribeAppBlocksError>>> {
        self.deref().describe_app_blocks(builder)
    }
    fn describe_application_fleet_associations(&self, builder: DescribeApplicationFleetAssociationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationFleetAssociationsOutput, SdkError<DescribeApplicationFleetAssociationsError>>> {
        self.deref().describe_application_fleet_associations(builder)
    }
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>> {
        self.deref().describe_applications(builder)
    }
    fn describe_directory_configs(&self, builder: DescribeDirectoryConfigsInputBuilder) -> impl Future<Output = Result<DescribeDirectoryConfigsOutput, SdkError<DescribeDirectoryConfigsError>>> {
        self.deref().describe_directory_configs(builder)
    }
    fn describe_entitlements(&self, builder: DescribeEntitlementsInputBuilder) -> impl Future<Output = Result<DescribeEntitlementsOutput, SdkError<DescribeEntitlementsError>>> {
        self.deref().describe_entitlements(builder)
    }
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>> {
        self.deref().describe_fleets(builder)
    }
    fn describe_image_builders(&self, builder: DescribeImageBuildersInputBuilder) -> impl Future<Output = Result<DescribeImageBuildersOutput, SdkError<DescribeImageBuildersError>>> {
        self.deref().describe_image_builders(builder)
    }
    fn describe_image_permissions(&self, builder: DescribeImagePermissionsInputBuilder) -> impl Future<Output = Result<DescribeImagePermissionsOutput, SdkError<DescribeImagePermissionsError>>> {
        self.deref().describe_image_permissions(builder)
    }
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>> {
        self.deref().describe_images(builder)
    }
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>> {
        self.deref().describe_sessions(builder)
    }
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>> {
        self.deref().describe_stacks(builder)
    }
    fn describe_usage_report_subscriptions(&self, builder: DescribeUsageReportSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeUsageReportSubscriptionsOutput, SdkError<DescribeUsageReportSubscriptionsError>>> {
        self.deref().describe_usage_report_subscriptions(builder)
    }
    fn describe_user_stack_associations(&self, builder: DescribeUserStackAssociationsInputBuilder) -> impl Future<Output = Result<DescribeUserStackAssociationsOutput, SdkError<DescribeUserStackAssociationsError>>> {
        self.deref().describe_user_stack_associations(builder)
    }
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>> {
        self.deref().describe_users(builder)
    }
    fn disable_user(&self, builder: DisableUserInputBuilder) -> impl Future<Output = Result<DisableUserOutput, SdkError<DisableUserError>>> {
        self.deref().disable_user(builder)
    }
    fn disassociate_app_block_builder_app_block(&self, builder: DisassociateAppBlockBuilderAppBlockInputBuilder) -> impl Future<Output = Result<DisassociateAppBlockBuilderAppBlockOutput, SdkError<DisassociateAppBlockBuilderAppBlockError>>> {
        self.deref().disassociate_app_block_builder_app_block(builder)
    }
    fn disassociate_application_fleet(&self, builder: DisassociateApplicationFleetInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFleetOutput, SdkError<DisassociateApplicationFleetError>>> {
        self.deref().disassociate_application_fleet(builder)
    }
    fn disassociate_application_from_entitlement(&self, builder: DisassociateApplicationFromEntitlementInputBuilder) -> impl Future<Output = Result<DisassociateApplicationFromEntitlementOutput, SdkError<DisassociateApplicationFromEntitlementError>>> {
        self.deref().disassociate_application_from_entitlement(builder)
    }
    fn disassociate_fleet(&self, builder: DisassociateFleetInputBuilder) -> impl Future<Output = Result<DisassociateFleetOutput, SdkError<DisassociateFleetError>>> {
        self.deref().disassociate_fleet(builder)
    }
    fn enable_user(&self, builder: EnableUserInputBuilder) -> impl Future<Output = Result<EnableUserOutput, SdkError<EnableUserError>>> {
        self.deref().enable_user(builder)
    }
    fn expire_session(&self, builder: ExpireSessionInputBuilder) -> impl Future<Output = Result<ExpireSessionOutput, SdkError<ExpireSessionError>>> {
        self.deref().expire_session(builder)
    }
    fn list_associated_fleets(&self, builder: ListAssociatedFleetsInputBuilder) -> impl Future<Output = Result<ListAssociatedFleetsOutput, SdkError<ListAssociatedFleetsError>>> {
        self.deref().list_associated_fleets(builder)
    }
    fn list_associated_stacks(&self, builder: ListAssociatedStacksInputBuilder) -> impl Future<Output = Result<ListAssociatedStacksOutput, SdkError<ListAssociatedStacksError>>> {
        self.deref().list_associated_stacks(builder)
    }
    fn list_entitled_applications(&self, builder: ListEntitledApplicationsInputBuilder) -> impl Future<Output = Result<ListEntitledApplicationsOutput, SdkError<ListEntitledApplicationsError>>> {
        self.deref().list_entitled_applications(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn start_app_block_builder(&self, builder: StartAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StartAppBlockBuilderOutput, SdkError<StartAppBlockBuilderError>>> {
        self.deref().start_app_block_builder(builder)
    }
    fn start_fleet(&self, builder: StartFleetInputBuilder) -> impl Future<Output = Result<StartFleetOutput, SdkError<StartFleetError>>> {
        self.deref().start_fleet(builder)
    }
    fn start_image_builder(&self, builder: StartImageBuilderInputBuilder) -> impl Future<Output = Result<StartImageBuilderOutput, SdkError<StartImageBuilderError>>> {
        self.deref().start_image_builder(builder)
    }
    fn stop_app_block_builder(&self, builder: StopAppBlockBuilderInputBuilder) -> impl Future<Output = Result<StopAppBlockBuilderOutput, SdkError<StopAppBlockBuilderError>>> {
        self.deref().stop_app_block_builder(builder)
    }
    fn stop_fleet(&self, builder: StopFleetInputBuilder) -> impl Future<Output = Result<StopFleetOutput, SdkError<StopFleetError>>> {
        self.deref().stop_fleet(builder)
    }
    fn stop_image_builder(&self, builder: StopImageBuilderInputBuilder) -> impl Future<Output = Result<StopImageBuilderOutput, SdkError<StopImageBuilderError>>> {
        self.deref().stop_image_builder(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_app_block_builder(&self, builder: UpdateAppBlockBuilderInputBuilder) -> impl Future<Output = Result<UpdateAppBlockBuilderOutput, SdkError<UpdateAppBlockBuilderError>>> {
        self.deref().update_app_block_builder(builder)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        self.deref().update_application(builder)
    }
    fn update_directory_config(&self, builder: UpdateDirectoryConfigInputBuilder) -> impl Future<Output = Result<UpdateDirectoryConfigOutput, SdkError<UpdateDirectoryConfigError>>> {
        self.deref().update_directory_config(builder)
    }
    fn update_entitlement(&self, builder: UpdateEntitlementInputBuilder) -> impl Future<Output = Result<UpdateEntitlementOutput, SdkError<UpdateEntitlementError>>> {
        self.deref().update_entitlement(builder)
    }
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>> {
        self.deref().update_fleet(builder)
    }
    fn update_image_permissions(&self, builder: UpdateImagePermissionsInputBuilder) -> impl Future<Output = Result<UpdateImagePermissionsOutput, SdkError<UpdateImagePermissionsError>>> {
        self.deref().update_image_permissions(builder)
    }
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>> {
        self.deref().update_stack(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppStreamClient {}
    impl AppStreamClient for edAppStreamClient {
        async fn associate_app_block_builder_app_block(&self, builder: AssociateAppBlockBuilderAppBlockInputBuilder) -> Result<AssociateAppBlockBuilderAppBlockOutput, SdkError<AssociateAppBlockBuilderAppBlockError>>;
        async fn associate_application_fleet(&self, builder: AssociateApplicationFleetInputBuilder) -> Result<AssociateApplicationFleetOutput, SdkError<AssociateApplicationFleetError>>;
        async fn associate_application_to_entitlement(&self, builder: AssociateApplicationToEntitlementInputBuilder) -> Result<AssociateApplicationToEntitlementOutput, SdkError<AssociateApplicationToEntitlementError>>;
        async fn associate_fleet(&self, builder: AssociateFleetInputBuilder) -> Result<AssociateFleetOutput, SdkError<AssociateFleetError>>;
        async fn batch_associate_user_stack(&self, builder: BatchAssociateUserStackInputBuilder) -> Result<BatchAssociateUserStackOutput, SdkError<BatchAssociateUserStackError>>;
        async fn batch_disassociate_user_stack(&self, builder: BatchDisassociateUserStackInputBuilder) -> Result<BatchDisassociateUserStackOutput, SdkError<BatchDisassociateUserStackError>>;
        async fn copy_image(&self, builder: CopyImageInputBuilder) -> Result<CopyImageOutput, SdkError<CopyImageError>>;
        async fn create_app_block(&self, builder: CreateAppBlockInputBuilder) -> Result<CreateAppBlockOutput, SdkError<CreateAppBlockError>>;
        async fn create_app_block_builder(&self, builder: CreateAppBlockBuilderInputBuilder) -> Result<CreateAppBlockBuilderOutput, SdkError<CreateAppBlockBuilderError>>;
        async fn create_app_block_builder_streaming_url(&self, builder: CreateAppBlockBuilderStreamingUrlInputBuilder) -> Result<CreateAppBlockBuilderStreamingUrlOutput, SdkError<CreateAppBlockBuilderStreamingURLError>>;
        async fn create_application(&self, builder: CreateApplicationInputBuilder) -> Result<CreateApplicationOutput, SdkError<CreateApplicationError>>;
        async fn create_directory_config(&self, builder: CreateDirectoryConfigInputBuilder) -> Result<CreateDirectoryConfigOutput, SdkError<CreateDirectoryConfigError>>;
        async fn create_entitlement(&self, builder: CreateEntitlementInputBuilder) -> Result<CreateEntitlementOutput, SdkError<CreateEntitlementError>>;
        async fn create_fleet(&self, builder: CreateFleetInputBuilder) -> Result<CreateFleetOutput, SdkError<CreateFleetError>>;
        async fn create_image_builder(&self, builder: CreateImageBuilderInputBuilder) -> Result<CreateImageBuilderOutput, SdkError<CreateImageBuilderError>>;
        async fn create_image_builder_streaming_url(&self, builder: CreateImageBuilderStreamingUrlInputBuilder) -> Result<CreateImageBuilderStreamingUrlOutput, SdkError<CreateImageBuilderStreamingURLError>>;
        async fn create_stack(&self, builder: CreateStackInputBuilder) -> Result<CreateStackOutput, SdkError<CreateStackError>>;
        async fn create_streaming_url(&self, builder: CreateStreamingUrlInputBuilder) -> Result<CreateStreamingUrlOutput, SdkError<CreateStreamingURLError>>;
        async fn create_updated_image(&self, builder: CreateUpdatedImageInputBuilder) -> Result<CreateUpdatedImageOutput, SdkError<CreateUpdatedImageError>>;
        async fn create_usage_report_subscription(&self, builder: CreateUsageReportSubscriptionInputBuilder) -> Result<CreateUsageReportSubscriptionOutput, SdkError<CreateUsageReportSubscriptionError>>;
        async fn create_user(&self, builder: CreateUserInputBuilder) -> Result<CreateUserOutput, SdkError<CreateUserError>>;
        async fn delete_app_block(&self, builder: DeleteAppBlockInputBuilder) -> Result<DeleteAppBlockOutput, SdkError<DeleteAppBlockError>>;
        async fn delete_app_block_builder(&self, builder: DeleteAppBlockBuilderInputBuilder) -> Result<DeleteAppBlockBuilderOutput, SdkError<DeleteAppBlockBuilderError>>;
        async fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>;
        async fn delete_directory_config(&self, builder: DeleteDirectoryConfigInputBuilder) -> Result<DeleteDirectoryConfigOutput, SdkError<DeleteDirectoryConfigError>>;
        async fn delete_entitlement(&self, builder: DeleteEntitlementInputBuilder) -> Result<DeleteEntitlementOutput, SdkError<DeleteEntitlementError>>;
        async fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> Result<DeleteFleetOutput, SdkError<DeleteFleetError>>;
        async fn delete_image(&self, builder: DeleteImageInputBuilder) -> Result<DeleteImageOutput, SdkError<DeleteImageError>>;
        async fn delete_image_builder(&self, builder: DeleteImageBuilderInputBuilder) -> Result<DeleteImageBuilderOutput, SdkError<DeleteImageBuilderError>>;
        async fn delete_image_permissions(&self, builder: DeleteImagePermissionsInputBuilder) -> Result<DeleteImagePermissionsOutput, SdkError<DeleteImagePermissionsError>>;
        async fn delete_stack(&self, builder: DeleteStackInputBuilder) -> Result<DeleteStackOutput, SdkError<DeleteStackError>>;
        async fn delete_usage_report_subscription(&self, builder: DeleteUsageReportSubscriptionInputBuilder) -> Result<DeleteUsageReportSubscriptionOutput, SdkError<DeleteUsageReportSubscriptionError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn describe_app_block_builder_app_block_associations(&self, builder: DescribeAppBlockBuilderAppBlockAssociationsInputBuilder) -> Result<DescribeAppBlockBuilderAppBlockAssociationsOutput, SdkError<DescribeAppBlockBuilderAppBlockAssociationsError>>;
        async fn describe_app_block_builders(&self, builder: DescribeAppBlockBuildersInputBuilder) -> Result<DescribeAppBlockBuildersOutput, SdkError<DescribeAppBlockBuildersError>>;
        async fn describe_app_blocks(&self, builder: DescribeAppBlocksInputBuilder) -> Result<DescribeAppBlocksOutput, SdkError<DescribeAppBlocksError>>;
        async fn describe_application_fleet_associations(&self, builder: DescribeApplicationFleetAssociationsInputBuilder) -> Result<DescribeApplicationFleetAssociationsOutput, SdkError<DescribeApplicationFleetAssociationsError>>;
        async fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>;
        async fn describe_directory_configs(&self, builder: DescribeDirectoryConfigsInputBuilder) -> Result<DescribeDirectoryConfigsOutput, SdkError<DescribeDirectoryConfigsError>>;
        async fn describe_entitlements(&self, builder: DescribeEntitlementsInputBuilder) -> Result<DescribeEntitlementsOutput, SdkError<DescribeEntitlementsError>>;
        async fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>;
        async fn describe_image_builders(&self, builder: DescribeImageBuildersInputBuilder) -> Result<DescribeImageBuildersOutput, SdkError<DescribeImageBuildersError>>;
        async fn describe_image_permissions(&self, builder: DescribeImagePermissionsInputBuilder) -> Result<DescribeImagePermissionsOutput, SdkError<DescribeImagePermissionsError>>;
        async fn describe_images(&self, builder: DescribeImagesInputBuilder) -> Result<DescribeImagesOutput, SdkError<DescribeImagesError>>;
        async fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>;
        async fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> Result<DescribeStacksOutput, SdkError<DescribeStacksError>>;
        async fn describe_usage_report_subscriptions(&self, builder: DescribeUsageReportSubscriptionsInputBuilder) -> Result<DescribeUsageReportSubscriptionsOutput, SdkError<DescribeUsageReportSubscriptionsError>>;
        async fn describe_user_stack_associations(&self, builder: DescribeUserStackAssociationsInputBuilder) -> Result<DescribeUserStackAssociationsOutput, SdkError<DescribeUserStackAssociationsError>>;
        async fn describe_users(&self, builder: DescribeUsersInputBuilder) -> Result<DescribeUsersOutput, SdkError<DescribeUsersError>>;
        async fn disable_user(&self, builder: DisableUserInputBuilder) -> Result<DisableUserOutput, SdkError<DisableUserError>>;
        async fn disassociate_app_block_builder_app_block(&self, builder: DisassociateAppBlockBuilderAppBlockInputBuilder) -> Result<DisassociateAppBlockBuilderAppBlockOutput, SdkError<DisassociateAppBlockBuilderAppBlockError>>;
        async fn disassociate_application_fleet(&self, builder: DisassociateApplicationFleetInputBuilder) -> Result<DisassociateApplicationFleetOutput, SdkError<DisassociateApplicationFleetError>>;
        async fn disassociate_application_from_entitlement(&self, builder: DisassociateApplicationFromEntitlementInputBuilder) -> Result<DisassociateApplicationFromEntitlementOutput, SdkError<DisassociateApplicationFromEntitlementError>>;
        async fn disassociate_fleet(&self, builder: DisassociateFleetInputBuilder) -> Result<DisassociateFleetOutput, SdkError<DisassociateFleetError>>;
        async fn enable_user(&self, builder: EnableUserInputBuilder) -> Result<EnableUserOutput, SdkError<EnableUserError>>;
        async fn expire_session(&self, builder: ExpireSessionInputBuilder) -> Result<ExpireSessionOutput, SdkError<ExpireSessionError>>;
        async fn list_associated_fleets(&self, builder: ListAssociatedFleetsInputBuilder) -> Result<ListAssociatedFleetsOutput, SdkError<ListAssociatedFleetsError>>;
        async fn list_associated_stacks(&self, builder: ListAssociatedStacksInputBuilder) -> Result<ListAssociatedStacksOutput, SdkError<ListAssociatedStacksError>>;
        async fn list_entitled_applications(&self, builder: ListEntitledApplicationsInputBuilder) -> Result<ListEntitledApplicationsOutput, SdkError<ListEntitledApplicationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn start_app_block_builder(&self, builder: StartAppBlockBuilderInputBuilder) -> Result<StartAppBlockBuilderOutput, SdkError<StartAppBlockBuilderError>>;
        async fn start_fleet(&self, builder: StartFleetInputBuilder) -> Result<StartFleetOutput, SdkError<StartFleetError>>;
        async fn start_image_builder(&self, builder: StartImageBuilderInputBuilder) -> Result<StartImageBuilderOutput, SdkError<StartImageBuilderError>>;
        async fn stop_app_block_builder(&self, builder: StopAppBlockBuilderInputBuilder) -> Result<StopAppBlockBuilderOutput, SdkError<StopAppBlockBuilderError>>;
        async fn stop_fleet(&self, builder: StopFleetInputBuilder) -> Result<StopFleetOutput, SdkError<StopFleetError>>;
        async fn stop_image_builder(&self, builder: StopImageBuilderInputBuilder) -> Result<StopImageBuilderOutput, SdkError<StopImageBuilderError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_app_block_builder(&self, builder: UpdateAppBlockBuilderInputBuilder) -> Result<UpdateAppBlockBuilderOutput, SdkError<UpdateAppBlockBuilderError>>;
        async fn update_application(&self, builder: UpdateApplicationInputBuilder) -> Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>;
        async fn update_directory_config(&self, builder: UpdateDirectoryConfigInputBuilder) -> Result<UpdateDirectoryConfigOutput, SdkError<UpdateDirectoryConfigError>>;
        async fn update_entitlement(&self, builder: UpdateEntitlementInputBuilder) -> Result<UpdateEntitlementOutput, SdkError<UpdateEntitlementError>>;
        async fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> Result<UpdateFleetOutput, SdkError<UpdateFleetError>>;
        async fn update_image_permissions(&self, builder: UpdateImagePermissionsInputBuilder) -> Result<UpdateImagePermissionsOutput, SdkError<UpdateImagePermissionsError>>;
        async fn update_stack(&self, builder: UpdateStackInputBuilder) -> Result<UpdateStackOutput, SdkError<UpdateStackError>>;
    }
}
