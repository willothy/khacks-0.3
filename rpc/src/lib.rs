use kos::{
    hal::{
        actuator_service_client::ActuatorServiceClient, imu_service_client::ImuServiceClient,
        inference_service_client::InferenceServiceClient,
        led_matrix_service_client::LedMatrixServiceClient,
        process_manager_service_client::ProcessManagerServiceClient,
        sound_service_client::SoundServiceClient,
    },
    kos_proto::system::system_service_client::SystemServiceClient,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub mod proto {
    pub use kos::google_proto as google;
    pub use kos::kos_proto::*;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Joint {
    LeftShoulder,
    LeftElbow,
    LeftGripper,

    RightShoulder,
    RightElbow,
    RightGripper,

    LeftHip,
    LeftKnee,
    LeftAnkle,

    RightHip,
    RightKnee,
    RightAnkle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Axis {
    Pitch,
    Yaw,
    Roll,
}

pub trait JointMapping {
    fn get_actuator_id(joint: Joint, axis: Axis) -> Option<u32>;
}

#[derive(Debug, Clone)]
pub struct Client {
    pub imu: Arc<Mutex<ImuServiceClient<Channel>>>,
    pub actuator: Arc<Mutex<ActuatorServiceClient<Channel>>>,
    pub sound: Arc<Mutex<SoundServiceClient<Channel>>>,
    pub processes: Arc<Mutex<ProcessManagerServiceClient<Channel>>>,
    pub led_matrix: Arc<Mutex<LedMatrixServiceClient<Channel>>>,
    pub inference: Arc<Mutex<InferenceServiceClient<Channel>>>,
    pub system: Arc<Mutex<SystemServiceClient<Channel>>>,
}

pub use proto::actuator::{ActuatorCommand, CommandActuatorsRequest};

impl Client {
    pub async fn connect(addr: impl Into<String>) -> eyre::Result<Self> {
        let conn = tonic::transport::Endpoint::new(addr.into())?
            .connect()
            .await?;

        Ok(Self {
            imu: Arc::new(Mutex::new(ImuServiceClient::new(conn.clone()))),
            actuator: Arc::new(Mutex::new(ActuatorServiceClient::new(conn.clone()))),
            sound: Arc::new(Mutex::new(SoundServiceClient::new(conn.clone()))),
            processes: Arc::new(Mutex::new(ProcessManagerServiceClient::new(conn.clone()))),
            led_matrix: Arc::new(Mutex::new(LedMatrixServiceClient::new(conn.clone()))),
            inference: Arc::new(Mutex::new(InferenceServiceClient::new(conn.clone()))),
            system: Arc::new(Mutex::new(SystemServiceClient::new(conn))),
        })
    }
}
