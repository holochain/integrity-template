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

// OLD
#[hdk_entry]
#[derive(Clone)]
pub struct MyThing1 {
    pub thing1: String,
}

#[hdk_entry]
#[derive(Clone)]
pub struct MyThing2 {
    pub thing1: String,
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
          Ok(ValidateCallbackResult::Valid)
        },
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid(
            "updating entries isn't valid".to_string(),
        )),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),

        // Validation for links
        Op::RegisterCreateLink { create_link: _, link_type: _ } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { delete_link: _, create_link: _, link_type: _ } => Ok(ValidateCallbackResult::Invalid(
            "deleting links isn't valid".to_string(),
        )),

        // Validation for elements based on header type
        Op::StoreElement { element } => {
            match element.header() {
                Header::AgentKey(_) => todo!(),
                Header::Create(create) => match create.app_entry_type {
                    EntryTypes::Fish => todo!(),
                    EntryTypes::Dog => todo!(),
                    EntryTypes::Cow => todo!(),
                },
                Header::Update(_) => todo!(),
                Header::Delete(_) => todo!(),
                Header::CreateLink(_) => todo!(),
                Header::DeleteLink(_) => todo!(),
                Header::OpenChain(_) => todo!(),
                Header::CloseChain(_) => todo!(),
                Header::AgentValidationPkg(_)=>todo!(),
                Header::InitZomesComplete(_)=>todo!(),
                Header::Dna(_)=>todo!(),
            };
            Ok(ValidateCallbackResult::Valid)
        },

        // Chain structure validation
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }

    // this is what we currently have to do
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
