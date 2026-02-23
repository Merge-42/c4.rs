use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Stable identifier for a C4 element, auto-generated from the element name.
///
/// Each element gets a unique `ElementId` at build time. The id contains a
/// name-derived slug (used for DSL output) and a unique internal sequence
/// number (used to distinguish elements with the same name).
///
/// Users cannot construct this directly — it is created automatically when
/// building an element via its builder. Use `element.id()` to obtain a
/// reference for passing into relationship declarations.
#[derive(Debug, Clone)]
pub struct ElementId {
    slug: String,
    seq: u64,
}

impl ElementId {
    pub(crate) fn from_name(name: &str) -> Self {
        Self {
            slug: name
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.chars()
                        .next()
                        .unwrap_or_default()
                        .to_lowercase()
                        .to_string()
                })
                .collect(),
            seq: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.slug
    }
}

impl PartialEq for ElementId {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for ElementId {}

impl std::hash::Hash for ElementId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.seq.hash(state);
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slug)
    }
}

pub trait Element {
    fn id(&self) -> &ElementId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn element_type(&self) -> ElementType;
    fn location(&self) -> Location;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementType {
    Person,
    SoftwareSystem,
    Container,
    Component,
    Code,
}

impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementType::Person => write!(f, "Person"),
            ElementType::SoftwareSystem => write!(f, "SoftwareSystem"),
            ElementType::Container => write!(f, "Container"),
            ElementType::Component => write!(f, "Component"),
            ElementType::Code => write!(f, "Code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Location {
    #[default]
    Internal,
    External,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Internal => write!(f, "Internal"),
            Location::External => write!(f, "External"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerType {
    WebApplication,
    DesktopApplication,
    MobileApplication,
    Database,
    FileSystem,
    Api,
    MessageBus,
    Other(String),
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainerType::WebApplication => write!(f, "Web Application"),
            ContainerType::DesktopApplication => write!(f, "Desktop Application"),
            ContainerType::MobileApplication => write!(f, "Mobile Application"),
            ContainerType::Database => write!(f, "Database"),
            ContainerType::FileSystem => write!(f, "File System"),
            ContainerType::Api => write!(f, "API"),
            ContainerType::MessageBus => write!(f, "Message Bus"),
            ContainerType::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InteractionStyle {
    #[default]
    Synchronous,
    Asynchronous,
    Bidirectional,
}

impl fmt::Display for InteractionStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractionStyle::Synchronous => write!(f, "Synchronous"),
            InteractionStyle::Asynchronous => write!(f, "Asynchronous"),
            InteractionStyle::Bidirectional => write!(f, "Bidirectional"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeType {
    Class,
    Struct,
    Function,
    Trait,
    Module,
    Enum,
}

impl fmt::Display for CodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeType::Class => write!(f, "Class"),
            CodeType::Struct => write!(f, "Struct"),
            CodeType::Function => write!(f, "Function"),
            CodeType::Trait => write!(f, "Trait"),
            CodeType::Module => write!(f, "Module"),
            CodeType::Enum => write!(f, "Enum"),
        }
    }
}
