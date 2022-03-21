use std::u8;

pub use hdk::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////
entry_defs![MyEntry1::entry_def(), MyEntry2::entry_def()];

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
// Link Types
////////////////////////////////////////////////////////////////////////////////

pub enum MyLink1 {
    Fish = 0,
    Dog,
    Cow
}
impl From<LinkType> for MyLink1 {
    fn from(x: LinkType) -> Self {
        match x.0 {
            0 => MyLink1::Fish,
            1 => MyLink1::Dog,
            2 => MyLink1::Cow,
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
                    MyLink1::Fish => todo!(),
                    _ => {}
                },
                Header::DeleteLink(_) => todo!(),
                Header::OpenChain(_) => todo!(),
                Header::CloseChain(_) => todo!(),
                Header::Create(create) => match create.entry_type {
                    EntryType::AgentPubKey => todo!(),
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "something" => _
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
                entry_def_index!(String::from("somethings")) => todo!(),
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
