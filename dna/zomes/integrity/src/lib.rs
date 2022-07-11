use holochain_deterministic_integrity::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////
/// old entry_defs! macro should be able to be deleted because the
/// `hdk_entry` proc macro will create the `EntryTypes` enum
/// entry_defs![MyThing1::entry_def(), MyThing2::entry_def()];

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////

#[hdk_entry_helper]
pub struct MyThing1 {
    pub thing1: String,
}

#[hdk_entry_helper]
pub struct MyThing2 {
    pub thing2: String,
}

impl MyThing2 {
    pub fn some_fn() {}
}

#[hdk_entry_helper]
pub struct MyThingPrivate {
    pub private_thing: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    MyThing1(MyThing1),
    MyThing2(MyThing2),
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
    Cow,
}

impl From<LinkType> for LinkTypes {
    fn from(x: LinkType) -> Self {
        match x.0 {
            1 => LinkTypes::Dog,
            2 => LinkTypes::Cow,
            _ => LinkTypes::Fish,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Genesis self-check callback
////////////////////////////////////////////////////////////////////////////////

#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
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
    match op {
        Op::StoreEntry {
            action:
                SignedHashed {
                    hashed:
                        HoloHashed {
                            content: action, ..
                        },
                    ..
                },
            entry,
        } => action
            .app_entry_type()
            .map(|AppEntryType { id, zome_id, .. }| (zome_id, id))
            .map_or(Ok(ValidateCallbackResult::Valid), |(zome_id, id)| {
                match EntryTypes::deserialize_from_type(*zome_id, *id, &entry)? {
                    Some(EntryTypes::MyThing1(my_thing1)) if my_thing1.thing1 == "Banana" => {
                        Ok(ValidateCallbackResult::Invalid("No Bananas!".to_string()))
                    }
                    Some(EntryTypes::MyThing2(my_thing2)) if my_thing2.thing2 == "Carrot" => {
                        Ok(ValidateCallbackResult::Invalid("Carrots no".to_string()))
                    }
                    _ => Ok(ValidateCallbackResult::Valid),
                }
            }),
        _ => Ok(ValidateCallbackResult::Invalid("no way hosay".to_string())),
    }

    //     Op::RegisterUpdate { .. } => {
    //         return Ok(ValidateCallbackResult::Invalid(
    //             "updating entries isn't valid".to_string(),
    //         ))
    //     }
    //     Op::RegisterDelete { .. } => {
    //         return Ok(ValidateCallbackResult::Invalid(
    //             "deleting entries isn't valid".to_string(),
    //         ))
    //     }

    //     // Validation for links
    //     Op::RegisterCreateLink { create_link } => {
    //         let (create_link, _) = create_link.hashed.into_inner();
    //         match create_link.link_type.into() {
    //             LinkTypes::Fish => todo!(),
    //             LinkTypes::Dog => todo!(),
    //             LinkTypes::Cow => todo!(),
    //         }
    //     }
    //     Op::RegisterDeleteLink {
    //         delete_link: _,
    //         create_link,
    //     } => match create_link.link_type.into() {
    //         LinkTypes::Fish => todo!(),
    //         LinkTypes::Dog => todo!(),
    //         LinkTypes::Cow => todo!(),
    //     },

    //     // Validation for records based on action type
    //     Op::StoreRecord { record } => {
    //         match record.action() {
    //             // Validate agent joining the network
    //             Action::AgentValidationPkg(_) => todo!(),

    //             // Validate entries
    //             // Action::Create(create) => match create.entry_type {
    //             //     EntryTypes::MyThing1 => todo!(),
    //             //     EntryTypes::MyThing2 => todo!(),
    //             // EntryTypes::MyThingPrivate => todo!(),
    //             // },
    //             Action::Update(_) => todo!(),
    //             Action::Delete(_) => todo!(),

    //             // Validate Links
    //             Action::CreateLink(_) => todo!(),
    //             Action::DeleteLink(_) => todo!(),

    //             // Validation chain migration
    //             Action::OpenChain(_) => todo!(),
    //             Action::CloseChain(_) => todo!(),

    //             // Validate capabilities, rarely used
    //             Action::Create(_) => todo!(),
    //             // Action::CapClaim() => todo!(),

    //             // Validate init and genesis entries, also rarely
    //             Action::InitZomesComplete(_) => todo!(),
    //             // Action::AgentValidationPkg(_) => todo!(), // mostly this will be validated in the process of using it to validate the Agent Key
    //             Action::Dna(_) => todo!(),
    //         };
    //     }

    //     // Agent joining network validation
    //     // this is a new DHT op
    //     // Op::RegisterAgent { action, agent_pub_key } => {
    //     //     // get validation package and then do stuff
    //     //  //   Ok(ValidateCallbackResult::Valid)
    //     // },
    //     // Chain structure validation
    //     Op::RegisterAgentActivity { .. } => todo!(), // Ok(ValidateCallbackResult::Valid),
    // }

    // // this is what we currently have to do to make things work
    // // let info = zome_info()?;
    // match op {
    //     Op::StoreRecord { record } => {
    //         match record.action() {
    //             Action::Dna(_) => todo!(),
    //             Action::AgentValidationPkg(_) => todo!(),
    //             Action::InitZomesComplete(_) => todo!(),
    //             Action::CreateLink(create) => match create.link_type.into() {
    //                 LinkTypes::Fish => todo!(),
    //                 _ => {}
    //             },
    //             Action::DeleteLink(_) => todo!(),
    //             Action::OpenChain(_) => todo!(),
    //             Action::CloseChain(_) => todo!(),
    //             Action::Create(create) => match create.entry_type {
    //                 EntryType::AgentPubKey => todo!(),
    //                 // EntryType::App(app_entry_type) => {
    //                 //     match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
    //                 //         "my_entry1" => _
    //                 //     }
    //                 // }
    //                 EntryType::App(_) => todo!(),
    //                 EntryType::CapClaim => todo!(),
    //                 EntryType::CapGrant => todo!(),
    //             },
    //             Action::Update(_) => todo!(),
    //             Action::Delete(_) => todo!(),
    //         }
    //         Ok(ValidateCallbackResult::Valid)
    //     }
    //     // Op::StoreEntry { action, .. } => {
    //     //     match action.hashed.content.entry_type() {
    //     //         entry_def_index!(String::from("my_entry1")) => todo!(),
    //     //         _ => {}
    //     //     }
    //     //     Ok(ValidateCallbackResult::Valid)
    //     // }
    //     Op::StoreEntry { action: _, .. } => todo!(),
    //     Op::RegisterCreateLink { create_link: _ } => Ok(ValidateCallbackResult::Valid),
    //     Op::RegisterDeleteLink { create_link: _, .. } => Ok(ValidateCallbackResult::Invalid(
    //         "deleting links isn't valid".to_string(),
    //     )),
    //     Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid(
    //         "updating entries isn't valid".to_string(),
    //     )),
    //     Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid(
    //         "deleting entries isn't valid".to_string(),
    //     )),
    //     Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    // }
}