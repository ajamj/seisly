//! CNN Model for Horizon Auto-Tracking
//!
//! Architecture:
//! Input: 64x64 seismic patch
//!   ↓
//! Conv2D(32) → ReLU → MaxPool
//!   ↓
//! Conv2D(64) → ReLU → MaxPool
//!   ↓
//! Conv2D(128) → ReLU → GlobalAvgPool
//!   ↓
//! Dense(64) → ReLU
//!   ↓
//! Output: Horizon offset (continuous)

use candle_core::{Module, Result, Tensor};
use candle_nn::{Conv2d, Conv2dConfig, Linear, VarBuilder};

pub struct HorizonCNN {
    conv1: Conv2d,
    conv2: Conv2d,
    conv3: Conv2d,
    fc: Linear,
}

impl HorizonCNN {
    pub fn new(vb: VarBuilder) -> Result<Self> {
        let conv1_cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };
        let conv2_cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };
        let conv3_cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };

        let conv1 = candle_nn::conv2d(1, 32, 3, conv1_cfg, vb.pp("conv1"))?;
        let conv2 = candle_nn::conv2d(32, 64, 3, conv2_cfg, vb.pp("conv2"))?;
        let conv3 = candle_nn::conv2d(64, 128, 3, conv3_cfg, vb.pp("conv3"))?;
        let fc = candle_nn::linear(128, 1, vb.pp("fc"))?;

        Ok(Self {
            conv1,
            conv2,
            conv3,
            fc,
        })
    }

    pub fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        // Conv1 + ReLU + MaxPool
        let xs = self.conv1.forward(xs)?.relu()?;
        let xs = xs.max_pool2d(2)?;

        // Conv2 + ReLU + MaxPool
        let xs = self.conv2.forward(&xs)?.relu()?;
        let xs = xs.max_pool2d(2)?;

        // Conv3 + ReLU + GlobalAvgPool
        let xs = self.conv3.forward(&xs)?.relu()?;
        let (_b, _c, h, w) = xs.dims4()?;
        let xs = xs.avg_pool2d((h, w))?;
        let xs = xs.squeeze(2)?.squeeze(2)?;

        // Fully connected
        self.fc.forward(&xs)
    }
}
