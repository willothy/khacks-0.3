use kos::{
    hal::{
        actuator_service_client::ActuatorServiceClient, imu_service_client::ImuServiceClient,
        inference_service_client::InferenceServiceClient,
        led_matrix_service_client::LedMatrixServiceClient,
        process_manager_service_client::ProcessManagerServiceClient,
        sound_service_client::SoundServiceClient,
    },
    kos_proto::{self as proto, system::system_service_client::SystemServiceClient},
};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct Client {
    pub imu: proto::imu::imu_service_client::ImuServiceClient<Channel>,
    pub actuator: proto::actuator::actuator_service_client::ActuatorServiceClient<Channel>,
    pub sound: proto::sound::sound_service_client::SoundServiceClient<Channel>,
    pub processes:
        proto::process_manager::process_manager_service_client::ProcessManagerServiceClient<
            Channel,
        >,
    pub led_matrix: proto::led_matrix::led_matrix_service_client::LedMatrixServiceClient<Channel>,
    pub inference: proto::inference::inference_service_client::InferenceServiceClient<Channel>,
    pub system: proto::system::system_service_client::SystemServiceClient<Channel>,
}

impl Client {
    pub async fn connect(addr: impl Into<String>) -> eyre::Result<Self> {
        let conn = tonic::transport::Endpoint::new(addr.into())?
            .connect()
            .await?;

        Ok(Self {
            imu: ImuServiceClient::new(conn.clone()),
            actuator: ActuatorServiceClient::new(conn.clone()),
            sound: SoundServiceClient::new(conn.clone()),
            processes: ProcessManagerServiceClient::new(conn.clone()),
            led_matrix: LedMatrixServiceClient::new(conn.clone()),
            inference: InferenceServiceClient::new(conn.clone()),
            system: SystemServiceClient::new(conn),
        })
    }
}
