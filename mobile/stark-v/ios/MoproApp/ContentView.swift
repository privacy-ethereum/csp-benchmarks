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
      Text("Stark-V Private TX Benchmark")
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

      Button("Prove Stark-V Private TX", action: runStarkVProveAction)
        .disabled(!isProveButtonEnabled)
        .buttonStyle(.borderedProminent)
        .accessibilityIdentifier("proveStarkVPrivateTx")

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
    guard let depth = UInt64(merkleDepth), depth > 0 else {
      textViewText += "Invalid Merkle depth.\n"
      return
    }

    isProveButtonEnabled = false
    textViewText += "Running Stark-V private TX proof (depth=\(depth))...\n"

    DispatchQueue.global(qos: .userInitiated).async {
      guard let binPath = Bundle.main.path(forResource: "private_tx", ofType: "bin") else {
        DispatchQueue.main.async {
          textViewText += "Error: private_tx.bin not found in app bundle.\n"
          isProveButtonEnabled = true
        }
        return
      }

      let result = starkVProvePrivateTx(inputSize: depth, compiledProgramPath: binPath)

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
