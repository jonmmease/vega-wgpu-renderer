use crate::value::EncodingValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RectMark {
    pub name: String,
    pub clip: bool,
    pub len: u32,
    pub x: EncodingValue<f32>,
    pub y: EncodingValue<f32>,
    pub width: EncodingValue<f32>,
    pub height: EncodingValue<f32>,
    pub fill: EncodingValue<[f32; 3]>,
}

impl RectMark {
    pub fn x_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.x.as_iter(self.len as usize)
    }

    pub fn y_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.y.as_iter(self.len as usize)
    }

    pub fn width_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.width.as_iter(self.len as usize)
    }

    pub fn height_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.height.as_iter(self.len as usize)
    }

    pub fn fill_iter(&self) -> Box<dyn Iterator<Item = &[f32; 3]> + '_> {
        self.fill.as_iter(self.len as usize)
    }
}

impl Default for RectMark {
    fn default() -> Self {
        Self {
            name: "rule_mark".to_string(),
            clip: true,
            len: 1,
            x: EncodingValue::Scalar { value: 0.0 },
            y: EncodingValue::Scalar { value: 0.0 },
            width: EncodingValue::Scalar { value: 0.0 },
            height: EncodingValue::Scalar { value: 0.0 },
            fill: EncodingValue::Scalar {
                value: [0.0, 0.0, 0.0],
            },
        }
    }
}
