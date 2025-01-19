import numpy as np

def compute_gravity_vector(ax, ay, az, alpha=0.98, previous_gravity=None):
    """
    Compute the gravity vector from raw IMU accelerometer data.

    Args:
        ax (float): X-axis accelerometer reading (in g or m/s^2).
        ay (float): Y-axis accelerometer reading (in g or m/s^2).
        az (float): Z-axis accelerometer reading (in g or m/s^2).
        alpha (float): Complementary filter coefficient (default: 0.98).
        previous_gravity (np.array): Previously computed gravity vector for filtering.

    Returns:
        np.array: Gravity vector (normalized).
    """
    # Raw accelerometer data as a vector
    accel_vector = np.array([ax, ay, az])

    # Normalize the accelerometer vector to get a direction
    gravity_vector = accel_vector / np.linalg.norm(accel_vector)

    if previous_gravity is not None:
        # Use complementary filtering to smooth the gravity vector
        gravity_vector = alpha * previous_gravity + (1 - alpha) * gravity_vector
        # Re-normalize after filtering
        gravity_vector = gravity_vector / np.linalg.norm(gravity_vector)

    return gravity_vector

