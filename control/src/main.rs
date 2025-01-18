use std::sync::Arc;

use axum::{extract::State, routing::post, Router};
use rpc::{
  proto::actuator::ConfigureActuatorRequest, ActuatorCommand, Axis,
  CommandActuatorsRequest, Config, JointCommand, KBot,
};

#[tokio::main]
async fn main() {
  // let app = rpc
  // let client = rpc::Client::connect("grpc://10.33.85.8:50051")
  //   .await
  //   .unwrap();

  // println!("Connected");

  let kbot = KBot::connect(
    "grpc://10.33.85.8:50051".to_string(),
    Config {
      server_url: "example.com".to_string(),
      imu_poll_interval_ms: 1000,
    },
  )
  .await
  .unwrap();

  println!("Connected");

  let app = Router::new()
    .route("/dab", post(dab))
    .route("/muscles", post(muscles))
    .with_state(Arc::new(kbot));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Listening on port 3000");
  axum::serve(listener, app).await.unwrap();

  // muscles(&kbot).await;
}

pub async fn dab(State(kbot): State<Arc<rpc::KBot>>) {
  kbot
    .command_joint(
      rpc::Joint::RightShoulder,
      Some(rpc::Axis::Pitch),
      JointCommand {
        position: Some(-45.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::RightShoulder,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(-25.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::RightElbow,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(0.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  // left
  kbot
    .command_joint(
      rpc::Joint::LeftShoulder,
      Some(rpc::Axis::Pitch),
      JointCommand {
        position: Some(0.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::LeftShoulder,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(-20.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::LeftElbow,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(30.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();
}

pub async fn muscles(State(kbot): State<Arc<rpc::KBot>>) {
  kbot
    .command_joint(
      rpc::Joint::RightElbow,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(180.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::RightShoulder,
      Some(Axis::Yaw),
      JointCommand {
        position: Some(90.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::RightShoulder,
      Some(Axis::Pitch),
      JointCommand {
        position: Some(45.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  // left

  kbot
    .command_joint(
      rpc::Joint::LeftShoulder,
      Some(rpc::Axis::Pitch),
      JointCommand {
        position: Some(0.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::LeftShoulder,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(-20.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();

  kbot
    .command_joint(
      rpc::Joint::LeftElbow,
      Some(rpc::Axis::Yaw),
      JointCommand {
        position: Some(-45.),
        velocity: None,
        torque: None,
      },
    )
    .await
    .unwrap();
}
