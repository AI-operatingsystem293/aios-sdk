pub struct Response {
    pub success: bool,
    pub output: String,
}

impl Response {
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            success: true,
            output: output.into(),
        }
    }

    pub fn error(output: impl Into<String>) -> Self {
        Self {
            success: false,
            output: output.into(),
        }
    }
}
