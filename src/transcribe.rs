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
use aws_sdk_transcribe::operation::create_call_analytics_category::{builders::*, *};
use aws_sdk_transcribe::operation::create_language_model::{builders::*, *};
use aws_sdk_transcribe::operation::create_medical_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::create_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::create_vocabulary_filter::{builders::*, *};
use aws_sdk_transcribe::operation::delete_call_analytics_category::{builders::*, *};
use aws_sdk_transcribe::operation::delete_call_analytics_job::{builders::*, *};
use aws_sdk_transcribe::operation::delete_language_model::{builders::*, *};
use aws_sdk_transcribe::operation::delete_medical_scribe_job::{builders::*, *};
use aws_sdk_transcribe::operation::delete_medical_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::delete_medical_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::delete_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::delete_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::delete_vocabulary_filter::{builders::*, *};
use aws_sdk_transcribe::operation::describe_language_model::{builders::*, *};
use aws_sdk_transcribe::operation::get_call_analytics_category::{builders::*, *};
use aws_sdk_transcribe::operation::get_call_analytics_job::{builders::*, *};
use aws_sdk_transcribe::operation::get_medical_scribe_job::{builders::*, *};
use aws_sdk_transcribe::operation::get_medical_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::get_medical_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::get_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::get_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::get_vocabulary_filter::{builders::*, *};
use aws_sdk_transcribe::operation::list_call_analytics_categories::{builders::*, *};
use aws_sdk_transcribe::operation::list_call_analytics_jobs::{builders::*, *};
use aws_sdk_transcribe::operation::list_language_models::{builders::*, *};
use aws_sdk_transcribe::operation::list_medical_scribe_jobs::{builders::*, *};
use aws_sdk_transcribe::operation::list_medical_transcription_jobs::{builders::*, *};
use aws_sdk_transcribe::operation::list_medical_vocabularies::{builders::*, *};
use aws_sdk_transcribe::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_transcribe::operation::list_transcription_jobs::{builders::*, *};
use aws_sdk_transcribe::operation::list_vocabularies::{builders::*, *};
use aws_sdk_transcribe::operation::list_vocabulary_filters::{builders::*, *};
use aws_sdk_transcribe::operation::start_call_analytics_job::{builders::*, *};
use aws_sdk_transcribe::operation::start_medical_scribe_job::{builders::*, *};
use aws_sdk_transcribe::operation::start_medical_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::start_transcription_job::{builders::*, *};
use aws_sdk_transcribe::operation::tag_resource::{builders::*, *};
use aws_sdk_transcribe::operation::untag_resource::{builders::*, *};
use aws_sdk_transcribe::operation::update_call_analytics_category::{builders::*, *};
use aws_sdk_transcribe::operation::update_medical_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::update_vocabulary::{builders::*, *};
use aws_sdk_transcribe::operation::update_vocabulary_filter::{builders::*, *};
use aws_sdk_transcribe::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_transcribe::Client;
use std::ops::Deref;

pub use aws_sdk_transcribe::*;

pub struct TranscribeClientImpl(Client);
impl TranscribeClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait TranscribeClient {
    fn create_call_analytics_category(&self, builder: CreateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<CreateCallAnalyticsCategoryOutput, SdkError<CreateCallAnalyticsCategoryError>>> + Send;
    fn create_language_model(&self, builder: CreateLanguageModelInputBuilder) -> impl Future<Output = Result<CreateLanguageModelOutput, SdkError<CreateLanguageModelError>>> + Send;
    fn create_medical_vocabulary(&self, builder: CreateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<CreateMedicalVocabularyOutput, SdkError<CreateMedicalVocabularyError>>> + Send;
    fn create_vocabulary(&self, builder: CreateVocabularyInputBuilder) -> impl Future<Output = Result<CreateVocabularyOutput, SdkError<CreateVocabularyError>>> + Send;
    fn create_vocabulary_filter(&self, builder: CreateVocabularyFilterInputBuilder) -> impl Future<Output = Result<CreateVocabularyFilterOutput, SdkError<CreateVocabularyFilterError>>> + Send;
    fn delete_call_analytics_category(&self, builder: DeleteCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsCategoryOutput, SdkError<DeleteCallAnalyticsCategoryError>>> + Send;
    fn delete_call_analytics_job(&self, builder: DeleteCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsJobOutput, SdkError<DeleteCallAnalyticsJobError>>> + Send;
    fn delete_language_model(&self, builder: DeleteLanguageModelInputBuilder) -> impl Future<Output = Result<DeleteLanguageModelOutput, SdkError<DeleteLanguageModelError>>> + Send;
    fn delete_medical_scribe_job(&self, builder: DeleteMedicalScribeJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalScribeJobOutput, SdkError<DeleteMedicalScribeJobError>>> + Send;
    fn delete_medical_transcription_job(&self, builder: DeleteMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalTranscriptionJobOutput, SdkError<DeleteMedicalTranscriptionJobError>>> + Send;
    fn delete_medical_vocabulary(&self, builder: DeleteMedicalVocabularyInputBuilder) -> impl Future<Output = Result<DeleteMedicalVocabularyOutput, SdkError<DeleteMedicalVocabularyError>>> + Send;
    fn delete_transcription_job(&self, builder: DeleteTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteTranscriptionJobOutput, SdkError<DeleteTranscriptionJobError>>> + Send;
    fn delete_vocabulary(&self, builder: DeleteVocabularyInputBuilder) -> impl Future<Output = Result<DeleteVocabularyOutput, SdkError<DeleteVocabularyError>>> + Send;
    fn delete_vocabulary_filter(&self, builder: DeleteVocabularyFilterInputBuilder) -> impl Future<Output = Result<DeleteVocabularyFilterOutput, SdkError<DeleteVocabularyFilterError>>> + Send;
    fn describe_language_model(&self, builder: DescribeLanguageModelInputBuilder) -> impl Future<Output = Result<DescribeLanguageModelOutput, SdkError<DescribeLanguageModelError>>> + Send;
    fn get_call_analytics_category(&self, builder: GetCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsCategoryOutput, SdkError<GetCallAnalyticsCategoryError>>> + Send;
    fn get_call_analytics_job(&self, builder: GetCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsJobOutput, SdkError<GetCallAnalyticsJobError>>> + Send;
    fn get_medical_scribe_job(&self, builder: GetMedicalScribeJobInputBuilder) -> impl Future<Output = Result<GetMedicalScribeJobOutput, SdkError<GetMedicalScribeJobError>>> + Send;
    fn get_medical_transcription_job(&self, builder: GetMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetMedicalTranscriptionJobOutput, SdkError<GetMedicalTranscriptionJobError>>> + Send;
    fn get_medical_vocabulary(&self, builder: GetMedicalVocabularyInputBuilder) -> impl Future<Output = Result<GetMedicalVocabularyOutput, SdkError<GetMedicalVocabularyError>>> + Send;
    fn get_transcription_job(&self, builder: GetTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetTranscriptionJobOutput, SdkError<GetTranscriptionJobError>>> + Send;
    fn get_vocabulary(&self, builder: GetVocabularyInputBuilder) -> impl Future<Output = Result<GetVocabularyOutput, SdkError<GetVocabularyError>>> + Send;
    fn get_vocabulary_filter(&self, builder: GetVocabularyFilterInputBuilder) -> impl Future<Output = Result<GetVocabularyFilterOutput, SdkError<GetVocabularyFilterError>>> + Send;
    fn list_call_analytics_categories(&self, builder: ListCallAnalyticsCategoriesInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsCategoriesOutput, SdkError<ListCallAnalyticsCategoriesError>>> + Send;
    fn list_call_analytics_jobs(&self, builder: ListCallAnalyticsJobsInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsJobsOutput, SdkError<ListCallAnalyticsJobsError>>> + Send;
    fn list_language_models(&self, builder: ListLanguageModelsInputBuilder) -> impl Future<Output = Result<ListLanguageModelsOutput, SdkError<ListLanguageModelsError>>> + Send;
    fn list_medical_scribe_jobs(&self, builder: ListMedicalScribeJobsInputBuilder) -> impl Future<Output = Result<ListMedicalScribeJobsOutput, SdkError<ListMedicalScribeJobsError>>> + Send;
    fn list_medical_transcription_jobs(&self, builder: ListMedicalTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListMedicalTranscriptionJobsOutput, SdkError<ListMedicalTranscriptionJobsError>>> + Send;
    fn list_medical_vocabularies(&self, builder: ListMedicalVocabulariesInputBuilder) -> impl Future<Output = Result<ListMedicalVocabulariesOutput, SdkError<ListMedicalVocabulariesError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_transcription_jobs(&self, builder: ListTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListTranscriptionJobsOutput, SdkError<ListTranscriptionJobsError>>> + Send;
    fn list_vocabularies(&self, builder: ListVocabulariesInputBuilder) -> impl Future<Output = Result<ListVocabulariesOutput, SdkError<ListVocabulariesError>>> + Send;
    fn list_vocabulary_filters(&self, builder: ListVocabularyFiltersInputBuilder) -> impl Future<Output = Result<ListVocabularyFiltersOutput, SdkError<ListVocabularyFiltersError>>> + Send;
    fn start_call_analytics_job(&self, builder: StartCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<StartCallAnalyticsJobOutput, SdkError<StartCallAnalyticsJobError>>> + Send;
    fn start_medical_scribe_job(&self, builder: StartMedicalScribeJobInputBuilder) -> impl Future<Output = Result<StartMedicalScribeJobOutput, SdkError<StartMedicalScribeJobError>>> + Send;
    fn start_medical_transcription_job(&self, builder: StartMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartMedicalTranscriptionJobOutput, SdkError<StartMedicalTranscriptionJobError>>> + Send;
    fn start_transcription_job(&self, builder: StartTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartTranscriptionJobOutput, SdkError<StartTranscriptionJobError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_call_analytics_category(&self, builder: UpdateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<UpdateCallAnalyticsCategoryOutput, SdkError<UpdateCallAnalyticsCategoryError>>> + Send;
    fn update_medical_vocabulary(&self, builder: UpdateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<UpdateMedicalVocabularyOutput, SdkError<UpdateMedicalVocabularyError>>> + Send;
    fn update_vocabulary(&self, builder: UpdateVocabularyInputBuilder) -> impl Future<Output = Result<UpdateVocabularyOutput, SdkError<UpdateVocabularyError>>> + Send;
    fn update_vocabulary_filter(&self, builder: UpdateVocabularyFilterInputBuilder) -> impl Future<Output = Result<UpdateVocabularyFilterOutput, SdkError<UpdateVocabularyFilterError>>> + Send;
}
impl TranscribeClient for TranscribeClientImpl {
    fn create_call_analytics_category(&self, builder: CreateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<CreateCallAnalyticsCategoryOutput, SdkError<CreateCallAnalyticsCategoryError>>> {
        builder.send_with(&self.0)
    }
    fn create_language_model(&self, builder: CreateLanguageModelInputBuilder) -> impl Future<Output = Result<CreateLanguageModelOutput, SdkError<CreateLanguageModelError>>> {
        builder.send_with(&self.0)
    }
    fn create_medical_vocabulary(&self, builder: CreateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<CreateMedicalVocabularyOutput, SdkError<CreateMedicalVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn create_vocabulary(&self, builder: CreateVocabularyInputBuilder) -> impl Future<Output = Result<CreateVocabularyOutput, SdkError<CreateVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn create_vocabulary_filter(&self, builder: CreateVocabularyFilterInputBuilder) -> impl Future<Output = Result<CreateVocabularyFilterOutput, SdkError<CreateVocabularyFilterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_call_analytics_category(&self, builder: DeleteCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsCategoryOutput, SdkError<DeleteCallAnalyticsCategoryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_call_analytics_job(&self, builder: DeleteCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsJobOutput, SdkError<DeleteCallAnalyticsJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_language_model(&self, builder: DeleteLanguageModelInputBuilder) -> impl Future<Output = Result<DeleteLanguageModelOutput, SdkError<DeleteLanguageModelError>>> {
        builder.send_with(&self.0)
    }
    fn delete_medical_scribe_job(&self, builder: DeleteMedicalScribeJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalScribeJobOutput, SdkError<DeleteMedicalScribeJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_medical_transcription_job(&self, builder: DeleteMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalTranscriptionJobOutput, SdkError<DeleteMedicalTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_medical_vocabulary(&self, builder: DeleteMedicalVocabularyInputBuilder) -> impl Future<Output = Result<DeleteMedicalVocabularyOutput, SdkError<DeleteMedicalVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transcription_job(&self, builder: DeleteTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteTranscriptionJobOutput, SdkError<DeleteTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vocabulary(&self, builder: DeleteVocabularyInputBuilder) -> impl Future<Output = Result<DeleteVocabularyOutput, SdkError<DeleteVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vocabulary_filter(&self, builder: DeleteVocabularyFilterInputBuilder) -> impl Future<Output = Result<DeleteVocabularyFilterOutput, SdkError<DeleteVocabularyFilterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_language_model(&self, builder: DescribeLanguageModelInputBuilder) -> impl Future<Output = Result<DescribeLanguageModelOutput, SdkError<DescribeLanguageModelError>>> {
        builder.send_with(&self.0)
    }
    fn get_call_analytics_category(&self, builder: GetCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsCategoryOutput, SdkError<GetCallAnalyticsCategoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_call_analytics_job(&self, builder: GetCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsJobOutput, SdkError<GetCallAnalyticsJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_medical_scribe_job(&self, builder: GetMedicalScribeJobInputBuilder) -> impl Future<Output = Result<GetMedicalScribeJobOutput, SdkError<GetMedicalScribeJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_medical_transcription_job(&self, builder: GetMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetMedicalTranscriptionJobOutput, SdkError<GetMedicalTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_medical_vocabulary(&self, builder: GetMedicalVocabularyInputBuilder) -> impl Future<Output = Result<GetMedicalVocabularyOutput, SdkError<GetMedicalVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn get_transcription_job(&self, builder: GetTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetTranscriptionJobOutput, SdkError<GetTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_vocabulary(&self, builder: GetVocabularyInputBuilder) -> impl Future<Output = Result<GetVocabularyOutput, SdkError<GetVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn get_vocabulary_filter(&self, builder: GetVocabularyFilterInputBuilder) -> impl Future<Output = Result<GetVocabularyFilterOutput, SdkError<GetVocabularyFilterError>>> {
        builder.send_with(&self.0)
    }
    fn list_call_analytics_categories(&self, builder: ListCallAnalyticsCategoriesInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsCategoriesOutput, SdkError<ListCallAnalyticsCategoriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_call_analytics_jobs(&self, builder: ListCallAnalyticsJobsInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsJobsOutput, SdkError<ListCallAnalyticsJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_language_models(&self, builder: ListLanguageModelsInputBuilder) -> impl Future<Output = Result<ListLanguageModelsOutput, SdkError<ListLanguageModelsError>>> {
        builder.send_with(&self.0)
    }
    fn list_medical_scribe_jobs(&self, builder: ListMedicalScribeJobsInputBuilder) -> impl Future<Output = Result<ListMedicalScribeJobsOutput, SdkError<ListMedicalScribeJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_medical_transcription_jobs(&self, builder: ListMedicalTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListMedicalTranscriptionJobsOutput, SdkError<ListMedicalTranscriptionJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_medical_vocabularies(&self, builder: ListMedicalVocabulariesInputBuilder) -> impl Future<Output = Result<ListMedicalVocabulariesOutput, SdkError<ListMedicalVocabulariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_transcription_jobs(&self, builder: ListTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListTranscriptionJobsOutput, SdkError<ListTranscriptionJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_vocabularies(&self, builder: ListVocabulariesInputBuilder) -> impl Future<Output = Result<ListVocabulariesOutput, SdkError<ListVocabulariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_vocabulary_filters(&self, builder: ListVocabularyFiltersInputBuilder) -> impl Future<Output = Result<ListVocabularyFiltersOutput, SdkError<ListVocabularyFiltersError>>> {
        builder.send_with(&self.0)
    }
    fn start_call_analytics_job(&self, builder: StartCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<StartCallAnalyticsJobOutput, SdkError<StartCallAnalyticsJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_medical_scribe_job(&self, builder: StartMedicalScribeJobInputBuilder) -> impl Future<Output = Result<StartMedicalScribeJobOutput, SdkError<StartMedicalScribeJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_medical_transcription_job(&self, builder: StartMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartMedicalTranscriptionJobOutput, SdkError<StartMedicalTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_transcription_job(&self, builder: StartTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartTranscriptionJobOutput, SdkError<StartTranscriptionJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_call_analytics_category(&self, builder: UpdateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<UpdateCallAnalyticsCategoryOutput, SdkError<UpdateCallAnalyticsCategoryError>>> {
        builder.send_with(&self.0)
    }
    fn update_medical_vocabulary(&self, builder: UpdateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<UpdateMedicalVocabularyOutput, SdkError<UpdateMedicalVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn update_vocabulary(&self, builder: UpdateVocabularyInputBuilder) -> impl Future<Output = Result<UpdateVocabularyOutput, SdkError<UpdateVocabularyError>>> {
        builder.send_with(&self.0)
    }
    fn update_vocabulary_filter(&self, builder: UpdateVocabularyFilterInputBuilder) -> impl Future<Output = Result<UpdateVocabularyFilterOutput, SdkError<UpdateVocabularyFilterError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> TranscribeClient for T
where T: Deref,
      T::Target: TranscribeClient {
    fn create_call_analytics_category(&self, builder: CreateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<CreateCallAnalyticsCategoryOutput, SdkError<CreateCallAnalyticsCategoryError>>> {
        self.deref().create_call_analytics_category(builder)
    }
    fn create_language_model(&self, builder: CreateLanguageModelInputBuilder) -> impl Future<Output = Result<CreateLanguageModelOutput, SdkError<CreateLanguageModelError>>> {
        self.deref().create_language_model(builder)
    }
    fn create_medical_vocabulary(&self, builder: CreateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<CreateMedicalVocabularyOutput, SdkError<CreateMedicalVocabularyError>>> {
        self.deref().create_medical_vocabulary(builder)
    }
    fn create_vocabulary(&self, builder: CreateVocabularyInputBuilder) -> impl Future<Output = Result<CreateVocabularyOutput, SdkError<CreateVocabularyError>>> {
        self.deref().create_vocabulary(builder)
    }
    fn create_vocabulary_filter(&self, builder: CreateVocabularyFilterInputBuilder) -> impl Future<Output = Result<CreateVocabularyFilterOutput, SdkError<CreateVocabularyFilterError>>> {
        self.deref().create_vocabulary_filter(builder)
    }
    fn delete_call_analytics_category(&self, builder: DeleteCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsCategoryOutput, SdkError<DeleteCallAnalyticsCategoryError>>> {
        self.deref().delete_call_analytics_category(builder)
    }
    fn delete_call_analytics_job(&self, builder: DeleteCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<DeleteCallAnalyticsJobOutput, SdkError<DeleteCallAnalyticsJobError>>> {
        self.deref().delete_call_analytics_job(builder)
    }
    fn delete_language_model(&self, builder: DeleteLanguageModelInputBuilder) -> impl Future<Output = Result<DeleteLanguageModelOutput, SdkError<DeleteLanguageModelError>>> {
        self.deref().delete_language_model(builder)
    }
    fn delete_medical_scribe_job(&self, builder: DeleteMedicalScribeJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalScribeJobOutput, SdkError<DeleteMedicalScribeJobError>>> {
        self.deref().delete_medical_scribe_job(builder)
    }
    fn delete_medical_transcription_job(&self, builder: DeleteMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteMedicalTranscriptionJobOutput, SdkError<DeleteMedicalTranscriptionJobError>>> {
        self.deref().delete_medical_transcription_job(builder)
    }
    fn delete_medical_vocabulary(&self, builder: DeleteMedicalVocabularyInputBuilder) -> impl Future<Output = Result<DeleteMedicalVocabularyOutput, SdkError<DeleteMedicalVocabularyError>>> {
        self.deref().delete_medical_vocabulary(builder)
    }
    fn delete_transcription_job(&self, builder: DeleteTranscriptionJobInputBuilder) -> impl Future<Output = Result<DeleteTranscriptionJobOutput, SdkError<DeleteTranscriptionJobError>>> {
        self.deref().delete_transcription_job(builder)
    }
    fn delete_vocabulary(&self, builder: DeleteVocabularyInputBuilder) -> impl Future<Output = Result<DeleteVocabularyOutput, SdkError<DeleteVocabularyError>>> {
        self.deref().delete_vocabulary(builder)
    }
    fn delete_vocabulary_filter(&self, builder: DeleteVocabularyFilterInputBuilder) -> impl Future<Output = Result<DeleteVocabularyFilterOutput, SdkError<DeleteVocabularyFilterError>>> {
        self.deref().delete_vocabulary_filter(builder)
    }
    fn describe_language_model(&self, builder: DescribeLanguageModelInputBuilder) -> impl Future<Output = Result<DescribeLanguageModelOutput, SdkError<DescribeLanguageModelError>>> {
        self.deref().describe_language_model(builder)
    }
    fn get_call_analytics_category(&self, builder: GetCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsCategoryOutput, SdkError<GetCallAnalyticsCategoryError>>> {
        self.deref().get_call_analytics_category(builder)
    }
    fn get_call_analytics_job(&self, builder: GetCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<GetCallAnalyticsJobOutput, SdkError<GetCallAnalyticsJobError>>> {
        self.deref().get_call_analytics_job(builder)
    }
    fn get_medical_scribe_job(&self, builder: GetMedicalScribeJobInputBuilder) -> impl Future<Output = Result<GetMedicalScribeJobOutput, SdkError<GetMedicalScribeJobError>>> {
        self.deref().get_medical_scribe_job(builder)
    }
    fn get_medical_transcription_job(&self, builder: GetMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetMedicalTranscriptionJobOutput, SdkError<GetMedicalTranscriptionJobError>>> {
        self.deref().get_medical_transcription_job(builder)
    }
    fn get_medical_vocabulary(&self, builder: GetMedicalVocabularyInputBuilder) -> impl Future<Output = Result<GetMedicalVocabularyOutput, SdkError<GetMedicalVocabularyError>>> {
        self.deref().get_medical_vocabulary(builder)
    }
    fn get_transcription_job(&self, builder: GetTranscriptionJobInputBuilder) -> impl Future<Output = Result<GetTranscriptionJobOutput, SdkError<GetTranscriptionJobError>>> {
        self.deref().get_transcription_job(builder)
    }
    fn get_vocabulary(&self, builder: GetVocabularyInputBuilder) -> impl Future<Output = Result<GetVocabularyOutput, SdkError<GetVocabularyError>>> {
        self.deref().get_vocabulary(builder)
    }
    fn get_vocabulary_filter(&self, builder: GetVocabularyFilterInputBuilder) -> impl Future<Output = Result<GetVocabularyFilterOutput, SdkError<GetVocabularyFilterError>>> {
        self.deref().get_vocabulary_filter(builder)
    }
    fn list_call_analytics_categories(&self, builder: ListCallAnalyticsCategoriesInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsCategoriesOutput, SdkError<ListCallAnalyticsCategoriesError>>> {
        self.deref().list_call_analytics_categories(builder)
    }
    fn list_call_analytics_jobs(&self, builder: ListCallAnalyticsJobsInputBuilder) -> impl Future<Output = Result<ListCallAnalyticsJobsOutput, SdkError<ListCallAnalyticsJobsError>>> {
        self.deref().list_call_analytics_jobs(builder)
    }
    fn list_language_models(&self, builder: ListLanguageModelsInputBuilder) -> impl Future<Output = Result<ListLanguageModelsOutput, SdkError<ListLanguageModelsError>>> {
        self.deref().list_language_models(builder)
    }
    fn list_medical_scribe_jobs(&self, builder: ListMedicalScribeJobsInputBuilder) -> impl Future<Output = Result<ListMedicalScribeJobsOutput, SdkError<ListMedicalScribeJobsError>>> {
        self.deref().list_medical_scribe_jobs(builder)
    }
    fn list_medical_transcription_jobs(&self, builder: ListMedicalTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListMedicalTranscriptionJobsOutput, SdkError<ListMedicalTranscriptionJobsError>>> {
        self.deref().list_medical_transcription_jobs(builder)
    }
    fn list_medical_vocabularies(&self, builder: ListMedicalVocabulariesInputBuilder) -> impl Future<Output = Result<ListMedicalVocabulariesOutput, SdkError<ListMedicalVocabulariesError>>> {
        self.deref().list_medical_vocabularies(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_transcription_jobs(&self, builder: ListTranscriptionJobsInputBuilder) -> impl Future<Output = Result<ListTranscriptionJobsOutput, SdkError<ListTranscriptionJobsError>>> {
        self.deref().list_transcription_jobs(builder)
    }
    fn list_vocabularies(&self, builder: ListVocabulariesInputBuilder) -> impl Future<Output = Result<ListVocabulariesOutput, SdkError<ListVocabulariesError>>> {
        self.deref().list_vocabularies(builder)
    }
    fn list_vocabulary_filters(&self, builder: ListVocabularyFiltersInputBuilder) -> impl Future<Output = Result<ListVocabularyFiltersOutput, SdkError<ListVocabularyFiltersError>>> {
        self.deref().list_vocabulary_filters(builder)
    }
    fn start_call_analytics_job(&self, builder: StartCallAnalyticsJobInputBuilder) -> impl Future<Output = Result<StartCallAnalyticsJobOutput, SdkError<StartCallAnalyticsJobError>>> {
        self.deref().start_call_analytics_job(builder)
    }
    fn start_medical_scribe_job(&self, builder: StartMedicalScribeJobInputBuilder) -> impl Future<Output = Result<StartMedicalScribeJobOutput, SdkError<StartMedicalScribeJobError>>> {
        self.deref().start_medical_scribe_job(builder)
    }
    fn start_medical_transcription_job(&self, builder: StartMedicalTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartMedicalTranscriptionJobOutput, SdkError<StartMedicalTranscriptionJobError>>> {
        self.deref().start_medical_transcription_job(builder)
    }
    fn start_transcription_job(&self, builder: StartTranscriptionJobInputBuilder) -> impl Future<Output = Result<StartTranscriptionJobOutput, SdkError<StartTranscriptionJobError>>> {
        self.deref().start_transcription_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_call_analytics_category(&self, builder: UpdateCallAnalyticsCategoryInputBuilder) -> impl Future<Output = Result<UpdateCallAnalyticsCategoryOutput, SdkError<UpdateCallAnalyticsCategoryError>>> {
        self.deref().update_call_analytics_category(builder)
    }
    fn update_medical_vocabulary(&self, builder: UpdateMedicalVocabularyInputBuilder) -> impl Future<Output = Result<UpdateMedicalVocabularyOutput, SdkError<UpdateMedicalVocabularyError>>> {
        self.deref().update_medical_vocabulary(builder)
    }
    fn update_vocabulary(&self, builder: UpdateVocabularyInputBuilder) -> impl Future<Output = Result<UpdateVocabularyOutput, SdkError<UpdateVocabularyError>>> {
        self.deref().update_vocabulary(builder)
    }
    fn update_vocabulary_filter(&self, builder: UpdateVocabularyFilterInputBuilder) -> impl Future<Output = Result<UpdateVocabularyFilterOutput, SdkError<UpdateVocabularyFilterError>>> {
        self.deref().update_vocabulary_filter(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edTranscribeClient {}
    impl TranscribeClient for edTranscribeClient {
        async fn create_call_analytics_category(&self, builder: CreateCallAnalyticsCategoryInputBuilder) -> Result<CreateCallAnalyticsCategoryOutput, SdkError<CreateCallAnalyticsCategoryError>>;
        async fn create_language_model(&self, builder: CreateLanguageModelInputBuilder) -> Result<CreateLanguageModelOutput, SdkError<CreateLanguageModelError>>;
        async fn create_medical_vocabulary(&self, builder: CreateMedicalVocabularyInputBuilder) -> Result<CreateMedicalVocabularyOutput, SdkError<CreateMedicalVocabularyError>>;
        async fn create_vocabulary(&self, builder: CreateVocabularyInputBuilder) -> Result<CreateVocabularyOutput, SdkError<CreateVocabularyError>>;
        async fn create_vocabulary_filter(&self, builder: CreateVocabularyFilterInputBuilder) -> Result<CreateVocabularyFilterOutput, SdkError<CreateVocabularyFilterError>>;
        async fn delete_call_analytics_category(&self, builder: DeleteCallAnalyticsCategoryInputBuilder) -> Result<DeleteCallAnalyticsCategoryOutput, SdkError<DeleteCallAnalyticsCategoryError>>;
        async fn delete_call_analytics_job(&self, builder: DeleteCallAnalyticsJobInputBuilder) -> Result<DeleteCallAnalyticsJobOutput, SdkError<DeleteCallAnalyticsJobError>>;
        async fn delete_language_model(&self, builder: DeleteLanguageModelInputBuilder) -> Result<DeleteLanguageModelOutput, SdkError<DeleteLanguageModelError>>;
        async fn delete_medical_scribe_job(&self, builder: DeleteMedicalScribeJobInputBuilder) -> Result<DeleteMedicalScribeJobOutput, SdkError<DeleteMedicalScribeJobError>>;
        async fn delete_medical_transcription_job(&self, builder: DeleteMedicalTranscriptionJobInputBuilder) -> Result<DeleteMedicalTranscriptionJobOutput, SdkError<DeleteMedicalTranscriptionJobError>>;
        async fn delete_medical_vocabulary(&self, builder: DeleteMedicalVocabularyInputBuilder) -> Result<DeleteMedicalVocabularyOutput, SdkError<DeleteMedicalVocabularyError>>;
        async fn delete_transcription_job(&self, builder: DeleteTranscriptionJobInputBuilder) -> Result<DeleteTranscriptionJobOutput, SdkError<DeleteTranscriptionJobError>>;
        async fn delete_vocabulary(&self, builder: DeleteVocabularyInputBuilder) -> Result<DeleteVocabularyOutput, SdkError<DeleteVocabularyError>>;
        async fn delete_vocabulary_filter(&self, builder: DeleteVocabularyFilterInputBuilder) -> Result<DeleteVocabularyFilterOutput, SdkError<DeleteVocabularyFilterError>>;
        async fn describe_language_model(&self, builder: DescribeLanguageModelInputBuilder) -> Result<DescribeLanguageModelOutput, SdkError<DescribeLanguageModelError>>;
        async fn get_call_analytics_category(&self, builder: GetCallAnalyticsCategoryInputBuilder) -> Result<GetCallAnalyticsCategoryOutput, SdkError<GetCallAnalyticsCategoryError>>;
        async fn get_call_analytics_job(&self, builder: GetCallAnalyticsJobInputBuilder) -> Result<GetCallAnalyticsJobOutput, SdkError<GetCallAnalyticsJobError>>;
        async fn get_medical_scribe_job(&self, builder: GetMedicalScribeJobInputBuilder) -> Result<GetMedicalScribeJobOutput, SdkError<GetMedicalScribeJobError>>;
        async fn get_medical_transcription_job(&self, builder: GetMedicalTranscriptionJobInputBuilder) -> Result<GetMedicalTranscriptionJobOutput, SdkError<GetMedicalTranscriptionJobError>>;
        async fn get_medical_vocabulary(&self, builder: GetMedicalVocabularyInputBuilder) -> Result<GetMedicalVocabularyOutput, SdkError<GetMedicalVocabularyError>>;
        async fn get_transcription_job(&self, builder: GetTranscriptionJobInputBuilder) -> Result<GetTranscriptionJobOutput, SdkError<GetTranscriptionJobError>>;
        async fn get_vocabulary(&self, builder: GetVocabularyInputBuilder) -> Result<GetVocabularyOutput, SdkError<GetVocabularyError>>;
        async fn get_vocabulary_filter(&self, builder: GetVocabularyFilterInputBuilder) -> Result<GetVocabularyFilterOutput, SdkError<GetVocabularyFilterError>>;
        async fn list_call_analytics_categories(&self, builder: ListCallAnalyticsCategoriesInputBuilder) -> Result<ListCallAnalyticsCategoriesOutput, SdkError<ListCallAnalyticsCategoriesError>>;
        async fn list_call_analytics_jobs(&self, builder: ListCallAnalyticsJobsInputBuilder) -> Result<ListCallAnalyticsJobsOutput, SdkError<ListCallAnalyticsJobsError>>;
        async fn list_language_models(&self, builder: ListLanguageModelsInputBuilder) -> Result<ListLanguageModelsOutput, SdkError<ListLanguageModelsError>>;
        async fn list_medical_scribe_jobs(&self, builder: ListMedicalScribeJobsInputBuilder) -> Result<ListMedicalScribeJobsOutput, SdkError<ListMedicalScribeJobsError>>;
        async fn list_medical_transcription_jobs(&self, builder: ListMedicalTranscriptionJobsInputBuilder) -> Result<ListMedicalTranscriptionJobsOutput, SdkError<ListMedicalTranscriptionJobsError>>;
        async fn list_medical_vocabularies(&self, builder: ListMedicalVocabulariesInputBuilder) -> Result<ListMedicalVocabulariesOutput, SdkError<ListMedicalVocabulariesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_transcription_jobs(&self, builder: ListTranscriptionJobsInputBuilder) -> Result<ListTranscriptionJobsOutput, SdkError<ListTranscriptionJobsError>>;
        async fn list_vocabularies(&self, builder: ListVocabulariesInputBuilder) -> Result<ListVocabulariesOutput, SdkError<ListVocabulariesError>>;
        async fn list_vocabulary_filters(&self, builder: ListVocabularyFiltersInputBuilder) -> Result<ListVocabularyFiltersOutput, SdkError<ListVocabularyFiltersError>>;
        async fn start_call_analytics_job(&self, builder: StartCallAnalyticsJobInputBuilder) -> Result<StartCallAnalyticsJobOutput, SdkError<StartCallAnalyticsJobError>>;
        async fn start_medical_scribe_job(&self, builder: StartMedicalScribeJobInputBuilder) -> Result<StartMedicalScribeJobOutput, SdkError<StartMedicalScribeJobError>>;
        async fn start_medical_transcription_job(&self, builder: StartMedicalTranscriptionJobInputBuilder) -> Result<StartMedicalTranscriptionJobOutput, SdkError<StartMedicalTranscriptionJobError>>;
        async fn start_transcription_job(&self, builder: StartTranscriptionJobInputBuilder) -> Result<StartTranscriptionJobOutput, SdkError<StartTranscriptionJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_call_analytics_category(&self, builder: UpdateCallAnalyticsCategoryInputBuilder) -> Result<UpdateCallAnalyticsCategoryOutput, SdkError<UpdateCallAnalyticsCategoryError>>;
        async fn update_medical_vocabulary(&self, builder: UpdateMedicalVocabularyInputBuilder) -> Result<UpdateMedicalVocabularyOutput, SdkError<UpdateMedicalVocabularyError>>;
        async fn update_vocabulary(&self, builder: UpdateVocabularyInputBuilder) -> Result<UpdateVocabularyOutput, SdkError<UpdateVocabularyError>>;
        async fn update_vocabulary_filter(&self, builder: UpdateVocabularyFilterInputBuilder) -> Result<UpdateVocabularyFilterOutput, SdkError<UpdateVocabularyFilterError>>;
    }
}
