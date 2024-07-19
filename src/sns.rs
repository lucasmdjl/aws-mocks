/*
 * aws_mocks - A mocking library for AWS.
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
use aws_sdk_sns::operation::add_permission::{builders::*, *};
use aws_sdk_sns::operation::check_if_phone_number_is_opted_out::{builders::*, *};
use aws_sdk_sns::operation::confirm_subscription::{builders::*, *};
use aws_sdk_sns::operation::create_platform_application::{builders::*, *};
use aws_sdk_sns::operation::create_platform_endpoint::{builders::*, *};
use aws_sdk_sns::operation::create_sms_sandbox_phone_number::{builders::*, *};
use aws_sdk_sns::operation::create_topic::{builders::*, *};
use aws_sdk_sns::operation::delete_endpoint::{builders::*, *};
use aws_sdk_sns::operation::delete_platform_application::{builders::*, *};
use aws_sdk_sns::operation::delete_sms_sandbox_phone_number::{builders::*, *};
use aws_sdk_sns::operation::delete_topic::{builders::*, *};
use aws_sdk_sns::operation::get_data_protection_policy::{builders::*, *};
use aws_sdk_sns::operation::get_endpoint_attributes::{builders::*, *};
use aws_sdk_sns::operation::get_platform_application_attributes::{builders::*, *};
use aws_sdk_sns::operation::get_sms_attributes::{builders::*, *};
use aws_sdk_sns::operation::get_sms_sandbox_account_status::{builders::*, *};
use aws_sdk_sns::operation::get_subscription_attributes::{builders::*, *};
use aws_sdk_sns::operation::get_topic_attributes::{builders::*, *};
use aws_sdk_sns::operation::list_endpoints_by_platform_application::{builders::*, *};
use aws_sdk_sns::operation::list_origination_numbers::{builders::*, *};
use aws_sdk_sns::operation::list_phone_numbers_opted_out::{builders::*, *};
use aws_sdk_sns::operation::list_platform_applications::{builders::*, *};
use aws_sdk_sns::operation::list_sms_sandbox_phone_numbers::{builders::*, *};
use aws_sdk_sns::operation::list_subscriptions::{builders::*, *};
use aws_sdk_sns::operation::list_subscriptions_by_topic::{builders::*, *};
use aws_sdk_sns::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_sns::operation::list_topics::{builders::*, *};
use aws_sdk_sns::operation::opt_in_phone_number::{builders::*, *};
use aws_sdk_sns::operation::publish::{builders::*, *};
use aws_sdk_sns::operation::publish_batch::{builders::*, *};
use aws_sdk_sns::operation::put_data_protection_policy::{builders::*, *};
use aws_sdk_sns::operation::remove_permission::{builders::*, *};
use aws_sdk_sns::operation::set_endpoint_attributes::{builders::*, *};
use aws_sdk_sns::operation::set_platform_application_attributes::{builders::*, *};
use aws_sdk_sns::operation::set_sms_attributes::{builders::*, *};
use aws_sdk_sns::operation::set_subscription_attributes::{builders::*, *};
use aws_sdk_sns::operation::set_topic_attributes::{builders::*, *};
use aws_sdk_sns::operation::subscribe::{builders::*, *};
use aws_sdk_sns::operation::tag_resource::{builders::*, *};
use aws_sdk_sns::operation::unsubscribe::{builders::*, *};
use aws_sdk_sns::operation::untag_resource::{builders::*, *};
use aws_sdk_sns::operation::verify_sms_sandbox_phone_number::{builders::*, *};
use aws_sdk_sns::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_sns::Client;

pub use aws_sdk_sns::*;

pub struct SNSClientImpl(Client);
impl SNSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait SNSClient {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>>;
    fn check_if_phone_number_is_opted_out(&self, builder: CheckIfPhoneNumberIsOptedOutInputBuilder) -> impl Future<Output = Result<CheckIfPhoneNumberIsOptedOutOutput, SdkError<CheckIfPhoneNumberIsOptedOutError>>>;
    fn confirm_subscription(&self, builder: ConfirmSubscriptionInputBuilder) -> impl Future<Output = Result<ConfirmSubscriptionOutput, SdkError<ConfirmSubscriptionError>>>;
    fn create_platform_application(&self, builder: CreatePlatformApplicationInputBuilder) -> impl Future<Output = Result<CreatePlatformApplicationOutput, SdkError<CreatePlatformApplicationError>>>;
    fn create_platform_endpoint(&self, builder: CreatePlatformEndpointInputBuilder) -> impl Future<Output = Result<CreatePlatformEndpointOutput, SdkError<CreatePlatformEndpointError>>>;
    fn create_sms_sandbox_phone_number(&self, builder: CreateSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<CreateSmsSandboxPhoneNumberOutput, SdkError<CreateSMSSandboxPhoneNumberError>>>;
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>>;
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>>;
    fn delete_platform_application(&self, builder: DeletePlatformApplicationInputBuilder) -> impl Future<Output = Result<DeletePlatformApplicationOutput, SdkError<DeletePlatformApplicationError>>>;
    fn delete_sms_sandbox_phone_number(&self, builder: DeleteSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<DeleteSmsSandboxPhoneNumberOutput, SdkError<DeleteSMSSandboxPhoneNumberError>>>;
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>>;
    fn get_data_protection_policy(&self, builder: GetDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<GetDataProtectionPolicyOutput, SdkError<GetDataProtectionPolicyError>>>;
    fn get_endpoint_attributes(&self, builder: GetEndpointAttributesInputBuilder) -> impl Future<Output = Result<GetEndpointAttributesOutput, SdkError<GetEndpointAttributesError>>>;
    fn get_platform_application_attributes(&self, builder: GetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<GetPlatformApplicationAttributesOutput, SdkError<GetPlatformApplicationAttributesError>>>;
    fn get_sms_attributes(&self, builder: GetSmsAttributesInputBuilder) -> impl Future<Output = Result<GetSmsAttributesOutput, SdkError<GetSMSAttributesError>>>;
    fn get_sms_sandbox_account_status(&self, builder: GetSmsSandboxAccountStatusInputBuilder) -> impl Future<Output = Result<GetSmsSandboxAccountStatusOutput, SdkError<GetSMSSandboxAccountStatusError>>>;
    fn get_subscription_attributes(&self, builder: GetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<GetSubscriptionAttributesOutput, SdkError<GetSubscriptionAttributesError>>>;
    fn get_topic_attributes(&self, builder: GetTopicAttributesInputBuilder) -> impl Future<Output = Result<GetTopicAttributesOutput, SdkError<GetTopicAttributesError>>>;
    fn list_endpoints_by_platform_application(&self, builder: ListEndpointsByPlatformApplicationInputBuilder) -> impl Future<Output = Result<ListEndpointsByPlatformApplicationOutput, SdkError<ListEndpointsByPlatformApplicationError>>>;
    fn list_origination_numbers(&self, builder: ListOriginationNumbersInputBuilder) -> impl Future<Output = Result<ListOriginationNumbersOutput, SdkError<ListOriginationNumbersError>>>;
    fn list_phone_numbers_opted_out(&self, builder: ListPhoneNumbersOptedOutInputBuilder) -> impl Future<Output = Result<ListPhoneNumbersOptedOutOutput, SdkError<ListPhoneNumbersOptedOutError>>>;
    fn list_platform_applications(&self, builder: ListPlatformApplicationsInputBuilder) -> impl Future<Output = Result<ListPlatformApplicationsOutput, SdkError<ListPlatformApplicationsError>>>;
    fn list_sms_sandbox_phone_numbers(&self, builder: ListSmsSandboxPhoneNumbersInputBuilder) -> impl Future<Output = Result<ListSmsSandboxPhoneNumbersOutput, SdkError<ListSMSSandboxPhoneNumbersError>>>;
    fn list_subscriptions(&self, builder: ListSubscriptionsInputBuilder) -> impl Future<Output = Result<ListSubscriptionsOutput, SdkError<ListSubscriptionsError>>>;
    fn list_subscriptions_by_topic(&self, builder: ListSubscriptionsByTopicInputBuilder) -> impl Future<Output = Result<ListSubscriptionsByTopicOutput, SdkError<ListSubscriptionsByTopicError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>>;
    fn opt_in_phone_number(&self, builder: OptInPhoneNumberInputBuilder) -> impl Future<Output = Result<OptInPhoneNumberOutput, SdkError<OptInPhoneNumberError>>>;
    fn publish(&self, builder: PublishInputBuilder) -> impl Future<Output = Result<PublishOutput, SdkError<PublishError>>>;
    fn publish_batch(&self, builder: PublishBatchInputBuilder) -> impl Future<Output = Result<PublishBatchOutput, SdkError<PublishBatchError>>>;
    fn put_data_protection_policy(&self, builder: PutDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<PutDataProtectionPolicyOutput, SdkError<PutDataProtectionPolicyError>>>;
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>>;
    fn set_endpoint_attributes(&self, builder: SetEndpointAttributesInputBuilder) -> impl Future<Output = Result<SetEndpointAttributesOutput, SdkError<SetEndpointAttributesError>>>;
    fn set_platform_application_attributes(&self, builder: SetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<SetPlatformApplicationAttributesOutput, SdkError<SetPlatformApplicationAttributesError>>>;
    fn set_sms_attributes(&self, builder: SetSmsAttributesInputBuilder) -> impl Future<Output = Result<SetSmsAttributesOutput, SdkError<SetSMSAttributesError>>>;
    fn set_subscription_attributes(&self, builder: SetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<SetSubscriptionAttributesOutput, SdkError<SetSubscriptionAttributesError>>>;
    fn set_topic_attributes(&self, builder: SetTopicAttributesInputBuilder) -> impl Future<Output = Result<SetTopicAttributesOutput, SdkError<SetTopicAttributesError>>>;
    fn subscribe(&self, builder: SubscribeInputBuilder) -> impl Future<Output = Result<SubscribeOutput, SdkError<SubscribeError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn unsubscribe(&self, builder: UnsubscribeInputBuilder) -> impl Future<Output = Result<UnsubscribeOutput, SdkError<UnsubscribeError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn verify_sms_sandbox_phone_number(&self, builder: VerifySmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<VerifySmsSandboxPhoneNumberOutput, SdkError<VerifySMSSandboxPhoneNumberError>>>;
}
impl SNSClient for SNSClientImpl {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn check_if_phone_number_is_opted_out(&self, builder: CheckIfPhoneNumberIsOptedOutInputBuilder) -> impl Future<Output = Result<CheckIfPhoneNumberIsOptedOutOutput, SdkError<CheckIfPhoneNumberIsOptedOutError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_subscription(&self, builder: ConfirmSubscriptionInputBuilder) -> impl Future<Output = Result<ConfirmSubscriptionOutput, SdkError<ConfirmSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_platform_application(&self, builder: CreatePlatformApplicationInputBuilder) -> impl Future<Output = Result<CreatePlatformApplicationOutput, SdkError<CreatePlatformApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_platform_endpoint(&self, builder: CreatePlatformEndpointInputBuilder) -> impl Future<Output = Result<CreatePlatformEndpointOutput, SdkError<CreatePlatformEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_sms_sandbox_phone_number(&self, builder: CreateSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<CreateSmsSandboxPhoneNumberOutput, SdkError<CreateSMSSandboxPhoneNumberError>>> {
        builder.send_with(&self.0)
    }
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>> {
        builder.send_with(&self.0)
    }
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_platform_application(&self, builder: DeletePlatformApplicationInputBuilder) -> impl Future<Output = Result<DeletePlatformApplicationOutput, SdkError<DeletePlatformApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_sms_sandbox_phone_number(&self, builder: DeleteSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<DeleteSmsSandboxPhoneNumberOutput, SdkError<DeleteSMSSandboxPhoneNumberError>>> {
        builder.send_with(&self.0)
    }
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_protection_policy(&self, builder: GetDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<GetDataProtectionPolicyOutput, SdkError<GetDataProtectionPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_endpoint_attributes(&self, builder: GetEndpointAttributesInputBuilder) -> impl Future<Output = Result<GetEndpointAttributesOutput, SdkError<GetEndpointAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_platform_application_attributes(&self, builder: GetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<GetPlatformApplicationAttributesOutput, SdkError<GetPlatformApplicationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sms_attributes(&self, builder: GetSmsAttributesInputBuilder) -> impl Future<Output = Result<GetSmsAttributesOutput, SdkError<GetSMSAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sms_sandbox_account_status(&self, builder: GetSmsSandboxAccountStatusInputBuilder) -> impl Future<Output = Result<GetSmsSandboxAccountStatusOutput, SdkError<GetSMSSandboxAccountStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_subscription_attributes(&self, builder: GetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<GetSubscriptionAttributesOutput, SdkError<GetSubscriptionAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_topic_attributes(&self, builder: GetTopicAttributesInputBuilder) -> impl Future<Output = Result<GetTopicAttributesOutput, SdkError<GetTopicAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn list_endpoints_by_platform_application(&self, builder: ListEndpointsByPlatformApplicationInputBuilder) -> impl Future<Output = Result<ListEndpointsByPlatformApplicationOutput, SdkError<ListEndpointsByPlatformApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn list_origination_numbers(&self, builder: ListOriginationNumbersInputBuilder) -> impl Future<Output = Result<ListOriginationNumbersOutput, SdkError<ListOriginationNumbersError>>> {
        builder.send_with(&self.0)
    }
    fn list_phone_numbers_opted_out(&self, builder: ListPhoneNumbersOptedOutInputBuilder) -> impl Future<Output = Result<ListPhoneNumbersOptedOutOutput, SdkError<ListPhoneNumbersOptedOutError>>> {
        builder.send_with(&self.0)
    }
    fn list_platform_applications(&self, builder: ListPlatformApplicationsInputBuilder) -> impl Future<Output = Result<ListPlatformApplicationsOutput, SdkError<ListPlatformApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_sms_sandbox_phone_numbers(&self, builder: ListSmsSandboxPhoneNumbersInputBuilder) -> impl Future<Output = Result<ListSmsSandboxPhoneNumbersOutput, SdkError<ListSMSSandboxPhoneNumbersError>>> {
        builder.send_with(&self.0)
    }
    fn list_subscriptions(&self, builder: ListSubscriptionsInputBuilder) -> impl Future<Output = Result<ListSubscriptionsOutput, SdkError<ListSubscriptionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_subscriptions_by_topic(&self, builder: ListSubscriptionsByTopicInputBuilder) -> impl Future<Output = Result<ListSubscriptionsByTopicOutput, SdkError<ListSubscriptionsByTopicError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>> {
        builder.send_with(&self.0)
    }
    fn opt_in_phone_number(&self, builder: OptInPhoneNumberInputBuilder) -> impl Future<Output = Result<OptInPhoneNumberOutput, SdkError<OptInPhoneNumberError>>> {
        builder.send_with(&self.0)
    }
    fn publish(&self, builder: PublishInputBuilder) -> impl Future<Output = Result<PublishOutput, SdkError<PublishError>>> {
        builder.send_with(&self.0)
    }
    fn publish_batch(&self, builder: PublishBatchInputBuilder) -> impl Future<Output = Result<PublishBatchOutput, SdkError<PublishBatchError>>> {
        builder.send_with(&self.0)
    }
    fn put_data_protection_policy(&self, builder: PutDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<PutDataProtectionPolicyOutput, SdkError<PutDataProtectionPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn set_endpoint_attributes(&self, builder: SetEndpointAttributesInputBuilder) -> impl Future<Output = Result<SetEndpointAttributesOutput, SdkError<SetEndpointAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn set_platform_application_attributes(&self, builder: SetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<SetPlatformApplicationAttributesOutput, SdkError<SetPlatformApplicationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn set_sms_attributes(&self, builder: SetSmsAttributesInputBuilder) -> impl Future<Output = Result<SetSmsAttributesOutput, SdkError<SetSMSAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn set_subscription_attributes(&self, builder: SetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<SetSubscriptionAttributesOutput, SdkError<SetSubscriptionAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn set_topic_attributes(&self, builder: SetTopicAttributesInputBuilder) -> impl Future<Output = Result<SetTopicAttributesOutput, SdkError<SetTopicAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn subscribe(&self, builder: SubscribeInputBuilder) -> impl Future<Output = Result<SubscribeOutput, SdkError<SubscribeError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn unsubscribe(&self, builder: UnsubscribeInputBuilder) -> impl Future<Output = Result<UnsubscribeOutput, SdkError<UnsubscribeError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn verify_sms_sandbox_phone_number(&self, builder: VerifySmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<VerifySmsSandboxPhoneNumberOutput, SdkError<VerifySMSSandboxPhoneNumberError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: SNSClient> SNSClient for &T {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        (*self).add_permission(builder)
    }
    fn check_if_phone_number_is_opted_out(&self, builder: CheckIfPhoneNumberIsOptedOutInputBuilder) -> impl Future<Output = Result<CheckIfPhoneNumberIsOptedOutOutput, SdkError<CheckIfPhoneNumberIsOptedOutError>>> {
        (*self).check_if_phone_number_is_opted_out(builder)
    }
    fn confirm_subscription(&self, builder: ConfirmSubscriptionInputBuilder) -> impl Future<Output = Result<ConfirmSubscriptionOutput, SdkError<ConfirmSubscriptionError>>> {
        (*self).confirm_subscription(builder)
    }
    fn create_platform_application(&self, builder: CreatePlatformApplicationInputBuilder) -> impl Future<Output = Result<CreatePlatformApplicationOutput, SdkError<CreatePlatformApplicationError>>> {
        (*self).create_platform_application(builder)
    }
    fn create_platform_endpoint(&self, builder: CreatePlatformEndpointInputBuilder) -> impl Future<Output = Result<CreatePlatformEndpointOutput, SdkError<CreatePlatformEndpointError>>> {
        (*self).create_platform_endpoint(builder)
    }
    fn create_sms_sandbox_phone_number(&self, builder: CreateSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<CreateSmsSandboxPhoneNumberOutput, SdkError<CreateSMSSandboxPhoneNumberError>>> {
        (*self).create_sms_sandbox_phone_number(builder)
    }
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>> {
        (*self).create_topic(builder)
    }
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>> {
        (*self).delete_endpoint(builder)
    }
    fn delete_platform_application(&self, builder: DeletePlatformApplicationInputBuilder) -> impl Future<Output = Result<DeletePlatformApplicationOutput, SdkError<DeletePlatformApplicationError>>> {
        (*self).delete_platform_application(builder)
    }
    fn delete_sms_sandbox_phone_number(&self, builder: DeleteSmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<DeleteSmsSandboxPhoneNumberOutput, SdkError<DeleteSMSSandboxPhoneNumberError>>> {
        (*self).delete_sms_sandbox_phone_number(builder)
    }
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>> {
        (*self).delete_topic(builder)
    }
    fn get_data_protection_policy(&self, builder: GetDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<GetDataProtectionPolicyOutput, SdkError<GetDataProtectionPolicyError>>> {
        (*self).get_data_protection_policy(builder)
    }
    fn get_endpoint_attributes(&self, builder: GetEndpointAttributesInputBuilder) -> impl Future<Output = Result<GetEndpointAttributesOutput, SdkError<GetEndpointAttributesError>>> {
        (*self).get_endpoint_attributes(builder)
    }
    fn get_platform_application_attributes(&self, builder: GetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<GetPlatformApplicationAttributesOutput, SdkError<GetPlatformApplicationAttributesError>>> {
        (*self).get_platform_application_attributes(builder)
    }
    fn get_sms_attributes(&self, builder: GetSmsAttributesInputBuilder) -> impl Future<Output = Result<GetSmsAttributesOutput, SdkError<GetSMSAttributesError>>> {
        (*self).get_sms_attributes(builder)
    }
    fn get_sms_sandbox_account_status(&self, builder: GetSmsSandboxAccountStatusInputBuilder) -> impl Future<Output = Result<GetSmsSandboxAccountStatusOutput, SdkError<GetSMSSandboxAccountStatusError>>> {
        (*self).get_sms_sandbox_account_status(builder)
    }
    fn get_subscription_attributes(&self, builder: GetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<GetSubscriptionAttributesOutput, SdkError<GetSubscriptionAttributesError>>> {
        (*self).get_subscription_attributes(builder)
    }
    fn get_topic_attributes(&self, builder: GetTopicAttributesInputBuilder) -> impl Future<Output = Result<GetTopicAttributesOutput, SdkError<GetTopicAttributesError>>> {
        (*self).get_topic_attributes(builder)
    }
    fn list_endpoints_by_platform_application(&self, builder: ListEndpointsByPlatformApplicationInputBuilder) -> impl Future<Output = Result<ListEndpointsByPlatformApplicationOutput, SdkError<ListEndpointsByPlatformApplicationError>>> {
        (*self).list_endpoints_by_platform_application(builder)
    }
    fn list_origination_numbers(&self, builder: ListOriginationNumbersInputBuilder) -> impl Future<Output = Result<ListOriginationNumbersOutput, SdkError<ListOriginationNumbersError>>> {
        (*self).list_origination_numbers(builder)
    }
    fn list_phone_numbers_opted_out(&self, builder: ListPhoneNumbersOptedOutInputBuilder) -> impl Future<Output = Result<ListPhoneNumbersOptedOutOutput, SdkError<ListPhoneNumbersOptedOutError>>> {
        (*self).list_phone_numbers_opted_out(builder)
    }
    fn list_platform_applications(&self, builder: ListPlatformApplicationsInputBuilder) -> impl Future<Output = Result<ListPlatformApplicationsOutput, SdkError<ListPlatformApplicationsError>>> {
        (*self).list_platform_applications(builder)
    }
    fn list_sms_sandbox_phone_numbers(&self, builder: ListSmsSandboxPhoneNumbersInputBuilder) -> impl Future<Output = Result<ListSmsSandboxPhoneNumbersOutput, SdkError<ListSMSSandboxPhoneNumbersError>>> {
        (*self).list_sms_sandbox_phone_numbers(builder)
    }
    fn list_subscriptions(&self, builder: ListSubscriptionsInputBuilder) -> impl Future<Output = Result<ListSubscriptionsOutput, SdkError<ListSubscriptionsError>>> {
        (*self).list_subscriptions(builder)
    }
    fn list_subscriptions_by_topic(&self, builder: ListSubscriptionsByTopicInputBuilder) -> impl Future<Output = Result<ListSubscriptionsByTopicOutput, SdkError<ListSubscriptionsByTopicError>>> {
        (*self).list_subscriptions_by_topic(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>> {
        (*self).list_topics(builder)
    }
    fn opt_in_phone_number(&self, builder: OptInPhoneNumberInputBuilder) -> impl Future<Output = Result<OptInPhoneNumberOutput, SdkError<OptInPhoneNumberError>>> {
        (*self).opt_in_phone_number(builder)
    }
    fn publish(&self, builder: PublishInputBuilder) -> impl Future<Output = Result<PublishOutput, SdkError<PublishError>>> {
        (*self).publish(builder)
    }
    fn publish_batch(&self, builder: PublishBatchInputBuilder) -> impl Future<Output = Result<PublishBatchOutput, SdkError<PublishBatchError>>> {
        (*self).publish_batch(builder)
    }
    fn put_data_protection_policy(&self, builder: PutDataProtectionPolicyInputBuilder) -> impl Future<Output = Result<PutDataProtectionPolicyOutput, SdkError<PutDataProtectionPolicyError>>> {
        (*self).put_data_protection_policy(builder)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        (*self).remove_permission(builder)
    }
    fn set_endpoint_attributes(&self, builder: SetEndpointAttributesInputBuilder) -> impl Future<Output = Result<SetEndpointAttributesOutput, SdkError<SetEndpointAttributesError>>> {
        (*self).set_endpoint_attributes(builder)
    }
    fn set_platform_application_attributes(&self, builder: SetPlatformApplicationAttributesInputBuilder) -> impl Future<Output = Result<SetPlatformApplicationAttributesOutput, SdkError<SetPlatformApplicationAttributesError>>> {
        (*self).set_platform_application_attributes(builder)
    }
    fn set_sms_attributes(&self, builder: SetSmsAttributesInputBuilder) -> impl Future<Output = Result<SetSmsAttributesOutput, SdkError<SetSMSAttributesError>>> {
        (*self).set_sms_attributes(builder)
    }
    fn set_subscription_attributes(&self, builder: SetSubscriptionAttributesInputBuilder) -> impl Future<Output = Result<SetSubscriptionAttributesOutput, SdkError<SetSubscriptionAttributesError>>> {
        (*self).set_subscription_attributes(builder)
    }
    fn set_topic_attributes(&self, builder: SetTopicAttributesInputBuilder) -> impl Future<Output = Result<SetTopicAttributesOutput, SdkError<SetTopicAttributesError>>> {
        (*self).set_topic_attributes(builder)
    }
    fn subscribe(&self, builder: SubscribeInputBuilder) -> impl Future<Output = Result<SubscribeOutput, SdkError<SubscribeError>>> {
        (*self).subscribe(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn unsubscribe(&self, builder: UnsubscribeInputBuilder) -> impl Future<Output = Result<UnsubscribeOutput, SdkError<UnsubscribeError>>> {
        (*self).unsubscribe(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn verify_sms_sandbox_phone_number(&self, builder: VerifySmsSandboxPhoneNumberInputBuilder) -> impl Future<Output = Result<VerifySmsSandboxPhoneNumberOutput, SdkError<VerifySMSSandboxPhoneNumberError>>> {
        (*self).verify_sms_sandbox_phone_number(builder)
    }
}
#[cfg(feature = "mocks")]
mockall::mock! {
    pub edSNSClient {}
    impl SNSClient for edSNSClient {
        async fn add_permission(&self, builder: AddPermissionInputBuilder) -> Result<AddPermissionOutput, SdkError<AddPermissionError>>;
        async fn check_if_phone_number_is_opted_out(&self, builder: CheckIfPhoneNumberIsOptedOutInputBuilder) -> Result<CheckIfPhoneNumberIsOptedOutOutput, SdkError<CheckIfPhoneNumberIsOptedOutError>>;
        async fn confirm_subscription(&self, builder: ConfirmSubscriptionInputBuilder) -> Result<ConfirmSubscriptionOutput, SdkError<ConfirmSubscriptionError>>;
        async fn create_platform_application(&self, builder: CreatePlatformApplicationInputBuilder) -> Result<CreatePlatformApplicationOutput, SdkError<CreatePlatformApplicationError>>;
        async fn create_platform_endpoint(&self, builder: CreatePlatformEndpointInputBuilder) -> Result<CreatePlatformEndpointOutput, SdkError<CreatePlatformEndpointError>>;
        async fn create_sms_sandbox_phone_number(&self, builder: CreateSmsSandboxPhoneNumberInputBuilder) -> Result<CreateSmsSandboxPhoneNumberOutput, SdkError<CreateSMSSandboxPhoneNumberError>>;
        async fn create_topic(&self, builder: CreateTopicInputBuilder) -> Result<CreateTopicOutput, SdkError<CreateTopicError>>;
        async fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>;
        async fn delete_platform_application(&self, builder: DeletePlatformApplicationInputBuilder) -> Result<DeletePlatformApplicationOutput, SdkError<DeletePlatformApplicationError>>;
        async fn delete_sms_sandbox_phone_number(&self, builder: DeleteSmsSandboxPhoneNumberInputBuilder) -> Result<DeleteSmsSandboxPhoneNumberOutput, SdkError<DeleteSMSSandboxPhoneNumberError>>;
        async fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> Result<DeleteTopicOutput, SdkError<DeleteTopicError>>;
        async fn get_data_protection_policy(&self, builder: GetDataProtectionPolicyInputBuilder) -> Result<GetDataProtectionPolicyOutput, SdkError<GetDataProtectionPolicyError>>;
        async fn get_endpoint_attributes(&self, builder: GetEndpointAttributesInputBuilder) -> Result<GetEndpointAttributesOutput, SdkError<GetEndpointAttributesError>>;
        async fn get_platform_application_attributes(&self, builder: GetPlatformApplicationAttributesInputBuilder) -> Result<GetPlatformApplicationAttributesOutput, SdkError<GetPlatformApplicationAttributesError>>;
        async fn get_sms_attributes(&self, builder: GetSmsAttributesInputBuilder) -> Result<GetSmsAttributesOutput, SdkError<GetSMSAttributesError>>;
        async fn get_sms_sandbox_account_status(&self, builder: GetSmsSandboxAccountStatusInputBuilder) -> Result<GetSmsSandboxAccountStatusOutput, SdkError<GetSMSSandboxAccountStatusError>>;
        async fn get_subscription_attributes(&self, builder: GetSubscriptionAttributesInputBuilder) -> Result<GetSubscriptionAttributesOutput, SdkError<GetSubscriptionAttributesError>>;
        async fn get_topic_attributes(&self, builder: GetTopicAttributesInputBuilder) -> Result<GetTopicAttributesOutput, SdkError<GetTopicAttributesError>>;
        async fn list_endpoints_by_platform_application(&self, builder: ListEndpointsByPlatformApplicationInputBuilder) -> Result<ListEndpointsByPlatformApplicationOutput, SdkError<ListEndpointsByPlatformApplicationError>>;
        async fn list_origination_numbers(&self, builder: ListOriginationNumbersInputBuilder) -> Result<ListOriginationNumbersOutput, SdkError<ListOriginationNumbersError>>;
        async fn list_phone_numbers_opted_out(&self, builder: ListPhoneNumbersOptedOutInputBuilder) -> Result<ListPhoneNumbersOptedOutOutput, SdkError<ListPhoneNumbersOptedOutError>>;
        async fn list_platform_applications(&self, builder: ListPlatformApplicationsInputBuilder) -> Result<ListPlatformApplicationsOutput, SdkError<ListPlatformApplicationsError>>;
        async fn list_sms_sandbox_phone_numbers(&self, builder: ListSmsSandboxPhoneNumbersInputBuilder) -> Result<ListSmsSandboxPhoneNumbersOutput, SdkError<ListSMSSandboxPhoneNumbersError>>;
        async fn list_subscriptions(&self, builder: ListSubscriptionsInputBuilder) -> Result<ListSubscriptionsOutput, SdkError<ListSubscriptionsError>>;
        async fn list_subscriptions_by_topic(&self, builder: ListSubscriptionsByTopicInputBuilder) -> Result<ListSubscriptionsByTopicOutput, SdkError<ListSubscriptionsByTopicError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_topics(&self, builder: ListTopicsInputBuilder) -> Result<ListTopicsOutput, SdkError<ListTopicsError>>;
        async fn opt_in_phone_number(&self, builder: OptInPhoneNumberInputBuilder) -> Result<OptInPhoneNumberOutput, SdkError<OptInPhoneNumberError>>;
        async fn publish(&self, builder: PublishInputBuilder) -> Result<PublishOutput, SdkError<PublishError>>;
        async fn publish_batch(&self, builder: PublishBatchInputBuilder) -> Result<PublishBatchOutput, SdkError<PublishBatchError>>;
        async fn put_data_protection_policy(&self, builder: PutDataProtectionPolicyInputBuilder) -> Result<PutDataProtectionPolicyOutput, SdkError<PutDataProtectionPolicyError>>;
        async fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> Result<RemovePermissionOutput, SdkError<RemovePermissionError>>;
        async fn set_endpoint_attributes(&self, builder: SetEndpointAttributesInputBuilder) -> Result<SetEndpointAttributesOutput, SdkError<SetEndpointAttributesError>>;
        async fn set_platform_application_attributes(&self, builder: SetPlatformApplicationAttributesInputBuilder) -> Result<SetPlatformApplicationAttributesOutput, SdkError<SetPlatformApplicationAttributesError>>;
        async fn set_sms_attributes(&self, builder: SetSmsAttributesInputBuilder) -> Result<SetSmsAttributesOutput, SdkError<SetSMSAttributesError>>;
        async fn set_subscription_attributes(&self, builder: SetSubscriptionAttributesInputBuilder) -> Result<SetSubscriptionAttributesOutput, SdkError<SetSubscriptionAttributesError>>;
        async fn set_topic_attributes(&self, builder: SetTopicAttributesInputBuilder) -> Result<SetTopicAttributesOutput, SdkError<SetTopicAttributesError>>;
        async fn subscribe(&self, builder: SubscribeInputBuilder) -> Result<SubscribeOutput, SdkError<SubscribeError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn unsubscribe(&self, builder: UnsubscribeInputBuilder) -> Result<UnsubscribeOutput, SdkError<UnsubscribeError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn verify_sms_sandbox_phone_number(&self, builder: VerifySmsSandboxPhoneNumberInputBuilder) -> Result<VerifySmsSandboxPhoneNumberOutput, SdkError<VerifySMSSandboxPhoneNumberError>>;
    }
}
