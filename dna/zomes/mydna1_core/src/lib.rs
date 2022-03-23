use std::u8;

pub use hdk::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////
/// old entry_defs! macro should be able to be deleted because the 
/// `hdk_entry` proc macro will create the `EntryTypes` enum
/// entry_defs![MyThing1::entry_def(), MyThing2::entry_def()];

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////

#[hdk_entry]
#[derive(Clone)]
pub struct MyThing1 {
    pub thing1: String,
}

#[hdk_entry]
#[derive(Clone)]
pub struct MyThing2 {
    pub thing2: String,
}
impl MyThing2 {
    pub fn some_fn() {
        debug!("Do something")
    }
}

// entry_types! macro generates the enum below and impls to go in the opposite direction
entry_types!([MyThing1, MyThing2]);

// pub enum EntryTypes {
//     #[MyThing1]
//     MyEntry1 = 0,
//     #[MyThing2]
//     MyEntry2
// }


////////////////////////////////////////////////////////////////////////////////
// Link Types
////////////////////////////////////////////////////////////////////////////////

// link_types! macro generates the enum below and impls to go in the opposite direction
link_types!([Fish, Dog, Cow])
pub enum LinkTypes {
    Fish = 0,
    Dog,
    Cow
}
impl From<LinkType> for LinkTypes {
    fn from(x: LinkType) -> Self {
        match x.0 {
            0 => LinkTypes::Fish,
            1 => LinkTypes::Dog,
            2 => LinkTypes::Cow,
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
// Validation callback
////////////////////////////////////////////////////////////////////////////////

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
 
    match op {
        // Validation for entries
        Op::StoreEntry { header, entry_type, .. } => {
            match entry_type {
                MyThing1 => todo!(),
                MyThing2 => todo!(),
            }
        },
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid(
            "updating entries isn't valid".to_string(),
        )),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),

        // Validation for links
        Op::RegisterCreateLink { create_link: _, link_type: _ } => {
            match link_type {
                LinkTypes::Fish => _,
                LinkTypes::Dog => _,
                LinkTypes::Cow => _,
            }
            Ok(ValidateCallbackResult::Valid)}
            ,
        Op::RegisterDeleteLink { delete_link: _, create_link: _, link_type: _ } => {
            match link_type {
                LinkTypes::Fish => _,
                LinkTypes::Dog => _,
                LinkTypes::Cow => _,
            }        
            Ok(ValidateCallbackResult::Valid)
        },

        // Validation for elements based on header type
        Op::StoreElement { element } => {
            match element.header() {
                // Validate agent joining the network
                Header::AgentKey(_) => todo!(),

                // Validate entries
                Header::Create(create) => match create.app_entry_type {
                    EntryTypes::MyThing1 => todo!(),
                    EntryTypes::MyThing2 => todo!(),
                },
                Header::Update(_) => todo!(),
                Header::Delete(_) => todo!(),

                // Validate Links
                Header::CreateLink(_) => todo!(),
                Header::DeleteLink(_) => todo!(),

                // Validation chain migration
                Header::OpenChain(_) => todo!(),
                Header::CloseChain(_) => todo!(),

                // Validate capabilities, rarely used
                Header::CapGrant() => todo!(), 
                Header::CapClaim() => todo!(),

                // Validate init and genesis entries, also rarely 
                Header::InitZomesComplete(_)=>todo!(),
                Header::AgentValidationPkg(_)=>todo!(), // mostly this will be validated in the process of using it to validate the Agent Key
                Header::Dna(_)=>todo!(),
            };
            Ok(ValidateCallbackResult::Valid)
        },

        // Agent joining network validation
        // this is a new DHT op
        Op::RegisterAgent { header, agent_pub_key } => {
            // get validation package and then do stuff
            Ok(ValidateCallbackResult::Valid)
        },
        // Chain structure validation
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }

    // this is what we currently have to do to make things work
    let info = zome_info()?;
    match op {
        Op::StoreElement { element } => {
            match element.header() {
                Header::Dna(_) => todo!(),
                Header::AgentValidationPkg(_) => todo!(),
                Header::InitZomesComplete(_) => todo!(),
                Header::CreateLink(create) => match create.link_type.into() {
                    LinkTypes::Fish => todo!(),
                    _ => {}
                },
                Header::DeleteLink(_) => todo!(),
                Header::OpenChain(_) => todo!(),
                Header::CloseChain(_) => todo!(),
                Header::Create(create) => match create.entry_type {
                    EntryType::AgentPubKey => todo!(),
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "my_entry1" => _
                        }
                    }
                    EntryType::CapClaim => todo!(),
                    EntryType::CapGrant => todo!(),
                },
                Header::Update(_) => todo!(),
                Header::Delete(_) => todo!(),
            }
            Ok(ValidateCallbackResult::Valid)
        }
        Op::StoreEntry { header, .. } => {
            match header.hashed.content.entry_type() {
                entry_def_index!(String::from("my_entry1")) => todo!(),
                _ => {}
            }
            Ok(ValidateCallbackResult::Valid)
        }
        Op::RegisterCreateLink { create_link: _ } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { create_link: _, .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting links isn't valid".to_string(),
        )),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid(
            "updating entries isn't valid".to_string(),
        )),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
}
