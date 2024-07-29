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
use aws_sdk_translate::operation::create_parallel_data::{builders::*, *};
use aws_sdk_translate::operation::delete_parallel_data::{builders::*, *};
use aws_sdk_translate::operation::delete_terminology::{builders::*, *};
use aws_sdk_translate::operation::describe_text_translation_job::{builders::*, *};
use aws_sdk_translate::operation::get_parallel_data::{builders::*, *};
use aws_sdk_translate::operation::get_terminology::{builders::*, *};
use aws_sdk_translate::operation::import_terminology::{builders::*, *};
use aws_sdk_translate::operation::list_languages::{builders::*, *};
use aws_sdk_translate::operation::list_parallel_data::{builders::*, *};
use aws_sdk_translate::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_translate::operation::list_terminologies::{builders::*, *};
use aws_sdk_translate::operation::list_text_translation_jobs::{builders::*, *};
use aws_sdk_translate::operation::start_text_translation_job::{builders::*, *};
use aws_sdk_translate::operation::stop_text_translation_job::{builders::*, *};
use aws_sdk_translate::operation::tag_resource::{builders::*, *};
use aws_sdk_translate::operation::translate_document::{builders::*, *};
use aws_sdk_translate::operation::translate_text::{builders::*, *};
use aws_sdk_translate::operation::untag_resource::{builders::*, *};
use aws_sdk_translate::operation::update_parallel_data::{builders::*, *};
use aws_sdk_translate::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_translate::Client;
use std::ops::Deref;

pub use aws_sdk_translate::*;

pub struct TranslateClientImpl(Client);
impl TranslateClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait TranslateClient {
    fn create_parallel_data(&self, builder: CreateParallelDataInputBuilder) -> impl Future<Output = Result<CreateParallelDataOutput, SdkError<CreateParallelDataError>>>;
    fn delete_parallel_data(&self, builder: DeleteParallelDataInputBuilder) -> impl Future<Output = Result<DeleteParallelDataOutput, SdkError<DeleteParallelDataError>>>;
    fn delete_terminology(&self, builder: DeleteTerminologyInputBuilder) -> impl Future<Output = Result<DeleteTerminologyOutput, SdkError<DeleteTerminologyError>>>;
    fn describe_text_translation_job(&self, builder: DescribeTextTranslationJobInputBuilder) -> impl Future<Output = Result<DescribeTextTranslationJobOutput, SdkError<DescribeTextTranslationJobError>>>;
    fn get_parallel_data(&self, builder: GetParallelDataInputBuilder) -> impl Future<Output = Result<GetParallelDataOutput, SdkError<GetParallelDataError>>>;
    fn get_terminology(&self, builder: GetTerminologyInputBuilder) -> impl Future<Output = Result<GetTerminologyOutput, SdkError<GetTerminologyError>>>;
    fn import_terminology(&self, builder: ImportTerminologyInputBuilder) -> impl Future<Output = Result<ImportTerminologyOutput, SdkError<ImportTerminologyError>>>;
    fn list_languages(&self, builder: ListLanguagesInputBuilder) -> impl Future<Output = Result<ListLanguagesOutput, SdkError<ListLanguagesError>>>;
    fn list_parallel_data(&self, builder: ListParallelDataInputBuilder) -> impl Future<Output = Result<ListParallelDataOutput, SdkError<ListParallelDataError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_terminologies(&self, builder: ListTerminologiesInputBuilder) -> impl Future<Output = Result<ListTerminologiesOutput, SdkError<ListTerminologiesError>>>;
    fn list_text_translation_jobs(&self, builder: ListTextTranslationJobsInputBuilder) -> impl Future<Output = Result<ListTextTranslationJobsOutput, SdkError<ListTextTranslationJobsError>>>;
    fn start_text_translation_job(&self, builder: StartTextTranslationJobInputBuilder) -> impl Future<Output = Result<StartTextTranslationJobOutput, SdkError<StartTextTranslationJobError>>>;
    fn stop_text_translation_job(&self, builder: StopTextTranslationJobInputBuilder) -> impl Future<Output = Result<StopTextTranslationJobOutput, SdkError<StopTextTranslationJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn translate_document(&self, builder: TranslateDocumentInputBuilder) -> impl Future<Output = Result<TranslateDocumentOutput, SdkError<TranslateDocumentError>>>;
    fn translate_text(&self, builder: TranslateTextInputBuilder) -> impl Future<Output = Result<TranslateTextOutput, SdkError<TranslateTextError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_parallel_data(&self, builder: UpdateParallelDataInputBuilder) -> impl Future<Output = Result<UpdateParallelDataOutput, SdkError<UpdateParallelDataError>>>;
}
impl TranslateClient for TranslateClientImpl {
    fn create_parallel_data(&self, builder: CreateParallelDataInputBuilder) -> impl Future<Output = Result<CreateParallelDataOutput, SdkError<CreateParallelDataError>>> {
        builder.send_with(&self.0)
    }
    fn delete_parallel_data(&self, builder: DeleteParallelDataInputBuilder) -> impl Future<Output = Result<DeleteParallelDataOutput, SdkError<DeleteParallelDataError>>> {
        builder.send_with(&self.0)
    }
    fn delete_terminology(&self, builder: DeleteTerminologyInputBuilder) -> impl Future<Output = Result<DeleteTerminologyOutput, SdkError<DeleteTerminologyError>>> {
        builder.send_with(&self.0)
    }
    fn describe_text_translation_job(&self, builder: DescribeTextTranslationJobInputBuilder) -> impl Future<Output = Result<DescribeTextTranslationJobOutput, SdkError<DescribeTextTranslationJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_parallel_data(&self, builder: GetParallelDataInputBuilder) -> impl Future<Output = Result<GetParallelDataOutput, SdkError<GetParallelDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_terminology(&self, builder: GetTerminologyInputBuilder) -> impl Future<Output = Result<GetTerminologyOutput, SdkError<GetTerminologyError>>> {
        builder.send_with(&self.0)
    }
    fn import_terminology(&self, builder: ImportTerminologyInputBuilder) -> impl Future<Output = Result<ImportTerminologyOutput, SdkError<ImportTerminologyError>>> {
        builder.send_with(&self.0)
    }
    fn list_languages(&self, builder: ListLanguagesInputBuilder) -> impl Future<Output = Result<ListLanguagesOutput, SdkError<ListLanguagesError>>> {
        builder.send_with(&self.0)
    }
    fn list_parallel_data(&self, builder: ListParallelDataInputBuilder) -> impl Future<Output = Result<ListParallelDataOutput, SdkError<ListParallelDataError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_terminologies(&self, builder: ListTerminologiesInputBuilder) -> impl Future<Output = Result<ListTerminologiesOutput, SdkError<ListTerminologiesError>>> {
        builder.send_with(&self.0)
    }
    fn list_text_translation_jobs(&self, builder: ListTextTranslationJobsInputBuilder) -> impl Future<Output = Result<ListTextTranslationJobsOutput, SdkError<ListTextTranslationJobsError>>> {
        builder.send_with(&self.0)
    }
    fn start_text_translation_job(&self, builder: StartTextTranslationJobInputBuilder) -> impl Future<Output = Result<StartTextTranslationJobOutput, SdkError<StartTextTranslationJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_text_translation_job(&self, builder: StopTextTranslationJobInputBuilder) -> impl Future<Output = Result<StopTextTranslationJobOutput, SdkError<StopTextTranslationJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn translate_document(&self, builder: TranslateDocumentInputBuilder) -> impl Future<Output = Result<TranslateDocumentOutput, SdkError<TranslateDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn translate_text(&self, builder: TranslateTextInputBuilder) -> impl Future<Output = Result<TranslateTextOutput, SdkError<TranslateTextError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_parallel_data(&self, builder: UpdateParallelDataInputBuilder) -> impl Future<Output = Result<UpdateParallelDataOutput, SdkError<UpdateParallelDataError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> TranslateClient for T
where T: Deref,
      T::Target: TranslateClient {
    fn create_parallel_data(&self, builder: CreateParallelDataInputBuilder) -> impl Future<Output = Result<CreateParallelDataOutput, SdkError<CreateParallelDataError>>> {
        self.deref().create_parallel_data(builder)
    }
    fn delete_parallel_data(&self, builder: DeleteParallelDataInputBuilder) -> impl Future<Output = Result<DeleteParallelDataOutput, SdkError<DeleteParallelDataError>>> {
        self.deref().delete_parallel_data(builder)
    }
    fn delete_terminology(&self, builder: DeleteTerminologyInputBuilder) -> impl Future<Output = Result<DeleteTerminologyOutput, SdkError<DeleteTerminologyError>>> {
        self.deref().delete_terminology(builder)
    }
    fn describe_text_translation_job(&self, builder: DescribeTextTranslationJobInputBuilder) -> impl Future<Output = Result<DescribeTextTranslationJobOutput, SdkError<DescribeTextTranslationJobError>>> {
        self.deref().describe_text_translation_job(builder)
    }
    fn get_parallel_data(&self, builder: GetParallelDataInputBuilder) -> impl Future<Output = Result<GetParallelDataOutput, SdkError<GetParallelDataError>>> {
        self.deref().get_parallel_data(builder)
    }
    fn get_terminology(&self, builder: GetTerminologyInputBuilder) -> impl Future<Output = Result<GetTerminologyOutput, SdkError<GetTerminologyError>>> {
        self.deref().get_terminology(builder)
    }
    fn import_terminology(&self, builder: ImportTerminologyInputBuilder) -> impl Future<Output = Result<ImportTerminologyOutput, SdkError<ImportTerminologyError>>> {
        self.deref().import_terminology(builder)
    }
    fn list_languages(&self, builder: ListLanguagesInputBuilder) -> impl Future<Output = Result<ListLanguagesOutput, SdkError<ListLanguagesError>>> {
        self.deref().list_languages(builder)
    }
    fn list_parallel_data(&self, builder: ListParallelDataInputBuilder) -> impl Future<Output = Result<ListParallelDataOutput, SdkError<ListParallelDataError>>> {
        self.deref().list_parallel_data(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_terminologies(&self, builder: ListTerminologiesInputBuilder) -> impl Future<Output = Result<ListTerminologiesOutput, SdkError<ListTerminologiesError>>> {
        self.deref().list_terminologies(builder)
    }
    fn list_text_translation_jobs(&self, builder: ListTextTranslationJobsInputBuilder) -> impl Future<Output = Result<ListTextTranslationJobsOutput, SdkError<ListTextTranslationJobsError>>> {
        self.deref().list_text_translation_jobs(builder)
    }
    fn start_text_translation_job(&self, builder: StartTextTranslationJobInputBuilder) -> impl Future<Output = Result<StartTextTranslationJobOutput, SdkError<StartTextTranslationJobError>>> {
        self.deref().start_text_translation_job(builder)
    }
    fn stop_text_translation_job(&self, builder: StopTextTranslationJobInputBuilder) -> impl Future<Output = Result<StopTextTranslationJobOutput, SdkError<StopTextTranslationJobError>>> {
        self.deref().stop_text_translation_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn translate_document(&self, builder: TranslateDocumentInputBuilder) -> impl Future<Output = Result<TranslateDocumentOutput, SdkError<TranslateDocumentError>>> {
        self.deref().translate_document(builder)
    }
    fn translate_text(&self, builder: TranslateTextInputBuilder) -> impl Future<Output = Result<TranslateTextOutput, SdkError<TranslateTextError>>> {
        self.deref().translate_text(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_parallel_data(&self, builder: UpdateParallelDataInputBuilder) -> impl Future<Output = Result<UpdateParallelDataOutput, SdkError<UpdateParallelDataError>>> {
        self.deref().update_parallel_data(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edTranslateClient {}
    impl TranslateClient for edTranslateClient {
        async fn create_parallel_data(&self, builder: CreateParallelDataInputBuilder) -> Result<CreateParallelDataOutput, SdkError<CreateParallelDataError>>;
        async fn delete_parallel_data(&self, builder: DeleteParallelDataInputBuilder) -> Result<DeleteParallelDataOutput, SdkError<DeleteParallelDataError>>;
        async fn delete_terminology(&self, builder: DeleteTerminologyInputBuilder) -> Result<DeleteTerminologyOutput, SdkError<DeleteTerminologyError>>;
        async fn describe_text_translation_job(&self, builder: DescribeTextTranslationJobInputBuilder) -> Result<DescribeTextTranslationJobOutput, SdkError<DescribeTextTranslationJobError>>;
        async fn get_parallel_data(&self, builder: GetParallelDataInputBuilder) -> Result<GetParallelDataOutput, SdkError<GetParallelDataError>>;
        async fn get_terminology(&self, builder: GetTerminologyInputBuilder) -> Result<GetTerminologyOutput, SdkError<GetTerminologyError>>;
        async fn import_terminology(&self, builder: ImportTerminologyInputBuilder) -> Result<ImportTerminologyOutput, SdkError<ImportTerminologyError>>;
        async fn list_languages(&self, builder: ListLanguagesInputBuilder) -> Result<ListLanguagesOutput, SdkError<ListLanguagesError>>;
        async fn list_parallel_data(&self, builder: ListParallelDataInputBuilder) -> Result<ListParallelDataOutput, SdkError<ListParallelDataError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_terminologies(&self, builder: ListTerminologiesInputBuilder) -> Result<ListTerminologiesOutput, SdkError<ListTerminologiesError>>;
        async fn list_text_translation_jobs(&self, builder: ListTextTranslationJobsInputBuilder) -> Result<ListTextTranslationJobsOutput, SdkError<ListTextTranslationJobsError>>;
        async fn start_text_translation_job(&self, builder: StartTextTranslationJobInputBuilder) -> Result<StartTextTranslationJobOutput, SdkError<StartTextTranslationJobError>>;
        async fn stop_text_translation_job(&self, builder: StopTextTranslationJobInputBuilder) -> Result<StopTextTranslationJobOutput, SdkError<StopTextTranslationJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn translate_document(&self, builder: TranslateDocumentInputBuilder) -> Result<TranslateDocumentOutput, SdkError<TranslateDocumentError>>;
        async fn translate_text(&self, builder: TranslateTextInputBuilder) -> Result<TranslateTextOutput, SdkError<TranslateTextError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_parallel_data(&self, builder: UpdateParallelDataInputBuilder) -> Result<UpdateParallelDataOutput, SdkError<UpdateParallelDataError>>;
    }
}
