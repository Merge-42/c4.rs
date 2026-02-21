use serde::{Deserialize, Serialize};
use std::fmt;

/// Common trait for all C4 elements.
pub trait Element {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn element_type(&self) -> ElementType;
    fn location(&self) -> Location;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
