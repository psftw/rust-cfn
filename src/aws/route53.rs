//! Types for the `Route53` service.

/// The [`AWS::Route53::HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html) resource type.
#[derive(Debug, Default)]
pub struct HealthCheck {
    properties: HealthCheckProperties
}

/// Properties for the `HealthCheck` resource.
#[derive(Debug, Default)]
pub struct HealthCheckProperties {
    /// Property [`HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html#cfn-route53-healthcheck-healthcheckconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_config: ::Value<self::health_check::HealthCheckConfig>,
    /// Property [`HealthCheckTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html#cfn-route53-healthcheck-healthchecktags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_tags: Option<::ValueList<self::health_check::HealthCheckTag>>,
}

impl ::serde::Serialize for HealthCheckProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckConfig", &self.health_check_config)?;
        if let Some(ref health_check_tags) = self.health_check_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckTags", health_check_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HealthCheckProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HealthCheckProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HealthCheckProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut health_check_config: Option<::Value<self::health_check::HealthCheckConfig>> = None;
                let mut health_check_tags: Option<::ValueList<self::health_check::HealthCheckTag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HealthCheckConfig" => {
                            health_check_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckTags" => {
                            health_check_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HealthCheckProperties {
                    health_check_config: health_check_config.ok_or(::serde::de::Error::missing_field("HealthCheckConfig"))?,
                    health_check_tags: health_check_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HealthCheck {
    type Properties = HealthCheckProperties;
    const TYPE: &'static str = "AWS::Route53::HealthCheck";
    fn properties(&self) -> &HealthCheckProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HealthCheckProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HealthCheck {}

impl From<HealthCheckProperties> for HealthCheck {
    fn from(properties: HealthCheckProperties) -> HealthCheck {
        HealthCheck { properties }
    }
}

/// The [`AWS::Route53::HostedZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html) resource type.
#[derive(Debug, Default)]
pub struct HostedZone {
    properties: HostedZoneProperties
}

/// Properties for the `HostedZone` resource.
#[derive(Debug, Default)]
pub struct HostedZoneProperties {
    /// Property [`HostedZoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html#cfn-route53-hostedzone-hostedzoneconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hosted_zone_config: Option<::Value<self::hosted_zone::HostedZoneConfig>>,
    /// Property [`HostedZoneTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html#cfn-route53-hostedzone-hostedzonetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hosted_zone_tags: Option<::ValueList<self::hosted_zone::HostedZoneTag>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html#cfn-route53-hostedzone-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QueryLoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html#cfn-route53-hostedzone-queryloggingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_logging_config: Option<::Value<self::hosted_zone::QueryLoggingConfig>>,
    /// Property [`VPCs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html#cfn-route53-hostedzone-vpcs).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub vp_cs: Option<::ValueList<self::hosted_zone::VPC>>,
}

impl ::serde::Serialize for HostedZoneProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref hosted_zone_config) = self.hosted_zone_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneConfig", hosted_zone_config)?;
        }
        if let Some(ref hosted_zone_tags) = self.hosted_zone_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneTags", hosted_zone_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref query_logging_config) = self.query_logging_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryLoggingConfig", query_logging_config)?;
        }
        if let Some(ref vp_cs) = self.vp_cs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCs", vp_cs)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HostedZoneProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HostedZoneProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HostedZoneProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hosted_zone_config: Option<::Value<self::hosted_zone::HostedZoneConfig>> = None;
                let mut hosted_zone_tags: Option<::ValueList<self::hosted_zone::HostedZoneTag>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut query_logging_config: Option<::Value<self::hosted_zone::QueryLoggingConfig>> = None;
                let mut vp_cs: Option<::ValueList<self::hosted_zone::VPC>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HostedZoneConfig" => {
                            hosted_zone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostedZoneTags" => {
                            hosted_zone_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryLoggingConfig" => {
                            query_logging_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCs" => {
                            vp_cs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HostedZoneProperties {
                    hosted_zone_config: hosted_zone_config,
                    hosted_zone_tags: hosted_zone_tags,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    query_logging_config: query_logging_config,
                    vp_cs: vp_cs,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HostedZone {
    type Properties = HostedZoneProperties;
    const TYPE: &'static str = "AWS::Route53::HostedZone";
    fn properties(&self) -> &HostedZoneProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HostedZoneProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HostedZone {}

impl From<HostedZoneProperties> for HostedZone {
    fn from(properties: HostedZoneProperties) -> HostedZone {
        HostedZone { properties }
    }
}

/// The [`AWS::Route53::RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) resource type.
#[derive(Debug, Default)]
pub struct RecordSet {
    properties: RecordSetProperties
}

/// Properties for the `RecordSet` resource.
#[derive(Debug, Default)]
pub struct RecordSetProperties {
    /// Property [`AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-aliastarget).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alias_target: Option<::Value<self::record_set::AliasTarget>>,
    /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-comment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub comment: Option<::Value<String>>,
    /// Property [`Failover`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-failover).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub failover: Option<::Value<String>>,
    /// Property [`GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-geolocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub geo_location: Option<::Value<self::record_set::GeoLocation>>,
    /// Property [`HealthCheckId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-healthcheckid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_id: Option<::Value<String>>,
    /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-hostedzoneid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub hosted_zone_id: Option<::Value<String>>,
    /// Property [`HostedZoneName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-hostedzonename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub hosted_zone_name: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-region).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub region: Option<::Value<String>>,
    /// Property [`ResourceRecords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-resourcerecords).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_records: Option<::ValueList<String>>,
    /// Property [`SetIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-setidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub set_identifier: Option<::Value<String>>,
    /// Property [`TTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-ttl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ttl: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_: ::Value<String>,
    /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-weight).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub weight: Option<::Value<u32>>,
}

impl ::serde::Serialize for RecordSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref alias_target) = self.alias_target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasTarget", alias_target)?;
        }
        if let Some(ref comment) = self.comment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
        }
        if let Some(ref failover) = self.failover {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Failover", failover)?;
        }
        if let Some(ref geo_location) = self.geo_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoLocation", geo_location)?;
        }
        if let Some(ref health_check_id) = self.health_check_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckId", health_check_id)?;
        }
        if let Some(ref hosted_zone_id) = self.hosted_zone_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", hosted_zone_id)?;
        }
        if let Some(ref hosted_zone_name) = self.hosted_zone_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", hosted_zone_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref region) = self.region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
        }
        if let Some(ref resource_records) = self.resource_records {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRecords", resource_records)?;
        }
        if let Some(ref set_identifier) = self.set_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetIdentifier", set_identifier)?;
        }
        if let Some(ref ttl) = self.ttl {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TTL", ttl)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        if let Some(ref weight) = self.weight {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecordSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecordSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecordSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alias_target: Option<::Value<self::record_set::AliasTarget>> = None;
                let mut comment: Option<::Value<String>> = None;
                let mut failover: Option<::Value<String>> = None;
                let mut geo_location: Option<::Value<self::record_set::GeoLocation>> = None;
                let mut health_check_id: Option<::Value<String>> = None;
                let mut hosted_zone_id: Option<::Value<String>> = None;
                let mut hosted_zone_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut region: Option<::Value<String>> = None;
                let mut resource_records: Option<::ValueList<String>> = None;
                let mut set_identifier: Option<::Value<String>> = None;
                let mut ttl: Option<::Value<String>> = None;
                let mut type_: Option<::Value<String>> = None;
                let mut weight: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AliasTarget" => {
                            alias_target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Comment" => {
                            comment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Failover" => {
                            failover = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GeoLocation" => {
                            geo_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckId" => {
                            health_check_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostedZoneId" => {
                            hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostedZoneName" => {
                            hosted_zone_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Region" => {
                            region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceRecords" => {
                            resource_records = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SetIdentifier" => {
                            set_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TTL" => {
                            ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Weight" => {
                            weight = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RecordSetProperties {
                    alias_target: alias_target,
                    comment: comment,
                    failover: failover,
                    geo_location: geo_location,
                    health_check_id: health_check_id,
                    hosted_zone_id: hosted_zone_id,
                    hosted_zone_name: hosted_zone_name,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    region: region,
                    resource_records: resource_records,
                    set_identifier: set_identifier,
                    ttl: ttl,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    weight: weight,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RecordSet {
    type Properties = RecordSetProperties;
    const TYPE: &'static str = "AWS::Route53::RecordSet";
    fn properties(&self) -> &RecordSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecordSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RecordSet {}

impl From<RecordSetProperties> for RecordSet {
    fn from(properties: RecordSetProperties) -> RecordSet {
        RecordSet { properties }
    }
}

/// The [`AWS::Route53::RecordSetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct RecordSetGroup {
    properties: RecordSetGroupProperties
}

/// Properties for the `RecordSetGroup` resource.
#[derive(Debug, Default)]
pub struct RecordSetGroupProperties {
    /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html#cfn-route53-recordsetgroup-comment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub comment: Option<::Value<String>>,
    /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html#cfn-route53-recordsetgroup-hostedzoneid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub hosted_zone_id: Option<::Value<String>>,
    /// Property [`HostedZoneName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html#cfn-route53-recordsetgroup-hostedzonename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub hosted_zone_name: Option<::Value<String>>,
    /// Property [`RecordSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html#cfn-route53-recordsetgroup-recordsets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub record_sets: Option<::ValueList<self::record_set_group::RecordSet>>,
}

impl ::serde::Serialize for RecordSetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref comment) = self.comment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
        }
        if let Some(ref hosted_zone_id) = self.hosted_zone_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", hosted_zone_id)?;
        }
        if let Some(ref hosted_zone_name) = self.hosted_zone_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", hosted_zone_name)?;
        }
        if let Some(ref record_sets) = self.record_sets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSets", record_sets)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecordSetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecordSetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecordSetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut comment: Option<::Value<String>> = None;
                let mut hosted_zone_id: Option<::Value<String>> = None;
                let mut hosted_zone_name: Option<::Value<String>> = None;
                let mut record_sets: Option<::ValueList<self::record_set_group::RecordSet>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Comment" => {
                            comment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostedZoneId" => {
                            hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostedZoneName" => {
                            hosted_zone_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecordSets" => {
                            record_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RecordSetGroupProperties {
                    comment: comment,
                    hosted_zone_id: hosted_zone_id,
                    hosted_zone_name: hosted_zone_name,
                    record_sets: record_sets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RecordSetGroup {
    type Properties = RecordSetGroupProperties;
    const TYPE: &'static str = "AWS::Route53::RecordSetGroup";
    fn properties(&self) -> &RecordSetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecordSetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RecordSetGroup {}

impl From<RecordSetGroupProperties> for RecordSetGroup {
    fn from(properties: RecordSetGroupProperties) -> RecordSetGroup {
        RecordSetGroup { properties }
    }
}

pub mod health_check {
    //! Property types for the `HealthCheck` resource.

    /// The [`AWS::Route53::HealthCheck.AlarmIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-alarmidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct AlarmIdentifier {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-alarmidentifier.html#cfn-route53-healthcheck-alarmidentifier-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-alarmidentifier.html#cfn-route53-healthcheck-alarmidentifier-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
    }

    impl ::codec::SerializeValue for AlarmIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlarmIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlarmIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlarmIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AlarmIdentifier {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheckConfig {
        /// Property [`AlarmIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-alarmidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_identifier: Option<::Value<AlarmIdentifier>>,
        /// Property [`ChildHealthChecks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-childhealthchecks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_health_checks: Option<::ValueList<String>>,
        /// Property [`EnableSNI`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-enablesni).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_sni: Option<::Value<bool>>,
        /// Property [`FailureThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-failurethreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_threshold: Option<::Value<u32>>,
        /// Property [`FullyQualifiedDomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-fullyqualifieddomainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_domain_name: Option<::Value<String>>,
        /// Property [`HealthThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-healththreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_threshold: Option<::Value<u32>>,
        /// Property [`IPAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-ipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_address: Option<::Value<String>>,
        /// Property [`InsufficientDataHealthStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-insufficientdatahealthstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insufficient_data_health_status: Option<::Value<String>>,
        /// Property [`Inverted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-inverted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inverted: Option<::Value<bool>>,
        /// Property [`MeasureLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-measurelatency).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub measure_latency: Option<::Value<bool>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`Regions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-regions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regions: Option<::ValueList<String>>,
        /// Property [`RequestInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-requestinterval).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub request_interval: Option<::Value<u32>>,
        /// Property [`ResourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-resourcepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_path: Option<::Value<String>>,
        /// Property [`SearchString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-searchstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search_string: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html#cfn-route53-healthcheck-healthcheckconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheckConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alarm_identifier) = self.alarm_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmIdentifier", alarm_identifier)?;
            }
            if let Some(ref child_health_checks) = self.child_health_checks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildHealthChecks", child_health_checks)?;
            }
            if let Some(ref enable_sni) = self.enable_sni {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSNI", enable_sni)?;
            }
            if let Some(ref failure_threshold) = self.failure_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureThreshold", failure_threshold)?;
            }
            if let Some(ref fully_qualified_domain_name) = self.fully_qualified_domain_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedDomainName", fully_qualified_domain_name)?;
            }
            if let Some(ref health_threshold) = self.health_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthThreshold", health_threshold)?;
            }
            if let Some(ref ip_address) = self.ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPAddress", ip_address)?;
            }
            if let Some(ref insufficient_data_health_status) = self.insufficient_data_health_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataHealthStatus", insufficient_data_health_status)?;
            }
            if let Some(ref inverted) = self.inverted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inverted", inverted)?;
            }
            if let Some(ref measure_latency) = self.measure_latency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeasureLatency", measure_latency)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref regions) = self.regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regions", regions)?;
            }
            if let Some(ref request_interval) = self.request_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestInterval", request_interval)?;
            }
            if let Some(ref resource_path) = self.resource_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", resource_path)?;
            }
            if let Some(ref search_string) = self.search_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchString", search_string)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_identifier: Option<::Value<AlarmIdentifier>> = None;
                    let mut child_health_checks: Option<::ValueList<String>> = None;
                    let mut enable_sni: Option<::Value<bool>> = None;
                    let mut failure_threshold: Option<::Value<u32>> = None;
                    let mut fully_qualified_domain_name: Option<::Value<String>> = None;
                    let mut health_threshold: Option<::Value<u32>> = None;
                    let mut ip_address: Option<::Value<String>> = None;
                    let mut insufficient_data_health_status: Option<::Value<String>> = None;
                    let mut inverted: Option<::Value<bool>> = None;
                    let mut measure_latency: Option<::Value<bool>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut regions: Option<::ValueList<String>> = None;
                    let mut request_interval: Option<::Value<u32>> = None;
                    let mut resource_path: Option<::Value<String>> = None;
                    let mut search_string: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmIdentifier" => {
                                alarm_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChildHealthChecks" => {
                                child_health_checks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableSNI" => {
                                enable_sni = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureThreshold" => {
                                failure_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedDomainName" => {
                                fully_qualified_domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthThreshold" => {
                                health_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InsufficientDataHealthStatus" => {
                                insufficient_data_health_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inverted" => {
                                inverted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MeasureLatency" => {
                                measure_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Regions" => {
                                regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestInterval" => {
                                request_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourcePath" => {
                                resource_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SearchString" => {
                                search_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckConfig {
                        alarm_identifier: alarm_identifier,
                        child_health_checks: child_health_checks,
                        enable_sni: enable_sni,
                        failure_threshold: failure_threshold,
                        fully_qualified_domain_name: fully_qualified_domain_name,
                        health_threshold: health_threshold,
                        ip_address: ip_address,
                        insufficient_data_health_status: insufficient_data_health_status,
                        inverted: inverted,
                        measure_latency: measure_latency,
                        port: port,
                        regions: regions,
                        request_interval: request_interval,
                        resource_path: resource_path,
                        search_string: search_string,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HealthCheck.HealthCheckTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheckTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html#cfn-route53-healthchecktags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html#cfn-route53-healthchecktags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheckTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod hosted_zone {
    //! Property types for the `HostedZone` resource.

    /// The [`AWS::Route53::HostedZone.HostedZoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzoneconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HostedZoneConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzoneconfig.html#cfn-route53-hostedzone-hostedzoneconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostedZoneConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostedZoneConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostedZoneConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostedZoneConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HostedZoneConfig {
                        comment: comment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.HostedZoneTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetags.html) property type.
    #[derive(Debug, Default)]
    pub struct HostedZoneTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetags.html#cfn-route53-hostedzonetags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetags.html#cfn-route53-hostedzonetags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HostedZoneTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostedZoneTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedZoneTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostedZoneTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostedZoneTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HostedZoneTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.QueryLoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-queryloggingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryLoggingConfig {
        /// Property [`CloudWatchLogsLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-queryloggingconfig.html#cfn-route53-hostedzone-queryloggingconfig-cloudwatchlogsloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_log_group_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueryLoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", &self.cloud_watch_logs_log_group_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryLoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryLoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryLoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryLoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_log_group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsLogGroupArn" => {
                                cloud_watch_logs_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryLoggingConfig {
                        cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn.ok_or(::serde::de::Error::missing_field("CloudWatchLogsLogGroupArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::HostedZone.VPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone-hostedzonevpcs.html) property type.
    #[derive(Debug, Default)]
    pub struct VPC {
        /// Property [`VPCId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone-hostedzonevpcs.html#cfn-route53-hostedzone-hostedzonevpcs-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: ::Value<String>,
        /// Property [`VPCRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone-hostedzonevpcs.html#cfn-route53-hostedzone-hostedzonevpcs-vpcregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_region: ::Value<String>,
    }

    impl ::codec::SerializeValue for VPC {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCId", &self.vpc_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCRegion", &self.vpc_region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VPC {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VPC, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VPC;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VPC")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vpc_id: Option<::Value<String>> = None;
                    let mut vpc_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VPCId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VPCRegion" => {
                                vpc_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VPC {
                        vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VPCId"))?,
                        vpc_region: vpc_region.ok_or(::serde::de::Error::missing_field("VPCRegion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod record_set {
    //! Property types for the `RecordSet` resource.

    /// The [`AWS::Route53::RecordSet.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug, Default)]
    pub struct AliasTarget {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-dnshostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: ::Value<String>,
        /// Property [`EvaluateTargetHealth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-evaluatetargethealth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluate_target_health: Option<::Value<bool>>,
        /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-hostedzoneid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AliasTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref evaluate_target_health) = self.evaluate_target_health {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateTargetHealth", evaluate_target_health)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AliasTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut evaluate_target_health: Option<::Value<bool>> = None;
                    let mut hosted_zone_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EvaluateTargetHealth" => {
                                evaluate_target_health = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasTarget {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        evaluate_target_health: evaluate_target_health,
                        hosted_zone_id: hosted_zone_id.ok_or(::serde::de::Error::missing_field("HostedZoneId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSet.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoLocation {
        /// Property [`ContinentCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordset-geolocation-continentcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub continent_code: Option<::Value<String>>,
        /// Property [`CountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordset-geolocation-countrycode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country_code: Option<::Value<String>>,
        /// Property [`SubdivisionCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordset-geolocation-subdivisioncode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subdivision_code: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeoLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref continent_code) = self.continent_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinentCode", continent_code)?;
            }
            if let Some(ref country_code) = self.country_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", country_code)?;
            }
            if let Some(ref subdivision_code) = self.subdivision_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubdivisionCode", subdivision_code)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut continent_code: Option<::Value<String>> = None;
                    let mut country_code: Option<::Value<String>> = None;
                    let mut subdivision_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContinentCode" => {
                                continent_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CountryCode" => {
                                country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubdivisionCode" => {
                                subdivision_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoLocation {
                        continent_code: continent_code,
                        country_code: country_code,
                        subdivision_code: subdivision_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod record_set_group {
    //! Property types for the `RecordSetGroup` resource.

    /// The [`AWS::Route53::RecordSetGroup.AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html) property type.
    #[derive(Debug, Default)]
    pub struct AliasTarget {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-dnshostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: ::Value<String>,
        /// Property [`EvaluateTargetHealth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-evaluatetargethealth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluate_target_health: Option<::Value<bool>>,
        /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html#cfn-route53-aliastarget-hostedzoneid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AliasTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref evaluate_target_health) = self.evaluate_target_health {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateTargetHealth", evaluate_target_health)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", &self.hosted_zone_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AliasTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut evaluate_target_health: Option<::Value<bool>> = None;
                    let mut hosted_zone_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EvaluateTargetHealth" => {
                                evaluate_target_health = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasTarget {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        evaluate_target_health: evaluate_target_health,
                        hosted_zone_id: hosted_zone_id.ok_or(::serde::de::Error::missing_field("HostedZoneId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSetGroup.GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoLocation {
        /// Property [`ContinentCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordsetgroup-geolocation-continentcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub continent_code: Option<::Value<String>>,
        /// Property [`CountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordset-geolocation-countrycode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country_code: Option<::Value<String>>,
        /// Property [`SubdivisionCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html#cfn-route53-recordset-geolocation-subdivisioncode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subdivision_code: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeoLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref continent_code) = self.continent_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinentCode", continent_code)?;
            }
            if let Some(ref country_code) = self.country_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", country_code)?;
            }
            if let Some(ref subdivision_code) = self.subdivision_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubdivisionCode", subdivision_code)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut continent_code: Option<::Value<String>> = None;
                    let mut country_code: Option<::Value<String>> = None;
                    let mut subdivision_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContinentCode" => {
                                continent_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CountryCode" => {
                                country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubdivisionCode" => {
                                subdivision_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoLocation {
                        continent_code: continent_code,
                        country_code: country_code,
                        subdivision_code: subdivision_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53::RecordSetGroup.RecordSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordSet {
        /// Property [`AliasTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-aliastarget).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alias_target: Option<::Value<AliasTarget>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`Failover`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-failover).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover: Option<::Value<String>>,
        /// Property [`GeoLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-geolocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub geo_location: Option<::Value<GeoLocation>>,
        /// Property [`HealthCheckId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-healthcheckid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_check_id: Option<::Value<String>>,
        /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-hostedzoneid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_id: Option<::Value<String>>,
        /// Property [`HostedZoneName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-hostedzonename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`ResourceRecords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-resourcerecords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_records: Option<::ValueList<String>>,
        /// Property [`SetIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-setidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_identifier: Option<::Value<String>>,
        /// Property [`TTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-ttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ttl: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html#cfn-route53-recordset-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RecordSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alias_target) = self.alias_target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasTarget", alias_target)?;
            }
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref failover) = self.failover {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Failover", failover)?;
            }
            if let Some(ref geo_location) = self.geo_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoLocation", geo_location)?;
            }
            if let Some(ref health_check_id) = self.health_check_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckId", health_check_id)?;
            }
            if let Some(ref hosted_zone_id) = self.hosted_zone_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", hosted_zone_id)?;
            }
            if let Some(ref hosted_zone_name) = self.hosted_zone_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneName", hosted_zone_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            if let Some(ref resource_records) = self.resource_records {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRecords", resource_records)?;
            }
            if let Some(ref set_identifier) = self.set_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetIdentifier", set_identifier)?;
            }
            if let Some(ref ttl) = self.ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TTL", ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alias_target: Option<::Value<AliasTarget>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut failover: Option<::Value<String>> = None;
                    let mut geo_location: Option<::Value<GeoLocation>> = None;
                    let mut health_check_id: Option<::Value<String>> = None;
                    let mut hosted_zone_id: Option<::Value<String>> = None;
                    let mut hosted_zone_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut resource_records: Option<::ValueList<String>> = None;
                    let mut set_identifier: Option<::Value<String>> = None;
                    let mut ttl: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AliasTarget" => {
                                alias_target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Failover" => {
                                failover = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeoLocation" => {
                                geo_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthCheckId" => {
                                health_check_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneName" => {
                                hosted_zone_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRecords" => {
                                resource_records = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetIdentifier" => {
                                set_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TTL" => {
                                ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordSet {
                        alias_target: alias_target,
                        comment: comment,
                        failover: failover,
                        geo_location: geo_location,
                        health_check_id: health_check_id,
                        hosted_zone_id: hosted_zone_id,
                        hosted_zone_name: hosted_zone_name,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        region: region,
                        resource_records: resource_records,
                        set_identifier: set_identifier,
                        ttl: ttl,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
