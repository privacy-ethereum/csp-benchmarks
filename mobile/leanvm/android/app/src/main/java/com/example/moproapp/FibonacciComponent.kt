package com.example.moproapp

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedCard
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.mopro.Halo2ProofResult
import uniffi.mopro.generateHalo2Proof
import uniffi.mopro.verifyHalo2Proof

@Composable
fun FibonacciComponent() {
    var provingTime by remember { mutableStateOf<String?>(null) }
    var verifyingTime by remember { mutableStateOf<String?>(null) }
    var valid by remember { mutableStateOf<String?>(null) }
    var isGeneratingProof by remember { mutableStateOf(false) }
    var isVerifyingProof by remember { mutableStateOf(false) }
    var res by remember {
        mutableStateOf<Halo2ProofResult>(
            Halo2ProofResult(proof = ByteArray(size = 0), inputs = ByteArray(size = 0))
        )
    }

    val srsPath = getFilePathFromAssets("plonk_fibonacci_srs.bin")
    val provingKeyPath = getFilePathFromAssets("plonk_fibonacci_pk.bin")
    val verifyingKeyPath = getFilePathFromAssets("plonk_fibonacci_vk.bin")

    val inputs = mutableMapOf<String, List<String>>()
    inputs["out"] = listOf("55")

    val scrollState = rememberScrollState()
    val isBusy = isGeneratingProof || isVerifyingProof

    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(scrollState)
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Text(
            text = "Halo2 Fibonacci",
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
            fontSize = 22.sp
        )
        Text(
            text = "Proves the 10th Fibonacci number is 55 using Plonk.",
            style = MaterialTheme.typography.bodyMedium,
            textAlign = TextAlign.Center,
            modifier = Modifier.padding(bottom = 8.dp)
        )

        OutlinedCard(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.outlinedCardColors()
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                if (isBusy) {
                    CircularProgressIndicator(modifier = Modifier.padding(8.dp))
                    Text(
                        text = if (isGeneratingProof) "Generating proof…" else "Verifying…",
                        style = MaterialTheme.typography.bodyMedium
                    )
                }

                Button(
                    onClick = {
                        isGeneratingProof = true
                        provingTime = null
                        valid = null
                        verifyingTime = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                res = generateHalo2Proof(srsPath, provingKeyPath, inputs)
                                val endTime = System.currentTimeMillis()
                                provingTime = "${endTime - startTime} ms"
                            } finally {
                                isGeneratingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("halo2GenerateProofButton"),
                    enabled = !isBusy
                ) {
                    Text("Generate proof")
                }

                Button(
                    onClick = {
                        isVerifyingProof = true
                        verifyingTime = null
                        Thread {
                            try {
                                val startTime = System.currentTimeMillis()
                                val result = verifyHalo2Proof(srsPath, verifyingKeyPath, res.proof, res.inputs)
                                val endTime = System.currentTimeMillis()
                                verifyingTime = "${endTime - startTime} ms"
                                valid = result.toString()
                            } finally {
                                isVerifyingProof = false
                            }
                        }.start()
                    },
                    modifier = Modifier.fillMaxWidth().testTag("halo2VerifyProofButton"),
                    enabled = !isBusy && res.proof.isNotEmpty()
                ) {
                    Text("Verify proof")
                }
            }
        }

        if (provingTime != null || verifyingTime != null || valid != null) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceVariant)
            ) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    Text("Results", fontWeight = FontWeight.SemiBold, fontSize = 16.sp)
                    provingTime?.let { Text("Proving: $it", style = MaterialTheme.typography.bodyMedium) }
                    verifyingTime?.let { Text("Verifying: $it", style = MaterialTheme.typography.bodyMedium) }
                    valid?.let { Text("Valid: $it", style = MaterialTheme.typography.bodyMedium, fontWeight = if (it == "true") FontWeight.Bold else FontWeight.Normal) }
                }
            }
        }

        Spacer(modifier = Modifier.height(24.dp))
    }
}
