use derive_more::derive::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From, Debug)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[from]
    Template(tera::Error),
}
