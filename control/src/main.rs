use std::{sync::Arc, time::Instant};

use axum::{extract::State, routing::post, Json, Router};
use kos::hal::{GetActuatorsStateRequest, ModelUids};
use rpc::{
  proto::actuator::ConfigureActuatorRequest, ActuatorCommand, Axis, Client,
  CommandActuatorsRequest, Config, JointCommand, KBot, Robot,
};
use serde_json::json;

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
    .route("/walk", post(walk))
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

pub async fn walk(State(kbot): State<Arc<rpc::KBot>>) {
  let start = std::time::Instant::now();
  let mut last_iteration = Instant::now();
  loop {
    println!("ELAPSED {:?}", last_iteration.elapsed());
    last_iteration = Instant::now();

    // if time is greater than 5 seconds, break
    if start.elapsed().as_secs() > 5 {
      break;
    }
    let data = kbot
      .imu
      .lock()
      .await
      .get_values(())
      .await
      .expect("failed to read IMU data :(");

    let data = data.into_inner();

    // kbot.inference.lock().await.load_models(ModelUids::)

    //   "R_Hip_Pitch",
    // "L_Hip_Pitch",
    // "R_Hip_Yaw",
    // "L_Hip_Yaw",
    // "R_Hip_Roll",
    // "L_Hip_Roll",
    // "R_Knee_Pitch",
    // "L_Knee_Pitch",
    // "R_Ankle_Pitch",
    // "L_Ankle_Pitch",

    let actuators = vec![
      KBot::get_actuator_id(rpc::Joint::RightHip, Some(Axis::Pitch)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::LeftHip, Some(Axis::Pitch)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::RightHip, Some(Axis::Yaw)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::LeftHip, Some(Axis::Yaw)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::RightHip, Some(Axis::Roll)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::LeftHip, Some(Axis::Roll)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::RightKnee, Some(Axis::Pitch)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::LeftKnee, Some(Axis::Pitch)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::RightAnkle, Some(Axis::Pitch)).unwrap(),
      KBot::get_actuator_id(rpc::Joint::LeftAnkle, Some(Axis::Pitch)).unwrap(),
    ];

    let mut client = kbot.actuator.lock().await;

    let Ok(states) = client
      .get_actuators_state(GetActuatorsStateRequest {
        actuator_ids: actuators,
      })
      .await
    else {
      eprintln!("damn it failed");
      continue;
    };

    // println!("STATE: {:?}", states);

    let states = states.into_inner();
    // kbot.actuator.lock().a

    let json_val = json!({
      "base_ang_vel": [data.gyro_x, data.gyro_y, data.gyro_z],
      "accel": [data.accel_x, data.accel_y, data.accel_z],
      "commands": [
        0.6, 0, 0
      ],
      "dof_pos": states.states.iter().map(|state|{
        state.position()
      }).collect::<Vec<f64>>(),
      "dof_vel": states.states.iter().map(|state|{
        state.velocity()
      }).collect::<Vec<f64>>(),
      "actions": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    });

    let response = reqwest::Client::new()
      .post("http://localhost:4242/infer")
      .json(&json_val)
      .send()
      .await;

    let response = response.unwrap();
    println!("RESPONSE {}", response.text().await.unwrap().to_string());
  }
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
