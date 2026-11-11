// WARNING: generated file — do not edit

#[derive(Debug, PartialEq, Eq)]
pub enum GatewayClassConditionType {
    Accepted,
}

impl std::fmt::Display for GatewayClassConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GatewayClassConditionReason {
    Accepted,
    InvalidParameters,
    Pending,
    Unsupported,
    Waiting,
}

impl std::fmt::Display for GatewayClassConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GatewayConditionType {
    AddressesAssigned,
    Programmed,
    InsecureFrontendValidationMode,
    Accepted,
    Scheduled,
    ResolvedRefs,
    Ready,
}

impl std::fmt::Display for GatewayConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GatewayConditionReason {
    AddressesAssigned,
    PartiallyAssigned,
    NotAssigned,
    Programmed,
    Invalid,
    NoResources,
    AddressNotAssigned,
    AddressNotUsable,
    ConfigurationChanged,
    Accepted,
    ListenersNotValid,
    Pending,
    UnsupportedAddress,
    InvalidParameters,
    Scheduled,
    NotReconciled,
    ResolvedRefs,
    InvalidClientCertificateRef,
    RefNotPermitted,
    ListenersNotResolved,
    Ready,
    ListenersNotReady,
}

impl std::fmt::Display for GatewayConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerConditionType {
    Conflicted,
    Accepted,
    Detached,
    ResolvedRefs,
    Programmed,
    OverlappingTLSConfig,
    Ready,
}

impl std::fmt::Display for ListenerConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerConditionReason {
    HostnameConflict,
    ProtocolConflict,
    NoConflicts,
    Accepted,
    Attached,
    PortUnavailable,
    UnsupportedProtocol,
    NoValidCACertificate,
    UnsupportedValue,
    ResolvedRefs,
    InvalidCertificateRef,
    InvalidRouteKinds,
    RefNotPermitted,
    InvalidCACertificateRef,
    InvalidCACertificateKind,
    Programmed,
    Invalid,
    Pending,
    OverlappingHostnames,
    OverlappingCertificates,
    Ready,
}

impl std::fmt::Display for ListenerConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerSetConditionType {
    Programmed,
    Accepted,
}

impl std::fmt::Display for ListenerSetConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerSetConditionReason {
    Programmed,
    Accepted,
    NotAllowed,
    ParentNotAccepted,
    ListenersNotValid,
    Invalid,
    Pending,
}

impl std::fmt::Display for ListenerSetConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerEntryConditionType {
    Conflicted,
    Accepted,
    ResolvedRefs,
    Programmed,
    Ready,
}

impl std::fmt::Display for ListenerEntryConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerEntryConditionReason {
    HostnameConflict,
    ProtocolConflict,
    ListenerConflict,
    Accepted,
    UnsupportedProtocol,
    TooManyListeners,
    ResolvedRefs,
    InvalidCertificateRef,
    InvalidRouteKinds,
    RefNotPermitted,
    Programmed,
    PortUnavailable,
    Pending,
    Invalid,
    Ready,
}

impl std::fmt::Display for ListenerEntryConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RouteConditionType {
    Accepted,
    ResolvedRefs,
    PartiallyInvalid,
}

impl std::fmt::Display for RouteConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RouteConditionReason {
    Accepted,
    NotAllowedByListeners,
    NoMatchingListenerHostname,
    NoMatchingParent,
    UnsupportedValue,
    Pending,
    IncompatibleFilters,
    ResolvedRefs,
    RefNotPermitted,
    InvalidKind,
    BackendNotFound,
    UnsupportedProtocol,
}

impl std::fmt::Display for RouteConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PolicyConditionType {
    Accepted,
    ResolvedRefs,
}

impl std::fmt::Display for PolicyConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PolicyConditionReason {
    Accepted,
    Conflicted,
    Invalid,
    TargetNotFound,
    NoValidCACertificate,
    ResolvedRefs,
    InvalidCACertificateRef,
    InvalidKind,
}

impl std::fmt::Display for PolicyConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
