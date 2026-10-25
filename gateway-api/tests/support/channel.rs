/// An upstream CRD channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Channel {
    Standard,
    Experimental,
}

impl Channel {
    pub const ALL: [Self; 2] = [Self::Standard, Self::Experimental];
    /// The channels this build checks; `experimental` needs its feature.
    pub const BUILT: &[Self] = &[
        Self::Standard,
        #[cfg(feature = "experimental")]
        Self::Experimental,
    ];

    /// The channel's module and fixture directory name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Experimental => "experimental",
        }
    }

    /// The channel whose module is `name`, e.g. from `stringify!` in an
    /// [`apis!`] callback.
    pub fn named(name: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|c| c.name() == name)
            .unwrap_or_else(|| panic!("unknown channel {name}"))
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
