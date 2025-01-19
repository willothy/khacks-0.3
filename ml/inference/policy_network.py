import torch
import torch.nn as nn

class PolicyNetwork(nn.Module):
    def __init__(self, input_dim, output_dim, activation="elu", hidden_dims=[512, 256, 128], init_noise_std=1.0):
        super(PolicyNetwork, self).__init__()

        # Define the activation function
        self.activation_fn = getattr(nn, activation.capitalize(), nn.ELU)()

        # Build the network layers
        layers = []
        prev_dim = input_dim

        for hidden_dim in hidden_dims:
            layers.append(nn.Linear(prev_dim, hidden_dim, bias=True))
            layers.append(self.activation_fn)
            prev_dim = hidden_dim

        # Final output layer
        layers.append(nn.Linear(prev_dim, output_dim, bias=True))

        self.actor = nn.Sequential(*layers)

        # Initialize weights with noise
        self.init_weights(init_noise_std)

    def init_weights(self, std):
        for m in self.actor:
            if isinstance(m, nn.Linear):
                nn.init.normal_(m.weight, mean=0.0, std=std)
                nn.init.constant_(m.bias, 0.0)

    def forward(self, x):
        return self.actor(x)

