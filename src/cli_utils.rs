use std::io::Write;

pub trait PasswordPrompt {
    fn prompt(&self, message: &str) -> anyhow::Result<String>;
}

pub struct RPasswordPrompt;

impl PasswordPrompt for RPasswordPrompt {
    fn prompt(&self, message: &str) -> anyhow::Result<String> {
        print!("{}", message);
        std::io::stdout().flush()?;
        let pass = rpassword::read_password()?;
        Ok(pass)
    }
}

pub struct MockPasswordPrompt {
    pub responses: std::cell::RefCell<Vec<String>>,
}

impl PasswordPrompt for MockPasswordPrompt {
    fn prompt(&self, _message: &str) -> anyhow::Result<String> {
        let mut responses = self.responses.borrow_mut();
        if responses.is_empty() {
            return Err(anyhow::anyhow!("No more mock responses"));
        }
        Ok(responses.remove(0))
    }
}
