---
title: "Information Geometry and Bounds on Redundant Source Coding"
date: "2026-09-13"
tags: ["information-theory", "compression", "algorithms", "statistics"]
wordNum: 850
readTime: 6
---

# 1. Introduction and Problem Formulation

Let $\mathcal{X} = \{x_1, x_2, \dots, x_m\}$ be a finite alphabet drawn according to a discrete probability mass function $P = (p_1, p_2, \dots, p_m)$ with $p_i > 0$ for all $i \in \{1, \dots, m\}$. We consider uniquely decodable prefix codes $C: \mathcal{X} \to \{0, 1\}^*$ with corresponding codeword lengths $\ell_1, \ell_2, \dots, \ell_m$.

The Shannon entropy $H(P)$ provides the fundamental theoretical lower bound for the expected codeword length:

$$
H(P) = -\sum_{i=1}^m p_i \log_2 p_i
$$

The expected codeword length $\bar{L}(C)$ achieved by a given prefix-free code assignment satisfies:

$$
\bar{L}(C) = \sum_{i=1}^m p_i \ell_i
$$

We define the operational redundancy $R(C)$ as the divergence between the actual average length and the empirical entropy:

$$
R(C) = \bar{L}(C) - H(P)
$$

By Kraft's inequality, a prefix-free code exists over a binary alphabet if and only if the codeword lengths satisfy:

$$
\sum_{i=1}^m 2^{-\ell_i} \le 1
$$

---

# 2. Asymptotic Bounds on Huffman Trees

For a standard greedy Huffman code tree, the redundancy $R_H(P)$ is bounded strictly within the open unit interval:

$$
0 \le R_H(P) < 1
$$

When the maximum source probability $p_1 = \max_{i} p_i$ is known, Gallager and Johnsen established sharpened upper bounds. Specifically, when $p_1 \ge \frac{1}{2}$:

$$
R_H(P) \le p_1 + \sigma - H_2(p_1)
$$

where $H_2(q) = -q \log_2 q - (1 - q) \log_2(1 - q)$ is the binary entropy function, and $\sigma \approx 0.08607$ represents the root of:

$$
\sigma = 1 - \log_2(e) + \log_2(\log_2(e))
$$

> **Lemma 1 (Kraft-McMillan Equality for Full Trees).**  
> For any saturated $D$-ary prefix tree where every internal node possesses exactly $D$ children, Kraft's inequality holds with strict equality:
> $$
> \sum_{i=1}^m D^{-\ell_i} = 1
> $$

---

# 3. Code Optimization Comparison

The table below contrasts standard lossless encoding paradigms across alphabet parameters and operational constraints:

| Code Construction | Optimality | Redundancy Bound $R(C)$ | Complexity | Tree Saturated |
| :--- | :--- | :--- | :--- | :--- |
| **Shannon-Fano** | Suboptimal | $R(C) < 1$ | $\mathcal{O}(m \log m)$ | No |
| **Gilbert-Moore** | Alphabetic Order | $R(C) < 2$ | $\mathcal{O}(m^3)$ | No |
| **Huffman** | Globally Optimal | $R(C) < 1$ | $\mathcal{O}(m \log m)$ | Yes |
| **Arithmetic** | Block Asymptotic | $R(C) \le \frac{2}{n}$ | $\mathcal{O}(n)$ | N/A |

---

# 4. Matrix Formulation of Co-Occurrence Invariants

Let $\mathbf{T} \in \mathbb{R}^{m \times m}$ represent the transition probability matrix of a first-order Markov source:

$$
\mathbf{T} = \begin{bmatrix}
p_{11} & p_{12} & \cdots & p_{1m} \\
p_{21} & p_{22} & \cdots & p_{2m} \\
\vdots & \vdots & \ddots & \vdots \\
p_{m1} & p_{m2} & \cdots & p_{mm}
\end{bmatrix}
$$

The stationary distribution $\boldsymbol{\pi} = (\pi_1, \dots, \pi_m)^T$ satisfies the left eigenvector equation:

$$
\boldsymbol{\pi}^T \mathbf{T} = \boldsymbol{\pi}^T, \quad \text{subject to} \quad \sum_{i=1}^m \pi_i = 1
$$

The entropy rate $\mathcal{H}(\mathcal{X})$ across the stationary sequence is given in matrix product form:

$$
\mathcal{H}(\mathcal{X}) = -\sum_{i=1}^m \pi_i \sum_{j=1}^m p_{ij} \log_2 p_{ij} = -\operatorname{Tr}\left( \operatorname{diag}(\boldsymbol{\pi}) \, \mathbf{T} \circ \log_2 \mathbf{T} \right)
$$

where $\circ$ denotes the Hadamard entrywise product.

---

# 5. Greedy Merge Implementation

The following subroutine demonstrates the priority-queue based merge pass used to build canonical trees:

```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
struct HuffmanNode {
    weight: f64,
    symbol: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Eq for HuffmanNode {}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order to make BinaryHeap act as a min-heap
        other.weight.partial_cmp(&self.weight).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_tree(freqs: &[(char, f64)]) -> Option<HuffmanNode> {
    let mut heap: BinaryHeap<HuffmanNode> = freqs
        .iter()
        .map(|&(symbol, weight)| HuffmanNode {
            weight,
            symbol: Some(symbol),
            left: None,
            right: None,
        })
        .collect();

    while heap.len() > 1 {
        let left = heap.pop()?;
        let right = heap.pop()?;

        let parent = HuffmanNode {
            weight: left.weight + right.weight,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        heap.push(parent);
    }

    heap.pop()
}
