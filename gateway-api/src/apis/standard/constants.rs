// WARNING: generated file - manual changes will be overriden

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
