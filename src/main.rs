mod chunk_type;
mod chunk;
mod png;
mod chunk_type_error;
mod chunk_error;
mod png_error;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}

