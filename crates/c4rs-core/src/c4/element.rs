use serde::{Deserialize, Serialize};
use std::fmt;

/// Trait representing any C4 model element.
///
/// All C4 elements (Person, SoftwareSystem, Container, Component, CodeElement)
/// implement this trait to provide common accessors.
pub trait Element {
    /// Returns the element's name.
    fn name(&self) -> &str;

    /// Returns the element's description.
    fn description(&self) -> &str;

    /// Returns the element's type classification.
    fn element_type(&self) -> ElementType;

    /// Returns whether the element is internal or external.
    fn location(&self) -> Location;
}

/// Classification of C4 element types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementType {
    /// Person - users and actors in the system
    Person,
    /// SoftwareSystem - the overall software being described
    SoftwareSystem,
    /// Container - deployable units within a software system
    Container,
    /// Component - logical groupings of code within a container
    Component,
    /// CodeElement - individual code units (classes, functions, etc.)
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

/// Location classification for elements (internal vs external).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Location {
    /// Elements within the system boundary being described
    #[default]
    Internal,
    /// Elements outside the system boundary
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

/// Types of containers in the C4 model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerType {
    /// Web application running in a browser
    WebApplication,
    /// Desktop application
    DesktopApplication,
    /// Mobile application
    MobileApplication,
    /// Database storage system
    Database,
    /// File system storage
    FileSystem,
    /// API or service interface
    Api,
    /// Message bus or queue
    MessageBus,
    /// Other container type
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

/// Interaction style between elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InteractionStyle {
    /// Synchronous request-response interaction
    #[default]
    Synchronous,
    /// Asynchronous message passing
    Asynchronous,
    /// Bidirectional communication
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

/// Types of code elements in the C4 model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeType {
    /// Class definition
    Class,
    /// Struct definition
    Struct,
    /// Function or method
    Function,
    /// Trait definition
    Trait,
    /// Module
    Module,
    /// Enum definition
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
