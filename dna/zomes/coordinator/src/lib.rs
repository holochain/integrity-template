use integrity::hdk::prelude::*;
use integrity::{EntryTypes, MyThing};

#[hdk_extern]
pub fn create(content: String) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::MyThing1(MyThing {
        thing1: content.clone(),
    }))?;
    Ok(action_hash)
}

pub fn update(entry_hash: ActionHash, content: String) -> ExternResult<ActionHash> {
    let action_hash = update_entry(
        entry_hash,
        EntryTypes::MyThing1(MyThing {
            thing1: content.clone(),
        }),
    )?;
    Ok(action_hash)
}
