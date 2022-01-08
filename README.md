# Replay Memory

Simple replay memory data structure implemented using a ring buffer in Rust. Intended to be used for reinforcement learning.

## Installation

```bash
pip install replay_memory
```

## Usage

```python
>>> import replay_memory
>>> rm = replay_memory.ReplayMemory(10)  # set capacity to 10
>>> rm.push_items(range(21))
>>> rm[0]  # show has overwritten oldest data
20
>>> len(rm)  # length unchanged
10
>>> rm.sample(3)  # get a randomly selected sample of 3 items
[20, 18, 11]
>>> rm.sample(3)
[12, 18, 16]
```

## Development

Package is written in rust using PyO3 and maturin.

### Unit tests

Rust unit tests can be run as below:

```bash
cargo test --no-default-features
```

Python unit tests are run using tox:

```bash
tox
```
