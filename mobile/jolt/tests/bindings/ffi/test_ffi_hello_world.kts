import uniffi.jolt.*

var helloWorld = moproHelloWorld()
assert(helloWorld == "Hello, World!") { "Test string mismatch" }
