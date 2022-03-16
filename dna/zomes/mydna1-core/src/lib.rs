pub use hdk::prelude::*;


////////////////////////////////////////////////////////////////////////////////
/// Entry declarations
////////////////////////////////////////////////////////////////////////////////
entry_defs![
    MyEntry1::entry_def(),
    MyEntry2::entry_def()
];


////////////////////////////////////////////////////////////////////////////////
/// Entry struct definitions with necessary impls
////////////////////////////////////////////////////////////////////////////////
#[hdk_entry(id = "my_entry1")]
#[derive(Clone)]
pub struct MyEntry1 {
    pub thing1 String,
}


#[hdk_entry(id = "my_entry2")]
#[derive(Clone)]
pub struct MyEntry2 {
    pub thing1 String,
}
impl MyEntry2 {
    pub fn some_fn() {
        debug!("Do something")
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Link Types
////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////
/// Validation callback
////////////////////////////////////////////////////////////////////////////////


#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreElement { element: _ } => Ok(ValidateCallbackResult::Valid),
        Op::StoreEntry { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterCreateLink {
            base: _,
            target,
            create_link,
        } => {
            Ok(ValidateCallbackResult::Valid)
        }
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
