use kos::{
    hal::{
        actuator_service_client::ActuatorServiceClient, imu_service_client::ImuServiceClient,
        inference_service_client::InferenceServiceClient,
        led_matrix_service_client::LedMatrixServiceClient,
        process_manager_service_client::ProcessManagerServiceClient,
        sound_service_client::SoundServiceClient, ConfigureActuatorRequest,
    },
    kos_proto::system::system_service_client::SystemServiceClient,
};
use std::{future::Future, ops::Deref, sync::Arc};
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

pub trait Robot: Sized {
    fn list_actuator_ids() -> Vec<u32>;

    fn get_actuator_id(joint: Joint, axis: Option<Axis>) -> Option<u32>;

    fn initialize(client: Client) -> impl Future<Output = eyre::Result<Self>>;
}

impl Robot for KBot {
    fn list_actuator_ids() -> Vec<u32> {
        vec![
            11, 12, 13, 14, // left upper
            21, 22, 23, 34, // right upper
            31, 32, 33, 34, // left lower
            41, 42, 43, 44, // right lower
        ]
    }

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

    async fn initialize(client: Client) -> eyre::Result<Self> {
        for actuator_id in Self::list_actuator_ids() {
            client
                .actuator
                .lock()
                .await
                .configure_actuator(ConfigureActuatorRequest {
                    actuator_id,
                    kp: None,
                    kd: None,
                    ki: None,
                    max_torque: None,
                    protective_torque: None,
                    protection_time: None,
                    torque_enabled: Some(true),
                    new_actuator_id: None,
                    zero_position: None,
                })
                .await?;
        }

        Ok(Self { client })
    }
}

#[derive(Debug, Clone)]
pub struct JointCommand {
    pub position: Option<f64>,
    pub velocity: Option<f64>,
    pub torque: Option<f64>,
}

impl KBot {
    pub async fn connect(addr: String) -> eyre::Result<Self> {
        let client = Client::connect(addr).await?;

        Self::initialize(client).await
    }

    pub async fn command_joint(
        &self,
        joint: Joint,
        axis: Option<Axis>,
        command: JointCommand,
    ) -> eyre::Result<()> {
        let Some(actuator_id) = Self::get_actuator_id(joint, axis) else {
            return Err(eyre::eyre!("Invalid actuator {joint:?} {axis:?}"));
        };

        self.client
            .actuator
            .lock()
            .await
            .command_actuators(CommandActuatorsRequest {
                commands: vec![ActuatorCommand {
                    actuator_id,
                    position: command.position,
                    velocity: command.velocity,
                    torque: command.torque,
                }],
            })
            .await?;

        Ok(())
    }
}

impl Deref for KBot {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
