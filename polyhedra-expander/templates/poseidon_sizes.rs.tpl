// Auto-generated. Do not edit.

// BEGIN_DECL
declare_circuit!(PoseidonCircuit{{LEN}} { input: [Variable; {{LEN}}], });
impl Define<M31SingleConfig> for PoseidonCircuit{{LEN}}<Variable> {
    fn define<Builder: RootAPI<M31SingleConfig>>(&self, api: &mut Builder) {
        api.memorized_simple_call(|api, inputs| check_poseidon(api, inputs), &self.input);
    }
}
// END_DECL

{{CIRCUIT_DECLS}}

macro_rules! match_poseidon_sizes {
    ($input_len:expr, $arm:ident) => { match $input_len {
{{MATCH_ARMS}}
        _ => panic!("unsupported input length: {}", $input_len),
    }};
}

// BEGIN_MATCH_ARM
        {{LEN}} => $arm!(PoseidonCircuit{{LEN}}, {{LEN}}),
// END_MATCH_ARM


