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
      Text("RISC0 Private TX Benchmark")
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

      Button("Prove RISC0 Private TX", action: runRisc0ProveAction)
        .disabled(!isProveButtonEnabled)
        .buttonStyle(.borderedProminent)
        .accessibilityIdentifier("proveRisc0PrivateTx")

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
  func runRisc0ProveAction() {
    guard let depth = UInt64(merkleDepth), depth > 0 else {
      textViewText += "Invalid Merkle depth.\n"
      return
    }

    isProveButtonEnabled = false
    textViewText += "Running RISC0 private TX proof (depth=\(depth))...\n"

    DispatchQueue.global(qos: .userInitiated).async {
      guard let binPath = Bundle.main.path(forResource: "private_tx", ofType: "bin") else {
        DispatchQueue.main.async {
          textViewText += "Error: private_tx.bin not found in app bundle.\n"
          isProveButtonEnabled = true
        }
        return
      }
      let result = risc0ProvePrivateTx(inputSize: depth, compiledProgramPath: binPath)

      // Parse "prove_time_ms=<N>"
      var proveMs: String = "?"
      for part in result.split(separator: ",") {
        let kv = part.split(separator: "=", maxSplits: 1)
        if kv.count == 2 && kv[0] == "prove_time_ms" {
          proveMs = String(kv[1])
        }
      }

      DispatchQueue.main.async {
        textViewText += "  prove: \(proveMs) ms\n"
        isProveButtonEnabled = true
      }
    }
  }
}
