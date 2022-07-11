use hdk::prelude::*;
use integrity::{EntryTypes, MyThing1};

#[hdk_extern]
pub fn create(content: String) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::MyThing1(MyThing1 {
        thing1: content.clone(),
    }))?;
    Ok(action_hash)
}
