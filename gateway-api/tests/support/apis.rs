// WARNING: generated file — do not edit

/// Calls `$callback!` once per channel this build checks, with each
/// resource's root type and every API version its CRD defines, e.g.
/// `$callback!(standard { httproutes: HttpRoute [v1, v1beta1] ... })`.
#[macro_export]
macro_rules! apis {
    ($callback:ident) => {
        $callback!(standard {
            backendtlspolicies: BackendTlsPolicy [v1, v1alpha3]
            gatewayclasses: GatewayClass [v1, v1beta1]
            gateways: Gateway [v1, v1beta1]
            grpcroutes: GrpcRoute [v1]
            httproutes: HttpRoute [v1, v1beta1]
            listenersets: ListenerSet [v1]
            referencegrants: ReferenceGrant [v1, v1beta1]
            tlsroutes: TlsRoute [v1, v1alpha2, v1alpha3]
        });
        #[cfg(feature = "experimental")]
        $callback!(experimental {
            backendtlspolicies: BackendTlsPolicy [v1, v1alpha3]
            gatewayclasses: GatewayClass [v1, v1beta1]
            gateways: Gateway [v1, v1beta1]
            grpcroutes: GrpcRoute [v1]
            httproutes: HttpRoute [v1, v1beta1]
            listenersets: ListenerSet [v1]
            referencegrants: ReferenceGrant [v1, v1beta1]
            tcproutes: TcpRoute [v1alpha2]
            tlsroutes: TlsRoute [v1, v1alpha2, v1alpha3]
            udproutes: UdpRoute [v1alpha2]
            xbackendtrafficpolicies: XBackendTrafficPolicy [v1alpha1]
            xmeshes: XMesh [v1alpha1]
        });
    };
}
