import torch
from collections import OrderedDict
from policy_network import PolicyNetwork
import time


def sanitize_model_weights(model, state_dict):
    model_state_keys = set(model.state_dict().keys())

    sanitized_weights = OrderedDict(
        {key: value for key, value in state_dict.items() if key in model_state_keys}
    )

    return sanitized_weights


def main():
  device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')

  input_dim = 39
  output_dim = 10

  # Load Model
  policy_net = PolicyNetwork(input_dim=input_dim, output_dim=output_dim,
                              activation="elu",
                              hidden_dims=[512, 256, 128],
                              init_noise_std=1.0)
  policy_net.to(device)

  # Load Pretrained Weights
  model_path = '../genesis_playground/zbot-walking/model_400.pt'
  state_dict = torch.load(model_path)["model_state_dict"]
  state_dict = sanitize_model_weights(policy_net, state_dict)
  policy_net.load_state_dict(state_dict)


  num_steps = 100000

  sample_input = torch.randn(1, input_dim).to(device)
  compiled = torch.compile(policy_net, mode="max-autotune")

  # Warmup loop
  with torch.inference_mode():
      for _ in range(10):
          _ = compiled(sample_input)

  # Computes Inference Speed Brrrr...
  # print("starting...")
  # start_time = time.time()
  # with torch.inference_mode():
  #     for _ in range(num_steps):
  #         output = compiled(sample_input)
  # end_time = time.time()
  # print(f"Inference freq: {num_steps/(end_time - start_time):.2f} hz")

  # This is how you inference!
  output = compiled(sample_input)

if __name__ == "__main__":
    main()
