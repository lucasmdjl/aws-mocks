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
use aws_sdk_polly::operation::delete_lexicon::{builders::*, *};
use aws_sdk_polly::operation::describe_voices::{builders::*, *};
use aws_sdk_polly::operation::get_lexicon::{builders::*, *};
use aws_sdk_polly::operation::get_speech_synthesis_task::{builders::*, *};
use aws_sdk_polly::operation::list_lexicons::{builders::*, *};
use aws_sdk_polly::operation::list_speech_synthesis_tasks::{builders::*, *};
use aws_sdk_polly::operation::put_lexicon::{builders::*, *};
use aws_sdk_polly::operation::start_speech_synthesis_task::{builders::*, *};
use aws_sdk_polly::operation::synthesize_speech::{builders::*, *};
use aws_sdk_polly::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_polly::Client;

pub use aws_sdk_polly::*;

pub struct PollyClientImpl(Client);
impl PollyClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait PollyClient {
    fn delete_lexicon(&self, builder: DeleteLexiconInputBuilder) -> impl Future<Output = Result<DeleteLexiconOutput, SdkError<DeleteLexiconError>>>;
    fn describe_voices(&self, builder: DescribeVoicesInputBuilder) -> impl Future<Output = Result<DescribeVoicesOutput, SdkError<DescribeVoicesError>>>;
    fn get_lexicon(&self, builder: GetLexiconInputBuilder) -> impl Future<Output = Result<GetLexiconOutput, SdkError<GetLexiconError>>>;
    fn get_speech_synthesis_task(&self, builder: GetSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<GetSpeechSynthesisTaskOutput, SdkError<GetSpeechSynthesisTaskError>>>;
    fn list_lexicons(&self, builder: ListLexiconsInputBuilder) -> impl Future<Output = Result<ListLexiconsOutput, SdkError<ListLexiconsError>>>;
    fn list_speech_synthesis_tasks(&self, builder: ListSpeechSynthesisTasksInputBuilder) -> impl Future<Output = Result<ListSpeechSynthesisTasksOutput, SdkError<ListSpeechSynthesisTasksError>>>;
    fn put_lexicon(&self, builder: PutLexiconInputBuilder) -> impl Future<Output = Result<PutLexiconOutput, SdkError<PutLexiconError>>>;
    fn start_speech_synthesis_task(&self, builder: StartSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<StartSpeechSynthesisTaskOutput, SdkError<StartSpeechSynthesisTaskError>>>;
    fn synthesize_speech(&self, builder: SynthesizeSpeechInputBuilder) -> impl Future<Output = Result<SynthesizeSpeechOutput, SdkError<SynthesizeSpeechError>>>;
}
impl PollyClient for PollyClientImpl {
    fn delete_lexicon(&self, builder: DeleteLexiconInputBuilder) -> impl Future<Output = Result<DeleteLexiconOutput, SdkError<DeleteLexiconError>>> {
        builder.send_with(&self.0)
    }
    fn describe_voices(&self, builder: DescribeVoicesInputBuilder) -> impl Future<Output = Result<DescribeVoicesOutput, SdkError<DescribeVoicesError>>> {
        builder.send_with(&self.0)
    }
    fn get_lexicon(&self, builder: GetLexiconInputBuilder) -> impl Future<Output = Result<GetLexiconOutput, SdkError<GetLexiconError>>> {
        builder.send_with(&self.0)
    }
    fn get_speech_synthesis_task(&self, builder: GetSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<GetSpeechSynthesisTaskOutput, SdkError<GetSpeechSynthesisTaskError>>> {
        builder.send_with(&self.0)
    }
    fn list_lexicons(&self, builder: ListLexiconsInputBuilder) -> impl Future<Output = Result<ListLexiconsOutput, SdkError<ListLexiconsError>>> {
        builder.send_with(&self.0)
    }
    fn list_speech_synthesis_tasks(&self, builder: ListSpeechSynthesisTasksInputBuilder) -> impl Future<Output = Result<ListSpeechSynthesisTasksOutput, SdkError<ListSpeechSynthesisTasksError>>> {
        builder.send_with(&self.0)
    }
    fn put_lexicon(&self, builder: PutLexiconInputBuilder) -> impl Future<Output = Result<PutLexiconOutput, SdkError<PutLexiconError>>> {
        builder.send_with(&self.0)
    }
    fn start_speech_synthesis_task(&self, builder: StartSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<StartSpeechSynthesisTaskOutput, SdkError<StartSpeechSynthesisTaskError>>> {
        builder.send_with(&self.0)
    }
    fn synthesize_speech(&self, builder: SynthesizeSpeechInputBuilder) -> impl Future<Output = Result<SynthesizeSpeechOutput, SdkError<SynthesizeSpeechError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: PollyClient> PollyClient for &T {
    fn delete_lexicon(&self, builder: DeleteLexiconInputBuilder) -> impl Future<Output = Result<DeleteLexiconOutput, SdkError<DeleteLexiconError>>> {
        (*self).delete_lexicon(builder)
    }
    fn describe_voices(&self, builder: DescribeVoicesInputBuilder) -> impl Future<Output = Result<DescribeVoicesOutput, SdkError<DescribeVoicesError>>> {
        (*self).describe_voices(builder)
    }
    fn get_lexicon(&self, builder: GetLexiconInputBuilder) -> impl Future<Output = Result<GetLexiconOutput, SdkError<GetLexiconError>>> {
        (*self).get_lexicon(builder)
    }
    fn get_speech_synthesis_task(&self, builder: GetSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<GetSpeechSynthesisTaskOutput, SdkError<GetSpeechSynthesisTaskError>>> {
        (*self).get_speech_synthesis_task(builder)
    }
    fn list_lexicons(&self, builder: ListLexiconsInputBuilder) -> impl Future<Output = Result<ListLexiconsOutput, SdkError<ListLexiconsError>>> {
        (*self).list_lexicons(builder)
    }
    fn list_speech_synthesis_tasks(&self, builder: ListSpeechSynthesisTasksInputBuilder) -> impl Future<Output = Result<ListSpeechSynthesisTasksOutput, SdkError<ListSpeechSynthesisTasksError>>> {
        (*self).list_speech_synthesis_tasks(builder)
    }
    fn put_lexicon(&self, builder: PutLexiconInputBuilder) -> impl Future<Output = Result<PutLexiconOutput, SdkError<PutLexiconError>>> {
        (*self).put_lexicon(builder)
    }
    fn start_speech_synthesis_task(&self, builder: StartSpeechSynthesisTaskInputBuilder) -> impl Future<Output = Result<StartSpeechSynthesisTaskOutput, SdkError<StartSpeechSynthesisTaskError>>> {
        (*self).start_speech_synthesis_task(builder)
    }
    fn synthesize_speech(&self, builder: SynthesizeSpeechInputBuilder) -> impl Future<Output = Result<SynthesizeSpeechOutput, SdkError<SynthesizeSpeechError>>> {
        (*self).synthesize_speech(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edPollyClient {}
    impl PollyClient for edPollyClient {
        async fn delete_lexicon(&self, builder: DeleteLexiconInputBuilder) -> Result<DeleteLexiconOutput, SdkError<DeleteLexiconError>>;
        async fn describe_voices(&self, builder: DescribeVoicesInputBuilder) -> Result<DescribeVoicesOutput, SdkError<DescribeVoicesError>>;
        async fn get_lexicon(&self, builder: GetLexiconInputBuilder) -> Result<GetLexiconOutput, SdkError<GetLexiconError>>;
        async fn get_speech_synthesis_task(&self, builder: GetSpeechSynthesisTaskInputBuilder) -> Result<GetSpeechSynthesisTaskOutput, SdkError<GetSpeechSynthesisTaskError>>;
        async fn list_lexicons(&self, builder: ListLexiconsInputBuilder) -> Result<ListLexiconsOutput, SdkError<ListLexiconsError>>;
        async fn list_speech_synthesis_tasks(&self, builder: ListSpeechSynthesisTasksInputBuilder) -> Result<ListSpeechSynthesisTasksOutput, SdkError<ListSpeechSynthesisTasksError>>;
        async fn put_lexicon(&self, builder: PutLexiconInputBuilder) -> Result<PutLexiconOutput, SdkError<PutLexiconError>>;
        async fn start_speech_synthesis_task(&self, builder: StartSpeechSynthesisTaskInputBuilder) -> Result<StartSpeechSynthesisTaskOutput, SdkError<StartSpeechSynthesisTaskError>>;
        async fn synthesize_speech(&self, builder: SynthesizeSpeechInputBuilder) -> Result<SynthesizeSpeechOutput, SdkError<SynthesizeSpeechError>>;
    }
}
