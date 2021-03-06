use std::sync::Arc;

use super::{error::Error, menu::Menu};
use crate::revault::Role;
use crate::revaultd::{
    model::{RevocationTransactions, Vault, VaultTransactions},
    RevaultD, RevaultDError,
};

#[derive(Debug, Clone)]
pub enum Message {
    Clipboard(String),
    Install,
    ChangeRole(Role),
    Syncing(Result<f64, RevaultDError>),
    Synced(Arc<RevaultD>),
    DaemonStarted(Result<Arc<RevaultD>, Error>),
    Vaults(Result<Vec<Vault>, RevaultDError>),
    SelectVault(String),
    Vault(VaultMessage),
    BlockHeight(Result<u64, RevaultDError>),
    Connected(Result<Arc<RevaultD>, Error>),
    Menu(Menu),
    Next,
    Previous,
    DepositAddress(Result<bitcoin::Address, RevaultDError>),
    Deposit(usize, DepositMessage),
    Recipient(usize, RecipientMessage),
    Input(usize, InputMessage),
    AddRecipient,
}

#[derive(Debug, Clone)]
pub enum VaultMessage {
    OnChainTransactions(Result<VaultTransactions, RevaultDError>),
}

#[derive(Debug, Clone)]
pub enum SignMessage {
    ChangeMethod,
    Sign,
    Clipboard(String),
    PsbtEdited(String),
}

#[derive(Debug, Clone)]
pub enum DepositMessage {
    RevocationTransactions(Result<RevocationTransactions, RevaultDError>),
    Sign(SignMessage),
    Signed(Result<(), RevaultDError>),
    /// Message ask for Deposit State to retry connecting to revaultd.
    Retry,
}

#[derive(Debug, Clone)]
pub enum InputMessage {
    Selected(bool),
}

#[derive(Debug, Clone)]
pub enum RecipientMessage {
    Delete,
    AddressEdited(String),
    AmountEdited(String),
}
