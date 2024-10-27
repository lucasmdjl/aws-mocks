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
use aws_sdk_sqs::operation::add_permission::{builders::*, *};
use aws_sdk_sqs::operation::cancel_message_move_task::{builders::*, *};
use aws_sdk_sqs::operation::change_message_visibility::{builders::*, *};
use aws_sdk_sqs::operation::change_message_visibility_batch::{builders::*, *};
use aws_sdk_sqs::operation::create_queue::{builders::*, *};
use aws_sdk_sqs::operation::delete_message::{builders::*, *};
use aws_sdk_sqs::operation::delete_message_batch::{builders::*, *};
use aws_sdk_sqs::operation::delete_queue::{builders::*, *};
use aws_sdk_sqs::operation::get_queue_attributes::{builders::*, *};
use aws_sdk_sqs::operation::get_queue_url::{builders::*, *};
use aws_sdk_sqs::operation::list_dead_letter_source_queues::{builders::*, *};
use aws_sdk_sqs::operation::list_message_move_tasks::{builders::*, *};
use aws_sdk_sqs::operation::list_queue_tags::{builders::*, *};
use aws_sdk_sqs::operation::list_queues::{builders::*, *};
use aws_sdk_sqs::operation::purge_queue::{builders::*, *};
use aws_sdk_sqs::operation::receive_message::{builders::*, *};
use aws_sdk_sqs::operation::remove_permission::{builders::*, *};
use aws_sdk_sqs::operation::send_message::{builders::*, *};
use aws_sdk_sqs::operation::send_message_batch::{builders::*, *};
use aws_sdk_sqs::operation::set_queue_attributes::{builders::*, *};
use aws_sdk_sqs::operation::start_message_move_task::{builders::*, *};
use aws_sdk_sqs::operation::tag_queue::{builders::*, *};
use aws_sdk_sqs::operation::untag_queue::{builders::*, *};
use aws_sdk_sqs::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_sqs::Client;
use std::ops::Deref;

pub use aws_sdk_sqs::*;

pub struct SQSClientImpl(Client);
impl SQSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait SQSClient {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> + Send;
    fn cancel_message_move_task(&self, builder: CancelMessageMoveTaskInputBuilder) -> impl Future<Output = Result<CancelMessageMoveTaskOutput, SdkError<CancelMessageMoveTaskError>>> + Send;
    fn change_message_visibility(&self, builder: ChangeMessageVisibilityInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityOutput, SdkError<ChangeMessageVisibilityError>>> + Send;
    fn change_message_visibility_batch(&self, builder: ChangeMessageVisibilityBatchInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityBatchOutput, SdkError<ChangeMessageVisibilityBatchError>>> + Send;
    fn create_queue(&self, builder: CreateQueueInputBuilder) -> impl Future<Output = Result<CreateQueueOutput, SdkError<CreateQueueError>>> + Send;
    fn delete_message(&self, builder: DeleteMessageInputBuilder) -> impl Future<Output = Result<DeleteMessageOutput, SdkError<DeleteMessageError>>> + Send;
    fn delete_message_batch(&self, builder: DeleteMessageBatchInputBuilder) -> impl Future<Output = Result<DeleteMessageBatchOutput, SdkError<DeleteMessageBatchError>>> + Send;
    fn delete_queue(&self, builder: DeleteQueueInputBuilder) -> impl Future<Output = Result<DeleteQueueOutput, SdkError<DeleteQueueError>>> + Send;
    fn get_queue_attributes(&self, builder: GetQueueAttributesInputBuilder) -> impl Future<Output = Result<GetQueueAttributesOutput, SdkError<GetQueueAttributesError>>> + Send;
    fn get_queue_url(&self, builder: GetQueueUrlInputBuilder) -> impl Future<Output = Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>> + Send;
    fn list_dead_letter_source_queues(&self, builder: ListDeadLetterSourceQueuesInputBuilder) -> impl Future<Output = Result<ListDeadLetterSourceQueuesOutput, SdkError<ListDeadLetterSourceQueuesError>>> + Send;
    fn list_message_move_tasks(&self, builder: ListMessageMoveTasksInputBuilder) -> impl Future<Output = Result<ListMessageMoveTasksOutput, SdkError<ListMessageMoveTasksError>>> + Send;
    fn list_queue_tags(&self, builder: ListQueueTagsInputBuilder) -> impl Future<Output = Result<ListQueueTagsOutput, SdkError<ListQueueTagsError>>> + Send;
    fn list_queues(&self, builder: ListQueuesInputBuilder) -> impl Future<Output = Result<ListQueuesOutput, SdkError<ListQueuesError>>> + Send;
    fn purge_queue(&self, builder: PurgeQueueInputBuilder) -> impl Future<Output = Result<PurgeQueueOutput, SdkError<PurgeQueueError>>> + Send;
    fn receive_message(&self, builder: ReceiveMessageInputBuilder) -> impl Future<Output = Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>> + Send;
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> + Send;
    fn send_message(&self, builder: SendMessageInputBuilder) -> impl Future<Output = Result<SendMessageOutput, SdkError<SendMessageError>>> + Send;
    fn send_message_batch(&self, builder: SendMessageBatchInputBuilder) -> impl Future<Output = Result<SendMessageBatchOutput, SdkError<SendMessageBatchError>>> + Send;
    fn set_queue_attributes(&self, builder: SetQueueAttributesInputBuilder) -> impl Future<Output = Result<SetQueueAttributesOutput, SdkError<SetQueueAttributesError>>> + Send;
    fn start_message_move_task(&self, builder: StartMessageMoveTaskInputBuilder) -> impl Future<Output = Result<StartMessageMoveTaskOutput, SdkError<StartMessageMoveTaskError>>> + Send;
    fn tag_queue(&self, builder: TagQueueInputBuilder) -> impl Future<Output = Result<TagQueueOutput, SdkError<TagQueueError>>> + Send;
    fn untag_queue(&self, builder: UntagQueueInputBuilder) -> impl Future<Output = Result<UntagQueueOutput, SdkError<UntagQueueError>>> + Send;
}
impl SQSClient for SQSClientImpl {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_message_move_task(&self, builder: CancelMessageMoveTaskInputBuilder) -> impl Future<Output = Result<CancelMessageMoveTaskOutput, SdkError<CancelMessageMoveTaskError>>> {
        builder.send_with(&self.0)
    }
    fn change_message_visibility(&self, builder: ChangeMessageVisibilityInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityOutput, SdkError<ChangeMessageVisibilityError>>> {
        builder.send_with(&self.0)
    }
    fn change_message_visibility_batch(&self, builder: ChangeMessageVisibilityBatchInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityBatchOutput, SdkError<ChangeMessageVisibilityBatchError>>> {
        builder.send_with(&self.0)
    }
    fn create_queue(&self, builder: CreateQueueInputBuilder) -> impl Future<Output = Result<CreateQueueOutput, SdkError<CreateQueueError>>> {
        builder.send_with(&self.0)
    }
    fn delete_message(&self, builder: DeleteMessageInputBuilder) -> impl Future<Output = Result<DeleteMessageOutput, SdkError<DeleteMessageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_message_batch(&self, builder: DeleteMessageBatchInputBuilder) -> impl Future<Output = Result<DeleteMessageBatchOutput, SdkError<DeleteMessageBatchError>>> {
        builder.send_with(&self.0)
    }
    fn delete_queue(&self, builder: DeleteQueueInputBuilder) -> impl Future<Output = Result<DeleteQueueOutput, SdkError<DeleteQueueError>>> {
        builder.send_with(&self.0)
    }
    fn get_queue_attributes(&self, builder: GetQueueAttributesInputBuilder) -> impl Future<Output = Result<GetQueueAttributesOutput, SdkError<GetQueueAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_queue_url(&self, builder: GetQueueUrlInputBuilder) -> impl Future<Output = Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>> {
        builder.send_with(&self.0)
    }
    fn list_dead_letter_source_queues(&self, builder: ListDeadLetterSourceQueuesInputBuilder) -> impl Future<Output = Result<ListDeadLetterSourceQueuesOutput, SdkError<ListDeadLetterSourceQueuesError>>> {
        builder.send_with(&self.0)
    }
    fn list_message_move_tasks(&self, builder: ListMessageMoveTasksInputBuilder) -> impl Future<Output = Result<ListMessageMoveTasksOutput, SdkError<ListMessageMoveTasksError>>> {
        builder.send_with(&self.0)
    }
    fn list_queue_tags(&self, builder: ListQueueTagsInputBuilder) -> impl Future<Output = Result<ListQueueTagsOutput, SdkError<ListQueueTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_queues(&self, builder: ListQueuesInputBuilder) -> impl Future<Output = Result<ListQueuesOutput, SdkError<ListQueuesError>>> {
        builder.send_with(&self.0)
    }
    fn purge_queue(&self, builder: PurgeQueueInputBuilder) -> impl Future<Output = Result<PurgeQueueOutput, SdkError<PurgeQueueError>>> {
        builder.send_with(&self.0)
    }
    fn receive_message(&self, builder: ReceiveMessageInputBuilder) -> impl Future<Output = Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>> {
        builder.send_with(&self.0)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn send_message(&self, builder: SendMessageInputBuilder) -> impl Future<Output = Result<SendMessageOutput, SdkError<SendMessageError>>> {
        builder.send_with(&self.0)
    }
    fn send_message_batch(&self, builder: SendMessageBatchInputBuilder) -> impl Future<Output = Result<SendMessageBatchOutput, SdkError<SendMessageBatchError>>> {
        builder.send_with(&self.0)
    }
    fn set_queue_attributes(&self, builder: SetQueueAttributesInputBuilder) -> impl Future<Output = Result<SetQueueAttributesOutput, SdkError<SetQueueAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn start_message_move_task(&self, builder: StartMessageMoveTaskInputBuilder) -> impl Future<Output = Result<StartMessageMoveTaskOutput, SdkError<StartMessageMoveTaskError>>> {
        builder.send_with(&self.0)
    }
    fn tag_queue(&self, builder: TagQueueInputBuilder) -> impl Future<Output = Result<TagQueueOutput, SdkError<TagQueueError>>> {
        builder.send_with(&self.0)
    }
    fn untag_queue(&self, builder: UntagQueueInputBuilder) -> impl Future<Output = Result<UntagQueueOutput, SdkError<UntagQueueError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> SQSClient for T
where T: Deref,
      T::Target: SQSClient {
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        self.deref().add_permission(builder)
    }
    fn cancel_message_move_task(&self, builder: CancelMessageMoveTaskInputBuilder) -> impl Future<Output = Result<CancelMessageMoveTaskOutput, SdkError<CancelMessageMoveTaskError>>> {
        self.deref().cancel_message_move_task(builder)
    }
    fn change_message_visibility(&self, builder: ChangeMessageVisibilityInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityOutput, SdkError<ChangeMessageVisibilityError>>> {
        self.deref().change_message_visibility(builder)
    }
    fn change_message_visibility_batch(&self, builder: ChangeMessageVisibilityBatchInputBuilder) -> impl Future<Output = Result<ChangeMessageVisibilityBatchOutput, SdkError<ChangeMessageVisibilityBatchError>>> {
        self.deref().change_message_visibility_batch(builder)
    }
    fn create_queue(&self, builder: CreateQueueInputBuilder) -> impl Future<Output = Result<CreateQueueOutput, SdkError<CreateQueueError>>> {
        self.deref().create_queue(builder)
    }
    fn delete_message(&self, builder: DeleteMessageInputBuilder) -> impl Future<Output = Result<DeleteMessageOutput, SdkError<DeleteMessageError>>> {
        self.deref().delete_message(builder)
    }
    fn delete_message_batch(&self, builder: DeleteMessageBatchInputBuilder) -> impl Future<Output = Result<DeleteMessageBatchOutput, SdkError<DeleteMessageBatchError>>> {
        self.deref().delete_message_batch(builder)
    }
    fn delete_queue(&self, builder: DeleteQueueInputBuilder) -> impl Future<Output = Result<DeleteQueueOutput, SdkError<DeleteQueueError>>> {
        self.deref().delete_queue(builder)
    }
    fn get_queue_attributes(&self, builder: GetQueueAttributesInputBuilder) -> impl Future<Output = Result<GetQueueAttributesOutput, SdkError<GetQueueAttributesError>>> {
        self.deref().get_queue_attributes(builder)
    }
    fn get_queue_url(&self, builder: GetQueueUrlInputBuilder) -> impl Future<Output = Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>> {
        self.deref().get_queue_url(builder)
    }
    fn list_dead_letter_source_queues(&self, builder: ListDeadLetterSourceQueuesInputBuilder) -> impl Future<Output = Result<ListDeadLetterSourceQueuesOutput, SdkError<ListDeadLetterSourceQueuesError>>> {
        self.deref().list_dead_letter_source_queues(builder)
    }
    fn list_message_move_tasks(&self, builder: ListMessageMoveTasksInputBuilder) -> impl Future<Output = Result<ListMessageMoveTasksOutput, SdkError<ListMessageMoveTasksError>>> {
        self.deref().list_message_move_tasks(builder)
    }
    fn list_queue_tags(&self, builder: ListQueueTagsInputBuilder) -> impl Future<Output = Result<ListQueueTagsOutput, SdkError<ListQueueTagsError>>> {
        self.deref().list_queue_tags(builder)
    }
    fn list_queues(&self, builder: ListQueuesInputBuilder) -> impl Future<Output = Result<ListQueuesOutput, SdkError<ListQueuesError>>> {
        self.deref().list_queues(builder)
    }
    fn purge_queue(&self, builder: PurgeQueueInputBuilder) -> impl Future<Output = Result<PurgeQueueOutput, SdkError<PurgeQueueError>>> {
        self.deref().purge_queue(builder)
    }
    fn receive_message(&self, builder: ReceiveMessageInputBuilder) -> impl Future<Output = Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>> {
        self.deref().receive_message(builder)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        self.deref().remove_permission(builder)
    }
    fn send_message(&self, builder: SendMessageInputBuilder) -> impl Future<Output = Result<SendMessageOutput, SdkError<SendMessageError>>> {
        self.deref().send_message(builder)
    }
    fn send_message_batch(&self, builder: SendMessageBatchInputBuilder) -> impl Future<Output = Result<SendMessageBatchOutput, SdkError<SendMessageBatchError>>> {
        self.deref().send_message_batch(builder)
    }
    fn set_queue_attributes(&self, builder: SetQueueAttributesInputBuilder) -> impl Future<Output = Result<SetQueueAttributesOutput, SdkError<SetQueueAttributesError>>> {
        self.deref().set_queue_attributes(builder)
    }
    fn start_message_move_task(&self, builder: StartMessageMoveTaskInputBuilder) -> impl Future<Output = Result<StartMessageMoveTaskOutput, SdkError<StartMessageMoveTaskError>>> {
        self.deref().start_message_move_task(builder)
    }
    fn tag_queue(&self, builder: TagQueueInputBuilder) -> impl Future<Output = Result<TagQueueOutput, SdkError<TagQueueError>>> {
        self.deref().tag_queue(builder)
    }
    fn untag_queue(&self, builder: UntagQueueInputBuilder) -> impl Future<Output = Result<UntagQueueOutput, SdkError<UntagQueueError>>> {
        self.deref().untag_queue(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edSQSClient {}
    impl SQSClient for edSQSClient {
        async fn add_permission(&self, builder: AddPermissionInputBuilder) -> Result<AddPermissionOutput, SdkError<AddPermissionError>>;
        async fn cancel_message_move_task(&self, builder: CancelMessageMoveTaskInputBuilder) -> Result<CancelMessageMoveTaskOutput, SdkError<CancelMessageMoveTaskError>>;
        async fn change_message_visibility(&self, builder: ChangeMessageVisibilityInputBuilder) -> Result<ChangeMessageVisibilityOutput, SdkError<ChangeMessageVisibilityError>>;
        async fn change_message_visibility_batch(&self, builder: ChangeMessageVisibilityBatchInputBuilder) -> Result<ChangeMessageVisibilityBatchOutput, SdkError<ChangeMessageVisibilityBatchError>>;
        async fn create_queue(&self, builder: CreateQueueInputBuilder) -> Result<CreateQueueOutput, SdkError<CreateQueueError>>;
        async fn delete_message(&self, builder: DeleteMessageInputBuilder) -> Result<DeleteMessageOutput, SdkError<DeleteMessageError>>;
        async fn delete_message_batch(&self, builder: DeleteMessageBatchInputBuilder) -> Result<DeleteMessageBatchOutput, SdkError<DeleteMessageBatchError>>;
        async fn delete_queue(&self, builder: DeleteQueueInputBuilder) -> Result<DeleteQueueOutput, SdkError<DeleteQueueError>>;
        async fn get_queue_attributes(&self, builder: GetQueueAttributesInputBuilder) -> Result<GetQueueAttributesOutput, SdkError<GetQueueAttributesError>>;
        async fn get_queue_url(&self, builder: GetQueueUrlInputBuilder) -> Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>;
        async fn list_dead_letter_source_queues(&self, builder: ListDeadLetterSourceQueuesInputBuilder) -> Result<ListDeadLetterSourceQueuesOutput, SdkError<ListDeadLetterSourceQueuesError>>;
        async fn list_message_move_tasks(&self, builder: ListMessageMoveTasksInputBuilder) -> Result<ListMessageMoveTasksOutput, SdkError<ListMessageMoveTasksError>>;
        async fn list_queue_tags(&self, builder: ListQueueTagsInputBuilder) -> Result<ListQueueTagsOutput, SdkError<ListQueueTagsError>>;
        async fn list_queues(&self, builder: ListQueuesInputBuilder) -> Result<ListQueuesOutput, SdkError<ListQueuesError>>;
        async fn purge_queue(&self, builder: PurgeQueueInputBuilder) -> Result<PurgeQueueOutput, SdkError<PurgeQueueError>>;
        async fn receive_message(&self, builder: ReceiveMessageInputBuilder) -> Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>;
        async fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> Result<RemovePermissionOutput, SdkError<RemovePermissionError>>;
        async fn send_message(&self, builder: SendMessageInputBuilder) -> Result<SendMessageOutput, SdkError<SendMessageError>>;
        async fn send_message_batch(&self, builder: SendMessageBatchInputBuilder) -> Result<SendMessageBatchOutput, SdkError<SendMessageBatchError>>;
        async fn set_queue_attributes(&self, builder: SetQueueAttributesInputBuilder) -> Result<SetQueueAttributesOutput, SdkError<SetQueueAttributesError>>;
        async fn start_message_move_task(&self, builder: StartMessageMoveTaskInputBuilder) -> Result<StartMessageMoveTaskOutput, SdkError<StartMessageMoveTaskError>>;
        async fn tag_queue(&self, builder: TagQueueInputBuilder) -> Result<TagQueueOutput, SdkError<TagQueueError>>;
        async fn untag_queue(&self, builder: UntagQueueInputBuilder) -> Result<UntagQueueOutput, SdkError<UntagQueueError>>;
    }
}
