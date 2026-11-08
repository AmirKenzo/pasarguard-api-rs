str_enum!(
    /// Spelled as in the panel API (`GeoFilseRegion`).
    GeoFilseRegion {
        Iran => "iran",
        China => "china",
        Russia => "russia",
    }
);

str_enum!(
    NodeConnectionType {
        Grpc => "grpc",
        Rest => "rest",
    }
);

str_enum!(
    NodeStatus {
        Connected => "connected",
        Connecting => "connecting",
        Error => "error",
        Disabled => "disabled",
        Limited => "limited",
    }
);

str_enum!(
    UsageTable {
        NodeUserUsages => "node_user_usages",
        NodeUsages => "node_usages",
    }
);

str_enum!(
    UserCountMetric {
        Online => "online",
        Expired => "expired",
        Limited => "limited",
    }
);
