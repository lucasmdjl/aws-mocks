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
use aws_sdk_accessanalyzer::operation::apply_archive_rule::{builders::*, *};
use aws_sdk_accessanalyzer::operation::cancel_policy_generation::{builders::*, *};
use aws_sdk_accessanalyzer::operation::check_access_not_granted::{builders::*, *};
use aws_sdk_accessanalyzer::operation::check_no_new_access::{builders::*, *};
use aws_sdk_accessanalyzer::operation::check_no_public_access::{builders::*, *};
use aws_sdk_accessanalyzer::operation::create_access_preview::{builders::*, *};
use aws_sdk_accessanalyzer::operation::create_analyzer::{builders::*, *};
use aws_sdk_accessanalyzer::operation::create_archive_rule::{builders::*, *};
use aws_sdk_accessanalyzer::operation::delete_analyzer::{builders::*, *};
use aws_sdk_accessanalyzer::operation::delete_archive_rule::{builders::*, *};
use aws_sdk_accessanalyzer::operation::generate_finding_recommendation::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_access_preview::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_analyzed_resource::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_analyzer::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_archive_rule::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_finding::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_finding_recommendation::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_finding_v2::{builders::*, *};
use aws_sdk_accessanalyzer::operation::get_generated_policy::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_access_preview_findings::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_access_previews::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_analyzed_resources::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_analyzers::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_archive_rules::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_findings::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_findings_v2::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_policy_generations::{builders::*, *};
use aws_sdk_accessanalyzer::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_accessanalyzer::operation::start_policy_generation::{builders::*, *};
use aws_sdk_accessanalyzer::operation::start_resource_scan::{builders::*, *};
use aws_sdk_accessanalyzer::operation::tag_resource::{builders::*, *};
use aws_sdk_accessanalyzer::operation::untag_resource::{builders::*, *};
use aws_sdk_accessanalyzer::operation::update_archive_rule::{builders::*, *};
use aws_sdk_accessanalyzer::operation::update_findings::{builders::*, *};
use aws_sdk_accessanalyzer::operation::validate_policy::{builders::*, *};
use aws_sdk_accessanalyzer::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_accessanalyzer::Client;
use std::ops::Deref;

pub use aws_sdk_accessanalyzer::*;

pub struct AccessAnalyzerClientImpl(Client);
impl AccessAnalyzerClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AccessAnalyzerClient {
    fn apply_archive_rule(&self, builder: ApplyArchiveRuleInputBuilder) -> impl Future<Output = Result<ApplyArchiveRuleOutput, SdkError<ApplyArchiveRuleError>>> + Send;
    fn cancel_policy_generation(&self, builder: CancelPolicyGenerationInputBuilder) -> impl Future<Output = Result<CancelPolicyGenerationOutput, SdkError<CancelPolicyGenerationError>>> + Send;
    fn check_access_not_granted(&self, builder: CheckAccessNotGrantedInputBuilder) -> impl Future<Output = Result<CheckAccessNotGrantedOutput, SdkError<CheckAccessNotGrantedError>>> + Send;
    fn check_no_new_access(&self, builder: CheckNoNewAccessInputBuilder) -> impl Future<Output = Result<CheckNoNewAccessOutput, SdkError<CheckNoNewAccessError>>> + Send;
    fn check_no_public_access(&self, builder: CheckNoPublicAccessInputBuilder) -> impl Future<Output = Result<CheckNoPublicAccessOutput, SdkError<CheckNoPublicAccessError>>> + Send;
    fn create_access_preview(&self, builder: CreateAccessPreviewInputBuilder) -> impl Future<Output = Result<CreateAccessPreviewOutput, SdkError<CreateAccessPreviewError>>> + Send;
    fn create_analyzer(&self, builder: CreateAnalyzerInputBuilder) -> impl Future<Output = Result<CreateAnalyzerOutput, SdkError<CreateAnalyzerError>>> + Send;
    fn create_archive_rule(&self, builder: CreateArchiveRuleInputBuilder) -> impl Future<Output = Result<CreateArchiveRuleOutput, SdkError<CreateArchiveRuleError>>> + Send;
    fn delete_analyzer(&self, builder: DeleteAnalyzerInputBuilder) -> impl Future<Output = Result<DeleteAnalyzerOutput, SdkError<DeleteAnalyzerError>>> + Send;
    fn delete_archive_rule(&self, builder: DeleteArchiveRuleInputBuilder) -> impl Future<Output = Result<DeleteArchiveRuleOutput, SdkError<DeleteArchiveRuleError>>> + Send;
    fn generate_finding_recommendation(&self, builder: GenerateFindingRecommendationInputBuilder) -> impl Future<Output = Result<GenerateFindingRecommendationOutput, SdkError<GenerateFindingRecommendationError>>> + Send;
    fn get_access_preview(&self, builder: GetAccessPreviewInputBuilder) -> impl Future<Output = Result<GetAccessPreviewOutput, SdkError<GetAccessPreviewError>>> + Send;
    fn get_analyzed_resource(&self, builder: GetAnalyzedResourceInputBuilder) -> impl Future<Output = Result<GetAnalyzedResourceOutput, SdkError<GetAnalyzedResourceError>>> + Send;
    fn get_analyzer(&self, builder: GetAnalyzerInputBuilder) -> impl Future<Output = Result<GetAnalyzerOutput, SdkError<GetAnalyzerError>>> + Send;
    fn get_archive_rule(&self, builder: GetArchiveRuleInputBuilder) -> impl Future<Output = Result<GetArchiveRuleOutput, SdkError<GetArchiveRuleError>>> + Send;
    fn get_finding(&self, builder: GetFindingInputBuilder) -> impl Future<Output = Result<GetFindingOutput, SdkError<GetFindingError>>> + Send;
    fn get_finding_recommendation(&self, builder: GetFindingRecommendationInputBuilder) -> impl Future<Output = Result<GetFindingRecommendationOutput, SdkError<GetFindingRecommendationError>>> + Send;
    fn get_finding_v2(&self, builder: GetFindingV2InputBuilder) -> impl Future<Output = Result<GetFindingV2Output, SdkError<GetFindingV2Error>>> + Send;
    fn get_generated_policy(&self, builder: GetGeneratedPolicyInputBuilder) -> impl Future<Output = Result<GetGeneratedPolicyOutput, SdkError<GetGeneratedPolicyError>>> + Send;
    fn list_access_preview_findings(&self, builder: ListAccessPreviewFindingsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewFindingsOutput, SdkError<ListAccessPreviewFindingsError>>> + Send;
    fn list_access_previews(&self, builder: ListAccessPreviewsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewsOutput, SdkError<ListAccessPreviewsError>>> + Send;
    fn list_analyzed_resources(&self, builder: ListAnalyzedResourcesInputBuilder) -> impl Future<Output = Result<ListAnalyzedResourcesOutput, SdkError<ListAnalyzedResourcesError>>> + Send;
    fn list_analyzers(&self, builder: ListAnalyzersInputBuilder) -> impl Future<Output = Result<ListAnalyzersOutput, SdkError<ListAnalyzersError>>> + Send;
    fn list_archive_rules(&self, builder: ListArchiveRulesInputBuilder) -> impl Future<Output = Result<ListArchiveRulesOutput, SdkError<ListArchiveRulesError>>> + Send;
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> + Send;
    fn list_findings_v2(&self, builder: ListFindingsV2InputBuilder) -> impl Future<Output = Result<ListFindingsV2Output, SdkError<ListFindingsV2Error>>> + Send;
    fn list_policy_generations(&self, builder: ListPolicyGenerationsInputBuilder) -> impl Future<Output = Result<ListPolicyGenerationsOutput, SdkError<ListPolicyGenerationsError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn start_policy_generation(&self, builder: StartPolicyGenerationInputBuilder) -> impl Future<Output = Result<StartPolicyGenerationOutput, SdkError<StartPolicyGenerationError>>> + Send;
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_archive_rule(&self, builder: UpdateArchiveRuleInputBuilder) -> impl Future<Output = Result<UpdateArchiveRuleOutput, SdkError<UpdateArchiveRuleError>>> + Send;
    fn update_findings(&self, builder: UpdateFindingsInputBuilder) -> impl Future<Output = Result<UpdateFindingsOutput, SdkError<UpdateFindingsError>>> + Send;
    fn validate_policy(&self, builder: ValidatePolicyInputBuilder) -> impl Future<Output = Result<ValidatePolicyOutput, SdkError<ValidatePolicyError>>> + Send;
}
impl AccessAnalyzerClient for AccessAnalyzerClientImpl {
    fn apply_archive_rule(&self, builder: ApplyArchiveRuleInputBuilder) -> impl Future<Output = Result<ApplyArchiveRuleOutput, SdkError<ApplyArchiveRuleError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_policy_generation(&self, builder: CancelPolicyGenerationInputBuilder) -> impl Future<Output = Result<CancelPolicyGenerationOutput, SdkError<CancelPolicyGenerationError>>> {
        builder.send_with(&self.0)
    }
    fn check_access_not_granted(&self, builder: CheckAccessNotGrantedInputBuilder) -> impl Future<Output = Result<CheckAccessNotGrantedOutput, SdkError<CheckAccessNotGrantedError>>> {
        builder.send_with(&self.0)
    }
    fn check_no_new_access(&self, builder: CheckNoNewAccessInputBuilder) -> impl Future<Output = Result<CheckNoNewAccessOutput, SdkError<CheckNoNewAccessError>>> {
        builder.send_with(&self.0)
    }
    fn check_no_public_access(&self, builder: CheckNoPublicAccessInputBuilder) -> impl Future<Output = Result<CheckNoPublicAccessOutput, SdkError<CheckNoPublicAccessError>>> {
        builder.send_with(&self.0)
    }
    fn create_access_preview(&self, builder: CreateAccessPreviewInputBuilder) -> impl Future<Output = Result<CreateAccessPreviewOutput, SdkError<CreateAccessPreviewError>>> {
        builder.send_with(&self.0)
    }
    fn create_analyzer(&self, builder: CreateAnalyzerInputBuilder) -> impl Future<Output = Result<CreateAnalyzerOutput, SdkError<CreateAnalyzerError>>> {
        builder.send_with(&self.0)
    }
    fn create_archive_rule(&self, builder: CreateArchiveRuleInputBuilder) -> impl Future<Output = Result<CreateArchiveRuleOutput, SdkError<CreateArchiveRuleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_analyzer(&self, builder: DeleteAnalyzerInputBuilder) -> impl Future<Output = Result<DeleteAnalyzerOutput, SdkError<DeleteAnalyzerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_archive_rule(&self, builder: DeleteArchiveRuleInputBuilder) -> impl Future<Output = Result<DeleteArchiveRuleOutput, SdkError<DeleteArchiveRuleError>>> {
        builder.send_with(&self.0)
    }
    fn generate_finding_recommendation(&self, builder: GenerateFindingRecommendationInputBuilder) -> impl Future<Output = Result<GenerateFindingRecommendationOutput, SdkError<GenerateFindingRecommendationError>>> {
        builder.send_with(&self.0)
    }
    fn get_access_preview(&self, builder: GetAccessPreviewInputBuilder) -> impl Future<Output = Result<GetAccessPreviewOutput, SdkError<GetAccessPreviewError>>> {
        builder.send_with(&self.0)
    }
    fn get_analyzed_resource(&self, builder: GetAnalyzedResourceInputBuilder) -> impl Future<Output = Result<GetAnalyzedResourceOutput, SdkError<GetAnalyzedResourceError>>> {
        builder.send_with(&self.0)
    }
    fn get_analyzer(&self, builder: GetAnalyzerInputBuilder) -> impl Future<Output = Result<GetAnalyzerOutput, SdkError<GetAnalyzerError>>> {
        builder.send_with(&self.0)
    }
    fn get_archive_rule(&self, builder: GetArchiveRuleInputBuilder) -> impl Future<Output = Result<GetArchiveRuleOutput, SdkError<GetArchiveRuleError>>> {
        builder.send_with(&self.0)
    }
    fn get_finding(&self, builder: GetFindingInputBuilder) -> impl Future<Output = Result<GetFindingOutput, SdkError<GetFindingError>>> {
        builder.send_with(&self.0)
    }
    fn get_finding_recommendation(&self, builder: GetFindingRecommendationInputBuilder) -> impl Future<Output = Result<GetFindingRecommendationOutput, SdkError<GetFindingRecommendationError>>> {
        builder.send_with(&self.0)
    }
    fn get_finding_v2(&self, builder: GetFindingV2InputBuilder) -> impl Future<Output = Result<GetFindingV2Output, SdkError<GetFindingV2Error>>> {
        builder.send_with(&self.0)
    }
    fn get_generated_policy(&self, builder: GetGeneratedPolicyInputBuilder) -> impl Future<Output = Result<GetGeneratedPolicyOutput, SdkError<GetGeneratedPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn list_access_preview_findings(&self, builder: ListAccessPreviewFindingsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewFindingsOutput, SdkError<ListAccessPreviewFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_access_previews(&self, builder: ListAccessPreviewsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewsOutput, SdkError<ListAccessPreviewsError>>> {
        builder.send_with(&self.0)
    }
    fn list_analyzed_resources(&self, builder: ListAnalyzedResourcesInputBuilder) -> impl Future<Output = Result<ListAnalyzedResourcesOutput, SdkError<ListAnalyzedResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_analyzers(&self, builder: ListAnalyzersInputBuilder) -> impl Future<Output = Result<ListAnalyzersOutput, SdkError<ListAnalyzersError>>> {
        builder.send_with(&self.0)
    }
    fn list_archive_rules(&self, builder: ListArchiveRulesInputBuilder) -> impl Future<Output = Result<ListArchiveRulesOutput, SdkError<ListArchiveRulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings_v2(&self, builder: ListFindingsV2InputBuilder) -> impl Future<Output = Result<ListFindingsV2Output, SdkError<ListFindingsV2Error>>> {
        builder.send_with(&self.0)
    }
    fn list_policy_generations(&self, builder: ListPolicyGenerationsInputBuilder) -> impl Future<Output = Result<ListPolicyGenerationsOutput, SdkError<ListPolicyGenerationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_policy_generation(&self, builder: StartPolicyGenerationInputBuilder) -> impl Future<Output = Result<StartPolicyGenerationOutput, SdkError<StartPolicyGenerationError>>> {
        builder.send_with(&self.0)
    }
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_archive_rule(&self, builder: UpdateArchiveRuleInputBuilder) -> impl Future<Output = Result<UpdateArchiveRuleOutput, SdkError<UpdateArchiveRuleError>>> {
        builder.send_with(&self.0)
    }
    fn update_findings(&self, builder: UpdateFindingsInputBuilder) -> impl Future<Output = Result<UpdateFindingsOutput, SdkError<UpdateFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn validate_policy(&self, builder: ValidatePolicyInputBuilder) -> impl Future<Output = Result<ValidatePolicyOutput, SdkError<ValidatePolicyError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AccessAnalyzerClient for T
where T: Deref,
      T::Target: AccessAnalyzerClient {
    fn apply_archive_rule(&self, builder: ApplyArchiveRuleInputBuilder) -> impl Future<Output = Result<ApplyArchiveRuleOutput, SdkError<ApplyArchiveRuleError>>> {
        self.deref().apply_archive_rule(builder)
    }
    fn cancel_policy_generation(&self, builder: CancelPolicyGenerationInputBuilder) -> impl Future<Output = Result<CancelPolicyGenerationOutput, SdkError<CancelPolicyGenerationError>>> {
        self.deref().cancel_policy_generation(builder)
    }
    fn check_access_not_granted(&self, builder: CheckAccessNotGrantedInputBuilder) -> impl Future<Output = Result<CheckAccessNotGrantedOutput, SdkError<CheckAccessNotGrantedError>>> {
        self.deref().check_access_not_granted(builder)
    }
    fn check_no_new_access(&self, builder: CheckNoNewAccessInputBuilder) -> impl Future<Output = Result<CheckNoNewAccessOutput, SdkError<CheckNoNewAccessError>>> {
        self.deref().check_no_new_access(builder)
    }
    fn check_no_public_access(&self, builder: CheckNoPublicAccessInputBuilder) -> impl Future<Output = Result<CheckNoPublicAccessOutput, SdkError<CheckNoPublicAccessError>>> {
        self.deref().check_no_public_access(builder)
    }
    fn create_access_preview(&self, builder: CreateAccessPreviewInputBuilder) -> impl Future<Output = Result<CreateAccessPreviewOutput, SdkError<CreateAccessPreviewError>>> {
        self.deref().create_access_preview(builder)
    }
    fn create_analyzer(&self, builder: CreateAnalyzerInputBuilder) -> impl Future<Output = Result<CreateAnalyzerOutput, SdkError<CreateAnalyzerError>>> {
        self.deref().create_analyzer(builder)
    }
    fn create_archive_rule(&self, builder: CreateArchiveRuleInputBuilder) -> impl Future<Output = Result<CreateArchiveRuleOutput, SdkError<CreateArchiveRuleError>>> {
        self.deref().create_archive_rule(builder)
    }
    fn delete_analyzer(&self, builder: DeleteAnalyzerInputBuilder) -> impl Future<Output = Result<DeleteAnalyzerOutput, SdkError<DeleteAnalyzerError>>> {
        self.deref().delete_analyzer(builder)
    }
    fn delete_archive_rule(&self, builder: DeleteArchiveRuleInputBuilder) -> impl Future<Output = Result<DeleteArchiveRuleOutput, SdkError<DeleteArchiveRuleError>>> {
        self.deref().delete_archive_rule(builder)
    }
    fn generate_finding_recommendation(&self, builder: GenerateFindingRecommendationInputBuilder) -> impl Future<Output = Result<GenerateFindingRecommendationOutput, SdkError<GenerateFindingRecommendationError>>> {
        self.deref().generate_finding_recommendation(builder)
    }
    fn get_access_preview(&self, builder: GetAccessPreviewInputBuilder) -> impl Future<Output = Result<GetAccessPreviewOutput, SdkError<GetAccessPreviewError>>> {
        self.deref().get_access_preview(builder)
    }
    fn get_analyzed_resource(&self, builder: GetAnalyzedResourceInputBuilder) -> impl Future<Output = Result<GetAnalyzedResourceOutput, SdkError<GetAnalyzedResourceError>>> {
        self.deref().get_analyzed_resource(builder)
    }
    fn get_analyzer(&self, builder: GetAnalyzerInputBuilder) -> impl Future<Output = Result<GetAnalyzerOutput, SdkError<GetAnalyzerError>>> {
        self.deref().get_analyzer(builder)
    }
    fn get_archive_rule(&self, builder: GetArchiveRuleInputBuilder) -> impl Future<Output = Result<GetArchiveRuleOutput, SdkError<GetArchiveRuleError>>> {
        self.deref().get_archive_rule(builder)
    }
    fn get_finding(&self, builder: GetFindingInputBuilder) -> impl Future<Output = Result<GetFindingOutput, SdkError<GetFindingError>>> {
        self.deref().get_finding(builder)
    }
    fn get_finding_recommendation(&self, builder: GetFindingRecommendationInputBuilder) -> impl Future<Output = Result<GetFindingRecommendationOutput, SdkError<GetFindingRecommendationError>>> {
        self.deref().get_finding_recommendation(builder)
    }
    fn get_finding_v2(&self, builder: GetFindingV2InputBuilder) -> impl Future<Output = Result<GetFindingV2Output, SdkError<GetFindingV2Error>>> {
        self.deref().get_finding_v2(builder)
    }
    fn get_generated_policy(&self, builder: GetGeneratedPolicyInputBuilder) -> impl Future<Output = Result<GetGeneratedPolicyOutput, SdkError<GetGeneratedPolicyError>>> {
        self.deref().get_generated_policy(builder)
    }
    fn list_access_preview_findings(&self, builder: ListAccessPreviewFindingsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewFindingsOutput, SdkError<ListAccessPreviewFindingsError>>> {
        self.deref().list_access_preview_findings(builder)
    }
    fn list_access_previews(&self, builder: ListAccessPreviewsInputBuilder) -> impl Future<Output = Result<ListAccessPreviewsOutput, SdkError<ListAccessPreviewsError>>> {
        self.deref().list_access_previews(builder)
    }
    fn list_analyzed_resources(&self, builder: ListAnalyzedResourcesInputBuilder) -> impl Future<Output = Result<ListAnalyzedResourcesOutput, SdkError<ListAnalyzedResourcesError>>> {
        self.deref().list_analyzed_resources(builder)
    }
    fn list_analyzers(&self, builder: ListAnalyzersInputBuilder) -> impl Future<Output = Result<ListAnalyzersOutput, SdkError<ListAnalyzersError>>> {
        self.deref().list_analyzers(builder)
    }
    fn list_archive_rules(&self, builder: ListArchiveRulesInputBuilder) -> impl Future<Output = Result<ListArchiveRulesOutput, SdkError<ListArchiveRulesError>>> {
        self.deref().list_archive_rules(builder)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        self.deref().list_findings(builder)
    }
    fn list_findings_v2(&self, builder: ListFindingsV2InputBuilder) -> impl Future<Output = Result<ListFindingsV2Output, SdkError<ListFindingsV2Error>>> {
        self.deref().list_findings_v2(builder)
    }
    fn list_policy_generations(&self, builder: ListPolicyGenerationsInputBuilder) -> impl Future<Output = Result<ListPolicyGenerationsOutput, SdkError<ListPolicyGenerationsError>>> {
        self.deref().list_policy_generations(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn start_policy_generation(&self, builder: StartPolicyGenerationInputBuilder) -> impl Future<Output = Result<StartPolicyGenerationOutput, SdkError<StartPolicyGenerationError>>> {
        self.deref().start_policy_generation(builder)
    }
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>> {
        self.deref().start_resource_scan(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_archive_rule(&self, builder: UpdateArchiveRuleInputBuilder) -> impl Future<Output = Result<UpdateArchiveRuleOutput, SdkError<UpdateArchiveRuleError>>> {
        self.deref().update_archive_rule(builder)
    }
    fn update_findings(&self, builder: UpdateFindingsInputBuilder) -> impl Future<Output = Result<UpdateFindingsOutput, SdkError<UpdateFindingsError>>> {
        self.deref().update_findings(builder)
    }
    fn validate_policy(&self, builder: ValidatePolicyInputBuilder) -> impl Future<Output = Result<ValidatePolicyOutput, SdkError<ValidatePolicyError>>> {
        self.deref().validate_policy(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAccessAnalyzerClient {}
    impl AccessAnalyzerClient for edAccessAnalyzerClient {
        async fn apply_archive_rule(&self, builder: ApplyArchiveRuleInputBuilder) -> Result<ApplyArchiveRuleOutput, SdkError<ApplyArchiveRuleError>>;
        async fn cancel_policy_generation(&self, builder: CancelPolicyGenerationInputBuilder) -> Result<CancelPolicyGenerationOutput, SdkError<CancelPolicyGenerationError>>;
        async fn check_access_not_granted(&self, builder: CheckAccessNotGrantedInputBuilder) -> Result<CheckAccessNotGrantedOutput, SdkError<CheckAccessNotGrantedError>>;
        async fn check_no_new_access(&self, builder: CheckNoNewAccessInputBuilder) -> Result<CheckNoNewAccessOutput, SdkError<CheckNoNewAccessError>>;
        async fn check_no_public_access(&self, builder: CheckNoPublicAccessInputBuilder) -> Result<CheckNoPublicAccessOutput, SdkError<CheckNoPublicAccessError>>;
        async fn create_access_preview(&self, builder: CreateAccessPreviewInputBuilder) -> Result<CreateAccessPreviewOutput, SdkError<CreateAccessPreviewError>>;
        async fn create_analyzer(&self, builder: CreateAnalyzerInputBuilder) -> Result<CreateAnalyzerOutput, SdkError<CreateAnalyzerError>>;
        async fn create_archive_rule(&self, builder: CreateArchiveRuleInputBuilder) -> Result<CreateArchiveRuleOutput, SdkError<CreateArchiveRuleError>>;
        async fn delete_analyzer(&self, builder: DeleteAnalyzerInputBuilder) -> Result<DeleteAnalyzerOutput, SdkError<DeleteAnalyzerError>>;
        async fn delete_archive_rule(&self, builder: DeleteArchiveRuleInputBuilder) -> Result<DeleteArchiveRuleOutput, SdkError<DeleteArchiveRuleError>>;
        async fn generate_finding_recommendation(&self, builder: GenerateFindingRecommendationInputBuilder) -> Result<GenerateFindingRecommendationOutput, SdkError<GenerateFindingRecommendationError>>;
        async fn get_access_preview(&self, builder: GetAccessPreviewInputBuilder) -> Result<GetAccessPreviewOutput, SdkError<GetAccessPreviewError>>;
        async fn get_analyzed_resource(&self, builder: GetAnalyzedResourceInputBuilder) -> Result<GetAnalyzedResourceOutput, SdkError<GetAnalyzedResourceError>>;
        async fn get_analyzer(&self, builder: GetAnalyzerInputBuilder) -> Result<GetAnalyzerOutput, SdkError<GetAnalyzerError>>;
        async fn get_archive_rule(&self, builder: GetArchiveRuleInputBuilder) -> Result<GetArchiveRuleOutput, SdkError<GetArchiveRuleError>>;
        async fn get_finding(&self, builder: GetFindingInputBuilder) -> Result<GetFindingOutput, SdkError<GetFindingError>>;
        async fn get_finding_recommendation(&self, builder: GetFindingRecommendationInputBuilder) -> Result<GetFindingRecommendationOutput, SdkError<GetFindingRecommendationError>>;
        async fn get_finding_v2(&self, builder: GetFindingV2InputBuilder) -> Result<GetFindingV2Output, SdkError<GetFindingV2Error>>;
        async fn get_generated_policy(&self, builder: GetGeneratedPolicyInputBuilder) -> Result<GetGeneratedPolicyOutput, SdkError<GetGeneratedPolicyError>>;
        async fn list_access_preview_findings(&self, builder: ListAccessPreviewFindingsInputBuilder) -> Result<ListAccessPreviewFindingsOutput, SdkError<ListAccessPreviewFindingsError>>;
        async fn list_access_previews(&self, builder: ListAccessPreviewsInputBuilder) -> Result<ListAccessPreviewsOutput, SdkError<ListAccessPreviewsError>>;
        async fn list_analyzed_resources(&self, builder: ListAnalyzedResourcesInputBuilder) -> Result<ListAnalyzedResourcesOutput, SdkError<ListAnalyzedResourcesError>>;
        async fn list_analyzers(&self, builder: ListAnalyzersInputBuilder) -> Result<ListAnalyzersOutput, SdkError<ListAnalyzersError>>;
        async fn list_archive_rules(&self, builder: ListArchiveRulesInputBuilder) -> Result<ListArchiveRulesOutput, SdkError<ListArchiveRulesError>>;
        async fn list_findings(&self, builder: ListFindingsInputBuilder) -> Result<ListFindingsOutput, SdkError<ListFindingsError>>;
        async fn list_findings_v2(&self, builder: ListFindingsV2InputBuilder) -> Result<ListFindingsV2Output, SdkError<ListFindingsV2Error>>;
        async fn list_policy_generations(&self, builder: ListPolicyGenerationsInputBuilder) -> Result<ListPolicyGenerationsOutput, SdkError<ListPolicyGenerationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn start_policy_generation(&self, builder: StartPolicyGenerationInputBuilder) -> Result<StartPolicyGenerationOutput, SdkError<StartPolicyGenerationError>>;
        async fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> Result<StartResourceScanOutput, SdkError<StartResourceScanError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_archive_rule(&self, builder: UpdateArchiveRuleInputBuilder) -> Result<UpdateArchiveRuleOutput, SdkError<UpdateArchiveRuleError>>;
        async fn update_findings(&self, builder: UpdateFindingsInputBuilder) -> Result<UpdateFindingsOutput, SdkError<UpdateFindingsError>>;
        async fn validate_policy(&self, builder: ValidatePolicyInputBuilder) -> Result<ValidatePolicyOutput, SdkError<ValidatePolicyError>>;
    }
}
