fn into_token_stream(mut self) -> proc_macro::TokenStream {
        self.evaluate_now();
        self.stream
    }