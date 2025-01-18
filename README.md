# KHACKS 0.3

## Installation

ML Stuff

```
git submodule update --init --recursive

# mujoco_playground
https://docs.kscale.dev/docs/mujoco#/

conda create -n kenv python=3.12 -y
conda activate kenv
cd ml/mujoco_playground && pip install -e .
```

## IMU Data

```jsonc
{
  "accel_x": 0.0, // Accelerometer X-axis in m/s^2
  "accel_y": 0.0, // Accelerometer Y-axis in m/s^2
  "accel_z": 0.0, // Accelerometer Z-axis in m/s^2

  "gyro_x": 0.0,  // Gyroscope X-axis in deg/s
  "gyro_y": 0.0,  // Gyroscope Y-axis in deg/s
  "gyro_z": 0.0,  // Gyroscope Z-axis in deg/s

  "mag_x": 0.0,   // Megnetometer X-axis in uT
  "mag_y": 0.0,   // Megnetometer Y-axis in uT
  "mag_z": 0.0,   // Megnetometer Z-axis in uT
}
```
