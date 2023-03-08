pub mod many_arguments {
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct BigStruct<'a> {
        a1: &'a str,
        a2: &'a str,
        a3: &'a str,
        a4: &'a str,
        a5: &'a str,
        a6: &'a str,
        a7: &'a str,
        a8: &'a str,
        a9: &'a str,
        a10: &'a str,
        a11: &'a str,
        a12: &'a str,
        a13: &'a str,
        a14: &'a str,
        a15: &'a str,
        a16: &'a str,
        a17: &'a str,
        a18: &'a str,
        a19: &'a str,
        a20: &'a str,
    }
    pub async fn many_args(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
        a11: u64,
        a12: u64,
        a13: u64,
        a14: u64,
        a15: u64,
        a16: u64,
    ) -> () {
        todo!()
    }
    pub async fn big_argument(x: BigStruct<'_>) -> () {
        todo!()
    }
}
