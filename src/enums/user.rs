str_enum!(
    DataLimitResetStrategy {
        NoReset => "no_reset",
        Day => "day",
        Week => "week",
        Month => "month",
        Year => "year",
    }
);

/// Alias kept for parity with the Python client.
pub type UserDataLimitResetStrategy = DataLimitResetStrategy;

str_enum!(
    UserStatus {
        Active => "active",
        Disabled => "disabled",
        Limited => "limited",
        Expired => "expired",
        OnHold => "on_hold",
    }
);

str_enum!(
    UserStatusCreate {
        Active => "active",
        OnHold => "on_hold",
    }
);

str_enum!(
    UsernameGenerationStrategy {
        Random => "random",
        Sequence => "sequence",
    }
);
