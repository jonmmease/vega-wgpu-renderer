use crate::value::{EncodingValue, StrokeCap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RuleMark {
    pub name: String,
    pub clip: bool,
    pub len: u32,
    pub x0: EncodingValue<f32>,
    pub y0: EncodingValue<f32>,
    pub x1: EncodingValue<f32>,
    pub y1: EncodingValue<f32>,
    pub stroke: EncodingValue<[f32; 3]>,
    pub stroke_width: EncodingValue<f32>,
    pub stroke_cap: EncodingValue<StrokeCap>,
}

impl RuleMark {
    pub fn x0_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.x0.as_iter(self.len as usize)
    }
    pub fn y0_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.y0.as_iter(self.len as usize)
    }
    pub fn x1_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.x1.as_iter(self.len as usize)
    }
    pub fn y1_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.y1.as_iter(self.len as usize)
    }
    pub fn stroke_iter(&self) -> Box<dyn Iterator<Item = &[f32; 3]> + '_> {
        self.stroke.as_iter(self.len as usize)
    }
    pub fn stroke_width_iter(&self) -> Box<dyn Iterator<Item = &f32> + '_> {
        self.stroke_width.as_iter(self.len as usize)
    }
    pub fn stroke_cap_iter(&self) -> Box<dyn Iterator<Item = &StrokeCap> + '_> {
        self.stroke_cap.as_iter(self.len as usize)
    }
}

impl Default for RuleMark {
    fn default() -> Self {
        Self {
            name: "rule_mark".to_string(),
            clip: true,
            len: 1,
            x0: EncodingValue::Scalar { value: 0.0 },
            y0: EncodingValue::Scalar { value: 0.0 },
            x1: EncodingValue::Scalar { value: 0.0 },
            y1: EncodingValue::Scalar { value: 0.0 },
            stroke: EncodingValue::Scalar {
                value: [0.0, 0.0, 0.0],
            },
            stroke_width: EncodingValue::Scalar { value: 1.0 },
            stroke_cap: EncodingValue::Scalar {
                value: StrokeCap::Butt,
            },
        }
    }
}
