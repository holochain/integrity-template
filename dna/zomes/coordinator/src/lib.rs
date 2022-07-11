use hdk::prelude::*;
use integrity::{EntryTypes, MyThing1};

#[hdk_extern]
pub fn create(_: ()) -> ExternResult<()> {
    let _header_hash = create_entry(EntryTypes::MyThing1(MyThing1 {
        thing1: String::from("Banana"),
    }));
    Ok(())
}
