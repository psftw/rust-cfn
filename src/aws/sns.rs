//! Types for the `SNS` service.

/// The [`AWS::SNS::Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html) resource type.
#[derive(Debug, Default)]
pub struct Subscription {
    properties: SubscriptionProperties
}

/// Properties for the `Subscription` resource.
#[derive(Debug, Default)]
pub struct SubscriptionProperties {
    /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html#cfn-sns-endpoint).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint: Option<::Value<String>>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html#cfn-sns-protocol).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol: Option<::Value<String>>,
    /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html#topicarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub topic_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for SubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint) = self.endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
        }
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        if let Some(ref topic_arn) = self.topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint: Option<::Value<String>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut topic_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Endpoint" => {
                            endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TopicArn" => {
                            topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubscriptionProperties {
                    endpoint: endpoint,
                    protocol: protocol,
                    topic_arn: topic_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Subscription {
    type Properties = SubscriptionProperties;
    const TYPE: &'static str = "AWS::SNS::Subscription";
    fn properties(&self) -> &SubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Subscription {}

impl From<SubscriptionProperties> for Subscription {
    fn from(properties: SubscriptionProperties) -> Subscription {
        Subscription { properties }
    }
}

/// The [`AWS::SNS::Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html) resource type.
#[derive(Debug, Default)]
pub struct Topic {
    properties: TopicProperties
}

/// Properties for the `Topic` resource.
#[derive(Debug, Default)]
pub struct TopicProperties {
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html#cfn-sns-topic-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html#cfn-sns-topic-subscription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscription: Option<::ValueList<self::topic::Subscription>>,
    /// Property [`TopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html#cfn-sns-topic-topicname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub topic_name: Option<::Value<String>>,
}

impl ::serde::Serialize for TopicProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref subscription) = self.subscription {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscription", subscription)?;
        }
        if let Some(ref topic_name) = self.topic_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicName", topic_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TopicProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopicProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TopicProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut display_name: Option<::Value<String>> = None;
                let mut subscription: Option<::ValueList<self::topic::Subscription>> = None;
                let mut topic_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subscription" => {
                            subscription = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TopicName" => {
                            topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TopicProperties {
                    display_name: display_name,
                    subscription: subscription,
                    topic_name: topic_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Topic {
    type Properties = TopicProperties;
    const TYPE: &'static str = "AWS::SNS::Topic";
    fn properties(&self) -> &TopicProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Topic {}

impl From<TopicProperties> for Topic {
    fn from(properties: TopicProperties) -> Topic {
        Topic { properties }
    }
}

/// The [`AWS::SNS::TopicPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html) resource type.
#[derive(Debug, Default)]
pub struct TopicPolicy {
    properties: TopicPolicyProperties
}

/// Properties for the `TopicPolicy` resource.
#[derive(Debug, Default)]
pub struct TopicPolicyProperties {
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html#cfn-sns-topicpolicy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
    /// Property [`Topics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html#cfn-sns-topicpolicy-topics).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topics: ::ValueList<String>,
}

impl ::serde::Serialize for TopicPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topics", &self.topics)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TopicPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopicPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TopicPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut topics: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Topics" => {
                            topics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TopicPolicyProperties {
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    topics: topics.ok_or(::serde::de::Error::missing_field("Topics"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TopicPolicy {
    type Properties = TopicPolicyProperties;
    const TYPE: &'static str = "AWS::SNS::TopicPolicy";
    fn properties(&self) -> &TopicPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicPolicy {}

impl From<TopicPolicyProperties> for TopicPolicy {
    fn from(properties: TopicPolicyProperties) -> TopicPolicy {
        TopicPolicy { properties }
    }
}

pub mod topic {
    //! Property types for the `Topic` resource.

    /// The [`AWS::SNS::Topic.Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-subscription.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscription {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-subscription.html#cfn-sns-topic-subscription-endpoint).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint: ::Value<String>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-subscription.html#cfn-sns-topic-subscription-protocol).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subscription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subscription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subscription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subscription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscription {
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
