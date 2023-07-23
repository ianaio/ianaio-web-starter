use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::utils::get_id;

/// A constant to prevent state collision.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum HistoryStateKind {
    #[serde(rename = "ianaio-web-starter_history_state")]
    IanaIO-Web-Starter,
}

/// The state used by browser history to store history id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HistoryState {
    id: u32,
    kind: HistoryStateKind,
}

impl HistoryState {
    pub fn new() -> HistoryState {
        Self {
            id: get_id(),
            kind: HistoryStateKind::IanaIO-Web-Starter,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

pub(crate) type StateMap = HashMap<u32, Rc<dyn Any>>;

