/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use super::{Mailbox, MailboxRights, Role};
use crate::{core::get::GetObject, principal::ACL, Get, Set};
use ahash::AHashMap;

impl Mailbox<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> Option<String> {
        self.id.take()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn parent_id(&self) -> Option<Option<String>> {
        self.parent_id.clone()
    }

    pub fn role(&self) -> Option<Role> {
        self.role.clone()
    }

    pub fn sort_order(&self) -> Option<u32> {
        self.sort_order.clone()
    }

    pub fn total_emails(&self) -> Option<usize> {
        self.total_emails.clone()
    }

    pub fn unread_emails(&self) -> Option<usize> {
        self.unread_emails.clone()
    }

    pub fn total_threads(&self) -> Option<usize> {
        self.total_threads.clone()
    }

    pub fn unread_threads(&self) -> Option<usize> {
        self.unread_threads.clone()
    }

    pub fn is_subscribed(&self) -> Option<bool> {
        self.is_subscribed.clone()
    }

    pub fn my_rights(&self) -> Option<&MailboxRights> {
        self.my_rights.as_ref()
    }

    pub fn acl(&self) -> Option<&AHashMap<String, AHashMap<ACL, bool>>> {
        self.share_with.as_ref()
    }

    pub fn take_acl(&mut self) -> Option<AHashMap<String, AHashMap<ACL, bool>>> {
        self.share_with.take()
    }
}

impl MailboxRights {
    pub fn may_read_items(&self) -> bool {
        self.may_read_items
    }

    pub fn may_add_items(&self) -> bool {
        self.may_add_items
    }

    pub fn may_remove_items(&self) -> bool {
        self.may_remove_items
    }

    pub fn may_set_seen(&self) -> bool {
        self.may_set_seen
    }

    pub fn may_set_keywords(&self) -> bool {
        self.may_set_keywords
    }

    pub fn may_create_child(&self) -> bool {
        self.may_create_child
    }

    pub fn may_rename(&self) -> bool {
        self.may_rename
    }

    pub fn may_delete(&self) -> bool {
        self.may_delete
    }

    pub fn may_submit(&self) -> bool {
        self.may_submit
    }

    pub fn acl_list(&self) -> Vec<ACL> {
        let mut acl_list = Vec::new();
        for (is_set, acl) in [
            (self.may_read_items, ACL::ReadItems),
            (self.may_add_items, ACL::AddItems),
            (self.may_remove_items, ACL::RemoveItems),
            (self.may_set_seen, ACL::SetSeen),
            (self.may_set_keywords, ACL::SetKeywords),
            (self.may_create_child, ACL::CreateChild),
            (self.may_rename, ACL::Rename),
            (self.may_delete, ACL::Delete),
            (self.may_submit, ACL::Submit),
        ] {
            if is_set && !acl_list.contains(&acl) {
                acl_list.push(acl);
            }
        }
        acl_list
    }
}

impl GetObject for Mailbox<Set> {
    type GetArguments = ();
}

impl GetObject for Mailbox<Get> {
    type GetArguments = ();
}
