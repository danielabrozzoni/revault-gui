use std::sync::Arc;
use std::time::Instant;

use super::error::Error;
use crate::revaultd::{
    model::{Vault, VaultTransactions},
    RevaultD, RevaultDError,
};

#[derive(Debug, Clone)]
pub enum Message {
    Install,
    Syncing(Result<f64, RevaultDError>),
    Synced(Arc<RevaultD>),
    Tick(Instant),
    DaemonStarted(Result<Arc<RevaultD>, Error>),
    Vaults(Result<Vec<Vault>, RevaultDError>),
    VaultTransactions(Result<Vec<VaultTransactions>, RevaultDError>),
    SelectVault(String),
    BlockHeight(Result<u64, RevaultDError>),
    Connected(Result<Arc<RevaultD>, Error>),
    Menu(MessageMenu),
}

#[derive(Debug, Clone)]
pub enum MessageMenu {
    Home,
    History,
}
