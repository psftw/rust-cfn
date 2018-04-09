//! Types for the `WAFRegional` service.

/// The [`AWS::WAFRegional::ByteMatchSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html) resource type.
#[derive(Debug)]
pub struct ByteMatchSet {
    properties: ByteMatchSetProperties
}

/// Properties for the `ByteMatchSet` resource.
#[derive(Debug)]
pub struct ByteMatchSetProperties {
    /// Property `ByteMatchTuples`.
    pub byte_match_tuples: Option<::ValueList<self::byte_match_set::ByteMatchTuple>>,
    /// Property `Name`.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for ByteMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ByteMatchTuples", &self.byte_match_tuples)?;
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
                let mut byte_match_tuples = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ByteMatchTuples" => {
                            byte_match_tuples = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for ByteMatchSet {
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
#[derive(Debug)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug)]
pub struct IPSetProperties {
    /// Property `IPSetDescriptors`.
    pub ip_set_descriptors: Option<::ValueList<self::ip_set::IPSetDescriptor>>,
    /// Property `Name`.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for IPSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetDescriptors", &self.ip_set_descriptors)?;
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
                let mut ip_set_descriptors = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IPSetDescriptors" => {
                            ip_set_descriptors = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for IPSet {
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
#[derive(Debug)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug)]
pub struct RuleProperties {
    /// Property `MetricName`.
    pub metric_name: ::Value<String>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Predicates`.
    pub predicates: Option<::ValueList<self::rule::Predicate>>,
}

impl ::serde::Serialize for RuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Predicates", &self.predicates)?;
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
                let mut metric_name = None;
                let mut name = None;
                let mut predicates = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MetricName" => {
                            metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Predicates" => {
                            predicates = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for Rule {
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
#[derive(Debug)]
pub struct SizeConstraintSet {
    properties: SizeConstraintSetProperties
}

/// Properties for the `SizeConstraintSet` resource.
#[derive(Debug)]
pub struct SizeConstraintSetProperties {
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `SizeConstraints`.
    pub size_constraints: Option<::ValueList<self::size_constraint_set::SizeConstraint>>,
}

impl ::serde::Serialize for SizeConstraintSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeConstraints", &self.size_constraints)?;
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
                let mut name = None;
                let mut size_constraints = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SizeConstraints" => {
                            size_constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for SizeConstraintSet {
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
#[derive(Debug)]
pub struct SqlInjectionMatchSet {
    properties: SqlInjectionMatchSetProperties
}

/// Properties for the `SqlInjectionMatchSet` resource.
#[derive(Debug)]
pub struct SqlInjectionMatchSetProperties {
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `SqlInjectionMatchTuples`.
    pub sql_injection_match_tuples: Option<::ValueList<self::sql_injection_match_set::SqlInjectionMatchTuple>>,
}

impl ::serde::Serialize for SqlInjectionMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlInjectionMatchTuples", &self.sql_injection_match_tuples)?;
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
                let mut name = None;
                let mut sql_injection_match_tuples = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SqlInjectionMatchTuples" => {
                            sql_injection_match_tuples = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for SqlInjectionMatchSet {
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
#[derive(Debug)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Debug)]
pub struct WebACLProperties {
    /// Property `DefaultAction`.
    pub default_action: ::Value<self::web_acl::Action>,
    /// Property `MetricName`.
    pub metric_name: ::Value<String>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Rules`.
    pub rules: Option<::ValueList<self::web_acl::Rule>>,
}

impl ::serde::Serialize for WebACLProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAction", &self.default_action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
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
                let mut default_action = None;
                let mut metric_name = None;
                let mut name = None;
                let mut rules = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultAction" => {
                            default_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MetricName" => {
                            metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Rules" => {
                            rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for WebACL {
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
#[derive(Debug)]
pub struct WebACLAssociation {
    properties: WebACLAssociationProperties
}

/// Properties for the `WebACLAssociation` resource.
#[derive(Debug)]
pub struct WebACLAssociationProperties {
    /// Property `ResourceArn`.
    pub resource_arn: ::Value<String>,
    /// Property `WebACLId`.
    pub web_acl_id: ::Value<String>,
}

impl ::serde::Serialize for WebACLAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
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
                let mut resource_arn = None;
                let mut web_acl_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceArn" => {
                            resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "WebACLId" => {
                            web_acl_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for WebACLAssociation {
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
#[derive(Debug)]
pub struct XssMatchSet {
    properties: XssMatchSetProperties
}

/// Properties for the `XssMatchSet` resource.
#[derive(Debug)]
pub struct XssMatchSetProperties {
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `XssMatchTuples`.
    pub xss_match_tuples: Option<::ValueList<self::xss_match_set::XssMatchTuple>>,
}

impl ::serde::Serialize for XssMatchSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "XssMatchTuples", &self.xss_match_tuples)?;
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
                let mut name = None;
                let mut xss_match_tuples = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "XssMatchTuples" => {
                            xss_match_tuples = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for XssMatchSet {
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
    #[derive(Debug)]
    pub struct ByteMatchTuple {
        /// Property `FieldToMatch`.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `PositionalConstraint`.
        pub positional_constraint: ::Value<String>,
        /// Property `TargetString`.
        pub target_string: Option<::Value<String>>,
        /// Property `TargetStringBase64`.
        pub target_string_base64: Option<::Value<String>>,
        /// Property `TextTransformation`.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for ByteMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PositionalConstraint", &self.positional_constraint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetString", &self.target_string)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetStringBase64", &self.target_string_base64)?;
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
                    let mut field_to_match = None;
                    let mut positional_constraint = None;
                    let mut target_string = None;
                    let mut target_string_base64 = None;
                    let mut text_transformation = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PositionalConstraint" => {
                                positional_constraint = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetString" => {
                                target_string = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetStringBase64" => {
                                target_string_base64 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TextTransformation" => {
                                text_transformation = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct FieldToMatch {
        /// Property `Data`.
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", &self.data)?;
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
                    let mut data = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct IPSetDescriptor {
        /// Property `Type`.
        pub type_: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for IPSetDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut type_ = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct Predicate {
        /// Property `DataId`.
        pub data_id: ::Value<String>,
        /// Property `Negated`.
        pub negated: ::Value<bool>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Predicate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut data_id = None;
                    let mut negated = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataId" => {
                                data_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Negated" => {
                                negated = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct FieldToMatch {
        /// Property `Data`.
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", &self.data)?;
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
                    let mut data = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct SizeConstraint {
        /// Property `ComparisonOperator`.
        pub comparison_operator: ::Value<String>,
        /// Property `FieldToMatch`.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `Size`.
        pub size: ::Value<u32>,
        /// Property `TextTransformation`.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for SizeConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut comparison_operator = None;
                    let mut field_to_match = None;
                    let mut size = None;
                    let mut text_transformation = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FieldToMatch" => {
                                field_to_match = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Size" => {
                                size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TextTransformation" => {
                                text_transformation = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct FieldToMatch {
        /// Property `Data`.
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", &self.data)?;
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
                    let mut data = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct SqlInjectionMatchTuple {
        /// Property `FieldToMatch`.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `TextTransformation`.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for SqlInjectionMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut field_to_match = None;
                    let mut text_transformation = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TextTransformation" => {
                                text_transformation = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct Action {
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct Rule {
        /// Property `Action`.
        pub action: ::Value<Action>,
        /// Property `Priority`.
        pub priority: ::Value<u32>,
        /// Property `RuleId`.
        pub rule_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut action = None;
                    let mut priority = None;
                    let mut rule_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Priority" => {
                                priority = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RuleId" => {
                                rule_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct FieldToMatch {
        /// Property `Data`.
        pub data: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Data", &self.data)?;
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
                    let mut data = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Data" => {
                                data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct XssMatchTuple {
        /// Property `FieldToMatch`.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property `TextTransformation`.
        pub text_transformation: ::Value<String>,
    }

    impl ::codec::SerializeValue for XssMatchTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut field_to_match = None;
                    let mut text_transformation = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TextTransformation" => {
                                text_transformation = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
