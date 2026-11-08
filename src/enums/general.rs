str_enum!(
    ConfigFormat {
        Links => "links",
        LinksBase64 => "links_base64",
        Xray => "xray",
        Wireguard => "wireguard",
        SingBox => "sing_box",
        Clash => "clash",
        ClashMeta => "clash_meta",
        Outline => "outline",
        Block => "block",
    }
);

str_enum!(
    Language {
        Fa => "fa",
        En => "en",
        Ru => "ru",
        Zh => "zh",
    }
);

str_enum!(
    Period {
        Minute => "minute",
        Hour => "hour",
        Day => "day",
        Month => "month",
    }
);

str_enum!(
    Platform {
        Android => "android",
        Ios => "ios",
        Windows => "windows",
        Macos => "macos",
        Linux => "linux",
        Appletv => "appletv",
        Androidtv => "androidtv",
    }
);

str_enum!(
    RunMethod {
        Webhook => "webhook",
        LongPolling => "long-polling",
    }
);

str_enum!(
    HWIDMode {
        Disabled => "disabled",
        UseGlobal => "use_global",
        Override => "override",
    }
);
