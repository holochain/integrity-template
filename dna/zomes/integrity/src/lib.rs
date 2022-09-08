#![allow(unused_variables, unused_imports)]

use std::hash::Hash;

pub use hdk;
pub use hdi;

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
#[derive(Clone)]
pub struct MyThing {
    pub thing: String,
}

impl MyThing {
    pub fn some_fn() {}
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct MyThingPrivate {
    pub private_thing: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
#[derive(Clone)]
pub enum EntryTypes {
    MyThing(MyThing),
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
/// Cases that are covered by the subconscious validation and don't need to be handled here
/// todo: understand and document guarantees
///
///     - hashes and signatures will be checked where the host knows about it
///     - validation is deterministic
///     - errors and timeouts on the host that can be caught and handled unambiguously will be
///
///     - all functions that are callable are deterministic (todo: which ones can be called?)
///     - must_get_entry / must_get_header will either return a result or the wasm call stops entirely
///     - example: wasm that only does introspection on the wasm itself
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // TODO: read the holochain_integrity_types docs to understand which ops yield what
    debug!("Validating integrity-template Op: {:?}", op );
    match op.to_type::<EntryTypes, LinkTypes>()? {
        OpType::StoreRecord(_store_record) => Ok(ValidateCallbackResult::Valid),
        OpType::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry {
                entry_hash,
                entry_type,
            }
            | OpEntry::UpdateEntry {
                entry_hash,
                entry_type,
                ..
            } => {
                // do something with the hash
                // match hdk::prelude::must_get_entry(entry_hash)? {
                //     hdk::prelude::Entry::Agent(_) => todo!(),
                //     hdk::prelude::Entry::App(_) => todo!(),
                //     hdk::prelude::Entry::CounterSign(_, _) => todo!(),
                //     hdk::prelude::Entry::CapClaim(_) => todo!(),
                //     hdk::prelude::Entry::CapGrant(_) => todo!(),
                // };

                match entry_type {
                    EntryTypes::MyThing(my_thing) if my_thing.thing == "invalid text" => {
                        Ok(ValidateCallbackResult::Invalid(my_thing.thing))
                    }
                    _ => Ok(ValidateCallbackResult::Valid),
                }
            }

            OpEntry::CreateAgent(_) | OpEntry::UpdateAgent { .. } => {
                Ok(ValidateCallbackResult::Valid)
            }
        },
        // this authority has the previous items of the chain. here we introduce rules based on
        // previous actions, with local (immediate) access to the source-chain.
        // TODO: show an invalidation use-case or explain why we signal valid by default here
        // TODO: could all cases marked with 'todo!()' really happen here as well?
        OpType::RegisterAgentActivity(agent_activity) => {
	    debug!("- Agent Activity: {:?}", agent_activity );
            match agent_activity {
                // Agent joining network validation
                OpActivity::AgentValidationPkg(_) => todo!(),
                OpActivity::CloseChain(_) => todo!(),
                OpActivity::CreateAgent(agent_pubkey) => {
                    // we could perform a check on the new agent's pubkey
                }
                OpActivity::CreateCapClaim(_) => todo!(),
                OpActivity::CreateCapGrant(_) => todo!(),
                OpActivity::CreateEntry{ entry_hash, entry_type } => match entry_type {
		    // We can check the created entry's type number
		    Some(UnitEntryTypes::MyThing) => (),
		    _ => return Ok(ValidateCallbackResult::Invalid(format!("unknown entry type {:?}", entry_type ))),
		},
                OpActivity::CreatePrivateEntry { .. } => todo!(),
                OpActivity::CreateLink { .. } => todo!(),
                OpActivity::DeleteEntry { .. } => todo!(),
                OpActivity::DeleteLink(_) => todo!(),
                OpActivity::Dna(_) => todo!(),
                OpActivity::InitZomesComplete => {
                    // we could perform an integrity check on the Zome genesis
                },
                OpActivity::OpenChain(_) => todo!(),
                OpActivity::UpdateAgent { .. } => todo!(),
                OpActivity::UpdateCapClaim { .. } => todo!(),
                OpActivity::UpdateCapGrant { .. } => todo!(),
                OpActivity::UpdateEntry { .. } => todo!(),
                OpActivity::UpdatePrivateEntry { .. } => todo!(),
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
                EntryTypes::MyThing(my_thing) if my_thing.thing == "invalid text" => {
                    Ok(ValidateCallbackResult::Invalid("invalid thing".to_string()))
                }
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

#[cfg(test)]
mod tests {
    use hdi::test_utils::short_hand;
    use hdk::prelude::OpEntry;
    use hdk::prelude::{holo_hash::HashableContent, HoloHash, MockHdkT};
    use hdk::{
        hdi::{self, prelude::set_hdi},
        prelude::{AppEntryType, EntryType, EntryVisibility, ValidateCallbackResult},
    };
    use holochain_mock_hdi::MockHdiT;

    #[test]
    fn invalid_entry() {
        // let mut mock_hdk = MockHdkT::new();

        // todo: provoke the invalid response for this match arm

        // OpType::StoreEntry(store_entry) => match store_entry {
        //     OpEntry::CreateEntry {
        //         entry_hash,
        //         entry_type,
        //     }
        //     | OpEntry::UpdateEntry {
        //         entry_hash,
        //         entry_type,
        //         ..
        //     } => match entry_type {
        //         EntryTypes::MyThing(my_thing) if my_thing.thing == "invalid text" => Ok(
        //             ValidateCallbackResult::Invalid("invalid thing".to_string()),
        //         ),
        //         _ => Ok(ValidateCallbackResult::Valid),
        //     },
        //     OpEntry::CreateAgent(_) | OpEntry::UpdateAgent { .. } => {
        //         Ok(ValidateCallbackResult::Valid)
        //     }
        // },

        // todo: invalid update entry

        // invalid create entry
        let e = crate::MyThing {
            thing: "invalid text".to_string(),
        };

        let et = crate::EntryTypes::MyThing(e.clone());

        let invalid_entry = OpEntry::CreateEntry {
            // todo: how can i hash the actual entry?
            entry_hash: short_hand::eh(0),
            entry_type: et,
        };

        let op = short_hand::s_entry(
            short_hand::c(EntryType::App(AppEntryType {
                // todo: can and should this be derived from the data?
                id: 0.into(),
                // todo: can and should this be derived from the data?
                zome_id: 0.into(),
                visibility: EntryVisibility::Public,
            }))
            .into(),
            short_hand::e(e),
        );

        // construct a mocked hdi for unit testing
        // this should evolve with the `validate` function
        let mut mock_hdi = MockHdiT::new();
        mock_hdi.expect_zome_info().return_once({
            // TODO: customise this for the happ?
            move |_input| {
                let zome_types = hdi::prelude::ScopedZomeTypesSet {
                    entries: Default::default(),
                    // entries: hdi::prelude::ScopedZomeTypes(
                    //     [(0, 0)]
                    //         .iter()
                    //         .map(|(z, types)| {
                    //             (
                    //                 hdi::prelude::ZomeId(*z),
                    //                 (0..*types)
                    //                     .map(|t| hdi::prelude::EntryDefIndex(t))
                    //                     .collect(),
                    //             )
                    //         })
                    //         .collect(),
                    // ),
                    links: Default::default(),
                };

                Ok(hdi::prelude::ZomeInfo {
                    name: "integrity".to_string().into(),
                    id: 0.into(),
                    properties: hdi::prelude::UnsafeBytes::from(vec![]).into(),
                    entry_defs: hdi::prelude::EntryDefs(Default::default()),
                    extern_fns: vec![],
                    zome_types,
                })
            }
        });

        set_hdi(mock_hdi);

        match super::validate(op) {
            Ok(ValidateCallbackResult::Invalid(_)) => (),
            other => panic!("invalid entry should cause an error: {:?}", other),
        };
    }
}
