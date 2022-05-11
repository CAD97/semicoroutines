use {proc_macro::TokenStream, watt::WasmMacro};

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("impl.wasm");

#[proc_macro]
pub fn co(input: TokenStream) -> TokenStream {
    MACRO.proc_macro("co", input)
}
