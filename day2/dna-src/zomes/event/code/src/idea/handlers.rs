
use hdk::error::ZomeApiResult;
use hdk::AGENT_ADDRESS;
use hdk::holochain_core_types::{
    hash::HashString,
    entry::Entry,
    cas::content::Address,
    json::RawString,
};

use crate::idea::{
    Idea,
};


use crate::utils;
use crate::message;

pub fn handle_create_idea(
    title: String,
    description: String,
    challenge_address: HashString,
) -> ZomeApiResult<Address> {

    let idea = Idea{title, description};

    let entry = Entry::App(
        "idea".into(),
        idea.into()
    );

    let idea_address = hdk::commit_entry(&entry)?;
    utils::link_entries_bidir(&AGENT_ADDRESS, &idea_address, "ideas_authored", "author")?;
    utils::link_entries_bidir(&AGENT_ADDRESS, &idea_address, "member_of", "has_member")?;
    utils::link_entries_bidir(&challenge_address, &idea_address, "ideas_submitted", "related_challenge")?; 

    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("idea").into(),
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &idea_address, "idea")?;
    
    Ok(idea_address)
}

pub fn handle_join_idea(idea_address: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&AGENT_ADDRESS, &idea_address, "member_of", "has_member")?;
    Ok(())
}

pub fn handle_get_all_ideas() -> ZomeApiResult<utils::GetLinksLoadResult<Idea>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("idea").into(),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    utils::get_links_and_load_type(&anchor_address, "idea")
}
