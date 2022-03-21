use std::u8;

pub use hdk::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////
entry_defs![MyEntry1::entry_def(), MyEntry2::entry_def()];

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////

// OLD
#[hdk_entry(id = "my_entry1")]
#[derive(Clone)]
pub struct MyEntry1 {
    pub thing1: String,
}

#[hdk_entry(id = "my_entry2")]
#[derive(Clone)]
pub struct MyEntry2 {
    pub thing1: String,
}
impl MyEntry2 {
    pub fn some_fn() {
        debug!("Do something")
    }
}

// NEW
#[hdk_entry]
pub enum EntryTypes {
    #[MyEntry1]
    MyEntry1,
    MyEntry2
}


////////////////////////////////////////////////////////////////////////////////
// Link Types
////////////////////////////////////////////////////////////////////////////////

#[hdk_link_type]
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
