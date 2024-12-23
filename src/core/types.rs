use core::fmt;
use informalsystems_malachitebft_core_types::{self, Context, SignedMessage, SigningProvider};
use informalsystems_malachitebft_core_types::{
    Extension, NilOrVal, Round, SignedProposal, SignedProposalPart, SignedVote, Validator,
    VoteType, VotingPower,
};
use libp2p::identity::ed25519::Keypair;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use tracing::warn;

pub use crate::proto; // TODO: reconsider how this is imported

use crate::proto::full_proposal::ProposedValue;
use crate::proto::{Block, FullProposal, ShardChunk};
pub use proto::Height;
pub use proto::ShardHash;

pub const FARCASTER_EPOCH: u64 = 1609459200; // January 1, 2021 UTC

// Fid must be a 32 bit unsigned integer for storage in RocksDB and the trie.
// However, protobuf uses 64 bit unsigned integers. So, map to the fid at the lowest level
pub type FidOnDisk = u32;

pub trait ShardId
where
    Self: Sized + Clone + Send + Sync + 'static,
{
    fn new(id: u32) -> Self;
    fn shard_id(&self) -> u32;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub struct SnapchainShard(u32);

impl ShardId for SnapchainShard {
    fn new(id: u32) -> Self {
        Self(id)
    }
    fn shard_id(&self) -> u32 {
        self.0
    }
}

pub trait ShardedContext {
    type ShardId: ShardId;
}

pub trait SnapchainContext:
    informalsystems_malachitebft_core_types::Context + ShardedContext
{
}

// TODO: Should validator keys be ECDSA?
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&vec);
        Self(bytes)
    }

    pub fn prefix(&self) -> String {
        format!("0x{}", &self.to_hex()[0..4])
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({})", self)
    }
}

impl informalsystems_malachitebft_core_types::Address for Address {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ed25519 {}

#[derive(Debug)]
pub struct Ed25519Provider {
    private_key: PrivateKey,
}

impl Ed25519Provider {
    pub fn new(private_key: PrivateKey) -> Self {
        Self { private_key }
    }
}

impl SigningProvider<SnapchainValidatorContext> for Ed25519Provider {
    fn sign_vote(
        &self,
        vote: <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Vote,
    ) -> SignedMessage<
        SnapchainValidatorContext,
        <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Vote,
    > {
        todo!()
    }

    fn verify_signed_vote(
        &self,
        vote: &<SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Vote,
        signature: &informalsystems_malachitebft_core_types::Signature<SnapchainValidatorContext>,
        public_key: &informalsystems_malachitebft_core_types::PublicKey<SnapchainValidatorContext>,
    ) -> bool {
        todo!()
    }

    fn sign_proposal(
        &self,
        proposal: <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Proposal,
    ) -> SignedMessage<
        SnapchainValidatorContext,
        <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Proposal,
    > {
        todo!()
    }

    fn verify_signed_proposal(
        &self,
        proposal: &<SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Proposal,
        signature: &informalsystems_malachitebft_core_types::Signature<SnapchainValidatorContext>,
        public_key: &informalsystems_malachitebft_core_types::PublicKey<SnapchainValidatorContext>,
    ) -> bool {
        todo!()
    }

    fn sign_proposal_part(
        &self,
        proposal_part: <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::ProposalPart,
    ) -> SignedMessage<SnapchainValidatorContext, <SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::ProposalPart>{
        todo!()
    }

    fn verify_signed_proposal_part(
        &self,
        proposal_part: &<SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::ProposalPart,
        signature: &informalsystems_malachitebft_core_types::Signature<SnapchainValidatorContext>,
        public_key: &informalsystems_malachitebft_core_types::PublicKey<SnapchainValidatorContext>,
    ) -> bool {
        todo!()
    }

    fn verify_commit_signature(
        &self,
        certificate: &informalsystems_malachitebft_core_types::CommitCertificate<
            SnapchainValidatorContext,
        >,
        commit_sig: &informalsystems_malachitebft_core_types::CommitSignature<
            SnapchainValidatorContext,
        >,
        validator: &<SnapchainValidatorContext as informalsystems_malachitebft_core_types::Context>::Validator,
    ) -> Result<
        VotingPower,
        informalsystems_malachitebft_core_types::CertificateError<SnapchainValidatorContext>,
    > {
        todo!()
    }
}

pub struct InvalidSignatureError();
impl fmt::Display for InvalidSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid signature")
    }
}

// Ed25519 signature
// Todo: Do we need the consensus-critical version? https://github.com/penumbra-zone/ed25519-consensus
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signature(pub Vec<u8>);
pub type PublicKey = libp2p::identity::ed25519::PublicKey;
pub type PrivateKey = libp2p::identity::ed25519::SecretKey;

impl informalsystems_malachitebft_core_types::SigningScheme for Ed25519 {
    type DecodingError = InvalidSignatureError;
    type Signature = Signature;
    type PublicKey = PublicKey;
    type PrivateKey = PrivateKey;

    fn decode_signature(_bytes: &[u8]) -> Result<Self::Signature, Self::DecodingError> {
        todo!()
    }

    fn encode_signature(_signature: &Self::Signature) -> Vec<u8> {
        todo!()
    }
}

// Blake3 20-byte hashes (same as Message/sync trie)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash([u8; 20]);

impl Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", hex::encode(&self.0))
    }
}

impl Height {
    pub const fn new(shard_index: u32, block_number: u64) -> Self {
        Self {
            shard_index,
            block_number,
        }
    }

    pub const fn as_u64(&self) -> u64 {
        self.block_number
    }

    pub const fn increment(&self) -> Self {
        self.increment_by(1)
    }

    pub const fn increment_by(&self, n: u64) -> Self {
        Self {
            shard_index: self.shard_index,
            block_number: self.block_number + n,
        }
    }

    pub fn decrement(&self) -> Option<Self> {
        self.block_number.checked_sub(1).map(|block_number| Self {
            shard_index: self.shard_index,
            block_number,
        })
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.shard_index, self.block_number)
    }
}

impl informalsystems_malachitebft_core_types::Height for Height {
    fn increment(&self) -> Self {
        self.increment()
    }

    fn as_u64(&self) -> u64 {
        self.block_number
    }

    fn increment_by(&self, n: u64) -> Self {
        todo!()
    }

    fn decrement_by(&self, n: u64) -> Option<Self> {
        todo!()
    }
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct ShardHash {
//     shard_index: u8,
//     hash: Hash,
// }

impl fmt::Display for ShardHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {:?}", self.shard_index, hex::encode(&self.hash))
    }
}

// impl ShardHash {
//     pub fn new(shard_id: u8, hash: Hash) -> Self {
//         Self { shard_id, hash }
//     }
// }

impl informalsystems_malachitebft_core_types::Value for ShardHash {
    type Id = ShardHash;

    fn id(&self) -> Self::Id {
        self.clone()
    }
}

impl FullProposal {
    pub fn shard_hash(&self) -> ShardHash {
        match &self.proposed_value {
            Some(ProposedValue::Block(block)) => ShardHash {
                shard_index: self.height().shard_index as u32,
                hash: block.hash.clone(),
            },
            Some(ProposedValue::Shard(shard_chunk)) => ShardHash {
                shard_index: self.height().shard_index as u32,
                hash: shard_chunk.hash.clone(),
            },
            _ => {
                panic!("Invalid proposal type");
            }
        }
    }

    pub fn block(&self) -> Option<Block> {
        match &self.proposed_value {
            Some(ProposedValue::Block(block)) => Some(block.clone()),
            _ => None,
        }
    }

    pub fn shard_chunk(&self) -> Option<&ShardChunk> {
        match &self.proposed_value {
            Some(ProposedValue::Shard(chunk)) => Some(&chunk),
            _ => None,
        }
    }

    pub fn proposer_address(&self) -> Address {
        Address::from_vec(self.proposer.clone())
    }

    pub fn height(&self) -> Height {
        self.height.clone().unwrap()
    }

    pub fn round(&self) -> Round {
        Round::new(self.round.try_into().unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SnapchainValidator {
    pub shard_index: u32,
    pub address: Address,
    pub public_key: PublicKey,
    pub rpc_address: Option<String>,
    pub current_height: u64,
}

impl SnapchainValidator {
    pub fn new(
        shard_index: SnapchainShard,
        public_key: PublicKey,
        rpc_address: Option<String>,
        current_height: u64,
    ) -> Self {
        Self {
            shard_index: shard_index.shard_id(),
            address: Address(public_key.to_bytes()),
            public_key,
            rpc_address,
            current_height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapchainValidatorSet {
    pub validators: Vec<SnapchainValidator>,
}

impl SnapchainValidatorSet {
    pub fn new(validators: Vec<SnapchainValidator>) -> Self {
        let mut set = Self { validators: vec![] };
        for validator in validators {
            set.add(validator);
        }
        set
    }

    pub fn add(&mut self, validator: SnapchainValidator) -> bool {
        if self.exists(&validator.address) {
            return false;
        }

        if self.validators.is_empty() || self.validators[0].shard_index == validator.shard_index {
            self.validators.push(validator);
            // Ensure validators are in the same order on all nodes
            self.validators.sort();
            true
        } else {
            // TODO: This should fail loudly
            false
        }
    }

    pub fn exists(&self, address: &Address) -> bool {
        self.validators.iter().any(|v| v.address == *address)
    }

    pub fn shard_id(&self) -> u32 {
        if self.validators.is_empty() {
            0
        } else {
            self.validators[0].shard_index
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vote {
    pub vote_type: VoteType,
    pub height: Height,
    pub round: Round,
    pub shard_hash: NilOrVal<ShardHash>,
    pub voter: Address,
    pub extension: Option<Extension>,
}

impl Vote {
    pub fn new_prevote(
        height: Height,
        round: Round,
        block_hash: NilOrVal<ShardHash>,
        voter: Address,
    ) -> Self {
        Self {
            vote_type: VoteType::Prevote,
            height,
            round,
            shard_hash: block_hash,
            voter,
            extension: None,
        }
    }

    pub fn new_precommit(
        height: Height,
        round: Round,
        value: NilOrVal<ShardHash>,
        address: Address,
    ) -> Self {
        Self {
            vote_type: VoteType::Precommit,
            height,
            round,
            shard_hash: value,
            voter: address,
            extension: None,
        }
    }

    pub fn new_precommit_with_extension(
        height: Height,
        round: Round,
        value: NilOrVal<ShardHash>,
        address: Address,
        extension: Extension,
    ) -> Self {
        Self {
            vote_type: VoteType::Precommit,
            height,
            round,
            shard_hash: value,
            voter: address,
            extension: Some(extension),
        }
    }

    pub fn to_proto(&self) -> proto::Vote {
        let vote_type = match self.vote_type {
            VoteType::Prevote => proto::VoteType::Prevote,
            VoteType::Precommit => proto::VoteType::Precommit,
        };
        let shard_hash = match &self.shard_hash {
            NilOrVal::Nil => None,
            NilOrVal::Val(shard_hash) => Some(shard_hash.clone()),
        };
        proto::Vote {
            height: Some(self.height.clone()),
            round: self.round.as_i64(),
            voter: self.voter.to_vec(),
            r#type: vote_type as i32,
            value: shard_hash,
        }
    }

    pub fn from_proto(proto: proto::Vote) -> Self {
        let vote_type = match proto.r#type {
            0 => VoteType::Prevote,
            1 => VoteType::Precommit,
            _ => panic!("Invalid vote type"),
        };
        let shard_hash = match proto.value {
            None => NilOrVal::Nil,
            Some(value) => NilOrVal::Val(value),
        };
        Self {
            vote_type,
            height: proto.height.unwrap(),
            round: Round::new(proto.round.try_into().unwrap()),
            voter: Address::from_vec(proto.voter),
            shard_hash,
            extension: None,
        }
    }

    pub fn to_sign_bytes(&self) -> Vec<u8> {
        self.to_proto().encode_to_vec()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub height: Height,
    pub round: Round,
    pub shard_hash: ShardHash,
    pub pol_round: Round,
    pub proposer: Address,
}

impl Proposal {
    pub fn to_proto(&self) -> proto::Proposal {
        proto::Proposal {
            height: Some(self.height),
            round: self.round.as_i64(),
            proposer: self.proposer.to_vec(),
            value: Some(self.shard_hash.clone()),
            pol_round: self.pol_round.as_i64(),
        }
    }

    pub fn from_proto(proto: proto::Proposal) -> Self {
        Self {
            height: proto.height.unwrap(),
            round: Round::new(proto.round.try_into().unwrap()),
            shard_hash: proto.value.unwrap(),
            pol_round: Round::new(proto.pol_round.try_into().unwrap()),
            proposer: Address::from_vec(proto.proposer),
        }
    }
    pub fn to_sign_bytes(&self) -> Vec<u8> {
        // TODO: Should we be signing the hash?
        self.to_proto().encode_to_vec()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SinglePartProposal {
    pub height: Height,
    pub proposal_round: Round,
    pub proposer: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalPart {
    FullProposal(SinglePartProposal),
}

#[derive(Clone, Debug)]
pub struct SnapchainValidatorContext {
    keypair: Arc<Keypair>,
}

impl SnapchainValidatorContext {
    pub fn new(keypair: Keypair) -> Self {
        Self {
            keypair: Arc::new(keypair),
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public()
    }
}

impl ShardedContext for SnapchainValidatorContext {
    type ShardId = SnapchainShard;
}

impl informalsystems_malachitebft_core_types::Context for SnapchainValidatorContext {
    type Address = Address;
    type Height = Height;
    type ProposalPart = ProposalPart;
    type Proposal = Proposal;
    type Validator = SnapchainValidator;
    type ValidatorSet = SnapchainValidatorSet;
    type Value = ShardHash;
    type Vote = Vote;
    type SigningScheme = Ed25519;
    type SigningProvider = Ed25519Provider;

    fn select_proposer<'a>(
        &self,
        validator_set: &'a Self::ValidatorSet,
        height: Self::Height,
        round: Round,
    ) -> &'a Self::Validator {
        assert!(validator_set.validators.len() > 0);
        assert!(round != Round::Nil && round.as_i64() >= 0);

        let proposer_index = {
            let height = height.as_u64() as usize;
            let round = round.as_i64() as usize;

            (height - 1 + round) % validator_set.validators.len()
        };

        validator_set
            .validators
            .get(proposer_index)
            .expect("proposer_index is valid")
    }

    fn new_proposal(
        height: Height,
        round: Round,
        shard_hash: ShardHash,
        pol_round: Round,
        address: Address,
    ) -> Proposal {
        Proposal {
            height,
            round,
            shard_hash,
            pol_round,
            proposer: address,
        }
    }

    fn new_prevote(
        height: Height,
        round: Round,
        value_id: NilOrVal<ShardHash>,
        address: Address,
    ) -> Vote {
        Vote::new_prevote(height, round, value_id, address)
    }

    fn new_precommit(
        height: Height,
        round: Round,
        value_id: NilOrVal<ShardHash>,
        address: Address,
    ) -> Vote {
        Vote::new_precommit(height, round, value_id, address)
    }

    fn signing_provider(&self) -> &Self::SigningProvider {
        todo!()
    }
}

impl SnapchainContext for SnapchainValidatorContext {}

impl informalsystems_malachitebft_core_types::ProposalPart<SnapchainValidatorContext>
    for ProposalPart
{
    fn is_first(&self) -> bool {
        // Only one part for now
        true
    }

    fn is_last(&self) -> bool {
        true
    }
}

impl informalsystems_malachitebft_core_types::Proposal<SnapchainValidatorContext> for Proposal {
    fn height(&self) -> Height {
        self.height
    }

    fn round(&self) -> Round {
        self.round
    }

    fn value(&self) -> &ShardHash {
        &self.shard_hash
    }

    fn take_value(self) -> ShardHash {
        self.shard_hash
    }

    fn pol_round(&self) -> Round {
        self.pol_round
    }

    fn validator_address(&self) -> &Address {
        &self.proposer
    }
}

impl informalsystems_malachitebft_core_types::Vote<SnapchainValidatorContext> for Vote {
    fn height(&self) -> Height {
        self.height
    }

    fn round(&self) -> Round {
        self.round
    }

    fn value(&self) -> &NilOrVal<ShardHash> {
        &self.shard_hash
    }

    fn take_value(self) -> NilOrVal<ShardHash> {
        self.shard_hash
    }

    fn vote_type(&self) -> VoteType {
        self.vote_type
    }

    fn validator_address(&self) -> &Address {
        &self.voter
    }

    fn extension(
        &self,
    ) -> std::option::Option<&SignedMessage<SnapchainValidatorContext, Extension>> {
        None
    }

    fn extend(self, extension: SignedMessage<SnapchainValidatorContext, Extension>) -> Self {
        Self {
            extension: Some(extension.message),
            ..self
        }
    }
}

impl informalsystems_malachitebft_core_types::ValidatorSet<SnapchainValidatorContext>
    for SnapchainValidatorSet
{
    fn count(&self) -> usize {
        self.validators.len()
    }

    fn total_voting_power(&self) -> VotingPower {
        self.validators.iter().map(|v| v.voting_power()).sum()
    }

    fn get_by_address(&self, address: &Address) -> Option<&SnapchainValidator> {
        let option = self.validators.iter().find(|v| &v.address == address);
        if option.is_none() {
            warn!("Validator not found: {}", address);
        }
        option
    }

    fn get_by_index(&self, index: usize) -> Option<&SnapchainValidator> {
        self.validators.get(index)
    }
}

impl informalsystems_malachitebft_core_types::Validator<SnapchainValidatorContext>
    for SnapchainValidator
{
    fn address(&self) -> &Address {
        &self.address
    }

    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    fn voting_power(&self) -> VotingPower {
        1
    }
}
