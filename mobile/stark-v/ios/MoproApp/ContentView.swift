//
//  ContentView.swift
//  MoproApp
//
import SwiftUI

private struct BenchmarkOption: Identifiable, Hashable {
  let id: String
  let label: String
  let defaultInputSize: String
  let resourceName: String
}

struct ContentView: View {
  @State private var textViewText = ""
  @State private var isProveButtonEnabled = true
  @State private var selectedBenchmark = "private_tx"
  @State private var inputSize: String = "32"

  private static let benchmarks = [
    BenchmarkOption(id: "private_tx", label: "private_tx", defaultInputSize: "32", resourceName: "private_tx"),
    BenchmarkOption(id: "constant_overhead", label: "constant_overhead", defaultInputSize: "1", resourceName: "constant_overhead"),
    BenchmarkOption(id: "merkle_fake", label: "merkle_fake", defaultInputSize: "4", resourceName: "merkle_fake"),
    BenchmarkOption(id: "hash_sha256", label: "hash_sha256", defaultInputSize: "128", resourceName: "hash_sha256"),
    BenchmarkOption(id: "merkle_sha256", label: "merkle_sha256", defaultInputSize: "4", resourceName: "merkle_sha256"),
    BenchmarkOption(id: "hash_keccak", label: "hash_keccak", defaultInputSize: "128", resourceName: "hash_keccak"),
    BenchmarkOption(id: "merkle_keccak", label: "merkle_keccak", defaultInputSize: "4", resourceName: "merkle_keccak"),
  ]

  var body: some View {
    VStack(spacing: 16) {
      Text("Stark-V Benchmarks")
        .font(.headline)

      Picker("Benchmark", selection: $selectedBenchmark) {
        ForEach(Self.benchmarks) { benchmark in
          Text(benchmark.label).tag(benchmark.id)
        }
      }
      .pickerStyle(.menu)

      HStack {
        Text("Input size:")
        TextField("input_size", text: $inputSize)
          #if canImport(UIKit)
          .keyboardType(.numberPad)
          #endif
          .textFieldStyle(.roundedBorder)
          .frame(width: 100)
      }

      Button("Run Stark-V Benchmark", action: runStarkVProveAction)
        .disabled(!isProveButtonEnabled)
        .buttonStyle(.borderedProminent)
        .accessibilityIdentifier("runStarkVBenchmark")

      ScrollView {
        Text(textViewText)
          .frame(maxWidth: .infinity, alignment: .leading)
          .padding()
          .accessibilityIdentifier("proof_log")
      }
      .frame(maxHeight: .infinity)
    }
    .padding()
  }
}

extension ContentView {
  func runStarkVProveAction() {
    guard let size = UInt64(inputSize), size > 0 else {
      textViewText += "Invalid input_size.\n"
      return
    }

    let benchmark = selectedOption()
    isProveButtonEnabled = false
    textViewText += "Running Stark-V \(benchmark.id) (input_size=\(size))...\n"

    DispatchQueue.global(qos: .userInitiated).async {
      guard let binPath = Bundle.main.path(forResource: benchmark.resourceName, ofType: "bin") else {
        DispatchQueue.main.async {
          textViewText += "Error: \(benchmark.resourceName).bin not found in app bundle.\n"
          isProveButtonEnabled = true
        }
        return
      }

      let result: String
      switch benchmark.id {
      case "private_tx":
        result = starkVProvePrivateTx(inputSize: size, compiledProgramPath: binPath)
      case "constant_overhead":
        result = starkVProveConstantOverhead(inputSize: size, compiledProgramPath: binPath)
      case "merkle_fake":
        result = starkVProveMerkleFake(inputSize: size, compiledProgramPath: binPath)
      case "hash_sha256":
        result = starkVProveHashSha256(inputSize: size, compiledProgramPath: binPath)
      case "merkle_sha256":
        result = starkVProveMerkleSha256(inputSize: size, compiledProgramPath: binPath)
      case "hash_keccak":
        result = starkVProveHashKeccak(inputSize: size, compiledProgramPath: binPath)
      case "merkle_keccak":
        result = starkVProveMerkleKeccak(inputSize: size, compiledProgramPath: binPath)
      default:
        result = "error=unknown_benchmark"
      }

      let summary = parseBenchmarkSummary(result)
      DispatchQueue.main.async {
        textViewText += "  prove mean: \(summary["prove_time_ms"] ?? "?") ms\n"
        textViewText += "  median: \(summary["prove_time_median_ms"] ?? "?") ms\n"
        textViewText += "  min/max: \(summary["prove_time_min_ms"] ?? "?") / \(summary["prove_time_max_ms"] ?? "?") ms\n"
        textViewText += "  stddev: \(summary["prove_time_stddev_ms"] ?? "?") ms\n"
        textViewText += "  samples: \(summary["samples_ms"] ?? "?")\n"
        isProveButtonEnabled = true
      }
    }
  }

  private func selectedOption() -> BenchmarkOption {
    Self.benchmarks.first { $0.id == selectedBenchmark } ?? Self.benchmarks[0]
  }

  private func parseBenchmarkSummary(_ result: String) -> [String: String] {
    var fields: [String: String] = [:]
    for part in result.split(separator: ",") {
      let kv = part.split(separator: "=", maxSplits: 1)
      if kv.count == 2 {
        fields[String(kv[0])] = String(kv[1])
      }
    }
    return fields
  }
}
