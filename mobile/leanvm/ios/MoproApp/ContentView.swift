//
//  ContentView.swift
//  MoproApp
//
import SwiftUI

private struct BenchmarkOption: Identifiable, Hashable {
  let id: String
  let label: String
  let defaultInputSize: String
}

struct ContentView: View {
  @State private var textViewText = ""
  @State private var isProveButtonEnabled = true
  @State private var selectedBenchmark = "private_tx"
  @State private var inputSize: String = "32"

  private static let benchmarks = [
    BenchmarkOption(id: "private_tx", label: "private_tx", defaultInputSize: "32"),
    BenchmarkOption(id: "constant_overhead", label: "constant_overhead", defaultInputSize: "1"),
    BenchmarkOption(id: "merkle_fake", label: "merkle_fake", defaultInputSize: "4"),
    BenchmarkOption(id: "hash_poseidon16", label: "hash_poseidon16", defaultInputSize: "128"),
    BenchmarkOption(id: "merkle_poseidon16", label: "merkle_poseidon16", defaultInputSize: "4"),
  ]

  var body: some View {
    VStack(spacing: 16) {
      Text("LeanVM Benchmarks")
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

      Button("Run LeanVM Benchmark", action: runLeanvmProveAction)
        .disabled(!isProveButtonEnabled)
        .buttonStyle(.borderedProminent)
        .accessibilityIdentifier("runLeanvmBenchmark")

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
  func runLeanvmProveAction() {
    guard let size = UInt64(inputSize), size > 0 else {
      textViewText += "Invalid input_size.\n"
      return
    }

    let benchmark = selectedOption()
    isProveButtonEnabled = false
    textViewText += "Running LeanVM \(benchmark.id) (input_size=\(size))...\n"

    DispatchQueue.global(qos: .userInitiated).async {
      let result: String
      switch benchmark.id {
      case "private_tx":
        result = leanvmProvePrivateTx(inputSize: size)
      case "constant_overhead":
        result = leanvmProveConstantOverhead(inputSize: size)
      case "merkle_fake":
        result = leanvmProveMerkleFake(inputSize: size)
      case "hash_poseidon16":
        result = leanvmProveHashPoseidon16(inputSize: size)
      case "merkle_poseidon16":
        result = leanvmProveMerklePoseidon16(inputSize: size)
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
