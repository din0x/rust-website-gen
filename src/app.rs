use crate::Route;
use std::{collections::HashMap, fs, io, path::Path, rc::Rc};

/// A static website builder.
#[derive(Clone, Default)]
pub struct App {
    routes: HashMap<String, Rc<dyn Route>>,
}

impl App {
    /// Returns an empty app.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add another route to the app.
    // TODO: Should this method take `Self`?
    #[track_caller]
    pub fn route(&mut self, path: impl AsRef<str>, route: impl Route + 'static) -> &mut Self {
        self.add_route(path.as_ref(), Rc::new(route));
        self
    }

    #[track_caller]
    fn add_route(&mut self, path: &str, route: Rc<dyn Route>) {
        let Some(path) = self.to_final_route(path) else {
            panic!("'{}' is not a valid route", path.escape_default());
        };

        self.routes.insert(path.to_owned(), route);
    }

    /// Build the app in the given directory.
    #[track_caller]
    pub fn build(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.build_impl(path.as_ref())
    }

    #[track_caller]
    fn build_impl(&self, prefix: &Path) -> io::Result<()> {
        if prefix.is_absolute() {
            // This is safety measure so you don't wipe your system.
            panic!("the output path MUST be relative")
        }

        for entry in fs::read_dir(prefix)? {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                fs::remove_dir_all(entry.path())?;
            } else {
                fs::remove_file(entry.path())?;
            }
        }

        Route::build(self, prefix)
    }

    fn to_final_route(&self, path: &str) -> Option<String> {
        let path = path.strip_prefix("/")?;

        if self.routes.keys().any(|key| key == &path) {
            return None;
        }

        Some(path.to_owned())
    }
}

impl Route for App {
    fn build(&self, prefix: &Path) -> io::Result<()> {
        fs::create_dir_all(prefix)?;

        for (path, route) in &self.routes {
            let full_path = prefix.join(path.trim_start_matches("/"));
            fs::create_dir_all(&full_path.parent().unwrap())?;
            route.build(&full_path)?;
        }

        Ok(())
    }
}
