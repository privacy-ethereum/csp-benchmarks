package com.example.moproapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import uniffi.mopro.leanvmProvePrivateTx
import java.io.File
import java.io.IOException
import java.io.InputStream
import java.io.OutputStream

@Throws(IOException::class)
fun copyFile(inputStream: InputStream, outputStream: OutputStream) {
    val buffer = ByteArray(1024)
    var read: Int
    while (inputStream.read(buffer).also { read = it } != -1) {
        outputStream.write(buffer, 0, read)
    }
}

@Composable
fun getFilePathFromAssets(name: String): String {
    val context = LocalContext.current
    return remember {
        val inputStream = context.assets.open(name)
        val file = File(context.filesDir, name)
        copyFile(inputStream, file.outputStream())
        file.absolutePath
    }
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LeanvmBenchmarkScreen()
        }
    }
}

@Composable
fun LeanvmBenchmarkScreen() {
    val scope = rememberCoroutineScope()
    var merkleDepth by remember { mutableStateOf("32") }
    var logText by remember { mutableStateOf("") }
    var isRunning by remember { mutableStateOf(false) }
    val scrollState = rememberScrollState()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Text("LeanVM Private TX Benchmark", style = MaterialTheme.typography.headlineSmall)

        Row(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
        ) {
            Text("Merkle depth:")
            OutlinedTextField(
                value = merkleDepth,
                onValueChange = { merkleDepth = it },
                modifier = Modifier.width(100.dp),
                singleLine = true,
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number)
            )
        }

        Button(
            onClick = {
                val depth = merkleDepth.toULongOrNull()
                if (depth == null || depth == 0UL) {
                    logText += "Invalid Merkle depth.\n"
                    return@Button
                }
                isRunning = true
                logText += "Running LeanVM private TX proof (depth=$depth)...\n"
                scope.launch(Dispatchers.Default) {
                    val result = leanvmProvePrivateTx(inputSize = depth)
                    var proveMs = "?"
                    for (part in result.split(",")) {
                        val kv = part.split("=", limit = 2)
                        if (kv.size == 2 && kv[0] == "prove_time_ms") proveMs = kv[1]
                    }
                    launch(Dispatchers.Main) {
                        logText += "  prove: $proveMs ms\n"
                        isRunning = false
                    }
                }
            },
            enabled = !isRunning
        ) {
            Text("Prove LeanVM Private TX")
        }

        Box(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(scrollState)
        ) {
            Text(logText, modifier = Modifier.padding(4.dp))
        }
    }
}
