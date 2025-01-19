use kos::{
  hal::{
    actuator_service_client::ActuatorServiceClient,
    imu_service_client::ImuServiceClient,
    inference_service_client::InferenceServiceClient,
    led_matrix_service_client::LedMatrixServiceClient,
    process_manager_service_client::ProcessManagerServiceClient,
    sound_service_client::SoundServiceClient, ConfigureActuatorRequest,
    WriteBufferRequest, WriteColorBufferRequest,
  },
  kos_proto::system::system_service_client::SystemServiceClient,
};
use std::{fmt::Debug, future::Future, ops::Deref, sync::Arc, time::Duration};
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

#[derive(Debug)]
pub struct ClientInner {
  pub imu: Mutex<ImuServiceClient<Channel>>,
  pub actuator: Mutex<ActuatorServiceClient<Channel>>,
  pub sound: Mutex<SoundServiceClient<Channel>>,
  pub processes: Mutex<ProcessManagerServiceClient<Channel>>,
  pub led_matrix: Mutex<LedMatrixServiceClient<Channel>>,
  pub inference: Mutex<InferenceServiceClient<Channel>>,
  pub system: Mutex<SystemServiceClient<Channel>>,
}

#[derive(Debug, Clone)]
pub struct Client {
  inner: Arc<ClientInner>,
}

pub use proto::actuator::{ActuatorCommand, CommandActuatorsRequest};

impl Client {
  pub async fn connect(addr: impl Into<String>) -> eyre::Result<Self> {
    let conn = tonic::transport::Endpoint::new(addr.into())?
      .connect()
      .await?;

    Ok(Self {
      inner: Arc::new(ClientInner {
        imu: Mutex::new(ImuServiceClient::new(conn.clone())),
        actuator: Mutex::new(ActuatorServiceClient::new(conn.clone())),
        sound: Mutex::new(SoundServiceClient::new(conn.clone())),
        processes: Mutex::new(ProcessManagerServiceClient::new(conn.clone())),
        led_matrix: Mutex::new(LedMatrixServiceClient::new(conn.clone())),
        inference: Mutex::new(InferenceServiceClient::new(conn.clone())),
        system: Mutex::new(SystemServiceClient::new(conn)),
      }),
    })
  }
}

impl Deref for Client {
  type Target = ClientInner;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

pub struct Config {
  pub server_url: String,
  pub imu_poll_interval_ms: u64,
}

pub struct KBot {
  pub client: Client,
  pub config: Arc<Config>,
}

pub trait Robot: Sized {
  fn list_actuator_ids() -> Vec<u32>;

  fn get_actuator_id(joint: Joint, axis: Option<Axis>) -> Option<u32>;

  fn initialize(
    client: Client,
    config: Config,
  ) -> impl Future<Output = eyre::Result<Self>>;
}

impl Robot for KBot {
  fn list_actuator_ids() -> Vec<u32> {
    vec![
      11, 12, 13, 14, // left upper
      21, 22, 23, // right upper
      31, 32, 33, 34, 35, // left lower
      41, 42, 43, 44, 45, // right lower
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

  async fn initialize(client: Client, config: Config) -> eyre::Result<Self> {
    for actuator_id in Self::list_actuator_ids() {
      println!("Initializing Actuator {}", actuator_id);
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

    Ok(Self {
      client,
      config: Arc::new(config),
    })
  }
}

#[derive(Debug, Clone)]
pub struct JointCommand {
  pub position: Option<f64>,
  pub velocity: Option<f64>,
  pub torque: Option<f64>,
}

const EMPTY_SCREEN: [[u8; 8]; 8] = [
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
];

const FACE_WINK: [[u8; 8]; 8] = [
  [
    0b01111000, 0b00000000, 0b00000000, 0b00011110, 0b00110000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00100000,
    0b00000100, 0b00000000,
  ],
  [
    0b00000000, 0b00011111, 0b11111000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
];

const FACE_BLINK: [[u8; 8]; 8] = [
  [
    0b01111000, 0b00000000, 0b00000000, 0b00011110, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00100000,
    0b00000100, 0b00000000,
  ],
  [
    0b00000000, 0b00011111, 0b11111000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
];

const FACE_EYES_OPEN: [[u8; 8]; 8] = [
  [
    0b01111000, 0b00000000, 0b00000000, 0b00011110, 0b00110000, 0b00000000,
    0b00000000, 0b00001100,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00100000,
    0b00000100, 0b00000000,
  ],
  [
    0b00000000, 0b00011111, 0b11111000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
  [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000,
  ],
];

impl KBot {
  pub async fn connect(addr: String, config: Config) -> eyre::Result<Self> {
    let client = Client::connect(addr).await?;

    println!("GRPC Connected");

    let bot = Self::initialize(client.clone(), config).await?;

    let buffer: Vec<u8> = FACE_EYES_OPEN.into_iter().flatten().collect();

    client
      .led_matrix
      .lock()
      .await
      .write_buffer(WriteBufferRequest {
        buffer,
        // buffer: std::iter::repeat(0x00).take(64).collect()
      })
      .await?;

    tokio::spawn(async move {
      let mut interval = tokio::time::interval(Duration::from_millis(5000));

      loop {
        interval.tick().await;

        client
          .led_matrix
          .lock()
          .await
          .write_buffer(WriteBufferRequest {
            buffer: FACE_BLINK.into_iter().flatten().collect(),
          })
          .await
          .ok();

        tokio::time::sleep(Duration::from_millis(250)).await;

        client
          .led_matrix
          .lock()
          .await
          .write_buffer(WriteBufferRequest {
            buffer: FACE_EYES_OPEN.into_iter().flatten().collect(),
          })
          .await
          .ok();
      }
    });

    // tokio::spawn({
    //   let client = client.clone();
    //   let config = bot.config.clone();
    //   async move {
    //     let mut interval = tokio::time::interval(Duration::from_millis(
    //       config.imu_poll_interval_ms,
    //     ));
    //     let requests = reqwest::Client::new();

    //     loop {
    //       interval.tick().await;

    //       let data = client
    //         .imu
    //         .lock()
    //         .await
    //         .get_values(())
    //         .await
    //         .expect("failed to read IMU data :(");

    //       if let Err(e) = requests
    //         .post(&config.server_url)
    //         .body(prost::Message::encode_to_vec(&data.into_inner()))
    //         .send()
    //         .await
    //       {
    //         eprintln!("Failed to send IMU data: {}", e);
    //         tracing::error!("{e}");
    //       }
    //     }
    //   }
    // });

    Ok(bot)
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

    self
      .client
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
