use rpc::{
  proto::actuator::ConfigureActuatorRequest, ActuatorCommand,
  CommandActuatorsRequest,
};

#[tokio::main]
async fn main() {
  // let app = rpc
  let mut client = rpc::Client::connect("grpc://10.33.85.8:50051")
    .await
    .unwrap();
  println!("connected");
  client
    .actuator
    .configure_actuator(ConfigureActuatorRequest {
      actuator_id: 21,
      torque_enabled: Some(true),
      ..Default::default()
    })
    .await
    .unwrap();
  println!("yoooo");
  client
    .actuator
    .command_actuators(CommandActuatorsRequest {
      commands: vec![ActuatorCommand {
        actuator_id: 21,
        position: Some(70.),
        velocity: None,
        torque: None,
      }],
    })
    .await
    .unwrap();
}
