mod membership;

pub use membership::{list_my_hosts, HostMembership};
mod contact;
mod contact_validation;
mod messages;

pub use contact::{update_contact, HostContact, UpdateHostContact};
