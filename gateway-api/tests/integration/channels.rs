// WARNING: generated file — do not edit

channel!(standard {
    backendtlspolicies { v1 => "backendtlspolicies.rs", v1alpha3 => "backendtlspolicies.rs" }
    gatewayclasses { v1 => "gatewayclasses.rs", v1beta1 => "gatewayclasses.rs" }
    gateways { v1 => "gateways.rs", v1beta1 => "gateways.rs" }
    grpcroutes { v1 => "grpcroutes.rs" }
    httproutes { v1 => "httproutes.rs", v1beta1 => "httproutes.rs" }
    listenersets { v1 => "listenersets.rs" }
    referencegrants { v1 => "referencegrants.rs", v1beta1 => "referencegrants.rs" }
    tlsroutes { v1 => "tlsroutes.rs", v1alpha2 => "tlsroutes_v1alpha2.rs", v1alpha3 => "tlsroutes.rs" }
});

#[cfg(feature = "experimental")]
channel!(experimental {
    backendtlspolicies { v1 => "backendtlspolicies.rs", v1alpha3 => "backendtlspolicies.rs" }
    gatewayclasses { v1 => "gatewayclasses.rs", v1beta1 => "gatewayclasses.rs" }
    gateways { v1 => "gateways.rs", v1beta1 => "gateways.rs" }
    grpcroutes { v1 => "grpcroutes.rs" }
    httproutes { v1 => "httproutes.rs", v1beta1 => "httproutes.rs" }
    listenersets { v1 => "listenersets.rs" }
    referencegrants { v1 => "referencegrants.rs", v1beta1 => "referencegrants.rs" }
    tcproutes { v1alpha2 => "tcproutes.rs" }
    tlsroutes { v1 => "tlsroutes.rs", v1alpha2 => "tlsroutes_v1alpha2.rs", v1alpha3 => "tlsroutes.rs" }
    udproutes { v1alpha2 => "udproutes.rs" }
    xbackendtrafficpolicies { v1alpha1 => "xbackendtrafficpolicies.rs" }
    xmeshes { v1alpha1 => "xmeshes.rs" }
});
