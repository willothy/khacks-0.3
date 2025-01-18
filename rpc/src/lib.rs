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
use tonic::{transport::Channel, Result};

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
    fn get_actuator_id(joint: Joint, axis: Option<Axis>) -> Option<u32>;
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

pub struct KBot {
    client: Client,
}

impl JointMapping for KBot {
    fn get_actuator_id(joint: Joint, axis: Option<Axis>) -> Option<u32> {
        Some(match (joint, axis) {
            (Joint::LeftShoulder, Some(Axis::Yaw)) => 11,
            (Joint::LeftShoulder, Some(Axis::Pitch)) => 12,
            (Joint::LeftElbow, Some(Axis::Yaw)) => 13,
            (Joint::LeftGripper, None) => 14,

            (Joint::RightShoulder, Some(Axis::Yaw)) => 21,
            (Joint::RightShoulder, Some(Axis::Pitch)) => 22,
            (Joint::RightElbow, Some(Axis::Yaw)) => 23,
            (Joint::RightGripper, None) => 24,

            (Joint::LeftHip, Some(Axis::Yaw)) => 31,
            (Joint::LeftHip, Some(Axis::Roll)) => 32,
            (Joint::LeftHip, Some(Axis::Pitch)) => 33,
            (Joint::LeftKnee, Some(Axis::Pitch)) => 34,
            (Joint::LeftAnkle, Some(Axis::Pitch)) => 35,

            (Joint::RightHip, Some(Axis::Yaw)) => 41,
            (Joint::RightHip, Some(Axis::Roll)) => 42,
            (Joint::RightHip, Some(Axis::Pitch)) => 43,
            (Joint::RightKnee, Some(Axis::Pitch)) => 44,
            (Joint::RightAnkle, Some(Axis::Pitch)) => 45,

            _ => return None,
        })
    }
}

impl KBot {
    pub async fn connect(addr: String) -> eyre::Result<Self> {
        let client = Client::connect(addr).await?;

        Ok(Self { client })
    }

    pub async fn set_joint(&self, joint: Joint, axis: Axis, value: f32) -> eyre::Result<()> {
        Ok(())
    }
}
