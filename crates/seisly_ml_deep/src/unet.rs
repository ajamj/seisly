//! U-Net Architecture for Horizon Auto-Tracking

use candle_core::{Device, IndexOp, Result, Tensor, D};
use candle_nn::{BatchNorm2d, Conv2d, Conv2dConfig, Module, VarBuilder};

/// U-Net for Horizon Tracking
/// 
/// Architecture:
/// Encoder: 1 -> 64 -> 128 -> 256 -> 512
/// Bottleneck: 1024
/// Decoder: 512 -> 256 -> 128 -> 64 -> 2 (offset + confidence)
pub struct HorizonUNet {
    // Encoder
    enc1: ConvBlock,
    enc2: ConvBlock,
    enc3: ConvBlock,
    enc4: ConvBlock,
    
    // Bottleneck
    bottleneck: ConvBlock,
    
    // Decoder
    upconv4: ConvTransposeBlock,
    dec4: ConvBlock,
    
    upconv3: ConvTransposeBlock,
    dec3: ConvBlock,
    
    upconv2: ConvTransposeBlock,
    dec2: ConvBlock,
    
    upconv1: ConvTransposeBlock,
    dec1: ConvBlock,
    
    // Output
    final_conv: Conv2d,
}

struct ConvBlock {
    conv1: Conv2d,
    bn1: BatchNorm2d,
    conv2: Conv2d,
    bn2: BatchNorm2d,
}

impl ConvBlock {
    fn new(in_channels: usize, out_channels: usize, vb: VarBuilder) -> Result<Self> {
        let cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };
        
        let conv1 = candle_nn::conv2d(in_channels, out_channels, 3, cfg, vb.pp("conv1"))?;
        let bn1 = candle_nn::batch_norm2d(out_channels, 1e-5, vb.pp("bn1"))?;
        let conv2 = candle_nn::conv2d(out_channels, out_channels, 3, cfg, vb.pp("conv2"))?;
        let bn2 = candle_nn::batch_norm2d(out_channels, 1e-5, vb.pp("bn2"))?;
        
        Ok(Self { conv1, bn1, conv2, bn2 })
    }
    
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.conv1.forward(x)?;
        let x = self.bn1.forward(&x, true)?;
        let x = x.relu()?;
        
        let x = self.conv2.forward(&x)?;
        let x = self.bn2.forward(&x, true)?;
        x.relu()
    }
}

struct ConvTransposeBlock {
    conv: Conv2d,
}

impl ConvTransposeBlock {
    fn new(in_channels: usize, out_channels: usize, vb: VarBuilder) -> Result<Self> {
        let cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };
        let conv = candle_nn::conv2d_transpose(in_channels, out_channels, 4, cfg, vb.pp("conv"))?;
        Ok(Self { conv })
    }
    
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        self.conv.forward(x)
    }
}

impl HorizonUNet {
    pub fn new(vb: VarBuilder) -> Result<Self> {
        let enc1 = ConvBlock::new(1, 64, vb.pp("enc1"))?;
        let enc2 = ConvBlock::new(64, 128, vb.pp("enc2"))?;
        let enc3 = ConvBlock::new(128, 256, vb.pp("enc3"))?;
        let enc4 = ConvBlock::new(256, 512, vb.pp("enc4"))?;
        
        let bottleneck = ConvBlock::new(512, 1024, vb.pp("bottleneck"))?;
        
        let upconv4 = ConvTransposeBlock::new(1024, 512, vb.pp("upconv4"))?;
        let dec4 = ConvBlock::new(1024, 512, vb.pp("dec4"))?;
        
        let upconv3 = ConvTransposeBlock::new(512, 256, vb.pp("upconv3"))?;
        let dec3 = ConvBlock::new(512, 256, vb.pp("dec3"))?;
        
        let upconv2 = ConvTransposeBlock::new(256, 128, vb.pp("upconv2"))?;
        let dec2 = ConvBlock::new(256, 128, vb.pp("dec2"))?;
        
        let upconv1 = ConvTransposeBlock::new(128, 64, vb.pp("upconv1"))?;
        let dec1 = ConvBlock::new(128, 64, vb.pp("dec1"))?;
        
        let final_cfg = Conv2dConfig {
            padding: 0,
            ..Default::default()
        };
        let final_conv = candle_nn::conv2d(64, 2, 1, final_cfg, vb.pp("final"))?;
        
        Ok(Self {
            enc1, enc2, enc3, enc4,
            bottleneck,
            upconv4, dec4,
            upconv3, dec3,
            upconv2, dec2,
            upconv1, dec1,
            final_conv,
        })
    }
    
    pub fn forward(&self, x: &Tensor) -> Result<(Tensor, Tensor)> {
        // Encoder with skip connections
        let e1 = self.enc1.forward(x)?;
        let e2 = self.enc2.forward(&e1.i(..)??.pool2d(2)?)?;
        let e3 = self.enc3.forward(&e2.i(..)??.pool2d(2)?)?;
        let e4 = self.enc4.forward(&e3.i(..)??.pool2d(2)?)?;
        
        // Bottleneck
        let b = self.bottleneck.forward(&e4.i(..)??.pool2d(2)?)?;
        
        // Decoder with skip connections
        let d4 = self.upconv4.forward(&b)?;
        let d4 = Tensor::cat(&[&d4, &e4], 1)?;
        let d4 = self.dec4.forward(&d4)?;
        
        let d3 = self.upconv3.forward(&d4.i(..)??.pool2d(2)?)?;
        let d3 = Tensor::cat(&[&d3, &e3], 1)?;
        let d3 = self.dec3.forward(&d3)?;
        
        let d2 = self.upconv2.forward(&d3.i(..)??.pool2d(2)?)?;
        let d2 = Tensor::cat(&[&d2, &e2], 1)?;
        let d2 = self.dec2.forward(&d2)?;
        
        let d1 = self.upconv1.forward(&d2.i(..)??.pool2d(2)?)?;
        let d1 = Tensor::cat(&[&d1, &e1], 1)?;
        let d1 = self.dec1.forward(&d1)?;
        
        // Output: (offset, confidence)
        let out = self.final_conv.forward(&d1)?;
        let offset = out.i((.., 0..1, .., ..))?.squeeze(1)?;
        let confidence = out.i((.., 1..2, .., ..))?.squeeze(1)?.sigmoid()?;
        
        Ok((offset, confidence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_nn::VarBuilderArgs;

    #[test]
    fn test_unet_creation() {
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(VarBuilderArgs::default(), &device);
        
        let model = HorizonUNet::new(vb);
        assert!(model.is_ok());
    }

    #[test]
    fn test_unet_forward() {
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(VarBuilderArgs::default(), &device);
        
        let model = HorizonUNet::new(vb).unwrap();
        
        // Create dummy input: batch=1, channels=1, 256x256
        let input = Tensor::zeros((1, 1, 256, 256), &device).unwrap();
        
        let (offset, confidence) = model.forward(&input).unwrap();
        
        assert_eq!(offset.dims()[0], 1); // batch
        assert_eq!(confidence.dims()[0], 1); // batch
        assert!(confidence.dims()[1] > 0); // horizon dimension
    }
}
