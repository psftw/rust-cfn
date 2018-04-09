//! Types for the `ElasticLoadBalancing` service.

/// The [`AWS::ElasticLoadBalancing::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html) resource type.
#[derive(Debug)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug)]
pub struct LoadBalancerProperties {
    /// Property `AccessLoggingPolicy`.
    pub access_logging_policy: Option<::Value<self::load_balancer::AccessLoggingPolicy>>,
    /// Property `AppCookieStickinessPolicy`.
    pub app_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::AppCookieStickinessPolicy>>,
    /// Property `AvailabilityZones`.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `ConnectionDrainingPolicy`.
    pub connection_draining_policy: Option<::Value<self::load_balancer::ConnectionDrainingPolicy>>,
    /// Property `ConnectionSettings`.
    pub connection_settings: Option<::Value<self::load_balancer::ConnectionSettings>>,
    /// Property `CrossZone`.
    pub cross_zone: Option<::Value<bool>>,
    /// Property `HealthCheck`.
    pub health_check: Option<::Value<self::load_balancer::HealthCheck>>,
    /// Property `Instances`.
    pub instances: Option<::ValueList<String>>,
    /// Property `LBCookieStickinessPolicy`.
    pub lb_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::LBCookieStickinessPolicy>>,
    /// Property `Listeners`.
    pub listeners: ::ValueList<self::load_balancer::Listeners>,
    /// Property `LoadBalancerName`.
    pub load_balancer_name: Option<::Value<String>>,
    /// Property `Policies`.
    pub policies: Option<::ValueList<self::load_balancer::Policies>>,
    /// Property `Scheme`.
    pub scheme: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    pub security_groups: Option<::ValueList<String>>,
    /// Property `Subnets`.
    pub subnets: Option<::ValueList<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LoadBalancerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLoggingPolicy", &self.access_logging_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppCookieStickinessPolicy", &self.app_cookie_stickiness_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", &self.availability_zones)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionDrainingPolicy", &self.connection_draining_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionSettings", &self.connection_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossZone", &self.cross_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheck", &self.health_check)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instances", &self.instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LBCookieStickinessPolicy", &self.lb_cookie_stickiness_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Listeners", &self.listeners)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", &self.load_balancer_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheme", &self.scheme)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoadBalancerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_logging_policy = None;
                let mut app_cookie_stickiness_policy = None;
                let mut availability_zones = None;
                let mut connection_draining_policy = None;
                let mut connection_settings = None;
                let mut cross_zone = None;
                let mut health_check = None;
                let mut instances = None;
                let mut lb_cookie_stickiness_policy = None;
                let mut listeners = None;
                let mut load_balancer_name = None;
                let mut policies = None;
                let mut scheme = None;
                let mut security_groups = None;
                let mut subnets = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessLoggingPolicy" => {
                            access_logging_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AppCookieStickinessPolicy" => {
                            app_cookie_stickiness_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZones" => {
                            availability_zones = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ConnectionDrainingPolicy" => {
                            connection_draining_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ConnectionSettings" => {
                            connection_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CrossZone" => {
                            cross_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheck" => {
                            health_check = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Instances" => {
                            instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LBCookieStickinessPolicy" => {
                            lb_cookie_stickiness_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Listeners" => {
                            listeners = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoadBalancerName" => {
                            load_balancer_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Policies" => {
                            policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Scheme" => {
                            scheme = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroups" => {
                            security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Subnets" => {
                            subnets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(LoadBalancerProperties {
                    access_logging_policy: access_logging_policy,
                    app_cookie_stickiness_policy: app_cookie_stickiness_policy,
                    availability_zones: availability_zones,
                    connection_draining_policy: connection_draining_policy,
                    connection_settings: connection_settings,
                    cross_zone: cross_zone,
                    health_check: health_check,
                    instances: instances,
                    lb_cookie_stickiness_policy: lb_cookie_stickiness_policy,
                    listeners: listeners.ok_or(::serde::de::Error::missing_field("Listeners"))?,
                    load_balancer_name: load_balancer_name,
                    policies: policies,
                    scheme: scheme,
                    security_groups: security_groups,
                    subnets: subnets,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for LoadBalancer {
    type Properties = LoadBalancerProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancing::LoadBalancer";
    fn properties(&self) -> &LoadBalancerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoadBalancerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoadBalancer {}

impl From<LoadBalancerProperties> for LoadBalancer {
    fn from(properties: LoadBalancerProperties) -> LoadBalancer {
        LoadBalancer { properties }
    }
}

pub mod load_balancer {
    //! Property types for the `LoadBalancer` resource.

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AccessLoggingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html) property type.
    #[derive(Debug)]
    pub struct AccessLoggingPolicy {
        /// Property `EmitInterval`.
        pub emit_interval: Option<::Value<u32>>,
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `S3BucketName`.
        pub s3_bucket_name: ::Value<String>,
        /// Property `S3BucketPrefix`.
        pub s3_bucket_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessLoggingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmitInterval", &self.emit_interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketPrefix", &self.s3_bucket_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessLoggingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessLoggingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessLoggingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessLoggingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut emit_interval = None;
                    let mut enabled = None;
                    let mut s3_bucket_name = None;
                    let mut s3_bucket_prefix = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmitInterval" => {
                                emit_interval = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3BucketName" => {
                                s3_bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3BucketPrefix" => {
                                s3_bucket_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessLoggingPolicy {
                        emit_interval: emit_interval,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                        s3_bucket_prefix: s3_bucket_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html) property type.
    #[derive(Debug)]
    pub struct AppCookieStickinessPolicy {
        /// Property `CookieName`.
        pub cookie_name: ::Value<String>,
        /// Property `PolicyName`.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AppCookieStickinessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieName", &self.cookie_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppCookieStickinessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppCookieStickinessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppCookieStickinessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppCookieStickinessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_name = None;
                    let mut policy_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieName" => {
                                cookie_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AppCookieStickinessPolicy {
                        cookie_name: cookie_name.ok_or(::serde::de::Error::missing_field("CookieName"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html) property type.
    #[derive(Debug)]
    pub struct ConnectionDrainingPolicy {
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `Timeout`.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ConnectionDrainingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", &self.timeout)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionDrainingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionDrainingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionDrainingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionDrainingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled = None;
                    let mut timeout = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Timeout" => {
                                timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionDrainingPolicy {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html) property type.
    #[derive(Debug)]
    pub struct ConnectionSettings {
        /// Property `IdleTimeout`.
        pub idle_timeout: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ConnectionSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleTimeout", &self.idle_timeout)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle_timeout = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IdleTimeout" => {
                                idle_timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionSettings {
                        idle_timeout: idle_timeout.ok_or(::serde::de::Error::missing_field("IdleTimeout"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html) property type.
    #[derive(Debug)]
    pub struct HealthCheck {
        /// Property `HealthyThreshold`.
        pub healthy_threshold: ::Value<String>,
        /// Property `Interval`.
        pub interval: ::Value<String>,
        /// Property `Target`.
        pub target: ::Value<String>,
        /// Property `Timeout`.
        pub timeout: ::Value<String>,
        /// Property `UnhealthyThreshold`.
        pub unhealthy_threshold: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheck {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", &self.healthy_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", &self.timeout)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", &self.unhealthy_threshold)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheck {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheck, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheck;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheck")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut healthy_threshold = None;
                    let mut interval = None;
                    let mut target = None;
                    let mut timeout = None;
                    let mut unhealthy_threshold = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Interval" => {
                                interval = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Target" => {
                                target = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Timeout" => {
                                timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheck {
                        healthy_threshold: healthy_threshold.ok_or(::serde::de::Error::missing_field("HealthyThreshold"))?,
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                        timeout: timeout.ok_or(::serde::de::Error::missing_field("Timeout"))?,
                        unhealthy_threshold: unhealthy_threshold.ok_or(::serde::de::Error::missing_field("UnhealthyThreshold"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html) property type.
    #[derive(Debug)]
    pub struct LBCookieStickinessPolicy {
        /// Property `CookieExpirationPeriod`.
        pub cookie_expiration_period: Option<::Value<String>>,
        /// Property `PolicyName`.
        pub policy_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LBCookieStickinessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieExpirationPeriod", &self.cookie_expiration_period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LBCookieStickinessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LBCookieStickinessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LBCookieStickinessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LBCookieStickinessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_expiration_period = None;
                    let mut policy_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieExpirationPeriod" => {
                                cookie_expiration_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LBCookieStickinessPolicy {
                        cookie_expiration_period: cookie_expiration_period,
                        policy_name: policy_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Debug)]
    pub struct Listeners {
        /// Property `InstancePort`.
        pub instance_port: ::Value<String>,
        /// Property `InstanceProtocol`.
        pub instance_protocol: Option<::Value<String>>,
        /// Property `LoadBalancerPort`.
        pub load_balancer_port: ::Value<String>,
        /// Property `PolicyNames`.
        pub policy_names: Option<::ValueList<String>>,
        /// Property `Protocol`.
        pub protocol: ::Value<String>,
        /// Property `SSLCertificateId`.
        pub ssl_certificate_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Listeners {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancePort", &self.instance_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProtocol", &self.instance_protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerPort", &self.load_balancer_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyNames", &self.policy_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSLCertificateId", &self.ssl_certificate_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Listeners {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Listeners, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Listeners;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Listeners")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_port = None;
                    let mut instance_protocol = None;
                    let mut load_balancer_port = None;
                    let mut policy_names = None;
                    let mut protocol = None;
                    let mut ssl_certificate_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstancePort" => {
                                instance_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceProtocol" => {
                                instance_protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoadBalancerPort" => {
                                load_balancer_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyNames" => {
                                policy_names = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Protocol" => {
                                protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SSLCertificateId" => {
                                ssl_certificate_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Listeners {
                        instance_port: instance_port.ok_or(::serde::de::Error::missing_field("InstancePort"))?,
                        instance_protocol: instance_protocol,
                        load_balancer_port: load_balancer_port.ok_or(::serde::de::Error::missing_field("LoadBalancerPort"))?,
                        policy_names: policy_names,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        ssl_certificate_id: ssl_certificate_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Debug)]
    pub struct Policies {
        /// Property `Attributes`.
        pub attributes: ::ValueList<::json::Value>,
        /// Property `InstancePorts`.
        pub instance_ports: Option<::ValueList<String>>,
        /// Property `LoadBalancerPorts`.
        pub load_balancer_ports: Option<::ValueList<String>>,
        /// Property `PolicyName`.
        pub policy_name: ::Value<String>,
        /// Property `PolicyType`.
        pub policy_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancePorts", &self.instance_ports)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerPorts", &self.load_balancer_ports)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", &self.policy_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes = None;
                    let mut instance_ports = None;
                    let mut load_balancer_ports = None;
                    let mut policy_name = None;
                    let mut policy_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstancePorts" => {
                                instance_ports = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoadBalancerPorts" => {
                                load_balancer_ports = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyName" => {
                                policy_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PolicyType" => {
                                policy_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Policies {
                        attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                        instance_ports: instance_ports,
                        load_balancer_ports: load_balancer_ports,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                        policy_type: policy_type.ok_or(::serde::de::Error::missing_field("PolicyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
