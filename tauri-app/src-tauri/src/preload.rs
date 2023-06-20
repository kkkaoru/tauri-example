use std::path::PathBuf;

use tauri::{
    generate_handler,
    plugin::{Plugin, Result},
    Invoke, Runtime,
};

pub struct PreloadPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
    app_dir: PathBuf,
}

impl<R: Runtime> PreloadPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(generate_handler![]),
            app_dir: PathBuf::new(),
        }
    }
}

impl<R: Runtime> Plugin<R> for PreloadPlugin<R> {
    fn name(&self) -> &'static str {
        "preload"
    }

    fn initialize(&mut self, app: &tauri::AppHandle<R>, _: serde_json::Value) -> Result<()> {
        self.app_dir = app.path_resolver().app_dir().unwrap();
        Ok(())
    }

    fn initialization_script(&self) -> Option<String> {
        Some(format!(
            r#"
                window.addEventListener('DOMContentLoaded', () => {{
                    console.log('hello world from preload');
                }})
            "#
        ))
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
