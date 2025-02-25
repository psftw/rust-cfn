//! Types for the `WAFRegional` service.

/// The [`AWS::WAFRegional::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html) resource type.
#[derive(Debug, Default)]
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Debug, Default)]
pub struct ByteMatchSetProperties {
    /// Property [`ByteMatchTuples`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html#cfn-wafregional-bytematchset-bytematchtuples).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub byte_match_tuples: Option<::ValueList<self::byte_match_set::ByteMatchTuple>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html#cfn-wafregional-bytematchset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for ByteMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref byte_match_tuples) = self.byte_match_tuples {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ByteMatchTuples", byte_match_tuples)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ByteMatchSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ByteMatchSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ByteMatchSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ByteMatchSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut byte_match_tuples: Option<::ValueList<self::byte_match_set::ByteMatchTuple>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ByteMatchTuples" => {
                            byte_match_tuples = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ByteMatchSetProperties {
                    byte_match_tuples: byte_match_tuples,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ByteMatchSet {
    type Properties = ByteMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::ByteMatchSet";
    fn properties(&self) -> &ByteMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ByteMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ByteMatchSet {}

impl From<ByteMatchSetProperties> for ByteMatchSet {
    fn from(properties: ByteMatchSetProperties) -> ByteMatchSet {
        ByteMatchSet { properties }
    }
}

/// The [`AWS::WAFRegional::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html) resource type.
#[derive(Debug, Default)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Default)]
pub struct IPSetProperties {
    /// Property [`IPSetDescriptors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html#cfn-wafregional-ipset-ipsetdescriptors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_set_descriptors: Option<::ValueList<self::ip_set::IPSetDescriptor>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html#cfn-wafregional-ipset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for IPSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref ip_set_descriptors) = self.ip_set_descriptors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetDescriptors", ip_set_descriptors)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IPSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IPSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IPSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ip_set_descriptors: Option<::ValueList<self::ip_set::IPSetDescriptor>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IPSetDescriptors" => {
                            ip_set_descriptors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IPSetProperties {
                    ip_set_descriptors: ip_set_descriptors,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IPSet {
    type Properties = IPSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::IPSet";
    fn properties(&self) -> &IPSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IPSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IPSet {}

impl From<IPSetProperties> for IPSet {
    fn from(properties: IPSetProperties) -> IPSet {
        IPSet { properties }
    }
}

/// The [`AWS::WAFRegional::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html) resource type.
#[derive(Debug, Default)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Default)]
pub struct RuleProperties {
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html#cfn-wafregional-rule-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html#cfn-wafregional-rule-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Predicates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html#cfn-wafregional-rule-predicates).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub predicates: Option<::ValueList<self::rule::Predicate>>,
}

impl ::serde::Serialize for RuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref predicates) = self.predicates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Predicates", predicates)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut metric_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut predicates: Option<::ValueList<self::rule::Predicate>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Predicates" => {
                            predicates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleProperties {
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    predicates: predicates,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::WAFRegional::Rule";
    fn properties(&self) -> &RuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Rule {}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

/// The [`AWS::WAFRegional::SizeConstraintSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html) resource type.
#[derive(Debug, Default)]
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Debug, Default)]
pub struct SizeConstraintSetProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html#cfn-wafregional-sizeconstraintset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SizeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html#cfn-wafregional-sizeconstraintset-sizeconstraints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub size_constraints: Option<::ValueList<self::size_constraint_set::SizeConstraint>>,
}

impl ::serde::Serialize for SizeConstraintSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref size_constraints) = self.size_constraints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeConstraints", size_constraints)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SizeConstraintSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SizeConstraintSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SizeConstraintSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SizeConstraintSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut size_constraints: Option<::ValueList<self::size_constraint_set::SizeConstraint>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SizeConstraints" => {
                            size_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SizeConstraintSetProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    size_constraints: size_constraints,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SizeConstraintSet {
    type Properties = SizeConstraintSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::SizeConstraintSet";
    fn properties(&self) -> &SizeConstraintSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SizeConstraintSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SizeConstraintSet {}

impl From<SizeConstraintSetProperties> for SizeConstraintSet {
    fn from(properties: SizeConstraintSetProperties) -> SizeConstraintSet {
        SizeConstraintSet { properties }
    }
}

/// The [`AWS::WAFRegional::SqlInjectionMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html) resource type.
#[derive(Debug, Default)]
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Debug, Default)]
pub struct SqlInjectionMatchSetProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html#cfn-wafregional-sqlinjectionmatchset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SqlInjectionMatchTuples`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html#cfn-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuples).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sql_injection_match_tuples: Option<::ValueList<self::sql_injection_match_set::SqlInjectionMatchTuple>>,
}

impl ::serde::Serialize for SqlInjectionMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref sql_injection_match_tuples) = self.sql_injection_match_tuples {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlInjectionMatchTuples", sql_injection_match_tuples)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SqlInjectionMatchSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SqlInjectionMatchSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SqlInjectionMatchSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SqlInjectionMatchSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut sql_injection_match_tuples: Option<::ValueList<self::sql_injection_match_set::SqlInjectionMatchTuple>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SqlInjectionMatchTuples" => {
                            sql_injection_match_tuples = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SqlInjectionMatchSetProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    sql_injection_match_tuples: sql_injection_match_tuples,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SqlInjectionMatchSet {
    type Properties = SqlInjectionMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::SqlInjectionMatchSet";
    fn properties(&self) -> &SqlInjectionMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SqlInjectionMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SqlInjectionMatchSet {}

impl From<SqlInjectionMatchSetProperties> for SqlInjectionMatchSet {
    fn from(properties: SqlInjectionMatchSetProperties) -> SqlInjectionMatchSet {
        SqlInjectionMatchSet { properties }
    }
}

/// The [`AWS::WAFRegional::WebACL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html) resource type.
#[derive(Debug, Default)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Debug, Default)]
pub struct WebACLProperties {
    /// Property [`DefaultAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html#cfn-wafregional-webacl-defaultaction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_action: ::Value<self::web_acl::Action>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html#cfn-wafregional-webacl-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html#cfn-wafregional-webacl-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html#cfn-wafregional-webacl-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: Option<::ValueList<self::web_acl::Rule>>,
}

impl ::serde::Serialize for WebACLProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAction", &self.default_action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref rules) = self.rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", rules)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WebACLProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WebACLProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WebACLProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WebACLProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut default_action: Option<::Value<self::web_acl::Action>> = None;
                let mut metric_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rules: Option<::ValueList<self::web_acl::Rule>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultAction" => {
                            default_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WebACLProperties {
                    default_action: default_action.ok_or(::serde::de::Error::missing_field("DefaultAction"))?,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rules: rules,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WebACL {
    type Properties = WebACLProperties;
    const TYPE: &'static str = "AWS::WAFRegional::WebACL";
    fn properties(&self) -> &WebACLProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACL {}

impl From<WebACLProperties> for WebACL {
    fn from(properties: WebACLProperties) -> WebACL {
        WebACL { properties }
    }
}

/// The [`AWS::WAFRegional::WebACLAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html) resource type.
#[derive(Debug, Default)]
pub struct WebACLAssociation {
    properties: WebACLAssociationProperties
}

/// Properties for the `WebACLAssociation` resource.
#[derive(Debug, Default)]
pub struct WebACLAssociationProperties {
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html#cfn-wafregional-webaclassociation-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`WebACLId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html#cfn-wafregional-webaclassociation-webaclid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub web_acl_id: ::Value<String>,
}

impl ::serde::Serialize for WebACLAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebACLId", &self.web_acl_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WebACLAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WebACLAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WebACLAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WebACLAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_arn: Option<::Value<String>> = None;
                let mut web_acl_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WebACLId" => {
                            web_acl_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WebACLAssociationProperties {
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    web_acl_id: web_acl_id.ok_or(::serde::de::Error::missing_field("WebACLId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WebACLAssociation {
    type Properties = WebACLAssociationProperties;
    const TYPE: &'static str = "AWS::WAFRegional::WebACLAssociation";
    fn properties(&self) -> &WebACLAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACLAssociation {}

impl From<WebACLAssociationProperties> for WebACLAssociation {
    fn from(properties: WebACLAssociationProperties) -> WebACLAssociation {
        WebACLAssociation { properties }
    }
}

/// The [`AWS::WAFRegional::XssMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html) resource type.
#[derive(Debug, Default)]
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Debug, Default)]
pub struct XssMatchSetProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html#cfn-wafregional-xssmatchset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`XssMatchTuples`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html#cfn-wafregional-xssmatchset-xssmatchtuples).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub xss_match_tuples: Option<::ValueList<self::xss_match_set::XssMatchTuple>>,
}

impl ::serde::Serialize for XssMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref xss_match_tuples) = self.xss_match_tuples {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "XssMatchTuples", xss_match_tuples)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for XssMatchSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<XssMatchSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = XssMatchSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type XssMatchSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut xss_match_tuples: Option<::ValueList<self::xss_match_set::XssMatchTuple>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "XssMatchTuples" => {
                            xss_match_tuples = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(XssMatchSetProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    xss_match_tuples: xss_match_tuples,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for XssMatchSet {
    type Properties = XssMatchSetProperties;
    const TYPE: &'static str = "AWS::WAFRegional::XssMatchSet";
    fn properties(&self) -> &XssMatchSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut XssMatchSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for XssMatchSet {}

impl From<XssMatchSetProperties> for XssMatchSet {
    fn from(properties: XssMatchSetProperties) -> XssMatchSet {
        XssMatchSet { properties }
    }
}

pub mod byte_match_set {
    //! Property types for the `ByteMatchSet` resource.

    /// The [`AWS::WAFRegional::ByteMatchSet.ByteMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html) property type.
    #[derive(Debug, Default)]
    pub struct ByteMatchTuple {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html#cfn-wafregional-bytematchset-bytematchtuple-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`PositionalConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html#cfn-wafregional-bytematchset-bytematchtuple-positionalconstraint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub positional_constraint: ::Value<String>,
        /// Property [`TargetString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html#cfn-wafregional-bytematchset-bytematchtuple-targetstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_string: Option<::Value<String>>,
        /// Property [`TargetStringBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html#cfn-wafregional-bytematchset-bytematchtuple-targetstringbase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_string_base64: Option<::Value<String>>,
        /// Property [`TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html#cfn-wafregional-bytematchset-bytematchtuple-texttransformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for ByteMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PositionalConstraint", &self.positional_constraint)?;
            if let Some(ref target_string) = self.target_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetString", target_string)?;
            }
            if let Some(ref target_string_base64) = self.target_string_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetStringBase64", target_string_base64)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformation", &self.text_transformation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ByteMatchTuple {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ByteMatchTuple, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ByteMatchTuple;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ByteMatchTuple")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut positional_constraint: Option<::Value<String>> = None;
                    let mut target_string: Option<::Value<String>> = None;
                    let mut target_string_base64: Option<::Value<String>> = None;
                    let mut text_transformation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PositionalConstraint" => {
                                positional_constraint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetString" => {
                                target_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetStringBase64" => {
                                target_string_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformation" => {
                                text_transformation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ByteMatchTuple {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        positional_constraint: positional_constraint.ok_or(::serde::de::Error::missing_field("PositionalConstraint"))?,
                        target_string: target_string,
                        target_string_base64: target_string_base64,
                        text_transformation: text_transformation.ok_or(::serde::de::Error::missing_field("TextTransformation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFRegional::ByteMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-fieldtomatch.html#cfn-wafregional-bytematchset-fieldtomatch-data).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-fieldtomatch.html#cfn-wafregional-bytematchset-fieldtomatch-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data) = self.data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", data)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        data: data,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod ip_set {
    //! Property types for the `IPSet` resource.

    /// The [`AWS::WAFRegional::IPSet.IPSetDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ipset-ipsetdescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetDescriptor {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ipset-ipsetdescriptor.html#cfn-wafregional-ipset-ipsetdescriptor-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ipset-ipsetdescriptor.html#cfn-wafregional-ipset-ipsetdescriptor-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for IPSetDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetDescriptor {
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::WAFRegional::Rule.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html) property type.
    #[derive(Debug, Default)]
    pub struct Predicate {
        /// Property [`DataId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html#cfn-wafregional-rule-predicate-dataid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_id: ::Value<String>,
        /// Property [`Negated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html#cfn-wafregional-rule-predicate-negated).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub negated: ::Value<bool>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html#cfn-wafregional-rule-predicate-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Predicate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataId", &self.data_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Negated", &self.negated)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Predicate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Predicate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Predicate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Predicate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_id: Option<::Value<String>> = None;
                    let mut negated: Option<::Value<bool>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataId" => {
                                data_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Negated" => {
                                negated = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Predicate {
                        data_id: data_id.ok_or(::serde::de::Error::missing_field("DataId"))?,
                        negated: negated.ok_or(::serde::de::Error::missing_field("Negated"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod size_constraint_set {
    //! Property types for the `SizeConstraintSet` resource.

    /// The [`AWS::WAFRegional::SizeConstraintSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-fieldtomatch.html#cfn-wafregional-sizeconstraintset-fieldtomatch-data).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-fieldtomatch.html#cfn-wafregional-sizeconstraintset-fieldtomatch-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data) = self.data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", data)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        data: data,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFRegional::SizeConstraintSet.SizeConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html) property type.
    #[derive(Debug, Default)]
    pub struct SizeConstraint {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html#cfn-wafregional-sizeconstraintset-sizeconstraint-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html#cfn-wafregional-sizeconstraintset-sizeconstraint-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html#cfn-wafregional-sizeconstraintset-sizeconstraint-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: ::Value<u32>,
        /// Property [`TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html#cfn-wafregional-sizeconstraintset-sizeconstraint-texttransformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for SizeConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformation", &self.text_transformation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SizeConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SizeConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SizeConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SizeConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut size: Option<::Value<u32>> = None;
                    let mut text_transformation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformation" => {
                                text_transformation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SizeConstraint {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                        text_transformation: text_transformation.ok_or(::serde::de::Error::missing_field("TextTransformation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod sql_injection_match_set {
    //! Property types for the `SqlInjectionMatchSet` resource.

    /// The [`AWS::WAFRegional::SqlInjectionMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-fieldtomatch.html#cfn-wafregional-sqlinjectionmatchset-fieldtomatch-data).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-fieldtomatch.html#cfn-wafregional-sqlinjectionmatchset-fieldtomatch-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data) = self.data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", data)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        data: data,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFRegional::SqlInjectionMatchSet.SqlInjectionMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple.html) property type.
    #[derive(Debug, Default)]
    pub struct SqlInjectionMatchTuple {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple.html#cfn-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple.html#cfn-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple-texttransformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for SqlInjectionMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformation", &self.text_transformation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqlInjectionMatchTuple {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqlInjectionMatchTuple, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqlInjectionMatchTuple;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqlInjectionMatchTuple")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformation" => {
                                text_transformation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqlInjectionMatchTuple {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformation: text_transformation.ok_or(::serde::de::Error::missing_field("TextTransformation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod web_acl {
    //! Property types for the `WebACL` resource.

    /// The [`AWS::WAFRegional::WebACL.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-action.html#cfn-wafregional-webacl-action-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFRegional::WebACL.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html#cfn-wafregional-webacl-rule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<Action>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html#cfn-wafregional-webacl-rule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`RuleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html#cfn-wafregional-webacl-rule-ruleid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleId", &self.rule_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<Action>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut rule_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleId" => {
                                rule_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        rule_id: rule_id.ok_or(::serde::de::Error::missing_field("RuleId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod xss_match_set {
    //! Property types for the `XssMatchSet` resource.

    /// The [`AWS::WAFRegional::XssMatchSet.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`Data`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-fieldtomatch.html#cfn-wafregional-xssmatchset-fieldtomatch-data).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-fieldtomatch.html#cfn-wafregional-xssmatchset-fieldtomatch-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data) = self.data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", data)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        data: data,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFRegional::XssMatchSet.XssMatchTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-xssmatchtuple.html) property type.
    #[derive(Debug, Default)]
    pub struct XssMatchTuple {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-xssmatchtuple.html#cfn-wafregional-xssmatchset-xssmatchtuple-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-xssmatchtuple.html#cfn-wafregional-xssmatchset-xssmatchtuple-texttransformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for XssMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformation", &self.text_transformation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for XssMatchTuple {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<XssMatchTuple, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = XssMatchTuple;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type XssMatchTuple")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformation" => {
                                text_transformation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(XssMatchTuple {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformation: text_transformation.ok_or(::serde::de::Error::missing_field("TextTransformation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
