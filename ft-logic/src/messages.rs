use crate::H256;
use ft_storage_io::{FTStorageAction, FTStorageEvent};
use gstd::{msg, ActorId};

pub async fn increase_balance(
    transaction_hash: H256,
    storage_id: &ActorId,
    account: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, FTStorageEvent>(
        *storage_id,
        FTStorageAction::IncreaseBalance {
            transaction_hash,
            account: *account,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `FTStorageAction::IncreaseBalance`")
    .await;
    match result {
        Ok(storage_event) => match storage_event {
            FTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn decrease_balance(
    transaction_hash: H256,
    storage_id: &ActorId,
    msg_source: &ActorId,
    account: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, FTStorageEvent>(
        *storage_id,
        FTStorageAction::DecreaseBalance {
            transaction_hash,
            msg_source: *msg_source,
            account: *account,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `FTStorageAction::DecreaseBalance`")
    .await;
    match result {
        Ok(storage_event) => match storage_event {
            FTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn approve(
    transaction_hash: H256,
    storage_id: &ActorId,
    msg_source: &ActorId,
    account: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, FTStorageEvent>(
        *storage_id,
        FTStorageAction::Approve {
            transaction_hash,
            msg_source: *msg_source,
            account: *account,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `FTStorageAction::DecreaseBalance`")
    .await;
    match result {
        Ok(storage_event) => match storage_event {
            FTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn get_balance(storage_id: &ActorId, account: &ActorId) -> u128 {
    let reply = msg::send_for_reply_as::<_, FTStorageEvent>(
        *storage_id,
        FTStorageAction::GetBalance(*account),
        0,
    )
    .expect("Error in sending a message `FTStorageAction::GetBalance")
    .await
    .expect("Unable to decode `FTStorageEvent");
    if let FTStorageEvent::Balance(balance) = reply {
        balance
    } else {
        0
    }
}
