use std::sync::Arc;

use tonic::{transport::Server, Request, Response, Status};

use grpc::video_processing;
use video_processing::raw_video_processor_server::{RawVideoProcessor, RawVideoProcessorServer};
use video_processing::{ProcessRawVideoRequest, ProcessRawVideoResponse};

use crate::queue::{Message, Queue};

#[derive(Debug)]
pub struct VideoProcessorImpl {
    queue: Arc<dyn Queue>,
}

#[tonic::async_trait]
impl RawVideoProcessor for VideoProcessorImpl {
    async fn process_raw_video(
        &self,
        request: Request<ProcessRawVideoRequest>,
    ) -> Result<Response<ProcessRawVideoResponse>, Status> {
        println!("Received a request!");
        // Push a job to the queue
        match self.queue.push(Message::ProcessRawVideo, None).await {
            Ok(_) => {
                let response = ProcessRawVideoResponse {
                    status: "Job added to queue successfully".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!(
                "Failed to add job to queue: {}",
                e
            ))),
        }
    }
}

pub async fn start_server(queue: Arc<dyn Queue>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let video_processor = VideoProcessorImpl { queue };

    Server::builder()
        .add_service(RawVideoProcessorServer::new(video_processor))
        .serve(addr)
        .await?;

    Ok(())
}