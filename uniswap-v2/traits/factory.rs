use crate::traits::pair::PairError;
use openbrush::traits::AccountId;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn all_pair_length(&self) -> u64;

    #[ink(message)]
    fn create_pair(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, FactoryError>;

    fn _instantiate_pair(&mut self, salt_bytes: &[u8]) -> AccountId;
}

#[openbrush::trait_definition]
pub trait Internal {
    fn _emit_create_pair_event(
        &self,
        _token_0: AccountId,
        _token_1: AccountId,
        _pair: AccountId,
        _pair_len: u64,
    );
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FactoryError {
    PairError(PairError),
    ZeroAddress,
    IdenticalAddresses,
}

impl From<PairError> for FactoryError {
    fn from(error: PairError) -> Self {
        FactoryError::PairError(error)
    }
}
