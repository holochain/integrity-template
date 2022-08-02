#![allow(unused_variables)]

use std::hash::Hash;

pub use hdk;
pub use hdk::hdi;

use hdi::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////
/// old entry_defs! macro should be able to be deleted because the
/// `hdk_entry` proc macro will create the `EntryTypes` enum
/// entry_defs![MyThing::entry_def()];

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////

#[hdk_entry_helper]
pub struct MyThing {
    pub thing1: String,
}

impl MyThing {
    pub fn some_fn() {}
}

#[hdk_entry_helper]
pub struct MyThingPrivate {
    pub private_thing: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    MyThing1(MyThing),
    #[entry_def(visibility = "private")]
    MyThingPrivate(MyThingPrivate),
}

////////////////////////////////////////////////////////////////////////////////
// Link Types
////////////////////////////////////////////////////////////////////////////////

// link_types! macro generates the enum below and impls to go in the opposite direction
#[hdk_link_types]
pub enum LinkTypes {
    Fish,
    Dog,
}

////////////////////////////////////////////////////////////////////////////////
// Genesis self-check callback
////////////////////////////////////////////////////////////////////////////////

#[hdk_extern]
pub fn genesis_self_check(data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    // TODO
    // check data.dna_def
    // check data.membrane_proof
    // check data.agent_key
    Ok(ValidateCallbackResult::Valid)
}

////////////////////////////////////////////////////////////////////////////////
// Validation callback
////////////////////////////////////////////////////////////////////////////////

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // TODO: read the holochain_integrity_types docs to understand which ops yield what
    match op.to_type::<EntryTypes, LinkTypes>().unwrap() {
        OpType::StoreRecord(_) => todo!(),
        OpType::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry {
                entry_hash,
                entry_type,
            }
            | OpEntry::UpdateEntry {
                entry_hash,
                entry_type,
                ..
            } => match entry_type {
                EntryTypes::MyThing1(my_thing1) if my_thing1.thing1 == "invalid text 1" => Ok(
                    ValidateCallbackResult::Invalid("invalid thing1".to_string()),
                ),
                _ => Ok(ValidateCallbackResult::Valid),
            },
            OpEntry::CreateAgent(_) | OpEntry::UpdateAgent { .. } => {
                Ok(ValidateCallbackResult::Valid)
            }
        },
        // this authority has the previous items of the chain. here we introduce rules based on previous actions
        // TODO: show an invalidation use-case or explain why we signal valid by default here
        // TODO: could all cases marked with 'todo!()' really happen here as well?
        OpType::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateEntry { .. } => todo!(),
                OpActivity::CreatePrivateEntry { .. } => todo!(),
                // Agent joining network validation
                OpActivity::CreateAgent(agent_pubkey) => {
                    // we could perform a check on the new agent's pubkey
                }
                OpActivity::CreateCapClaim(_) => todo!(),
                OpActivity::CreateCapGrant(_) => todo!(),
                OpActivity::UpdateEntry { .. } => todo!(),
                OpActivity::UpdatePrivateEntry { .. } => todo!(),
                OpActivity::UpdateAgent { .. } => todo!(),
                OpActivity::UpdateCapClaim { .. } => todo!(),
                OpActivity::UpdateCapGrant { .. } => todo!(),
                OpActivity::DeleteEntry { .. } => todo!(),
                OpActivity::CreateLink { .. } => todo!(),
                OpActivity::DeleteLink(_) => todo!(),
                OpActivity::Dna(_) => todo!(),
                OpActivity::OpenChain(_) => todo!(),
                OpActivity::CloseChain(_) => todo!(),
                OpActivity::AgentValidationPkg(_) => todo!(),
                OpActivity::InitZomesComplete => todo!(),
            }

            Ok(ValidateCallbackResult::Valid)
        }

        // Validation for creating links
        OpType::RegisterCreateLink {
            link_type,
            // base_address,
            // target_address,
            // tag,
            ..
        } => match link_type {
            LinkTypes::Fish => Ok(ValidateCallbackResult::Invalid(
                "fish cannot be linked".to_string(),
            )),
            LinkTypes::Dog => Ok(ValidateCallbackResult::Valid),
        },

        // Validation for deleting links
        OpType::RegisterDeleteLink {
            link_type,
            // original_link_hash,
            // base_address,
            // target_address,
            // tag,
            ..
        } => match link_type {
            LinkTypes::Fish => Ok(ValidateCallbackResult::Invalid(
                "fish cannot be linked".to_string(),
            )),
            LinkTypes::Dog => Ok(ValidateCallbackResult::Valid),
        },

        OpType::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry {
                entry_hash,
                original_action_hash,
                original_entry_hash,
                original_entry_type,
                new_entry_type,
            } => match new_entry_type {
                EntryTypes::MyThing1(my_thing1) if my_thing1.thing1 == "invalid text 1" => Ok(
                    ValidateCallbackResult::Invalid("invalid thing1".to_string()),
                ),
                _ => Ok(ValidateCallbackResult::Valid),
            },
            OpUpdate::PrivateEntry {
                entry_hash,
                original_action_hash,
                original_entry_hash,
                original_entry_type,
                new_entry_type,
            } => todo!(),
            OpUpdate::Agent {
                new_key,
                original_key,
                original_action_hash,
            } => todo!(),
            OpUpdate::CapClaim {
                entry_hash,
                original_action_hash,
                original_entry_hash,
            } => todo!(),
            OpUpdate::CapGrant {
                entry_hash,
                original_action_hash,
                original_entry_hash,
            } => todo!(),
        },

        OpType::RegisterDelete(_) => Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),
    }
}
