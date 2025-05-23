# Secure and Private Computing — Fuzzy PSI Implementation

This repository contains coursework for the "Secure and Private Computing" course at HSG MCS. It implements and explores the protocol from the 2024 paper by Richardson et al., "Fuzzy PSI via Oblivious Protocol Routing".

## Paper and Protocol Overview

The referenced paper introduces a novel framework for **fuzzy Private Set Intersection (PSI)**, enabling two parties to compute the intersection of their sets with approximate (not just exact) matches, according to a public distance metric and threshold. The protocol achieves high efficiency and flexibility, supporting arbitrary distance metrics (such as $\ell_1$, $\ell_2$, and $\ell_\infty$) and significantly reducing communication complexity compared to previous approaches.

Key innovations include:

- **Oblivious protocol routing**: Securely routes protocol messages using conditionally-overlapping hash functions, ensuring that similar items are mapped to at least one common bin.
- **Oblivious Key-Value Stores (OKVS)**: Used to encode and decode protocol messages tied to bins, hiding the structure of the keys and supporting efficient, privacy-preserving message routing.
- **Secure proximity subprotocols**: Intended to use Yao's garbled circuits to securely test whether two items are "close enough" without revealing their actual values.

## Implementation Summary

This implementation provides two versions of the OKVS:

- **Lagrange Polynomial OKVS**: Information-theoretically optimal but extremely slow for large sets, as it relies on polynomial interpolation.
- **Random Binary Matrix (RB-OKVS)**: Much more efficient and scalable, but not information-theoretically optimal. The code was adapted from an older, unmaintained repository and updated for modern Rust.

The protocol logic follows the high-level structure of the paper, including hashing, binning, OKVS encoding, and proximity testing.

### Trade-offs and Security Note

**This implementation does not include actual secure proximity subprotocols (garbled circuits)**, as no suitable Rust library was available and implementing them from scratch was out of scope. Instead, a non-secure placeholder is used: the actual points are shared between parties for testing and benchmarking. This means the protocol is **not secure** and should not be used in practice.

Benchmarks are provided for the OKVS components, demonstrating the trade-off between theoretical optimality (Lagrange) and practical efficiency (RB-OKVS).

## References

- Richardson, D., et al. (2024). "Fuzzy PSI via Oblivious Protocol Routing."
- [Report and Results](report.pdf)
