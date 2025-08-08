use std::{
    borrow::Cow,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

/// Build a route in the fs.
pub trait Route {
    /// Build `self` at the given path.
    fn build(&self, path: &Path) -> io::Result<()>;
}

impl<T> Route for &T
where
    T: Route + ?Sized,
{
    fn build(&self, path: &Path) -> io::Result<()> {
        T::build(self, path)
    }
}

impl Route for str {
    fn build(&self, path: &Path) -> io::Result<()> {
        build_file_route_impl(path, self.as_ref())
    }
}

impl Route for String {
    fn build(&self, path: &Path) -> io::Result<()> {
        self.as_str().build(path)
    }
}

fn build_file_route_impl(path: &Path, data: &[u8]) -> io::Result<()> {
    let path = to_final_path(path);

    fs::create_dir_all(&path.parent().unwrap())?;
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    file.write_all(data)?;
    file.flush()
}

fn to_final_path(path: &Path) -> Cow<'_, Path> {
    if path.extension().is_some() {
        path.into()
    } else {
        path.join("index.html").into()
    }
}

pub struct ServeDir(pub PathBuf);

impl Route for ServeDir {
    fn build(&self, path: &Path) -> io::Result<()> {
        walk_dir(&self.0, &mut |entry_path| {
            let dest = path.join(entry_path.strip_prefix(&self.0).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).expect("failed to create dir");
            fs::copy(entry_path, dest).expect("failed to copy files");
        })
    }
}

fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, cb)?;
        } else {
            cb(&path)
        }
    }

    Ok(())
}
