
# 我的`rust`区块链项目

这是一个基于 Rust 的简单区块链演示项目，我按照 [behrouz-rfa/blockchain-rust](https://github.com/behrouz-rfa/blockchain-rust) 项目的代码进行了完整的实现。

## 项目概述

该项目旨在帮助学习区块链的基本概念和实现。通过以下功能，你可以创建钱包、创建区块链以及发送币。

## 教学视频

你可以查看以下教学视频以获得更多信息和演示：
[观看教学视频](https://www.youtube.com/watch?v=qT5YeRZ_DYY&list=PLc0PxFU2AtMQJ0ocblyewzWG60k6vLzLL)

## 功能

- **创建钱包**：
  ```bash
  cargo run createwallet
  ```

- **创建区块链**：
  ```bash
  cargo run create <address>
  ```

- **发送币**（如果指定了 `-m`，块将立即在同一节点上被挖掘）：
  ```bash
  cargo run send <from> <to> <amount> -m
  ```
