# Design — P0-017 Benchmarking Harness

## Architecture

### Framework: Criterion 0.5
- Statistical benchmarking with configurable sample sizes
- HTML report generation for visual regression tracking
- Parameterized benchmarks with throughput metrics
- `black_box` for preventing compiler optimizations on benchmark inputs/outputs

### Benchmark Organization

```
k0/benches/
├── digest_bench.rs    — SHA-3-256 and BLAKE3 throughput
├── codec_bench.rs     — LyraCodec encode/decode/roundtrip
├── time_bench.rs      — VirtualClock operations
└── entropy_bench.rs   — EntropyPool throughput
```

### Benchmark Groups

#### digest_bench.rs
| Group | Benchmarks | Parameterized |
|---|---|---|
| `sha3_256` | SHA-3-256 at 32B, 256B, 1KB, 4KB, 16KB, 64KB | Yes (input size) |
| `blake3` | BLAKE3 at same sizes | Yes (input size) |
| `digest_routing` | SHA-3-256 vs BLAKE3 via routing API at 1KB | No |

#### codec_bench.rs
| Group | Benchmarks | Types Covered |
|---|---|---|
| `codec_encode` | uint, string, bytes, struct, vector, map | All 6 value types |
| `codec_decode` | uint, string, struct, vector, map | 5 value types |
| `codec_roundtrip` | struct encode+decode, map encode+decode | Composite types |

#### time_bench.rs
| Group | Benchmarks |
|---|---|
| `virtual_clock_tick` | Single tick operation |
| `virtual_clock_advance` | advance(1), advance(1000) |
| `virtual_clock_merge` | Merge two clocks |
| `virtual_time_next` | VirtualTime::next() |

#### entropy_bench.rs
| Group | Benchmarks | Parameterized |
|---|---|---|
| `entropy_next_u64` | Single u64 generation | No |
| `entropy_next_u32` | Single u32 generation | No |
| `entropy_next_bytes` | 8B, 32B, 64B, 128B, 256B | Yes (output size) |
| `entropy_fork` | Pool fork operation | No |
| `entropy_from_seed_bytes` | Pool construction from seed | No |

## Design Decisions
1. **Criterion over built-in bench** — statistical rigor, HTML reports, regression detection
2. **Throughput metrics** — digest and entropy benchmarks report bytes/second
3. **Parameterized sizes** — captures performance scaling behavior
4. **Separate bench files** — one per k0 module for independent execution
5. **`harness = false`** — required for criterion to control the benchmark runner
