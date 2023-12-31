use crate::marks::group::VegaGroupItem;
use crate::marks::rect::VegaRectItem;
use crate::marks::rule::VegaRuleItem;
use crate::marks::symbol::VegaSymbolItem;
use crate::marks::text::VegaTextItem;
use serde::{Deserialize, Serialize};

pub trait VegaMarkItem {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "marktype")]
pub enum VegaMark {
    Arc,
    Area,
    Image,
    Group(VegaMarkContainer<VegaGroupItem>),
    Line,
    Path,
    Rect(VegaMarkContainer<VegaRectItem>),
    Rule(VegaMarkContainer<VegaRuleItem>),
    Shape,
    Symbol(VegaMarkContainer<VegaSymbolItem>),
    Text(VegaMarkContainer<VegaTextItem>),
    Trail,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VegaMarkContainer<T: VegaMarkItem> {
    #[serde(default)]
    pub clip: bool,
    pub interactive: bool,
    #[serde(default)]
    pub items: Vec<T>,
    pub name: Option<String>,
    role: Option<String>,
    zindex: Option<i64>,
}
