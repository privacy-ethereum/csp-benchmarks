// Run with: node circom/tests/hash-circuits.cjs <compiled-circuit-directory> [...]
// Requires snarkjs 0.7.5 (local or global); uses its js-sha3 dependency.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const { createRequire } = require('node:module');
const { execFileSync } = require('node:child_process');
let resolveFrom = require;
try {
    require.resolve('snarkjs');
} catch {
    const globalRoot = execFileSync('npm', ['root', '-g'], { encoding: 'utf8' }).trim();
    resolveFrom = createRequire(path.join(globalRoot, 'snarkjs', 'package.json'));
}
const snarkjs = resolveFrom('snarkjs');
const { keccak256 } = createRequire(resolveFrom.resolve('snarkjs'))('js-sha3');
const quiet = { info() {}, debug() {}, warn() {}, error() {} };

async function checkBitwise(directory, calculator) {
    const r1cs = await snarkjs.r1cs.exportJson(path.join(directory, 'bitwise.r1cs'), quiet);
    const prime = BigInt(r1cs.prime);
    const mod = x => ((x % prime) + prime) % prime;
    const evaluate = (lc, witness) => Object.entries(lc).reduce(
        (sum, [wire, coefficient]) => mod(sum + BigInt(coefficient) * witness[wire]), 0n
    );
    const satisfies = witness => r1cs.constraints.every(([a, b, c]) =>
        mod(evaluate(a, witness) * evaluate(b, witness) - evaluate(c, witness)) === 0n
    );
    for (let v = 0; v < 8; v++) {
        const [a, b, c] = [v & 1, (v >> 1) & 1, (v >> 2) & 1];
        const witness = await calculator.calculateWitness({ bits: [a, b, c] }, true);
        assert.deepEqual(witness.slice(1, 4).map(Number), [a ^ b ^ c, (a & b) ^ (a & c) ^ (b & c), a ^ ((1 - b) & c)]);
        assert(satisfies(witness));
        for (let wire = 1; wire <= 3; wire++) {
            const forged = witness.slice();
            forged[wire] += 1n;
            assert.equal(satisfies(forged), false);
        }
        // The coefficient of the output is nonzero in each single-row identity,
        // establishing uniqueness over the full field, not just boolean outputs.
        for (const coefficient of [a + b - 4*c + 1, a + b + 6*c - 4, 4*a + b + c - 3]) {
            assert.notEqual(mod(BigInt(coefficient)), 0n);
        }
    }
    await assert.rejects(calculator.calculateWitness({ bits: [2, 0, 0] }, true));
    console.log('bitwise: all 8 truth-table rows passed; output uniqueness and forged witnesses checked');
}

async function check(directory) {
    directory = path.resolve(directory);
    const name = path.basename(directory);
    const js = path.join(directory, `${name}_js`);
    const calculator = await require(path.join(js, 'witness_calculator.js'))(
        fs.readFileSync(path.join(js, `${name}.wasm`))
    );
    if (name === 'bitwise') return checkBitwise(directory, calculator);
    const [, algorithm, sizeText] = /^(sha256|keccak)_(\d+)$/.exec(name) || [];
    assert(algorithm, `Expected sha256_<bytes> or keccak_<bytes>: ${name}`);
    const size = Number(sizeText);
    const digest = message => Array.from(Buffer.from(
        algorithm === 'sha256'
            ? crypto.createHash('sha256').update(message).digest('hex')
            : keccak256(message), 'hex'
    ));
    const vectors = [
        Buffer.alloc(size),
        Buffer.alloc(size, 255),
        Buffer.from(Array.from({ length: size }, (_, i) => (i * 73 + 19) & 255)),
    ];
    const witnessPath = path.join(directory, 'checked.wtns');
    for (const message of vectors) {
        const hash = digest(message);
        const input = { in: Array.from(message), hash };
        const witness = await calculator.calculateWitness(input, true);
        assert.deepEqual(witness.slice(1, 33).map(Number), hash);
        fs.writeFileSync(witnessPath, await calculator.calculateWTNSBin(input, true));
        assert(await snarkjs.wtns.check(path.join(directory, `${name}.r1cs`), witnessPath, quiet));
    }

    const message = vectors[2];
    const hash = digest(message);
    const wrongHash = hash.slice();
    wrongHash[0] ^= 1;
    await assert.rejects(calculator.calculateWitness({ in: Array.from(message), hash: wrongHash }, true));
    if (size > 0) {
        const changed = Array.from(message);
        changed[0] ^= 1;
        await assert.rejects(calculator.calculateWitness({ in: changed, hash }, true));
        for (const invalid of [256, 511, -1]) {
            const invalidBytes = Array.from(message);
            invalidBytes[0] = invalid;
            // Supply the digest of the low byte so rejection depends on the
            // byte constraint, even if a faulty calculator truncates its input.
            const truncated = Buffer.from(message);
            const canonical = (BigInt(invalid) + calculator.prime) % calculator.prime;
            truncated[0] = Number(canonical & 255n);
            await assert.rejects(calculator.calculateWitness({ in: invalidBytes, hash: digest(truncated) }, true));
        }
    }

    // Check that the R1CS rejects a forged digest even without witness-generator
    // assertions. In these wrappers hash aliases out after optimization.
    const binary = fs.readFileSync(witnessPath);
    let offset = 12;
    let width;
    while (offset < binary.length) {
        const type = binary.readUInt32LE(offset);
        const length = Number(binary.readBigUInt64LE(offset + 4));
        offset += 12;
        if (type === 1) width = binary.readUInt32LE(offset);
        if (type === 2) {
            binary[offset + width] ^= 1; // wire 1 is out[0]
            break;
        }
        offset += length;
    }
    const forged = path.join(directory, 'forged.wtns');
    fs.writeFileSync(forged, binary);
    assert.equal(await snarkjs.wtns.check(path.join(directory, `${name}.r1cs`), forged, quiet), false);
    fs.rmSync(forged);
    console.log(`${name}: 3 reference digests and R1CS checks passed; invalid inputs and forged digest rejected`);
}

(async () => {
    assert(process.argv.length > 2, 'Pass at least one compiled circuit directory');
    for (const directory of process.argv.slice(2)) await check(directory);
})().then(() => process.exit(0), error => { console.error(error); process.exit(1); });
