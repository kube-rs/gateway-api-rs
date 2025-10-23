// WARNING: generated file - manual changes will be overriden

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
    Programmed,
    Accepted,
    Ready,
}
impl std::fmt::Display for GatewayConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum GatewayConditionReason {
    Programmed,
    Invalid,
    NoResources,
    AddressNotAssigned,
    AddressNotUsable,
    Accepted,
    ListenersNotValid,
    Pending,
    UnsupportedAddress,
    InvalidParameters,
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
    ResolvedRefs,
    Programmed,
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
    PortUnavailable,
    UnsupportedProtocol,
    ResolvedRefs,
    InvalidCertificateRef,
    InvalidRouteKinds,
    RefNotPermitted,
    Programmed,
    Invalid,
    Pending,
    Ready,
}
impl std::fmt::Display for ListenerConditionReason {
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
