use anyhow::Result;
use tonic::transport::Channel;

pub mod strataforge {
    pub mod analysis {
        tonic::include_proto!("strataforge.analysis");
    }
}

use strataforge::analysis::detection_client::DetectionClient;
use strataforge::analysis::SliceRequest;

// AI Client - reserved for future ML integration
#[allow(dead_code)]
pub struct AiClient {
    client: DetectionClient<Channel>,
}

#[allow(dead_code)]
impl AiClient {
    pub async fn connect(addr: String) -> Result<Self> {
        let client = DetectionClient::connect(addr).await?;
        Ok(Self { client })
    }

    pub async fn detect_faults(
        &mut self,
        data: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Result<(Vec<u8>, f32)> {
        let request = tonic::Request::new(SliceRequest {
            data,
            width,
            height,
        });

        let response = self.client.detect_faults(request).await?.into_inner();
        Ok((response.mask, response.confidence))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_ai_client_placeholder() {
        // Placeholder test - just ensure it compiles
        assert!(true);
    }
}
