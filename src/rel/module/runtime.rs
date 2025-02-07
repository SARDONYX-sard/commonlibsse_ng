#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Runtime {
    /// Unknown runtime
    #[default]
    Unknown = 0,
    /// The Skyrim runtime is a post-Anniversary Edition Skyrim SE release (version 1.6.x and later).
    Ae = 1,
    /// The Skyrim runtime is a pre-Anniversary Edition Skyrim SE release (version 1.5.97 and prior).
    Se = 1 << 1,
    /// The Skyrim runtime is Skyrim VR.
    Vr = 1 << 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_enum() {
        assert_eq!(Runtime::Unknown as u8, 0);
        assert_eq!(Runtime::Ae as u8, 1);
        assert_eq!(Runtime::Se as u8, 2);
        assert_eq!(Runtime::Vr as u8, 4);
    }
}
