// WARNING: generated file — do not edit

channel!(standard {
    backendtlspolicies: BackendTlsPolicy { v1 => "backendtlspolicies.rs", v1alpha3 => "backendtlspolicies.rs" }
    gatewayclasses: GatewayClass { v1 => "gatewayclasses.rs", v1beta1 => "gatewayclasses.rs" }
    gateways: Gateway { v1 => "gateways.rs", v1beta1 => "gateways.rs" }
    grpcroutes: GrpcRoute { v1 => "grpcroutes.rs" }
    httproutes: HttpRoute { v1 => "httproutes.rs", v1beta1 => "httproutes.rs" }
    listenersets: ListenerSet { v1 => "listenersets.rs" }
    referencegrants: ReferenceGrant { v1 => "referencegrants.rs", v1beta1 => "referencegrants.rs" }
    tlsroutes: TlsRoute { v1 => "tlsroutes.rs", v1alpha2 => "tlsroutes_v1alpha2.rs", v1alpha3 => "tlsroutes.rs" }
});

#[cfg(feature = "experimental")]
channel!(experimental {
    backendtlspolicies: BackendTlsPolicy { v1 => "backendtlspolicies.rs", v1alpha3 => "backendtlspolicies.rs" }
    gatewayclasses: GatewayClass { v1 => "gatewayclasses.rs", v1beta1 => "gatewayclasses.rs" }
    gateways: Gateway { v1 => "gateways.rs", v1beta1 => "gateways.rs" }
    grpcroutes: GrpcRoute { v1 => "grpcroutes.rs" }
    httproutes: HttpRoute { v1 => "httproutes.rs", v1beta1 => "httproutes.rs" }
    listenersets: ListenerSet { v1 => "listenersets.rs" }
    referencegrants: ReferenceGrant { v1 => "referencegrants.rs", v1beta1 => "referencegrants.rs" }
    tcproutes: TcpRoute { v1alpha2 => "tcproutes.rs" }
    tlsroutes: TlsRoute { v1 => "tlsroutes.rs", v1alpha2 => "tlsroutes_v1alpha2.rs", v1alpha3 => "tlsroutes.rs" }
    udproutes: UdpRoute { v1alpha2 => "udproutes.rs" }
    xbackendtrafficpolicies: XBackendTrafficPolicy { v1alpha1 => "xbackendtrafficpolicies.rs" }
    xmeshes: XMesh { v1alpha1 => "xmeshes.rs" }
});
