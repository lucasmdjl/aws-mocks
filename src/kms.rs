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
use aws_sdk_kms::operation::cancel_key_deletion::{builders::*, *};
use aws_sdk_kms::operation::connect_custom_key_store::{builders::*, *};
use aws_sdk_kms::operation::create_alias::{builders::*, *};
use aws_sdk_kms::operation::create_custom_key_store::{builders::*, *};
use aws_sdk_kms::operation::create_grant::{builders::*, *};
use aws_sdk_kms::operation::create_key::{builders::*, *};
use aws_sdk_kms::operation::decrypt::{builders::*, *};
use aws_sdk_kms::operation::delete_alias::{builders::*, *};
use aws_sdk_kms::operation::delete_custom_key_store::{builders::*, *};
use aws_sdk_kms::operation::delete_imported_key_material::{builders::*, *};
use aws_sdk_kms::operation::derive_shared_secret::{builders::*, *};
use aws_sdk_kms::operation::describe_custom_key_stores::{builders::*, *};
use aws_sdk_kms::operation::describe_key::{builders::*, *};
use aws_sdk_kms::operation::disable_key::{builders::*, *};
use aws_sdk_kms::operation::disable_key_rotation::{builders::*, *};
use aws_sdk_kms::operation::disconnect_custom_key_store::{builders::*, *};
use aws_sdk_kms::operation::enable_key::{builders::*, *};
use aws_sdk_kms::operation::enable_key_rotation::{builders::*, *};
use aws_sdk_kms::operation::encrypt::{builders::*, *};
use aws_sdk_kms::operation::generate_data_key::{builders::*, *};
use aws_sdk_kms::operation::generate_data_key_pair::{builders::*, *};
use aws_sdk_kms::operation::generate_data_key_pair_without_plaintext::{builders::*, *};
use aws_sdk_kms::operation::generate_data_key_without_plaintext::{builders::*, *};
use aws_sdk_kms::operation::generate_mac::{builders::*, *};
use aws_sdk_kms::operation::generate_random::{builders::*, *};
use aws_sdk_kms::operation::get_key_policy::{builders::*, *};
use aws_sdk_kms::operation::get_key_rotation_status::{builders::*, *};
use aws_sdk_kms::operation::get_parameters_for_import::{builders::*, *};
use aws_sdk_kms::operation::get_public_key::{builders::*, *};
use aws_sdk_kms::operation::import_key_material::{builders::*, *};
use aws_sdk_kms::operation::list_aliases::{builders::*, *};
use aws_sdk_kms::operation::list_grants::{builders::*, *};
use aws_sdk_kms::operation::list_key_policies::{builders::*, *};
use aws_sdk_kms::operation::list_key_rotations::{builders::*, *};
use aws_sdk_kms::operation::list_keys::{builders::*, *};
use aws_sdk_kms::operation::list_resource_tags::{builders::*, *};
use aws_sdk_kms::operation::list_retirable_grants::{builders::*, *};
use aws_sdk_kms::operation::put_key_policy::{builders::*, *};
use aws_sdk_kms::operation::re_encrypt::{builders::*, *};
use aws_sdk_kms::operation::replicate_key::{builders::*, *};
use aws_sdk_kms::operation::retire_grant::{builders::*, *};
use aws_sdk_kms::operation::revoke_grant::{builders::*, *};
use aws_sdk_kms::operation::rotate_key_on_demand::{builders::*, *};
use aws_sdk_kms::operation::schedule_key_deletion::{builders::*, *};
use aws_sdk_kms::operation::sign::{builders::*, *};
use aws_sdk_kms::operation::tag_resource::{builders::*, *};
use aws_sdk_kms::operation::untag_resource::{builders::*, *};
use aws_sdk_kms::operation::update_alias::{builders::*, *};
use aws_sdk_kms::operation::update_custom_key_store::{builders::*, *};
use aws_sdk_kms::operation::update_key_description::{builders::*, *};
use aws_sdk_kms::operation::update_primary_region::{builders::*, *};
use aws_sdk_kms::operation::verify::{builders::*, *};
use aws_sdk_kms::operation::verify_mac::{builders::*, *};
use aws_sdk_kms::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_kms::Client;
use std::ops::Deref;

pub use aws_sdk_kms::*;

pub struct KMSClientImpl(Client);
impl KMSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait KMSClient {
    fn cancel_key_deletion(&self, builder: CancelKeyDeletionInputBuilder) -> impl Future<Output = Result<CancelKeyDeletionOutput, SdkError<CancelKeyDeletionError>>> + Send;
    fn connect_custom_key_store(&self, builder: ConnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<ConnectCustomKeyStoreOutput, SdkError<ConnectCustomKeyStoreError>>> + Send;
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>> + Send;
    fn create_custom_key_store(&self, builder: CreateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<CreateCustomKeyStoreOutput, SdkError<CreateCustomKeyStoreError>>> + Send;
    fn create_grant(&self, builder: CreateGrantInputBuilder) -> impl Future<Output = Result<CreateGrantOutput, SdkError<CreateGrantError>>> + Send;
    fn create_key(&self, builder: CreateKeyInputBuilder) -> impl Future<Output = Result<CreateKeyOutput, SdkError<CreateKeyError>>> + Send;
    fn decrypt(&self, builder: DecryptInputBuilder) -> impl Future<Output = Result<DecryptOutput, SdkError<DecryptError>>> + Send;
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>> + Send;
    fn delete_custom_key_store(&self, builder: DeleteCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DeleteCustomKeyStoreOutput, SdkError<DeleteCustomKeyStoreError>>> + Send;
    fn delete_imported_key_material(&self, builder: DeleteImportedKeyMaterialInputBuilder) -> impl Future<Output = Result<DeleteImportedKeyMaterialOutput, SdkError<DeleteImportedKeyMaterialError>>> + Send;
    fn derive_shared_secret(&self, builder: DeriveSharedSecretInputBuilder) -> impl Future<Output = Result<DeriveSharedSecretOutput, SdkError<DeriveSharedSecretError>>> + Send;
    fn describe_custom_key_stores(&self, builder: DescribeCustomKeyStoresInputBuilder) -> impl Future<Output = Result<DescribeCustomKeyStoresOutput, SdkError<DescribeCustomKeyStoresError>>> + Send;
    fn describe_key(&self, builder: DescribeKeyInputBuilder) -> impl Future<Output = Result<DescribeKeyOutput, SdkError<DescribeKeyError>>> + Send;
    fn disable_key(&self, builder: DisableKeyInputBuilder) -> impl Future<Output = Result<DisableKeyOutput, SdkError<DisableKeyError>>> + Send;
    fn disable_key_rotation(&self, builder: DisableKeyRotationInputBuilder) -> impl Future<Output = Result<DisableKeyRotationOutput, SdkError<DisableKeyRotationError>>> + Send;
    fn disconnect_custom_key_store(&self, builder: DisconnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DisconnectCustomKeyStoreOutput, SdkError<DisconnectCustomKeyStoreError>>> + Send;
    fn enable_key(&self, builder: EnableKeyInputBuilder) -> impl Future<Output = Result<EnableKeyOutput, SdkError<EnableKeyError>>> + Send;
    fn enable_key_rotation(&self, builder: EnableKeyRotationInputBuilder) -> impl Future<Output = Result<EnableKeyRotationOutput, SdkError<EnableKeyRotationError>>> + Send;
    fn encrypt(&self, builder: EncryptInputBuilder) -> impl Future<Output = Result<EncryptOutput, SdkError<EncryptError>>> + Send;
    fn generate_data_key(&self, builder: GenerateDataKeyInputBuilder) -> impl Future<Output = Result<GenerateDataKeyOutput, SdkError<GenerateDataKeyError>>> + Send;
    fn generate_data_key_pair(&self, builder: GenerateDataKeyPairInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairOutput, SdkError<GenerateDataKeyPairError>>> + Send;
    fn generate_data_key_pair_without_plaintext(&self, builder: GenerateDataKeyPairWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairWithoutPlaintextOutput, SdkError<GenerateDataKeyPairWithoutPlaintextError>>> + Send;
    fn generate_data_key_without_plaintext(&self, builder: GenerateDataKeyWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyWithoutPlaintextOutput, SdkError<GenerateDataKeyWithoutPlaintextError>>> + Send;
    fn generate_mac(&self, builder: GenerateMacInputBuilder) -> impl Future<Output = Result<GenerateMacOutput, SdkError<GenerateMacError>>> + Send;
    fn generate_random(&self, builder: GenerateRandomInputBuilder) -> impl Future<Output = Result<GenerateRandomOutput, SdkError<GenerateRandomError>>> + Send;
    fn get_key_policy(&self, builder: GetKeyPolicyInputBuilder) -> impl Future<Output = Result<GetKeyPolicyOutput, SdkError<GetKeyPolicyError>>> + Send;
    fn get_key_rotation_status(&self, builder: GetKeyRotationStatusInputBuilder) -> impl Future<Output = Result<GetKeyRotationStatusOutput, SdkError<GetKeyRotationStatusError>>> + Send;
    fn get_parameters_for_import(&self, builder: GetParametersForImportInputBuilder) -> impl Future<Output = Result<GetParametersForImportOutput, SdkError<GetParametersForImportError>>> + Send;
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> + Send;
    fn import_key_material(&self, builder: ImportKeyMaterialInputBuilder) -> impl Future<Output = Result<ImportKeyMaterialOutput, SdkError<ImportKeyMaterialError>>> + Send;
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> + Send;
    fn list_grants(&self, builder: ListGrantsInputBuilder) -> impl Future<Output = Result<ListGrantsOutput, SdkError<ListGrantsError>>> + Send;
    fn list_key_policies(&self, builder: ListKeyPoliciesInputBuilder) -> impl Future<Output = Result<ListKeyPoliciesOutput, SdkError<ListKeyPoliciesError>>> + Send;
    fn list_key_rotations(&self, builder: ListKeyRotationsInputBuilder) -> impl Future<Output = Result<ListKeyRotationsOutput, SdkError<ListKeyRotationsError>>> + Send;
    fn list_keys(&self, builder: ListKeysInputBuilder) -> impl Future<Output = Result<ListKeysOutput, SdkError<ListKeysError>>> + Send;
    fn list_resource_tags(&self, builder: ListResourceTagsInputBuilder) -> impl Future<Output = Result<ListResourceTagsOutput, SdkError<ListResourceTagsError>>> + Send;
    fn list_retirable_grants(&self, builder: ListRetirableGrantsInputBuilder) -> impl Future<Output = Result<ListRetirableGrantsOutput, SdkError<ListRetirableGrantsError>>> + Send;
    fn put_key_policy(&self, builder: PutKeyPolicyInputBuilder) -> impl Future<Output = Result<PutKeyPolicyOutput, SdkError<PutKeyPolicyError>>> + Send;
    fn re_encrypt(&self, builder: ReEncryptInputBuilder) -> impl Future<Output = Result<ReEncryptOutput, SdkError<ReEncryptError>>> + Send;
    fn replicate_key(&self, builder: ReplicateKeyInputBuilder) -> impl Future<Output = Result<ReplicateKeyOutput, SdkError<ReplicateKeyError>>> + Send;
    fn retire_grant(&self, builder: RetireGrantInputBuilder) -> impl Future<Output = Result<RetireGrantOutput, SdkError<RetireGrantError>>> + Send;
    fn revoke_grant(&self, builder: RevokeGrantInputBuilder) -> impl Future<Output = Result<RevokeGrantOutput, SdkError<RevokeGrantError>>> + Send;
    fn rotate_key_on_demand(&self, builder: RotateKeyOnDemandInputBuilder) -> impl Future<Output = Result<RotateKeyOnDemandOutput, SdkError<RotateKeyOnDemandError>>> + Send;
    fn schedule_key_deletion(&self, builder: ScheduleKeyDeletionInputBuilder) -> impl Future<Output = Result<ScheduleKeyDeletionOutput, SdkError<ScheduleKeyDeletionError>>> + Send;
    fn sign(&self, builder: SignInputBuilder) -> impl Future<Output = Result<SignOutput, SdkError<SignError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>> + Send;
    fn update_custom_key_store(&self, builder: UpdateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<UpdateCustomKeyStoreOutput, SdkError<UpdateCustomKeyStoreError>>> + Send;
    fn update_key_description(&self, builder: UpdateKeyDescriptionInputBuilder) -> impl Future<Output = Result<UpdateKeyDescriptionOutput, SdkError<UpdateKeyDescriptionError>>> + Send;
    fn update_primary_region(&self, builder: UpdatePrimaryRegionInputBuilder) -> impl Future<Output = Result<UpdatePrimaryRegionOutput, SdkError<UpdatePrimaryRegionError>>> + Send;
    fn verify(&self, builder: VerifyInputBuilder) -> impl Future<Output = Result<VerifyOutput, SdkError<VerifyError>>> + Send;
    fn verify_mac(&self, builder: VerifyMacInputBuilder) -> impl Future<Output = Result<VerifyMacOutput, SdkError<VerifyMacError>>> + Send;
}
impl KMSClient for KMSClientImpl {
    fn cancel_key_deletion(&self, builder: CancelKeyDeletionInputBuilder) -> impl Future<Output = Result<CancelKeyDeletionOutput, SdkError<CancelKeyDeletionError>>> {
        builder.send_with(&self.0)
    }
    fn connect_custom_key_store(&self, builder: ConnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<ConnectCustomKeyStoreOutput, SdkError<ConnectCustomKeyStoreError>>> {
        builder.send_with(&self.0)
    }
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_key_store(&self, builder: CreateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<CreateCustomKeyStoreOutput, SdkError<CreateCustomKeyStoreError>>> {
        builder.send_with(&self.0)
    }
    fn create_grant(&self, builder: CreateGrantInputBuilder) -> impl Future<Output = Result<CreateGrantOutput, SdkError<CreateGrantError>>> {
        builder.send_with(&self.0)
    }
    fn create_key(&self, builder: CreateKeyInputBuilder) -> impl Future<Output = Result<CreateKeyOutput, SdkError<CreateKeyError>>> {
        builder.send_with(&self.0)
    }
    fn decrypt(&self, builder: DecryptInputBuilder) -> impl Future<Output = Result<DecryptOutput, SdkError<DecryptError>>> {
        builder.send_with(&self.0)
    }
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_key_store(&self, builder: DeleteCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DeleteCustomKeyStoreOutput, SdkError<DeleteCustomKeyStoreError>>> {
        builder.send_with(&self.0)
    }
    fn delete_imported_key_material(&self, builder: DeleteImportedKeyMaterialInputBuilder) -> impl Future<Output = Result<DeleteImportedKeyMaterialOutput, SdkError<DeleteImportedKeyMaterialError>>> {
        builder.send_with(&self.0)
    }
    fn derive_shared_secret(&self, builder: DeriveSharedSecretInputBuilder) -> impl Future<Output = Result<DeriveSharedSecretOutput, SdkError<DeriveSharedSecretError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_key_stores(&self, builder: DescribeCustomKeyStoresInputBuilder) -> impl Future<Output = Result<DescribeCustomKeyStoresOutput, SdkError<DescribeCustomKeyStoresError>>> {
        builder.send_with(&self.0)
    }
    fn describe_key(&self, builder: DescribeKeyInputBuilder) -> impl Future<Output = Result<DescribeKeyOutput, SdkError<DescribeKeyError>>> {
        builder.send_with(&self.0)
    }
    fn disable_key(&self, builder: DisableKeyInputBuilder) -> impl Future<Output = Result<DisableKeyOutput, SdkError<DisableKeyError>>> {
        builder.send_with(&self.0)
    }
    fn disable_key_rotation(&self, builder: DisableKeyRotationInputBuilder) -> impl Future<Output = Result<DisableKeyRotationOutput, SdkError<DisableKeyRotationError>>> {
        builder.send_with(&self.0)
    }
    fn disconnect_custom_key_store(&self, builder: DisconnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DisconnectCustomKeyStoreOutput, SdkError<DisconnectCustomKeyStoreError>>> {
        builder.send_with(&self.0)
    }
    fn enable_key(&self, builder: EnableKeyInputBuilder) -> impl Future<Output = Result<EnableKeyOutput, SdkError<EnableKeyError>>> {
        builder.send_with(&self.0)
    }
    fn enable_key_rotation(&self, builder: EnableKeyRotationInputBuilder) -> impl Future<Output = Result<EnableKeyRotationOutput, SdkError<EnableKeyRotationError>>> {
        builder.send_with(&self.0)
    }
    fn encrypt(&self, builder: EncryptInputBuilder) -> impl Future<Output = Result<EncryptOutput, SdkError<EncryptError>>> {
        builder.send_with(&self.0)
    }
    fn generate_data_key(&self, builder: GenerateDataKeyInputBuilder) -> impl Future<Output = Result<GenerateDataKeyOutput, SdkError<GenerateDataKeyError>>> {
        builder.send_with(&self.0)
    }
    fn generate_data_key_pair(&self, builder: GenerateDataKeyPairInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairOutput, SdkError<GenerateDataKeyPairError>>> {
        builder.send_with(&self.0)
    }
    fn generate_data_key_pair_without_plaintext(&self, builder: GenerateDataKeyPairWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairWithoutPlaintextOutput, SdkError<GenerateDataKeyPairWithoutPlaintextError>>> {
        builder.send_with(&self.0)
    }
    fn generate_data_key_without_plaintext(&self, builder: GenerateDataKeyWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyWithoutPlaintextOutput, SdkError<GenerateDataKeyWithoutPlaintextError>>> {
        builder.send_with(&self.0)
    }
    fn generate_mac(&self, builder: GenerateMacInputBuilder) -> impl Future<Output = Result<GenerateMacOutput, SdkError<GenerateMacError>>> {
        builder.send_with(&self.0)
    }
    fn generate_random(&self, builder: GenerateRandomInputBuilder) -> impl Future<Output = Result<GenerateRandomOutput, SdkError<GenerateRandomError>>> {
        builder.send_with(&self.0)
    }
    fn get_key_policy(&self, builder: GetKeyPolicyInputBuilder) -> impl Future<Output = Result<GetKeyPolicyOutput, SdkError<GetKeyPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_key_rotation_status(&self, builder: GetKeyRotationStatusInputBuilder) -> impl Future<Output = Result<GetKeyRotationStatusOutput, SdkError<GetKeyRotationStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_parameters_for_import(&self, builder: GetParametersForImportInputBuilder) -> impl Future<Output = Result<GetParametersForImportOutput, SdkError<GetParametersForImportError>>> {
        builder.send_with(&self.0)
    }
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn import_key_material(&self, builder: ImportKeyMaterialInputBuilder) -> impl Future<Output = Result<ImportKeyMaterialOutput, SdkError<ImportKeyMaterialError>>> {
        builder.send_with(&self.0)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_grants(&self, builder: ListGrantsInputBuilder) -> impl Future<Output = Result<ListGrantsOutput, SdkError<ListGrantsError>>> {
        builder.send_with(&self.0)
    }
    fn list_key_policies(&self, builder: ListKeyPoliciesInputBuilder) -> impl Future<Output = Result<ListKeyPoliciesOutput, SdkError<ListKeyPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_key_rotations(&self, builder: ListKeyRotationsInputBuilder) -> impl Future<Output = Result<ListKeyRotationsOutput, SdkError<ListKeyRotationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_keys(&self, builder: ListKeysInputBuilder) -> impl Future<Output = Result<ListKeysOutput, SdkError<ListKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_tags(&self, builder: ListResourceTagsInputBuilder) -> impl Future<Output = Result<ListResourceTagsOutput, SdkError<ListResourceTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_retirable_grants(&self, builder: ListRetirableGrantsInputBuilder) -> impl Future<Output = Result<ListRetirableGrantsOutput, SdkError<ListRetirableGrantsError>>> {
        builder.send_with(&self.0)
    }
    fn put_key_policy(&self, builder: PutKeyPolicyInputBuilder) -> impl Future<Output = Result<PutKeyPolicyOutput, SdkError<PutKeyPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn re_encrypt(&self, builder: ReEncryptInputBuilder) -> impl Future<Output = Result<ReEncryptOutput, SdkError<ReEncryptError>>> {
        builder.send_with(&self.0)
    }
    fn replicate_key(&self, builder: ReplicateKeyInputBuilder) -> impl Future<Output = Result<ReplicateKeyOutput, SdkError<ReplicateKeyError>>> {
        builder.send_with(&self.0)
    }
    fn retire_grant(&self, builder: RetireGrantInputBuilder) -> impl Future<Output = Result<RetireGrantOutput, SdkError<RetireGrantError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_grant(&self, builder: RevokeGrantInputBuilder) -> impl Future<Output = Result<RevokeGrantOutput, SdkError<RevokeGrantError>>> {
        builder.send_with(&self.0)
    }
    fn rotate_key_on_demand(&self, builder: RotateKeyOnDemandInputBuilder) -> impl Future<Output = Result<RotateKeyOnDemandOutput, SdkError<RotateKeyOnDemandError>>> {
        builder.send_with(&self.0)
    }
    fn schedule_key_deletion(&self, builder: ScheduleKeyDeletionInputBuilder) -> impl Future<Output = Result<ScheduleKeyDeletionOutput, SdkError<ScheduleKeyDeletionError>>> {
        builder.send_with(&self.0)
    }
    fn sign(&self, builder: SignInputBuilder) -> impl Future<Output = Result<SignOutput, SdkError<SignError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn update_custom_key_store(&self, builder: UpdateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<UpdateCustomKeyStoreOutput, SdkError<UpdateCustomKeyStoreError>>> {
        builder.send_with(&self.0)
    }
    fn update_key_description(&self, builder: UpdateKeyDescriptionInputBuilder) -> impl Future<Output = Result<UpdateKeyDescriptionOutput, SdkError<UpdateKeyDescriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_primary_region(&self, builder: UpdatePrimaryRegionInputBuilder) -> impl Future<Output = Result<UpdatePrimaryRegionOutput, SdkError<UpdatePrimaryRegionError>>> {
        builder.send_with(&self.0)
    }
    fn verify(&self, builder: VerifyInputBuilder) -> impl Future<Output = Result<VerifyOutput, SdkError<VerifyError>>> {
        builder.send_with(&self.0)
    }
    fn verify_mac(&self, builder: VerifyMacInputBuilder) -> impl Future<Output = Result<VerifyMacOutput, SdkError<VerifyMacError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> KMSClient for T
where T: Deref,
      T::Target: KMSClient {
    fn cancel_key_deletion(&self, builder: CancelKeyDeletionInputBuilder) -> impl Future<Output = Result<CancelKeyDeletionOutput, SdkError<CancelKeyDeletionError>>> {
        self.deref().cancel_key_deletion(builder)
    }
    fn connect_custom_key_store(&self, builder: ConnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<ConnectCustomKeyStoreOutput, SdkError<ConnectCustomKeyStoreError>>> {
        self.deref().connect_custom_key_store(builder)
    }
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>> {
        self.deref().create_alias(builder)
    }
    fn create_custom_key_store(&self, builder: CreateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<CreateCustomKeyStoreOutput, SdkError<CreateCustomKeyStoreError>>> {
        self.deref().create_custom_key_store(builder)
    }
    fn create_grant(&self, builder: CreateGrantInputBuilder) -> impl Future<Output = Result<CreateGrantOutput, SdkError<CreateGrantError>>> {
        self.deref().create_grant(builder)
    }
    fn create_key(&self, builder: CreateKeyInputBuilder) -> impl Future<Output = Result<CreateKeyOutput, SdkError<CreateKeyError>>> {
        self.deref().create_key(builder)
    }
    fn decrypt(&self, builder: DecryptInputBuilder) -> impl Future<Output = Result<DecryptOutput, SdkError<DecryptError>>> {
        self.deref().decrypt(builder)
    }
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>> {
        self.deref().delete_alias(builder)
    }
    fn delete_custom_key_store(&self, builder: DeleteCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DeleteCustomKeyStoreOutput, SdkError<DeleteCustomKeyStoreError>>> {
        self.deref().delete_custom_key_store(builder)
    }
    fn delete_imported_key_material(&self, builder: DeleteImportedKeyMaterialInputBuilder) -> impl Future<Output = Result<DeleteImportedKeyMaterialOutput, SdkError<DeleteImportedKeyMaterialError>>> {
        self.deref().delete_imported_key_material(builder)
    }
    fn derive_shared_secret(&self, builder: DeriveSharedSecretInputBuilder) -> impl Future<Output = Result<DeriveSharedSecretOutput, SdkError<DeriveSharedSecretError>>> {
        self.deref().derive_shared_secret(builder)
    }
    fn describe_custom_key_stores(&self, builder: DescribeCustomKeyStoresInputBuilder) -> impl Future<Output = Result<DescribeCustomKeyStoresOutput, SdkError<DescribeCustomKeyStoresError>>> {
        self.deref().describe_custom_key_stores(builder)
    }
    fn describe_key(&self, builder: DescribeKeyInputBuilder) -> impl Future<Output = Result<DescribeKeyOutput, SdkError<DescribeKeyError>>> {
        self.deref().describe_key(builder)
    }
    fn disable_key(&self, builder: DisableKeyInputBuilder) -> impl Future<Output = Result<DisableKeyOutput, SdkError<DisableKeyError>>> {
        self.deref().disable_key(builder)
    }
    fn disable_key_rotation(&self, builder: DisableKeyRotationInputBuilder) -> impl Future<Output = Result<DisableKeyRotationOutput, SdkError<DisableKeyRotationError>>> {
        self.deref().disable_key_rotation(builder)
    }
    fn disconnect_custom_key_store(&self, builder: DisconnectCustomKeyStoreInputBuilder) -> impl Future<Output = Result<DisconnectCustomKeyStoreOutput, SdkError<DisconnectCustomKeyStoreError>>> {
        self.deref().disconnect_custom_key_store(builder)
    }
    fn enable_key(&self, builder: EnableKeyInputBuilder) -> impl Future<Output = Result<EnableKeyOutput, SdkError<EnableKeyError>>> {
        self.deref().enable_key(builder)
    }
    fn enable_key_rotation(&self, builder: EnableKeyRotationInputBuilder) -> impl Future<Output = Result<EnableKeyRotationOutput, SdkError<EnableKeyRotationError>>> {
        self.deref().enable_key_rotation(builder)
    }
    fn encrypt(&self, builder: EncryptInputBuilder) -> impl Future<Output = Result<EncryptOutput, SdkError<EncryptError>>> {
        self.deref().encrypt(builder)
    }
    fn generate_data_key(&self, builder: GenerateDataKeyInputBuilder) -> impl Future<Output = Result<GenerateDataKeyOutput, SdkError<GenerateDataKeyError>>> {
        self.deref().generate_data_key(builder)
    }
    fn generate_data_key_pair(&self, builder: GenerateDataKeyPairInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairOutput, SdkError<GenerateDataKeyPairError>>> {
        self.deref().generate_data_key_pair(builder)
    }
    fn generate_data_key_pair_without_plaintext(&self, builder: GenerateDataKeyPairWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyPairWithoutPlaintextOutput, SdkError<GenerateDataKeyPairWithoutPlaintextError>>> {
        self.deref().generate_data_key_pair_without_plaintext(builder)
    }
    fn generate_data_key_without_plaintext(&self, builder: GenerateDataKeyWithoutPlaintextInputBuilder) -> impl Future<Output = Result<GenerateDataKeyWithoutPlaintextOutput, SdkError<GenerateDataKeyWithoutPlaintextError>>> {
        self.deref().generate_data_key_without_plaintext(builder)
    }
    fn generate_mac(&self, builder: GenerateMacInputBuilder) -> impl Future<Output = Result<GenerateMacOutput, SdkError<GenerateMacError>>> {
        self.deref().generate_mac(builder)
    }
    fn generate_random(&self, builder: GenerateRandomInputBuilder) -> impl Future<Output = Result<GenerateRandomOutput, SdkError<GenerateRandomError>>> {
        self.deref().generate_random(builder)
    }
    fn get_key_policy(&self, builder: GetKeyPolicyInputBuilder) -> impl Future<Output = Result<GetKeyPolicyOutput, SdkError<GetKeyPolicyError>>> {
        self.deref().get_key_policy(builder)
    }
    fn get_key_rotation_status(&self, builder: GetKeyRotationStatusInputBuilder) -> impl Future<Output = Result<GetKeyRotationStatusOutput, SdkError<GetKeyRotationStatusError>>> {
        self.deref().get_key_rotation_status(builder)
    }
    fn get_parameters_for_import(&self, builder: GetParametersForImportInputBuilder) -> impl Future<Output = Result<GetParametersForImportOutput, SdkError<GetParametersForImportError>>> {
        self.deref().get_parameters_for_import(builder)
    }
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> {
        self.deref().get_public_key(builder)
    }
    fn import_key_material(&self, builder: ImportKeyMaterialInputBuilder) -> impl Future<Output = Result<ImportKeyMaterialOutput, SdkError<ImportKeyMaterialError>>> {
        self.deref().import_key_material(builder)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        self.deref().list_aliases(builder)
    }
    fn list_grants(&self, builder: ListGrantsInputBuilder) -> impl Future<Output = Result<ListGrantsOutput, SdkError<ListGrantsError>>> {
        self.deref().list_grants(builder)
    }
    fn list_key_policies(&self, builder: ListKeyPoliciesInputBuilder) -> impl Future<Output = Result<ListKeyPoliciesOutput, SdkError<ListKeyPoliciesError>>> {
        self.deref().list_key_policies(builder)
    }
    fn list_key_rotations(&self, builder: ListKeyRotationsInputBuilder) -> impl Future<Output = Result<ListKeyRotationsOutput, SdkError<ListKeyRotationsError>>> {
        self.deref().list_key_rotations(builder)
    }
    fn list_keys(&self, builder: ListKeysInputBuilder) -> impl Future<Output = Result<ListKeysOutput, SdkError<ListKeysError>>> {
        self.deref().list_keys(builder)
    }
    fn list_resource_tags(&self, builder: ListResourceTagsInputBuilder) -> impl Future<Output = Result<ListResourceTagsOutput, SdkError<ListResourceTagsError>>> {
        self.deref().list_resource_tags(builder)
    }
    fn list_retirable_grants(&self, builder: ListRetirableGrantsInputBuilder) -> impl Future<Output = Result<ListRetirableGrantsOutput, SdkError<ListRetirableGrantsError>>> {
        self.deref().list_retirable_grants(builder)
    }
    fn put_key_policy(&self, builder: PutKeyPolicyInputBuilder) -> impl Future<Output = Result<PutKeyPolicyOutput, SdkError<PutKeyPolicyError>>> {
        self.deref().put_key_policy(builder)
    }
    fn re_encrypt(&self, builder: ReEncryptInputBuilder) -> impl Future<Output = Result<ReEncryptOutput, SdkError<ReEncryptError>>> {
        self.deref().re_encrypt(builder)
    }
    fn replicate_key(&self, builder: ReplicateKeyInputBuilder) -> impl Future<Output = Result<ReplicateKeyOutput, SdkError<ReplicateKeyError>>> {
        self.deref().replicate_key(builder)
    }
    fn retire_grant(&self, builder: RetireGrantInputBuilder) -> impl Future<Output = Result<RetireGrantOutput, SdkError<RetireGrantError>>> {
        self.deref().retire_grant(builder)
    }
    fn revoke_grant(&self, builder: RevokeGrantInputBuilder) -> impl Future<Output = Result<RevokeGrantOutput, SdkError<RevokeGrantError>>> {
        self.deref().revoke_grant(builder)
    }
    fn rotate_key_on_demand(&self, builder: RotateKeyOnDemandInputBuilder) -> impl Future<Output = Result<RotateKeyOnDemandOutput, SdkError<RotateKeyOnDemandError>>> {
        self.deref().rotate_key_on_demand(builder)
    }
    fn schedule_key_deletion(&self, builder: ScheduleKeyDeletionInputBuilder) -> impl Future<Output = Result<ScheduleKeyDeletionOutput, SdkError<ScheduleKeyDeletionError>>> {
        self.deref().schedule_key_deletion(builder)
    }
    fn sign(&self, builder: SignInputBuilder) -> impl Future<Output = Result<SignOutput, SdkError<SignError>>> {
        self.deref().sign(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>> {
        self.deref().update_alias(builder)
    }
    fn update_custom_key_store(&self, builder: UpdateCustomKeyStoreInputBuilder) -> impl Future<Output = Result<UpdateCustomKeyStoreOutput, SdkError<UpdateCustomKeyStoreError>>> {
        self.deref().update_custom_key_store(builder)
    }
    fn update_key_description(&self, builder: UpdateKeyDescriptionInputBuilder) -> impl Future<Output = Result<UpdateKeyDescriptionOutput, SdkError<UpdateKeyDescriptionError>>> {
        self.deref().update_key_description(builder)
    }
    fn update_primary_region(&self, builder: UpdatePrimaryRegionInputBuilder) -> impl Future<Output = Result<UpdatePrimaryRegionOutput, SdkError<UpdatePrimaryRegionError>>> {
        self.deref().update_primary_region(builder)
    }
    fn verify(&self, builder: VerifyInputBuilder) -> impl Future<Output = Result<VerifyOutput, SdkError<VerifyError>>> {
        self.deref().verify(builder)
    }
    fn verify_mac(&self, builder: VerifyMacInputBuilder) -> impl Future<Output = Result<VerifyMacOutput, SdkError<VerifyMacError>>> {
        self.deref().verify_mac(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edKMSClient {}
    impl KMSClient for edKMSClient {
        async fn cancel_key_deletion(&self, builder: CancelKeyDeletionInputBuilder) -> Result<CancelKeyDeletionOutput, SdkError<CancelKeyDeletionError>>;
        async fn connect_custom_key_store(&self, builder: ConnectCustomKeyStoreInputBuilder) -> Result<ConnectCustomKeyStoreOutput, SdkError<ConnectCustomKeyStoreError>>;
        async fn create_alias(&self, builder: CreateAliasInputBuilder) -> Result<CreateAliasOutput, SdkError<CreateAliasError>>;
        async fn create_custom_key_store(&self, builder: CreateCustomKeyStoreInputBuilder) -> Result<CreateCustomKeyStoreOutput, SdkError<CreateCustomKeyStoreError>>;
        async fn create_grant(&self, builder: CreateGrantInputBuilder) -> Result<CreateGrantOutput, SdkError<CreateGrantError>>;
        async fn create_key(&self, builder: CreateKeyInputBuilder) -> Result<CreateKeyOutput, SdkError<CreateKeyError>>;
        async fn decrypt(&self, builder: DecryptInputBuilder) -> Result<DecryptOutput, SdkError<DecryptError>>;
        async fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> Result<DeleteAliasOutput, SdkError<DeleteAliasError>>;
        async fn delete_custom_key_store(&self, builder: DeleteCustomKeyStoreInputBuilder) -> Result<DeleteCustomKeyStoreOutput, SdkError<DeleteCustomKeyStoreError>>;
        async fn delete_imported_key_material(&self, builder: DeleteImportedKeyMaterialInputBuilder) -> Result<DeleteImportedKeyMaterialOutput, SdkError<DeleteImportedKeyMaterialError>>;
        async fn derive_shared_secret(&self, builder: DeriveSharedSecretInputBuilder) -> Result<DeriveSharedSecretOutput, SdkError<DeriveSharedSecretError>>;
        async fn describe_custom_key_stores(&self, builder: DescribeCustomKeyStoresInputBuilder) -> Result<DescribeCustomKeyStoresOutput, SdkError<DescribeCustomKeyStoresError>>;
        async fn describe_key(&self, builder: DescribeKeyInputBuilder) -> Result<DescribeKeyOutput, SdkError<DescribeKeyError>>;
        async fn disable_key(&self, builder: DisableKeyInputBuilder) -> Result<DisableKeyOutput, SdkError<DisableKeyError>>;
        async fn disable_key_rotation(&self, builder: DisableKeyRotationInputBuilder) -> Result<DisableKeyRotationOutput, SdkError<DisableKeyRotationError>>;
        async fn disconnect_custom_key_store(&self, builder: DisconnectCustomKeyStoreInputBuilder) -> Result<DisconnectCustomKeyStoreOutput, SdkError<DisconnectCustomKeyStoreError>>;
        async fn enable_key(&self, builder: EnableKeyInputBuilder) -> Result<EnableKeyOutput, SdkError<EnableKeyError>>;
        async fn enable_key_rotation(&self, builder: EnableKeyRotationInputBuilder) -> Result<EnableKeyRotationOutput, SdkError<EnableKeyRotationError>>;
        async fn encrypt(&self, builder: EncryptInputBuilder) -> Result<EncryptOutput, SdkError<EncryptError>>;
        async fn generate_data_key(&self, builder: GenerateDataKeyInputBuilder) -> Result<GenerateDataKeyOutput, SdkError<GenerateDataKeyError>>;
        async fn generate_data_key_pair(&self, builder: GenerateDataKeyPairInputBuilder) -> Result<GenerateDataKeyPairOutput, SdkError<GenerateDataKeyPairError>>;
        async fn generate_data_key_pair_without_plaintext(&self, builder: GenerateDataKeyPairWithoutPlaintextInputBuilder) -> Result<GenerateDataKeyPairWithoutPlaintextOutput, SdkError<GenerateDataKeyPairWithoutPlaintextError>>;
        async fn generate_data_key_without_plaintext(&self, builder: GenerateDataKeyWithoutPlaintextInputBuilder) -> Result<GenerateDataKeyWithoutPlaintextOutput, SdkError<GenerateDataKeyWithoutPlaintextError>>;
        async fn generate_mac(&self, builder: GenerateMacInputBuilder) -> Result<GenerateMacOutput, SdkError<GenerateMacError>>;
        async fn generate_random(&self, builder: GenerateRandomInputBuilder) -> Result<GenerateRandomOutput, SdkError<GenerateRandomError>>;
        async fn get_key_policy(&self, builder: GetKeyPolicyInputBuilder) -> Result<GetKeyPolicyOutput, SdkError<GetKeyPolicyError>>;
        async fn get_key_rotation_status(&self, builder: GetKeyRotationStatusInputBuilder) -> Result<GetKeyRotationStatusOutput, SdkError<GetKeyRotationStatusError>>;
        async fn get_parameters_for_import(&self, builder: GetParametersForImportInputBuilder) -> Result<GetParametersForImportOutput, SdkError<GetParametersForImportError>>;
        async fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>;
        async fn import_key_material(&self, builder: ImportKeyMaterialInputBuilder) -> Result<ImportKeyMaterialOutput, SdkError<ImportKeyMaterialError>>;
        async fn list_aliases(&self, builder: ListAliasesInputBuilder) -> Result<ListAliasesOutput, SdkError<ListAliasesError>>;
        async fn list_grants(&self, builder: ListGrantsInputBuilder) -> Result<ListGrantsOutput, SdkError<ListGrantsError>>;
        async fn list_key_policies(&self, builder: ListKeyPoliciesInputBuilder) -> Result<ListKeyPoliciesOutput, SdkError<ListKeyPoliciesError>>;
        async fn list_key_rotations(&self, builder: ListKeyRotationsInputBuilder) -> Result<ListKeyRotationsOutput, SdkError<ListKeyRotationsError>>;
        async fn list_keys(&self, builder: ListKeysInputBuilder) -> Result<ListKeysOutput, SdkError<ListKeysError>>;
        async fn list_resource_tags(&self, builder: ListResourceTagsInputBuilder) -> Result<ListResourceTagsOutput, SdkError<ListResourceTagsError>>;
        async fn list_retirable_grants(&self, builder: ListRetirableGrantsInputBuilder) -> Result<ListRetirableGrantsOutput, SdkError<ListRetirableGrantsError>>;
        async fn put_key_policy(&self, builder: PutKeyPolicyInputBuilder) -> Result<PutKeyPolicyOutput, SdkError<PutKeyPolicyError>>;
        async fn re_encrypt(&self, builder: ReEncryptInputBuilder) -> Result<ReEncryptOutput, SdkError<ReEncryptError>>;
        async fn replicate_key(&self, builder: ReplicateKeyInputBuilder) -> Result<ReplicateKeyOutput, SdkError<ReplicateKeyError>>;
        async fn retire_grant(&self, builder: RetireGrantInputBuilder) -> Result<RetireGrantOutput, SdkError<RetireGrantError>>;
        async fn revoke_grant(&self, builder: RevokeGrantInputBuilder) -> Result<RevokeGrantOutput, SdkError<RevokeGrantError>>;
        async fn rotate_key_on_demand(&self, builder: RotateKeyOnDemandInputBuilder) -> Result<RotateKeyOnDemandOutput, SdkError<RotateKeyOnDemandError>>;
        async fn schedule_key_deletion(&self, builder: ScheduleKeyDeletionInputBuilder) -> Result<ScheduleKeyDeletionOutput, SdkError<ScheduleKeyDeletionError>>;
        async fn sign(&self, builder: SignInputBuilder) -> Result<SignOutput, SdkError<SignError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_alias(&self, builder: UpdateAliasInputBuilder) -> Result<UpdateAliasOutput, SdkError<UpdateAliasError>>;
        async fn update_custom_key_store(&self, builder: UpdateCustomKeyStoreInputBuilder) -> Result<UpdateCustomKeyStoreOutput, SdkError<UpdateCustomKeyStoreError>>;
        async fn update_key_description(&self, builder: UpdateKeyDescriptionInputBuilder) -> Result<UpdateKeyDescriptionOutput, SdkError<UpdateKeyDescriptionError>>;
        async fn update_primary_region(&self, builder: UpdatePrimaryRegionInputBuilder) -> Result<UpdatePrimaryRegionOutput, SdkError<UpdatePrimaryRegionError>>;
        async fn verify(&self, builder: VerifyInputBuilder) -> Result<VerifyOutput, SdkError<VerifyError>>;
        async fn verify_mac(&self, builder: VerifyMacInputBuilder) -> Result<VerifyMacOutput, SdkError<VerifyMacError>>;
    }
}
