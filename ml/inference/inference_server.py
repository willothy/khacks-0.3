from flask import Flask, request, jsonify
import torch
from policy_network import PolicyNetwork
from inference_policy import sanitize_model_weights

app = Flask(__name__)

# Device configuration
device = torch.device('mps')

# Model configuration
INPUT_DIM = 39
OUTPUT_DIM = 10

# Load Model
policy_net = PolicyNetwork(
    input_dim=INPUT_DIM,
    output_dim=OUTPUT_DIM,
    activation="elu",
    hidden_dims=[512, 256, 128],
    init_noise_std=1.0
)


# Load Pretrained Weights
model_path = '../genesis_playground/zbot-walking/model_400.pt'
state_dict = torch.load(model_path, map_location="cpu")["model_state_dict"]
state_dict = sanitize_model_weights(policy_net, state_dict)
policy_net.load_state_dict(state_dict)
policy_net.to(device)

# Compile the model for inference
compiled_model = None
if True:
  compiled_model = policy_net
else:
  compiled_model = torch.compile(policy_net, mode="max-autotune")

# Define scalar constants
OBS_SCALES = {
    "ang_vel": 1.0,
    "dof_pos": 1.0,
    "dof_vel": 1.0
}
COMMANDS_SCALE = 1.0
DEFAULT_DOF_POS = torch.zeros(12)
OUTPUT_FIELDS = [
    "LeftShoulder", "LeftElbow", "LeftGripper",
    "RightShoulder", "RightElbow", "RightGripper",
    "LeftHip", "LeftKnee", "LeftAnkle",
    "RightHip", "RightKnee", "RightAnkle"
]

@app.route('/infer', methods=['POST'])
def infer():
    data = request.json

    input_data = torch.cat([
        torch.tensor(data['base_ang_vel']) * OBS_SCALES['ang_vel'],
        torch.tensor(data['projected_gravity']),
        torch.tensor(data['commands']) * COMMANDS_SCALE,
        (torch.tensor(data['dof_pos']) - DEFAULT_DOF_POS) * OBS_SCALES['dof_pos'],
        torch.tensor(data['dof_vel']) * OBS_SCALES['dof_vel'],
        torch.tensor(data['actions'])
    ], axis=-1).float().to(device)

    with torch.inference_mode():
        output = compiled_model(input_data.unsqueeze(0))

    output_data = {field: value for field, value in zip(OUTPUT_FIELDS, output.squeeze(0).tolist())}
    return jsonify(output_data)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=4242)
