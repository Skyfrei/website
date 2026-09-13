---
title: "State estimation under Gaussian noise"
date: "2026-09-13"
tags: ["information-theory", "compression", "algorithms", "statistics"]
wordNum: 850
readTime: 6
---


State estimation under Gaussian noise remains a foundational problem in signal processing and tracking systems. When the underlying dynamical process is linear, the discrete Kalman filter provides the minimum mean-square error (MMSE) estimator.

## System Model

Consider a discrete-time linear dynamical system governed by the state equation:

$$x_k = F_k x_{k-1} + B_k u_k + w_k$$

and measurement equation:

$$z_k = H_k x_k + v_k$$

where the noise terms are zero-mean Gaussian white processes:

$$w_k \sim \mathcal{N}(0, Q_k), \quad v_k \sim \mathcal{N}(0, R_k)$$

## Prediction and Update Steps

The algorithm operates as a recursive two-step predictor-corrector:

1. **A priori state estimate:**
   $$\hat{x}_{k|k-1} = F_k \hat{x}_{k-1|k-1} + B_k u_k$$

2. **A priori error covariance:**
   $$P_{k|k-1} = F_k P_{k-1|k-1} F_k^T + Q_k$$

3. **Optimal Kalman Gain:**
   $$K_k = P_{k|k-1} H_k^T (H_k P_{k|k-1} H_k^T + R_k)^{-1}$$

4. **A posteriori state estimate:**
   $$\hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k (z_k - H_k \hat{x}_{k|k-1})$$

5. **A posteriori error covariance update:**
   $$P_{k|k} = (I - K_k H_k) P_{k|k-1}$$

## Scalar Implementation

In the 1D case where $F = 1$ and $H = 1$, the matrix inverses collapse into simple scalar divisions:

```cpp
struct Kalman1D {
    float x; // State estimate
    float p; // Estimate error variance
    float q; // Process noise covariance
    float r; // Measurement noise covariance

    float update(float measurement) {
        // Predict
        p += q;

        // Update
        float k = p / (p + r);
        x += k * (measurement - x);
        p *= (1.0f - k);

        return x;
    }
};
