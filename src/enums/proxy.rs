str_enum!(
    ECHQueryStrategy {
        None => "none",
        Half => "half",
        Full => "full",
    }
);

str_enum!(
    MultiplexProtocol {
        Smux => "smux",
        Yamux => "yamux",
        H2mux => "h2mux",
    }
);

str_enum!(
    ProxyHostALPN {
        Http11 => "http/1.1",
        H2 => "h2",
        H3 => "h3",
    }
);

str_enum!(
    ProxyHostFingerprint {
        Empty => "",
        Chrome => "chrome",
        Firefox => "firefox",
        Safari => "safari",
        Ios => "ios",
        Android => "android",
        Edge => "edge",
        V360 => "360",
        Qq => "qq",
        Random => "random",
        Randomized => "randomized",
        Randomizednoalpn => "randomizednoalpn",
        Unsafe => "unsafe",
    }
);

str_enum!(
    ProxyHostSecurity {
        InboundDefault => "inbound_default",
        None => "none",
        Tls => "tls",
    }
);

str_enum!(
    ShadowsocksMethods {
        Aes128Gcm => "aes-128-gcm",
        Aes256Gcm => "aes-256-gcm",
        Chacha20IetfPoly1305 => "chacha20-ietf-poly1305",
        Xchacha20Poly1305 => "xchacha20-poly1305",
    }
);

/// Alias kept for parity with the Python client.
pub type ShadowsocksMethod = ShadowsocksMethods;

str_enum!(
    XHttpModes {
        Auto => "auto",
        PacketUp => "packet-up",
        StreamUp => "stream-up",
        StreamOne => "stream-one",
    }
);

str_enum!(
    XUDP {
        Reject => "reject",
        Allow => "allow",
        Skip => "skip",
    }
);

str_enum!(
    FlowOption {
        None => "",
        XtlsRprxVision => "xtls-rprx-vision",
    }
);
