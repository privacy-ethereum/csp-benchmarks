//
//  ContentView.swift
//  MoproApp
//
import SwiftUI

struct ContentView: View {
  @State private var textViewText = ""
  @State private var isProveButtonEnabled = true
  @State private var merkleDepth: String = "32"

  var body: some View {
    VStack(spacing: 16) {
      Text("LeanVM Private TX Benchmark")
        .font(.headline)

      HStack {
        Text("Merkle depth:")
        TextField("depth", text: $merkleDepth)
          #if canImport(UIKit)
          .keyboardType(.numberPad)
          #endif
          .textFieldStyle(.roundedBorder)
          .frame(width: 80)
      }

      Button("Prove LeanVM Private TX", action: runLeanvmProveAction)
        .disabled(!isProveButtonEnabled)
        .buttonStyle(.borderedProminent)
        .accessibilityIdentifier("proveLeanvmPrivateTx")

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
    guard let depth = UInt64(merkleDepth), depth > 0 else {
      textViewText += "Invalid Merkle depth.\n"
      return
    }

    isProveButtonEnabled = false
    textViewText += "Running LeanVM private TX proof (depth=\(depth))...\n"

    DispatchQueue.global(qos: .userInitiated).async {
      let result = leanvmProvePrivateTx(inputSize: depth)

      var proveMs: String = "?"
      var samplesMs: String = "?"
      for part in result.split(separator: ",") {
        let kv = part.split(separator: "=", maxSplits: 1)
        if kv.count == 2 && kv[0] == "prove_time_ms" {
          proveMs = String(kv[1])
        }
        if kv.count == 2 && kv[0] == "samples_ms" {
          samplesMs = String(kv[1])
        }
      }

      DispatchQueue.main.async {
        textViewText += "  prove mean: \(proveMs) ms\n"
        textViewText += "  samples: \(samplesMs)\n"
        isProveButtonEnabled = true
      }
    }
  }
}
