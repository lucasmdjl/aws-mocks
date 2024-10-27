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
use aws_sdk_macie2::operation::accept_invitation::{builders::*, *};
use aws_sdk_macie2::operation::batch_get_custom_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::batch_update_automated_discovery_accounts::{builders::*, *};
use aws_sdk_macie2::operation::create_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::create_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::create_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::create_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::create_invitations::{builders::*, *};
use aws_sdk_macie2::operation::create_member::{builders::*, *};
use aws_sdk_macie2::operation::create_sample_findings::{builders::*, *};
use aws_sdk_macie2::operation::decline_invitations::{builders::*, *};
use aws_sdk_macie2::operation::delete_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::delete_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::delete_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::delete_invitations::{builders::*, *};
use aws_sdk_macie2::operation::delete_member::{builders::*, *};
use aws_sdk_macie2::operation::describe_buckets::{builders::*, *};
use aws_sdk_macie2::operation::describe_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::describe_organization_configuration::{builders::*, *};
use aws_sdk_macie2::operation::disable_macie::{builders::*, *};
use aws_sdk_macie2::operation::disable_organization_admin_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_from_administrator_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_from_master_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_member::{builders::*, *};
use aws_sdk_macie2::operation::enable_macie::{builders::*, *};
use aws_sdk_macie2::operation::enable_organization_admin_account::{builders::*, *};
use aws_sdk_macie2::operation::get_administrator_account::{builders::*, *};
use aws_sdk_macie2::operation::get_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::get_automated_discovery_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_bucket_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_classification_export_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_classification_scope::{builders::*, *};
use aws_sdk_macie2::operation::get_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::get_finding_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_findings::{builders::*, *};
use aws_sdk_macie2::operation::get_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::get_findings_publication_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_invitations_count::{builders::*, *};
use aws_sdk_macie2::operation::get_macie_session::{builders::*, *};
use aws_sdk_macie2::operation::get_master_account::{builders::*, *};
use aws_sdk_macie2::operation::get_member::{builders::*, *};
use aws_sdk_macie2::operation::get_resource_profile::{builders::*, *};
use aws_sdk_macie2::operation::get_reveal_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitive_data_occurrences::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitive_data_occurrences_availability::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitivity_inspection_template::{builders::*, *};
use aws_sdk_macie2::operation::get_usage_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_usage_totals::{builders::*, *};
use aws_sdk_macie2::operation::list_allow_lists::{builders::*, *};
use aws_sdk_macie2::operation::list_automated_discovery_accounts::{builders::*, *};
use aws_sdk_macie2::operation::list_classification_jobs::{builders::*, *};
use aws_sdk_macie2::operation::list_classification_scopes::{builders::*, *};
use aws_sdk_macie2::operation::list_custom_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::list_findings::{builders::*, *};
use aws_sdk_macie2::operation::list_findings_filters::{builders::*, *};
use aws_sdk_macie2::operation::list_invitations::{builders::*, *};
use aws_sdk_macie2::operation::list_managed_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::list_members::{builders::*, *};
use aws_sdk_macie2::operation::list_organization_admin_accounts::{builders::*, *};
use aws_sdk_macie2::operation::list_resource_profile_artifacts::{builders::*, *};
use aws_sdk_macie2::operation::list_resource_profile_detections::{builders::*, *};
use aws_sdk_macie2::operation::list_sensitivity_inspection_templates::{builders::*, *};
use aws_sdk_macie2::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_macie2::operation::put_classification_export_configuration::{builders::*, *};
use aws_sdk_macie2::operation::put_findings_publication_configuration::{builders::*, *};
use aws_sdk_macie2::operation::search_resources::{builders::*, *};
use aws_sdk_macie2::operation::tag_resource::{builders::*, *};
use aws_sdk_macie2::operation::test_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::untag_resource::{builders::*, *};
use aws_sdk_macie2::operation::update_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::update_automated_discovery_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::update_classification_scope::{builders::*, *};
use aws_sdk_macie2::operation::update_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::update_macie_session::{builders::*, *};
use aws_sdk_macie2::operation::update_member_session::{builders::*, *};
use aws_sdk_macie2::operation::update_organization_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_resource_profile::{builders::*, *};
use aws_sdk_macie2::operation::update_resource_profile_detections::{builders::*, *};
use aws_sdk_macie2::operation::update_reveal_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_sensitivity_inspection_template::{builders::*, *};
use aws_sdk_macie2::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_macie2::Client;
use std::ops::Deref;

pub use aws_sdk_macie2::*;

pub struct Macie2ClientImpl(Client);
impl Macie2ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait Macie2Client {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>> + Send;
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>> + Send;
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>> + Send;
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>> + Send;
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>> + Send;
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>> + Send;
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>> + Send;
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>> + Send;
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>> + Send;
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>> + Send;
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>> + Send;
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>> + Send;
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>> + Send;
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>> + Send;
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>> + Send;
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>> + Send;
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>> + Send;
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>> + Send;
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>> + Send;
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>> + Send;
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>> + Send;
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>> + Send;
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>> + Send;
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>> + Send;
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>> + Send;
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>> + Send;
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>> + Send;
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>> + Send;
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>> + Send;
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>> + Send;
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>> + Send;
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>> + Send;
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>> + Send;
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>> + Send;
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>> + Send;
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>> + Send;
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>> + Send;
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>> + Send;
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>> + Send;
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>> + Send;
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>> + Send;
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>> + Send;
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>> + Send;
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>> + Send;
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>> + Send;
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>> + Send;
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>> + Send;
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>> + Send;
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>> + Send;
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>> + Send;
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>> + Send;
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>> + Send;
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>> + Send;
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> + Send;
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>> + Send;
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>> + Send;
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>> + Send;
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>> + Send;
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>> + Send;
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>> + Send;
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>> + Send;
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>> + Send;
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>> + Send;
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>> + Send;
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>> + Send;
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>> + Send;
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>> + Send;
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>> + Send;
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>> + Send;
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>> + Send;
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>> + Send;
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>> + Send;
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>> + Send;
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>> + Send;
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>> + Send;
}
impl Macie2Client for Macie2ClientImpl {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>> {
        builder.send_with(&self.0)
    }
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>> {
        builder.send_with(&self.0)
    }
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>> {
        builder.send_with(&self.0)
    }
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>> {
        builder.send_with(&self.0)
    }
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>> {
        builder.send_with(&self.0)
    }
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>> {
        builder.send_with(&self.0)
    }
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>> {
        builder.send_with(&self.0)
    }
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>> {
        builder.send_with(&self.0)
    }
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>> {
        builder.send_with(&self.0)
    }
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>> {
        builder.send_with(&self.0)
    }
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>> {
        builder.send_with(&self.0)
    }
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>> {
        builder.send_with(&self.0)
    }
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>> {
        builder.send_with(&self.0)
    }
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>> {
        builder.send_with(&self.0)
    }
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>> {
        builder.send_with(&self.0)
    }
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> Macie2Client for T
where T: Deref,
      T::Target: Macie2Client {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>> {
        self.deref().accept_invitation(builder)
    }
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>> {
        self.deref().batch_get_custom_data_identifiers(builder)
    }
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>> {
        self.deref().batch_update_automated_discovery_accounts(builder)
    }
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>> {
        self.deref().create_allow_list(builder)
    }
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>> {
        self.deref().create_classification_job(builder)
    }
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>> {
        self.deref().create_custom_data_identifier(builder)
    }
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>> {
        self.deref().create_findings_filter(builder)
    }
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>> {
        self.deref().create_invitations(builder)
    }
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>> {
        self.deref().create_member(builder)
    }
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>> {
        self.deref().create_sample_findings(builder)
    }
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>> {
        self.deref().decline_invitations(builder)
    }
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>> {
        self.deref().delete_allow_list(builder)
    }
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>> {
        self.deref().delete_custom_data_identifier(builder)
    }
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>> {
        self.deref().delete_findings_filter(builder)
    }
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>> {
        self.deref().delete_invitations(builder)
    }
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>> {
        self.deref().delete_member(builder)
    }
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>> {
        self.deref().describe_buckets(builder)
    }
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>> {
        self.deref().describe_classification_job(builder)
    }
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>> {
        self.deref().describe_organization_configuration(builder)
    }
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>> {
        self.deref().disable_macie(builder)
    }
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>> {
        self.deref().disable_organization_admin_account(builder)
    }
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>> {
        self.deref().disassociate_from_administrator_account(builder)
    }
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>> {
        self.deref().disassociate_from_master_account(builder)
    }
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>> {
        self.deref().disassociate_member(builder)
    }
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>> {
        self.deref().enable_macie(builder)
    }
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>> {
        self.deref().enable_organization_admin_account(builder)
    }
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>> {
        self.deref().get_administrator_account(builder)
    }
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>> {
        self.deref().get_allow_list(builder)
    }
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>> {
        self.deref().get_automated_discovery_configuration(builder)
    }
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>> {
        self.deref().get_bucket_statistics(builder)
    }
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>> {
        self.deref().get_classification_export_configuration(builder)
    }
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>> {
        self.deref().get_classification_scope(builder)
    }
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>> {
        self.deref().get_custom_data_identifier(builder)
    }
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>> {
        self.deref().get_finding_statistics(builder)
    }
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>> {
        self.deref().get_findings(builder)
    }
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>> {
        self.deref().get_findings_filter(builder)
    }
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>> {
        self.deref().get_findings_publication_configuration(builder)
    }
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>> {
        self.deref().get_invitations_count(builder)
    }
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>> {
        self.deref().get_macie_session(builder)
    }
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>> {
        self.deref().get_master_account(builder)
    }
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>> {
        self.deref().get_member(builder)
    }
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>> {
        self.deref().get_resource_profile(builder)
    }
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>> {
        self.deref().get_reveal_configuration(builder)
    }
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>> {
        self.deref().get_sensitive_data_occurrences(builder)
    }
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>> {
        self.deref().get_sensitive_data_occurrences_availability(builder)
    }
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>> {
        self.deref().get_sensitivity_inspection_template(builder)
    }
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>> {
        self.deref().get_usage_statistics(builder)
    }
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>> {
        self.deref().get_usage_totals(builder)
    }
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>> {
        self.deref().list_allow_lists(builder)
    }
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>> {
        self.deref().list_automated_discovery_accounts(builder)
    }
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>> {
        self.deref().list_classification_jobs(builder)
    }
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>> {
        self.deref().list_classification_scopes(builder)
    }
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>> {
        self.deref().list_custom_data_identifiers(builder)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        self.deref().list_findings(builder)
    }
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>> {
        self.deref().list_findings_filters(builder)
    }
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>> {
        self.deref().list_invitations(builder)
    }
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>> {
        self.deref().list_managed_data_identifiers(builder)
    }
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>> {
        self.deref().list_members(builder)
    }
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>> {
        self.deref().list_organization_admin_accounts(builder)
    }
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>> {
        self.deref().list_resource_profile_artifacts(builder)
    }
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>> {
        self.deref().list_resource_profile_detections(builder)
    }
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>> {
        self.deref().list_sensitivity_inspection_templates(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>> {
        self.deref().put_classification_export_configuration(builder)
    }
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>> {
        self.deref().put_findings_publication_configuration(builder)
    }
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>> {
        self.deref().search_resources(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>> {
        self.deref().test_custom_data_identifier(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>> {
        self.deref().update_allow_list(builder)
    }
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>> {
        self.deref().update_automated_discovery_configuration(builder)
    }
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>> {
        self.deref().update_classification_job(builder)
    }
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>> {
        self.deref().update_classification_scope(builder)
    }
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>> {
        self.deref().update_findings_filter(builder)
    }
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>> {
        self.deref().update_macie_session(builder)
    }
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>> {
        self.deref().update_member_session(builder)
    }
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>> {
        self.deref().update_organization_configuration(builder)
    }
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>> {
        self.deref().update_resource_profile(builder)
    }
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>> {
        self.deref().update_resource_profile_detections(builder)
    }
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>> {
        self.deref().update_reveal_configuration(builder)
    }
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>> {
        self.deref().update_sensitivity_inspection_template(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edMacie2Client {}
    impl Macie2Client for edMacie2Client {
        async fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>;
        async fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>;
        async fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>;
        async fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> Result<CreateAllowListOutput, SdkError<CreateAllowListError>>;
        async fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>;
        async fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>;
        async fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>;
        async fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>;
        async fn create_member(&self, builder: CreateMemberInputBuilder) -> Result<CreateMemberOutput, SdkError<CreateMemberError>>;
        async fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>;
        async fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>;
        async fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>;
        async fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>;
        async fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>;
        async fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>;
        async fn delete_member(&self, builder: DeleteMemberInputBuilder) -> Result<DeleteMemberOutput, SdkError<DeleteMemberError>>;
        async fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>;
        async fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>;
        async fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>;
        async fn disable_macie(&self, builder: DisableMacieInputBuilder) -> Result<DisableMacieOutput, SdkError<DisableMacieError>>;
        async fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>;
        async fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>;
        async fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>;
        async fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>;
        async fn enable_macie(&self, builder: EnableMacieInputBuilder) -> Result<EnableMacieOutput, SdkError<EnableMacieError>>;
        async fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>;
        async fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>;
        async fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> Result<GetAllowListOutput, SdkError<GetAllowListError>>;
        async fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>;
        async fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>;
        async fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>;
        async fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>;
        async fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>;
        async fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>;
        async fn get_findings(&self, builder: GetFindingsInputBuilder) -> Result<GetFindingsOutput, SdkError<GetFindingsError>>;
        async fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>;
        async fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>;
        async fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>;
        async fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>;
        async fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>;
        async fn get_member(&self, builder: GetMemberInputBuilder) -> Result<GetMemberOutput, SdkError<GetMemberError>>;
        async fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>;
        async fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>;
        async fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>;
        async fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>;
        async fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>;
        async fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>;
        async fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>;
        async fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> Result<ListAllowListsOutput, SdkError<ListAllowListsError>>;
        async fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>;
        async fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>;
        async fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>;
        async fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>;
        async fn list_findings(&self, builder: ListFindingsInputBuilder) -> Result<ListFindingsOutput, SdkError<ListFindingsError>>;
        async fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>;
        async fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> Result<ListInvitationsOutput, SdkError<ListInvitationsError>>;
        async fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>;
        async fn list_members(&self, builder: ListMembersInputBuilder) -> Result<ListMembersOutput, SdkError<ListMembersError>>;
        async fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>;
        async fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>;
        async fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>;
        async fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>;
        async fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>;
        async fn search_resources(&self, builder: SearchResourcesInputBuilder) -> Result<SearchResourcesOutput, SdkError<SearchResourcesError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>;
        async fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>;
        async fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>;
        async fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>;
        async fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>;
        async fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>;
        async fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>;
        async fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>;
        async fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>;
        async fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>;
        async fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>;
        async fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>;
    }
}
