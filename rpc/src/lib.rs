use std::io::Result;

use kos::kos_proto as proto;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct Client {
    imu: proto::imu::imu_service_client::ImuServiceClient<Channel>,
    actuator: proto::actuator::actuator_service_client::ActuatorServiceClient<Channel>,
    sound: proto::sound::sound_service_client::SoundServiceClient<Channel>,
    processes: proto::process_manager::process_manager_service_client::ProcessManagerServiceClient<
        Channel,
    >,
    led_matrix: proto::led_matrix::led_matrix_service_client::LedMatrixServiceClient<Channel>,
    inference: proto::inference::inference_service_client::InferenceServiceClient<Channel>,
    system: proto::system::system_service_client::SystemServiceClient<Channel>,
}

impl Client {
    pub async fn connect() -> eyre::Result<Self> {
        Err(eyre::eyre!("Not implemented"))
    }
}
