pub type Config = bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::NoLimit,
>;

pub type EncResult = Result<(), bincode::error::EncodeError>;
pub type DecResult<T> = Result<T, bincode::error::DecodeError>;

pub const fn bincode_config() -> Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_no_limit()
}