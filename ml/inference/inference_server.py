from flask import Flask, request, jsonify
import torch
from policy_network import PolicyNetwork
from inference_policy import sanitize_model_weights
from inference_utils import compute_gravity_vector

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
model_path = '../genesis_playground/artifacts/model_360.pt'
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
    "lin_vel": 2.0,
    "ang_vel": 0.25,
    "command": 1.0,
    "dof_pos": 1.0,
    "dof_vel": 0.05,
}

ACTION_SCALE = 0.25

DEFAULT_DOF_POS = torch.zeros(10)
OUTPUT_FIELDS = [
    "R_Hip_Pitch",
    "L_Hip_Pitch",
    "R_Hip_Yaw",
    "L_Hip_Yaw",
    "R_Hip_Roll",
    "L_Hip_Roll",
    "R_Knee_Pitch",
    "L_Knee_Pitch",
    "R_Ankle_Pitch",
    "L_Ankle_Pitch",
]

action_buffer = torch.zeros(10)

@app.route('/infer', methods=['POST'])
def infer():
    global action_buffer

    data = request.json

    projected_gravity = compute_gravity_vector(*data['accel'])

    input_data = torch.cat([
        torch.tensor(data['base_ang_vel']) * OBS_SCALES['ang_vel'],
        torch.tensor(projected_gravity),
        torch.tensor(data['commands']) * OBS_SCALES['command'],
        (torch.tensor(data['dof_pos']) - DEFAULT_DOF_POS) * OBS_SCALES['dof_pos'],
        torch.tensor(data['dof_vel']) * OBS_SCALES['dof_vel'],
        action_buffer
    ], axis=-1).float().to(device)

    with torch.inference_mode():
        output = compiled_model(input_data.unsqueeze(0)) * ACTION_SCALE

    action_buffer = output.squeeze(0).detach().cpu()

    output_data = {field: value for field, value in zip(OUTPUT_FIELDS, output.squeeze(0).tolist())}
    return jsonify(output_data)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=4242)
