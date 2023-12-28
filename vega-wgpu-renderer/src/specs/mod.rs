pub mod dims;
pub mod group;
pub mod mark;
pub mod rect;
pub mod rule;
pub mod symbol;

use crate::specs::group::GroupItemSpec;
use crate::specs::mark::MarkContainerSpec;

pub type SceneGraphSpec = MarkContainerSpec<GroupItemSpec>;
