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
use aws_sdk_eks::operation::associate_access_policy::{builders::*, *};
use aws_sdk_eks::operation::associate_encryption_config::{builders::*, *};
use aws_sdk_eks::operation::associate_identity_provider_config::{builders::*, *};
use aws_sdk_eks::operation::create_access_entry::{builders::*, *};
use aws_sdk_eks::operation::create_addon::{builders::*, *};
use aws_sdk_eks::operation::create_cluster::{builders::*, *};
use aws_sdk_eks::operation::create_eks_anywhere_subscription::{builders::*, *};
use aws_sdk_eks::operation::create_fargate_profile::{builders::*, *};
use aws_sdk_eks::operation::create_nodegroup::{builders::*, *};
use aws_sdk_eks::operation::create_pod_identity_association::{builders::*, *};
use aws_sdk_eks::operation::delete_access_entry::{builders::*, *};
use aws_sdk_eks::operation::delete_addon::{builders::*, *};
use aws_sdk_eks::operation::delete_cluster::{builders::*, *};
use aws_sdk_eks::operation::delete_eks_anywhere_subscription::{builders::*, *};
use aws_sdk_eks::operation::delete_fargate_profile::{builders::*, *};
use aws_sdk_eks::operation::delete_nodegroup::{builders::*, *};
use aws_sdk_eks::operation::delete_pod_identity_association::{builders::*, *};
use aws_sdk_eks::operation::deregister_cluster::{builders::*, *};
use aws_sdk_eks::operation::describe_access_entry::{builders::*, *};
use aws_sdk_eks::operation::describe_addon::{builders::*, *};
use aws_sdk_eks::operation::describe_addon_configuration::{builders::*, *};
use aws_sdk_eks::operation::describe_addon_versions::{builders::*, *};
use aws_sdk_eks::operation::describe_cluster::{builders::*, *};
use aws_sdk_eks::operation::describe_eks_anywhere_subscription::{builders::*, *};
use aws_sdk_eks::operation::describe_fargate_profile::{builders::*, *};
use aws_sdk_eks::operation::describe_identity_provider_config::{builders::*, *};
use aws_sdk_eks::operation::describe_insight::{builders::*, *};
use aws_sdk_eks::operation::describe_nodegroup::{builders::*, *};
use aws_sdk_eks::operation::describe_pod_identity_association::{builders::*, *};
use aws_sdk_eks::operation::describe_update::{builders::*, *};
use aws_sdk_eks::operation::disassociate_access_policy::{builders::*, *};
use aws_sdk_eks::operation::disassociate_identity_provider_config::{builders::*, *};
use aws_sdk_eks::operation::list_access_entries::{builders::*, *};
use aws_sdk_eks::operation::list_access_policies::{builders::*, *};
use aws_sdk_eks::operation::list_addons::{builders::*, *};
use aws_sdk_eks::operation::list_associated_access_policies::{builders::*, *};
use aws_sdk_eks::operation::list_clusters::{builders::*, *};
use aws_sdk_eks::operation::list_eks_anywhere_subscriptions::{builders::*, *};
use aws_sdk_eks::operation::list_fargate_profiles::{builders::*, *};
use aws_sdk_eks::operation::list_identity_provider_configs::{builders::*, *};
use aws_sdk_eks::operation::list_insights::{builders::*, *};
use aws_sdk_eks::operation::list_nodegroups::{builders::*, *};
use aws_sdk_eks::operation::list_pod_identity_associations::{builders::*, *};
use aws_sdk_eks::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_eks::operation::list_updates::{builders::*, *};
use aws_sdk_eks::operation::register_cluster::{builders::*, *};
use aws_sdk_eks::operation::tag_resource::{builders::*, *};
use aws_sdk_eks::operation::untag_resource::{builders::*, *};
use aws_sdk_eks::operation::update_access_entry::{builders::*, *};
use aws_sdk_eks::operation::update_addon::{builders::*, *};
use aws_sdk_eks::operation::update_cluster_config::{builders::*, *};
use aws_sdk_eks::operation::update_cluster_version::{builders::*, *};
use aws_sdk_eks::operation::update_eks_anywhere_subscription::{builders::*, *};
use aws_sdk_eks::operation::update_nodegroup_config::{builders::*, *};
use aws_sdk_eks::operation::update_nodegroup_version::{builders::*, *};
use aws_sdk_eks::operation::update_pod_identity_association::{builders::*, *};
use aws_sdk_eks::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_eks::Client;
use std::ops::Deref;

pub use aws_sdk_eks::*;

pub struct EKSClientImpl(Client);
impl EKSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait EKSClient {
    fn associate_access_policy(&self, builder: AssociateAccessPolicyInputBuilder) -> impl Future<Output = Result<AssociateAccessPolicyOutput, SdkError<AssociateAccessPolicyError>>> + Send;
    fn associate_encryption_config(&self, builder: AssociateEncryptionConfigInputBuilder) -> impl Future<Output = Result<AssociateEncryptionConfigOutput, SdkError<AssociateEncryptionConfigError>>> + Send;
    fn associate_identity_provider_config(&self, builder: AssociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<AssociateIdentityProviderConfigOutput, SdkError<AssociateIdentityProviderConfigError>>> + Send;
    fn create_access_entry(&self, builder: CreateAccessEntryInputBuilder) -> impl Future<Output = Result<CreateAccessEntryOutput, SdkError<CreateAccessEntryError>>> + Send;
    fn create_addon(&self, builder: CreateAddonInputBuilder) -> impl Future<Output = Result<CreateAddonOutput, SdkError<CreateAddonError>>> + Send;
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> + Send;
    fn create_eks_anywhere_subscription(&self, builder: CreateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEksAnywhereSubscriptionOutput, SdkError<CreateEksAnywhereSubscriptionError>>> + Send;
    fn create_fargate_profile(&self, builder: CreateFargateProfileInputBuilder) -> impl Future<Output = Result<CreateFargateProfileOutput, SdkError<CreateFargateProfileError>>> + Send;
    fn create_nodegroup(&self, builder: CreateNodegroupInputBuilder) -> impl Future<Output = Result<CreateNodegroupOutput, SdkError<CreateNodegroupError>>> + Send;
    fn create_pod_identity_association(&self, builder: CreatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<CreatePodIdentityAssociationOutput, SdkError<CreatePodIdentityAssociationError>>> + Send;
    fn delete_access_entry(&self, builder: DeleteAccessEntryInputBuilder) -> impl Future<Output = Result<DeleteAccessEntryOutput, SdkError<DeleteAccessEntryError>>> + Send;
    fn delete_addon(&self, builder: DeleteAddonInputBuilder) -> impl Future<Output = Result<DeleteAddonOutput, SdkError<DeleteAddonError>>> + Send;
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> + Send;
    fn delete_eks_anywhere_subscription(&self, builder: DeleteEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEksAnywhereSubscriptionOutput, SdkError<DeleteEksAnywhereSubscriptionError>>> + Send;
    fn delete_fargate_profile(&self, builder: DeleteFargateProfileInputBuilder) -> impl Future<Output = Result<DeleteFargateProfileOutput, SdkError<DeleteFargateProfileError>>> + Send;
    fn delete_nodegroup(&self, builder: DeleteNodegroupInputBuilder) -> impl Future<Output = Result<DeleteNodegroupOutput, SdkError<DeleteNodegroupError>>> + Send;
    fn delete_pod_identity_association(&self, builder: DeletePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DeletePodIdentityAssociationOutput, SdkError<DeletePodIdentityAssociationError>>> + Send;
    fn deregister_cluster(&self, builder: DeregisterClusterInputBuilder) -> impl Future<Output = Result<DeregisterClusterOutput, SdkError<DeregisterClusterError>>> + Send;
    fn describe_access_entry(&self, builder: DescribeAccessEntryInputBuilder) -> impl Future<Output = Result<DescribeAccessEntryOutput, SdkError<DescribeAccessEntryError>>> + Send;
    fn describe_addon(&self, builder: DescribeAddonInputBuilder) -> impl Future<Output = Result<DescribeAddonOutput, SdkError<DescribeAddonError>>> + Send;
    fn describe_addon_configuration(&self, builder: DescribeAddonConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAddonConfigurationOutput, SdkError<DescribeAddonConfigurationError>>> + Send;
    fn describe_addon_versions(&self, builder: DescribeAddonVersionsInputBuilder) -> impl Future<Output = Result<DescribeAddonVersionsOutput, SdkError<DescribeAddonVersionsError>>> + Send;
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> + Send;
    fn describe_eks_anywhere_subscription(&self, builder: DescribeEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeEksAnywhereSubscriptionOutput, SdkError<DescribeEksAnywhereSubscriptionError>>> + Send;
    fn describe_fargate_profile(&self, builder: DescribeFargateProfileInputBuilder) -> impl Future<Output = Result<DescribeFargateProfileOutput, SdkError<DescribeFargateProfileError>>> + Send;
    fn describe_identity_provider_config(&self, builder: DescribeIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderConfigOutput, SdkError<DescribeIdentityProviderConfigError>>> + Send;
    fn describe_insight(&self, builder: DescribeInsightInputBuilder) -> impl Future<Output = Result<DescribeInsightOutput, SdkError<DescribeInsightError>>> + Send;
    fn describe_nodegroup(&self, builder: DescribeNodegroupInputBuilder) -> impl Future<Output = Result<DescribeNodegroupOutput, SdkError<DescribeNodegroupError>>> + Send;
    fn describe_pod_identity_association(&self, builder: DescribePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DescribePodIdentityAssociationOutput, SdkError<DescribePodIdentityAssociationError>>> + Send;
    fn describe_update(&self, builder: DescribeUpdateInputBuilder) -> impl Future<Output = Result<DescribeUpdateOutput, SdkError<DescribeUpdateError>>> + Send;
    fn disassociate_access_policy(&self, builder: DisassociateAccessPolicyInputBuilder) -> impl Future<Output = Result<DisassociateAccessPolicyOutput, SdkError<DisassociateAccessPolicyError>>> + Send;
    fn disassociate_identity_provider_config(&self, builder: DisassociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DisassociateIdentityProviderConfigOutput, SdkError<DisassociateIdentityProviderConfigError>>> + Send;
    fn list_access_entries(&self, builder: ListAccessEntriesInputBuilder) -> impl Future<Output = Result<ListAccessEntriesOutput, SdkError<ListAccessEntriesError>>> + Send;
    fn list_access_policies(&self, builder: ListAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAccessPoliciesOutput, SdkError<ListAccessPoliciesError>>> + Send;
    fn list_addons(&self, builder: ListAddonsInputBuilder) -> impl Future<Output = Result<ListAddonsOutput, SdkError<ListAddonsError>>> + Send;
    fn list_associated_access_policies(&self, builder: ListAssociatedAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAssociatedAccessPoliciesOutput, SdkError<ListAssociatedAccessPoliciesError>>> + Send;
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> + Send;
    fn list_eks_anywhere_subscriptions(&self, builder: ListEksAnywhereSubscriptionsInputBuilder) -> impl Future<Output = Result<ListEksAnywhereSubscriptionsOutput, SdkError<ListEksAnywhereSubscriptionsError>>> + Send;
    fn list_fargate_profiles(&self, builder: ListFargateProfilesInputBuilder) -> impl Future<Output = Result<ListFargateProfilesOutput, SdkError<ListFargateProfilesError>>> + Send;
    fn list_identity_provider_configs(&self, builder: ListIdentityProviderConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityProviderConfigsOutput, SdkError<ListIdentityProviderConfigsError>>> + Send;
    fn list_insights(&self, builder: ListInsightsInputBuilder) -> impl Future<Output = Result<ListInsightsOutput, SdkError<ListInsightsError>>> + Send;
    fn list_nodegroups(&self, builder: ListNodegroupsInputBuilder) -> impl Future<Output = Result<ListNodegroupsOutput, SdkError<ListNodegroupsError>>> + Send;
    fn list_pod_identity_associations(&self, builder: ListPodIdentityAssociationsInputBuilder) -> impl Future<Output = Result<ListPodIdentityAssociationsOutput, SdkError<ListPodIdentityAssociationsError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_updates(&self, builder: ListUpdatesInputBuilder) -> impl Future<Output = Result<ListUpdatesOutput, SdkError<ListUpdatesError>>> + Send;
    fn register_cluster(&self, builder: RegisterClusterInputBuilder) -> impl Future<Output = Result<RegisterClusterOutput, SdkError<RegisterClusterError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_access_entry(&self, builder: UpdateAccessEntryInputBuilder) -> impl Future<Output = Result<UpdateAccessEntryOutput, SdkError<UpdateAccessEntryError>>> + Send;
    fn update_addon(&self, builder: UpdateAddonInputBuilder) -> impl Future<Output = Result<UpdateAddonOutput, SdkError<UpdateAddonError>>> + Send;
    fn update_cluster_config(&self, builder: UpdateClusterConfigInputBuilder) -> impl Future<Output = Result<UpdateClusterConfigOutput, SdkError<UpdateClusterConfigError>>> + Send;
    fn update_cluster_version(&self, builder: UpdateClusterVersionInputBuilder) -> impl Future<Output = Result<UpdateClusterVersionOutput, SdkError<UpdateClusterVersionError>>> + Send;
    fn update_eks_anywhere_subscription(&self, builder: UpdateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<UpdateEksAnywhereSubscriptionOutput, SdkError<UpdateEksAnywhereSubscriptionError>>> + Send;
    fn update_nodegroup_config(&self, builder: UpdateNodegroupConfigInputBuilder) -> impl Future<Output = Result<UpdateNodegroupConfigOutput, SdkError<UpdateNodegroupConfigError>>> + Send;
    fn update_nodegroup_version(&self, builder: UpdateNodegroupVersionInputBuilder) -> impl Future<Output = Result<UpdateNodegroupVersionOutput, SdkError<UpdateNodegroupVersionError>>> + Send;
    fn update_pod_identity_association(&self, builder: UpdatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<UpdatePodIdentityAssociationOutput, SdkError<UpdatePodIdentityAssociationError>>> + Send;
}
impl EKSClient for EKSClientImpl {
    fn associate_access_policy(&self, builder: AssociateAccessPolicyInputBuilder) -> impl Future<Output = Result<AssociateAccessPolicyOutput, SdkError<AssociateAccessPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn associate_encryption_config(&self, builder: AssociateEncryptionConfigInputBuilder) -> impl Future<Output = Result<AssociateEncryptionConfigOutput, SdkError<AssociateEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn associate_identity_provider_config(&self, builder: AssociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<AssociateIdentityProviderConfigOutput, SdkError<AssociateIdentityProviderConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_access_entry(&self, builder: CreateAccessEntryInputBuilder) -> impl Future<Output = Result<CreateAccessEntryOutput, SdkError<CreateAccessEntryError>>> {
        builder.send_with(&self.0)
    }
    fn create_addon(&self, builder: CreateAddonInputBuilder) -> impl Future<Output = Result<CreateAddonOutput, SdkError<CreateAddonError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_eks_anywhere_subscription(&self, builder: CreateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEksAnywhereSubscriptionOutput, SdkError<CreateEksAnywhereSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_fargate_profile(&self, builder: CreateFargateProfileInputBuilder) -> impl Future<Output = Result<CreateFargateProfileOutput, SdkError<CreateFargateProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_nodegroup(&self, builder: CreateNodegroupInputBuilder) -> impl Future<Output = Result<CreateNodegroupOutput, SdkError<CreateNodegroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_pod_identity_association(&self, builder: CreatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<CreatePodIdentityAssociationOutput, SdkError<CreatePodIdentityAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_access_entry(&self, builder: DeleteAccessEntryInputBuilder) -> impl Future<Output = Result<DeleteAccessEntryOutput, SdkError<DeleteAccessEntryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_addon(&self, builder: DeleteAddonInputBuilder) -> impl Future<Output = Result<DeleteAddonOutput, SdkError<DeleteAddonError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_eks_anywhere_subscription(&self, builder: DeleteEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEksAnywhereSubscriptionOutput, SdkError<DeleteEksAnywhereSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_fargate_profile(&self, builder: DeleteFargateProfileInputBuilder) -> impl Future<Output = Result<DeleteFargateProfileOutput, SdkError<DeleteFargateProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_nodegroup(&self, builder: DeleteNodegroupInputBuilder) -> impl Future<Output = Result<DeleteNodegroupOutput, SdkError<DeleteNodegroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_pod_identity_association(&self, builder: DeletePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DeletePodIdentityAssociationOutput, SdkError<DeletePodIdentityAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_cluster(&self, builder: DeregisterClusterInputBuilder) -> impl Future<Output = Result<DeregisterClusterOutput, SdkError<DeregisterClusterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_access_entry(&self, builder: DescribeAccessEntryInputBuilder) -> impl Future<Output = Result<DescribeAccessEntryOutput, SdkError<DescribeAccessEntryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_addon(&self, builder: DescribeAddonInputBuilder) -> impl Future<Output = Result<DescribeAddonOutput, SdkError<DescribeAddonError>>> {
        builder.send_with(&self.0)
    }
    fn describe_addon_configuration(&self, builder: DescribeAddonConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAddonConfigurationOutput, SdkError<DescribeAddonConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_addon_versions(&self, builder: DescribeAddonVersionsInputBuilder) -> impl Future<Output = Result<DescribeAddonVersionsOutput, SdkError<DescribeAddonVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_eks_anywhere_subscription(&self, builder: DescribeEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeEksAnywhereSubscriptionOutput, SdkError<DescribeEksAnywhereSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fargate_profile(&self, builder: DescribeFargateProfileInputBuilder) -> impl Future<Output = Result<DescribeFargateProfileOutput, SdkError<DescribeFargateProfileError>>> {
        builder.send_with(&self.0)
    }
    fn describe_identity_provider_config(&self, builder: DescribeIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderConfigOutput, SdkError<DescribeIdentityProviderConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_insight(&self, builder: DescribeInsightInputBuilder) -> impl Future<Output = Result<DescribeInsightOutput, SdkError<DescribeInsightError>>> {
        builder.send_with(&self.0)
    }
    fn describe_nodegroup(&self, builder: DescribeNodegroupInputBuilder) -> impl Future<Output = Result<DescribeNodegroupOutput, SdkError<DescribeNodegroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pod_identity_association(&self, builder: DescribePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DescribePodIdentityAssociationOutput, SdkError<DescribePodIdentityAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_update(&self, builder: DescribeUpdateInputBuilder) -> impl Future<Output = Result<DescribeUpdateOutput, SdkError<DescribeUpdateError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_access_policy(&self, builder: DisassociateAccessPolicyInputBuilder) -> impl Future<Output = Result<DisassociateAccessPolicyOutput, SdkError<DisassociateAccessPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_identity_provider_config(&self, builder: DisassociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DisassociateIdentityProviderConfigOutput, SdkError<DisassociateIdentityProviderConfigError>>> {
        builder.send_with(&self.0)
    }
    fn list_access_entries(&self, builder: ListAccessEntriesInputBuilder) -> impl Future<Output = Result<ListAccessEntriesOutput, SdkError<ListAccessEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_access_policies(&self, builder: ListAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAccessPoliciesOutput, SdkError<ListAccessPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_addons(&self, builder: ListAddonsInputBuilder) -> impl Future<Output = Result<ListAddonsOutput, SdkError<ListAddonsError>>> {
        builder.send_with(&self.0)
    }
    fn list_associated_access_policies(&self, builder: ListAssociatedAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAssociatedAccessPoliciesOutput, SdkError<ListAssociatedAccessPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        builder.send_with(&self.0)
    }
    fn list_eks_anywhere_subscriptions(&self, builder: ListEksAnywhereSubscriptionsInputBuilder) -> impl Future<Output = Result<ListEksAnywhereSubscriptionsOutput, SdkError<ListEksAnywhereSubscriptionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_fargate_profiles(&self, builder: ListFargateProfilesInputBuilder) -> impl Future<Output = Result<ListFargateProfilesOutput, SdkError<ListFargateProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_identity_provider_configs(&self, builder: ListIdentityProviderConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityProviderConfigsOutput, SdkError<ListIdentityProviderConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_insights(&self, builder: ListInsightsInputBuilder) -> impl Future<Output = Result<ListInsightsOutput, SdkError<ListInsightsError>>> {
        builder.send_with(&self.0)
    }
    fn list_nodegroups(&self, builder: ListNodegroupsInputBuilder) -> impl Future<Output = Result<ListNodegroupsOutput, SdkError<ListNodegroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_pod_identity_associations(&self, builder: ListPodIdentityAssociationsInputBuilder) -> impl Future<Output = Result<ListPodIdentityAssociationsOutput, SdkError<ListPodIdentityAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_updates(&self, builder: ListUpdatesInputBuilder) -> impl Future<Output = Result<ListUpdatesOutput, SdkError<ListUpdatesError>>> {
        builder.send_with(&self.0)
    }
    fn register_cluster(&self, builder: RegisterClusterInputBuilder) -> impl Future<Output = Result<RegisterClusterOutput, SdkError<RegisterClusterError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_access_entry(&self, builder: UpdateAccessEntryInputBuilder) -> impl Future<Output = Result<UpdateAccessEntryOutput, SdkError<UpdateAccessEntryError>>> {
        builder.send_with(&self.0)
    }
    fn update_addon(&self, builder: UpdateAddonInputBuilder) -> impl Future<Output = Result<UpdateAddonOutput, SdkError<UpdateAddonError>>> {
        builder.send_with(&self.0)
    }
    fn update_cluster_config(&self, builder: UpdateClusterConfigInputBuilder) -> impl Future<Output = Result<UpdateClusterConfigOutput, SdkError<UpdateClusterConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_cluster_version(&self, builder: UpdateClusterVersionInputBuilder) -> impl Future<Output = Result<UpdateClusterVersionOutput, SdkError<UpdateClusterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_eks_anywhere_subscription(&self, builder: UpdateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<UpdateEksAnywhereSubscriptionOutput, SdkError<UpdateEksAnywhereSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_nodegroup_config(&self, builder: UpdateNodegroupConfigInputBuilder) -> impl Future<Output = Result<UpdateNodegroupConfigOutput, SdkError<UpdateNodegroupConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_nodegroup_version(&self, builder: UpdateNodegroupVersionInputBuilder) -> impl Future<Output = Result<UpdateNodegroupVersionOutput, SdkError<UpdateNodegroupVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_pod_identity_association(&self, builder: UpdatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<UpdatePodIdentityAssociationOutput, SdkError<UpdatePodIdentityAssociationError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> EKSClient for T
where T: Deref,
      T::Target: EKSClient {
    fn associate_access_policy(&self, builder: AssociateAccessPolicyInputBuilder) -> impl Future<Output = Result<AssociateAccessPolicyOutput, SdkError<AssociateAccessPolicyError>>> {
        self.deref().associate_access_policy(builder)
    }
    fn associate_encryption_config(&self, builder: AssociateEncryptionConfigInputBuilder) -> impl Future<Output = Result<AssociateEncryptionConfigOutput, SdkError<AssociateEncryptionConfigError>>> {
        self.deref().associate_encryption_config(builder)
    }
    fn associate_identity_provider_config(&self, builder: AssociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<AssociateIdentityProviderConfigOutput, SdkError<AssociateIdentityProviderConfigError>>> {
        self.deref().associate_identity_provider_config(builder)
    }
    fn create_access_entry(&self, builder: CreateAccessEntryInputBuilder) -> impl Future<Output = Result<CreateAccessEntryOutput, SdkError<CreateAccessEntryError>>> {
        self.deref().create_access_entry(builder)
    }
    fn create_addon(&self, builder: CreateAddonInputBuilder) -> impl Future<Output = Result<CreateAddonOutput, SdkError<CreateAddonError>>> {
        self.deref().create_addon(builder)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        self.deref().create_cluster(builder)
    }
    fn create_eks_anywhere_subscription(&self, builder: CreateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEksAnywhereSubscriptionOutput, SdkError<CreateEksAnywhereSubscriptionError>>> {
        self.deref().create_eks_anywhere_subscription(builder)
    }
    fn create_fargate_profile(&self, builder: CreateFargateProfileInputBuilder) -> impl Future<Output = Result<CreateFargateProfileOutput, SdkError<CreateFargateProfileError>>> {
        self.deref().create_fargate_profile(builder)
    }
    fn create_nodegroup(&self, builder: CreateNodegroupInputBuilder) -> impl Future<Output = Result<CreateNodegroupOutput, SdkError<CreateNodegroupError>>> {
        self.deref().create_nodegroup(builder)
    }
    fn create_pod_identity_association(&self, builder: CreatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<CreatePodIdentityAssociationOutput, SdkError<CreatePodIdentityAssociationError>>> {
        self.deref().create_pod_identity_association(builder)
    }
    fn delete_access_entry(&self, builder: DeleteAccessEntryInputBuilder) -> impl Future<Output = Result<DeleteAccessEntryOutput, SdkError<DeleteAccessEntryError>>> {
        self.deref().delete_access_entry(builder)
    }
    fn delete_addon(&self, builder: DeleteAddonInputBuilder) -> impl Future<Output = Result<DeleteAddonOutput, SdkError<DeleteAddonError>>> {
        self.deref().delete_addon(builder)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        self.deref().delete_cluster(builder)
    }
    fn delete_eks_anywhere_subscription(&self, builder: DeleteEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEksAnywhereSubscriptionOutput, SdkError<DeleteEksAnywhereSubscriptionError>>> {
        self.deref().delete_eks_anywhere_subscription(builder)
    }
    fn delete_fargate_profile(&self, builder: DeleteFargateProfileInputBuilder) -> impl Future<Output = Result<DeleteFargateProfileOutput, SdkError<DeleteFargateProfileError>>> {
        self.deref().delete_fargate_profile(builder)
    }
    fn delete_nodegroup(&self, builder: DeleteNodegroupInputBuilder) -> impl Future<Output = Result<DeleteNodegroupOutput, SdkError<DeleteNodegroupError>>> {
        self.deref().delete_nodegroup(builder)
    }
    fn delete_pod_identity_association(&self, builder: DeletePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DeletePodIdentityAssociationOutput, SdkError<DeletePodIdentityAssociationError>>> {
        self.deref().delete_pod_identity_association(builder)
    }
    fn deregister_cluster(&self, builder: DeregisterClusterInputBuilder) -> impl Future<Output = Result<DeregisterClusterOutput, SdkError<DeregisterClusterError>>> {
        self.deref().deregister_cluster(builder)
    }
    fn describe_access_entry(&self, builder: DescribeAccessEntryInputBuilder) -> impl Future<Output = Result<DescribeAccessEntryOutput, SdkError<DescribeAccessEntryError>>> {
        self.deref().describe_access_entry(builder)
    }
    fn describe_addon(&self, builder: DescribeAddonInputBuilder) -> impl Future<Output = Result<DescribeAddonOutput, SdkError<DescribeAddonError>>> {
        self.deref().describe_addon(builder)
    }
    fn describe_addon_configuration(&self, builder: DescribeAddonConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAddonConfigurationOutput, SdkError<DescribeAddonConfigurationError>>> {
        self.deref().describe_addon_configuration(builder)
    }
    fn describe_addon_versions(&self, builder: DescribeAddonVersionsInputBuilder) -> impl Future<Output = Result<DescribeAddonVersionsOutput, SdkError<DescribeAddonVersionsError>>> {
        self.deref().describe_addon_versions(builder)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        self.deref().describe_cluster(builder)
    }
    fn describe_eks_anywhere_subscription(&self, builder: DescribeEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeEksAnywhereSubscriptionOutput, SdkError<DescribeEksAnywhereSubscriptionError>>> {
        self.deref().describe_eks_anywhere_subscription(builder)
    }
    fn describe_fargate_profile(&self, builder: DescribeFargateProfileInputBuilder) -> impl Future<Output = Result<DescribeFargateProfileOutput, SdkError<DescribeFargateProfileError>>> {
        self.deref().describe_fargate_profile(builder)
    }
    fn describe_identity_provider_config(&self, builder: DescribeIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderConfigOutput, SdkError<DescribeIdentityProviderConfigError>>> {
        self.deref().describe_identity_provider_config(builder)
    }
    fn describe_insight(&self, builder: DescribeInsightInputBuilder) -> impl Future<Output = Result<DescribeInsightOutput, SdkError<DescribeInsightError>>> {
        self.deref().describe_insight(builder)
    }
    fn describe_nodegroup(&self, builder: DescribeNodegroupInputBuilder) -> impl Future<Output = Result<DescribeNodegroupOutput, SdkError<DescribeNodegroupError>>> {
        self.deref().describe_nodegroup(builder)
    }
    fn describe_pod_identity_association(&self, builder: DescribePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<DescribePodIdentityAssociationOutput, SdkError<DescribePodIdentityAssociationError>>> {
        self.deref().describe_pod_identity_association(builder)
    }
    fn describe_update(&self, builder: DescribeUpdateInputBuilder) -> impl Future<Output = Result<DescribeUpdateOutput, SdkError<DescribeUpdateError>>> {
        self.deref().describe_update(builder)
    }
    fn disassociate_access_policy(&self, builder: DisassociateAccessPolicyInputBuilder) -> impl Future<Output = Result<DisassociateAccessPolicyOutput, SdkError<DisassociateAccessPolicyError>>> {
        self.deref().disassociate_access_policy(builder)
    }
    fn disassociate_identity_provider_config(&self, builder: DisassociateIdentityProviderConfigInputBuilder) -> impl Future<Output = Result<DisassociateIdentityProviderConfigOutput, SdkError<DisassociateIdentityProviderConfigError>>> {
        self.deref().disassociate_identity_provider_config(builder)
    }
    fn list_access_entries(&self, builder: ListAccessEntriesInputBuilder) -> impl Future<Output = Result<ListAccessEntriesOutput, SdkError<ListAccessEntriesError>>> {
        self.deref().list_access_entries(builder)
    }
    fn list_access_policies(&self, builder: ListAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAccessPoliciesOutput, SdkError<ListAccessPoliciesError>>> {
        self.deref().list_access_policies(builder)
    }
    fn list_addons(&self, builder: ListAddonsInputBuilder) -> impl Future<Output = Result<ListAddonsOutput, SdkError<ListAddonsError>>> {
        self.deref().list_addons(builder)
    }
    fn list_associated_access_policies(&self, builder: ListAssociatedAccessPoliciesInputBuilder) -> impl Future<Output = Result<ListAssociatedAccessPoliciesOutput, SdkError<ListAssociatedAccessPoliciesError>>> {
        self.deref().list_associated_access_policies(builder)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        self.deref().list_clusters(builder)
    }
    fn list_eks_anywhere_subscriptions(&self, builder: ListEksAnywhereSubscriptionsInputBuilder) -> impl Future<Output = Result<ListEksAnywhereSubscriptionsOutput, SdkError<ListEksAnywhereSubscriptionsError>>> {
        self.deref().list_eks_anywhere_subscriptions(builder)
    }
    fn list_fargate_profiles(&self, builder: ListFargateProfilesInputBuilder) -> impl Future<Output = Result<ListFargateProfilesOutput, SdkError<ListFargateProfilesError>>> {
        self.deref().list_fargate_profiles(builder)
    }
    fn list_identity_provider_configs(&self, builder: ListIdentityProviderConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityProviderConfigsOutput, SdkError<ListIdentityProviderConfigsError>>> {
        self.deref().list_identity_provider_configs(builder)
    }
    fn list_insights(&self, builder: ListInsightsInputBuilder) -> impl Future<Output = Result<ListInsightsOutput, SdkError<ListInsightsError>>> {
        self.deref().list_insights(builder)
    }
    fn list_nodegroups(&self, builder: ListNodegroupsInputBuilder) -> impl Future<Output = Result<ListNodegroupsOutput, SdkError<ListNodegroupsError>>> {
        self.deref().list_nodegroups(builder)
    }
    fn list_pod_identity_associations(&self, builder: ListPodIdentityAssociationsInputBuilder) -> impl Future<Output = Result<ListPodIdentityAssociationsOutput, SdkError<ListPodIdentityAssociationsError>>> {
        self.deref().list_pod_identity_associations(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_updates(&self, builder: ListUpdatesInputBuilder) -> impl Future<Output = Result<ListUpdatesOutput, SdkError<ListUpdatesError>>> {
        self.deref().list_updates(builder)
    }
    fn register_cluster(&self, builder: RegisterClusterInputBuilder) -> impl Future<Output = Result<RegisterClusterOutput, SdkError<RegisterClusterError>>> {
        self.deref().register_cluster(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_access_entry(&self, builder: UpdateAccessEntryInputBuilder) -> impl Future<Output = Result<UpdateAccessEntryOutput, SdkError<UpdateAccessEntryError>>> {
        self.deref().update_access_entry(builder)
    }
    fn update_addon(&self, builder: UpdateAddonInputBuilder) -> impl Future<Output = Result<UpdateAddonOutput, SdkError<UpdateAddonError>>> {
        self.deref().update_addon(builder)
    }
    fn update_cluster_config(&self, builder: UpdateClusterConfigInputBuilder) -> impl Future<Output = Result<UpdateClusterConfigOutput, SdkError<UpdateClusterConfigError>>> {
        self.deref().update_cluster_config(builder)
    }
    fn update_cluster_version(&self, builder: UpdateClusterVersionInputBuilder) -> impl Future<Output = Result<UpdateClusterVersionOutput, SdkError<UpdateClusterVersionError>>> {
        self.deref().update_cluster_version(builder)
    }
    fn update_eks_anywhere_subscription(&self, builder: UpdateEksAnywhereSubscriptionInputBuilder) -> impl Future<Output = Result<UpdateEksAnywhereSubscriptionOutput, SdkError<UpdateEksAnywhereSubscriptionError>>> {
        self.deref().update_eks_anywhere_subscription(builder)
    }
    fn update_nodegroup_config(&self, builder: UpdateNodegroupConfigInputBuilder) -> impl Future<Output = Result<UpdateNodegroupConfigOutput, SdkError<UpdateNodegroupConfigError>>> {
        self.deref().update_nodegroup_config(builder)
    }
    fn update_nodegroup_version(&self, builder: UpdateNodegroupVersionInputBuilder) -> impl Future<Output = Result<UpdateNodegroupVersionOutput, SdkError<UpdateNodegroupVersionError>>> {
        self.deref().update_nodegroup_version(builder)
    }
    fn update_pod_identity_association(&self, builder: UpdatePodIdentityAssociationInputBuilder) -> impl Future<Output = Result<UpdatePodIdentityAssociationOutput, SdkError<UpdatePodIdentityAssociationError>>> {
        self.deref().update_pod_identity_association(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edEKSClient {}
    impl EKSClient for edEKSClient {
        async fn associate_access_policy(&self, builder: AssociateAccessPolicyInputBuilder) -> Result<AssociateAccessPolicyOutput, SdkError<AssociateAccessPolicyError>>;
        async fn associate_encryption_config(&self, builder: AssociateEncryptionConfigInputBuilder) -> Result<AssociateEncryptionConfigOutput, SdkError<AssociateEncryptionConfigError>>;
        async fn associate_identity_provider_config(&self, builder: AssociateIdentityProviderConfigInputBuilder) -> Result<AssociateIdentityProviderConfigOutput, SdkError<AssociateIdentityProviderConfigError>>;
        async fn create_access_entry(&self, builder: CreateAccessEntryInputBuilder) -> Result<CreateAccessEntryOutput, SdkError<CreateAccessEntryError>>;
        async fn create_addon(&self, builder: CreateAddonInputBuilder) -> Result<CreateAddonOutput, SdkError<CreateAddonError>>;
        async fn create_cluster(&self, builder: CreateClusterInputBuilder) -> Result<CreateClusterOutput, SdkError<CreateClusterError>>;
        async fn create_eks_anywhere_subscription(&self, builder: CreateEksAnywhereSubscriptionInputBuilder) -> Result<CreateEksAnywhereSubscriptionOutput, SdkError<CreateEksAnywhereSubscriptionError>>;
        async fn create_fargate_profile(&self, builder: CreateFargateProfileInputBuilder) -> Result<CreateFargateProfileOutput, SdkError<CreateFargateProfileError>>;
        async fn create_nodegroup(&self, builder: CreateNodegroupInputBuilder) -> Result<CreateNodegroupOutput, SdkError<CreateNodegroupError>>;
        async fn create_pod_identity_association(&self, builder: CreatePodIdentityAssociationInputBuilder) -> Result<CreatePodIdentityAssociationOutput, SdkError<CreatePodIdentityAssociationError>>;
        async fn delete_access_entry(&self, builder: DeleteAccessEntryInputBuilder) -> Result<DeleteAccessEntryOutput, SdkError<DeleteAccessEntryError>>;
        async fn delete_addon(&self, builder: DeleteAddonInputBuilder) -> Result<DeleteAddonOutput, SdkError<DeleteAddonError>>;
        async fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> Result<DeleteClusterOutput, SdkError<DeleteClusterError>>;
        async fn delete_eks_anywhere_subscription(&self, builder: DeleteEksAnywhereSubscriptionInputBuilder) -> Result<DeleteEksAnywhereSubscriptionOutput, SdkError<DeleteEksAnywhereSubscriptionError>>;
        async fn delete_fargate_profile(&self, builder: DeleteFargateProfileInputBuilder) -> Result<DeleteFargateProfileOutput, SdkError<DeleteFargateProfileError>>;
        async fn delete_nodegroup(&self, builder: DeleteNodegroupInputBuilder) -> Result<DeleteNodegroupOutput, SdkError<DeleteNodegroupError>>;
        async fn delete_pod_identity_association(&self, builder: DeletePodIdentityAssociationInputBuilder) -> Result<DeletePodIdentityAssociationOutput, SdkError<DeletePodIdentityAssociationError>>;
        async fn deregister_cluster(&self, builder: DeregisterClusterInputBuilder) -> Result<DeregisterClusterOutput, SdkError<DeregisterClusterError>>;
        async fn describe_access_entry(&self, builder: DescribeAccessEntryInputBuilder) -> Result<DescribeAccessEntryOutput, SdkError<DescribeAccessEntryError>>;
        async fn describe_addon(&self, builder: DescribeAddonInputBuilder) -> Result<DescribeAddonOutput, SdkError<DescribeAddonError>>;
        async fn describe_addon_configuration(&self, builder: DescribeAddonConfigurationInputBuilder) -> Result<DescribeAddonConfigurationOutput, SdkError<DescribeAddonConfigurationError>>;
        async fn describe_addon_versions(&self, builder: DescribeAddonVersionsInputBuilder) -> Result<DescribeAddonVersionsOutput, SdkError<DescribeAddonVersionsError>>;
        async fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> Result<DescribeClusterOutput, SdkError<DescribeClusterError>>;
        async fn describe_eks_anywhere_subscription(&self, builder: DescribeEksAnywhereSubscriptionInputBuilder) -> Result<DescribeEksAnywhereSubscriptionOutput, SdkError<DescribeEksAnywhereSubscriptionError>>;
        async fn describe_fargate_profile(&self, builder: DescribeFargateProfileInputBuilder) -> Result<DescribeFargateProfileOutput, SdkError<DescribeFargateProfileError>>;
        async fn describe_identity_provider_config(&self, builder: DescribeIdentityProviderConfigInputBuilder) -> Result<DescribeIdentityProviderConfigOutput, SdkError<DescribeIdentityProviderConfigError>>;
        async fn describe_insight(&self, builder: DescribeInsightInputBuilder) -> Result<DescribeInsightOutput, SdkError<DescribeInsightError>>;
        async fn describe_nodegroup(&self, builder: DescribeNodegroupInputBuilder) -> Result<DescribeNodegroupOutput, SdkError<DescribeNodegroupError>>;
        async fn describe_pod_identity_association(&self, builder: DescribePodIdentityAssociationInputBuilder) -> Result<DescribePodIdentityAssociationOutput, SdkError<DescribePodIdentityAssociationError>>;
        async fn describe_update(&self, builder: DescribeUpdateInputBuilder) -> Result<DescribeUpdateOutput, SdkError<DescribeUpdateError>>;
        async fn disassociate_access_policy(&self, builder: DisassociateAccessPolicyInputBuilder) -> Result<DisassociateAccessPolicyOutput, SdkError<DisassociateAccessPolicyError>>;
        async fn disassociate_identity_provider_config(&self, builder: DisassociateIdentityProviderConfigInputBuilder) -> Result<DisassociateIdentityProviderConfigOutput, SdkError<DisassociateIdentityProviderConfigError>>;
        async fn list_access_entries(&self, builder: ListAccessEntriesInputBuilder) -> Result<ListAccessEntriesOutput, SdkError<ListAccessEntriesError>>;
        async fn list_access_policies(&self, builder: ListAccessPoliciesInputBuilder) -> Result<ListAccessPoliciesOutput, SdkError<ListAccessPoliciesError>>;
        async fn list_addons(&self, builder: ListAddonsInputBuilder) -> Result<ListAddonsOutput, SdkError<ListAddonsError>>;
        async fn list_associated_access_policies(&self, builder: ListAssociatedAccessPoliciesInputBuilder) -> Result<ListAssociatedAccessPoliciesOutput, SdkError<ListAssociatedAccessPoliciesError>>;
        async fn list_clusters(&self, builder: ListClustersInputBuilder) -> Result<ListClustersOutput, SdkError<ListClustersError>>;
        async fn list_eks_anywhere_subscriptions(&self, builder: ListEksAnywhereSubscriptionsInputBuilder) -> Result<ListEksAnywhereSubscriptionsOutput, SdkError<ListEksAnywhereSubscriptionsError>>;
        async fn list_fargate_profiles(&self, builder: ListFargateProfilesInputBuilder) -> Result<ListFargateProfilesOutput, SdkError<ListFargateProfilesError>>;
        async fn list_identity_provider_configs(&self, builder: ListIdentityProviderConfigsInputBuilder) -> Result<ListIdentityProviderConfigsOutput, SdkError<ListIdentityProviderConfigsError>>;
        async fn list_insights(&self, builder: ListInsightsInputBuilder) -> Result<ListInsightsOutput, SdkError<ListInsightsError>>;
        async fn list_nodegroups(&self, builder: ListNodegroupsInputBuilder) -> Result<ListNodegroupsOutput, SdkError<ListNodegroupsError>>;
        async fn list_pod_identity_associations(&self, builder: ListPodIdentityAssociationsInputBuilder) -> Result<ListPodIdentityAssociationsOutput, SdkError<ListPodIdentityAssociationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_updates(&self, builder: ListUpdatesInputBuilder) -> Result<ListUpdatesOutput, SdkError<ListUpdatesError>>;
        async fn register_cluster(&self, builder: RegisterClusterInputBuilder) -> Result<RegisterClusterOutput, SdkError<RegisterClusterError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_access_entry(&self, builder: UpdateAccessEntryInputBuilder) -> Result<UpdateAccessEntryOutput, SdkError<UpdateAccessEntryError>>;
        async fn update_addon(&self, builder: UpdateAddonInputBuilder) -> Result<UpdateAddonOutput, SdkError<UpdateAddonError>>;
        async fn update_cluster_config(&self, builder: UpdateClusterConfigInputBuilder) -> Result<UpdateClusterConfigOutput, SdkError<UpdateClusterConfigError>>;
        async fn update_cluster_version(&self, builder: UpdateClusterVersionInputBuilder) -> Result<UpdateClusterVersionOutput, SdkError<UpdateClusterVersionError>>;
        async fn update_eks_anywhere_subscription(&self, builder: UpdateEksAnywhereSubscriptionInputBuilder) -> Result<UpdateEksAnywhereSubscriptionOutput, SdkError<UpdateEksAnywhereSubscriptionError>>;
        async fn update_nodegroup_config(&self, builder: UpdateNodegroupConfigInputBuilder) -> Result<UpdateNodegroupConfigOutput, SdkError<UpdateNodegroupConfigError>>;
        async fn update_nodegroup_version(&self, builder: UpdateNodegroupVersionInputBuilder) -> Result<UpdateNodegroupVersionOutput, SdkError<UpdateNodegroupVersionError>>;
        async fn update_pod_identity_association(&self, builder: UpdatePodIdentityAssociationInputBuilder) -> Result<UpdatePodIdentityAssociationOutput, SdkError<UpdatePodIdentityAssociationError>>;
    }
}
