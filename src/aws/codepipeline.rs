//! Types for the `CodePipeline` service.

/// The [`AWS::CodePipeline::CustomActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html) resource type.
#[derive(Debug)]
pub struct CustomActionType {
    properties: CustomActionTypeProperties
}

/// Properties for the `CustomActionType` resource.
#[derive(Debug)]
pub struct CustomActionTypeProperties {
    /// Property `Category`.
    pub category: ::Value<String>,
    /// Property `ConfigurationProperties`.
    pub configuration_properties: Option<::ValueList<self::custom_action_type::ConfigurationProperties>>,
    /// Property `InputArtifactDetails`.
    pub input_artifact_details: ::Value<self::custom_action_type::ArtifactDetails>,
    /// Property `OutputArtifactDetails`.
    pub output_artifact_details: ::Value<self::custom_action_type::ArtifactDetails>,
    /// Property `Provider`.
    pub provider: ::Value<String>,
    /// Property `Settings`.
    pub settings: Option<::Value<self::custom_action_type::Settings>>,
    /// Property `Version`.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for CustomActionTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Category", &self.category)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", &self.configuration_properties)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputArtifactDetails", &self.input_artifact_details)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputArtifactDetails", &self.output_artifact_details)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", &self.provider)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Settings", &self.settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomActionTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomActionTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomActionTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomActionTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut category = None;
                let mut configuration_properties = None;
                let mut input_artifact_details = None;
                let mut output_artifact_details = None;
                let mut provider = None;
                let mut settings = None;
                let mut version = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Category" => {
                            category = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ConfigurationProperties" => {
                            configuration_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InputArtifactDetails" => {
                            input_artifact_details = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OutputArtifactDetails" => {
                            output_artifact_details = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Provider" => {
                            provider = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Settings" => {
                            settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Version" => {
                            version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(CustomActionTypeProperties {
                    category: category.ok_or(::serde::de::Error::missing_field("Category"))?,
                    configuration_properties: configuration_properties,
                    input_artifact_details: input_artifact_details.ok_or(::serde::de::Error::missing_field("InputArtifactDetails"))?,
                    output_artifact_details: output_artifact_details.ok_or(::serde::de::Error::missing_field("OutputArtifactDetails"))?,
                    provider: provider.ok_or(::serde::de::Error::missing_field("Provider"))?,
                    settings: settings,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for CustomActionType {
    type Properties = CustomActionTypeProperties;
    const TYPE: &'static str = "AWS::CodePipeline::CustomActionType";
    fn properties(&self) -> &CustomActionTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomActionTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomActionType {}

impl From<CustomActionTypeProperties> for CustomActionType {
    fn from(properties: CustomActionTypeProperties) -> CustomActionType {
        CustomActionType { properties }
    }
}

/// The [`AWS::CodePipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-pipeline.html) resource type.
#[derive(Debug)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug)]
pub struct PipelineProperties {
    /// Property `ArtifactStore`.
    pub artifact_store: ::Value<self::pipeline::ArtifactStore>,
    /// Property `DisableInboundStageTransitions`.
    pub disable_inbound_stage_transitions: Option<::ValueList<self::pipeline::StageTransition>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `RestartExecutionOnUpdate`.
    pub restart_execution_on_update: Option<::Value<bool>>,
    /// Property `RoleArn`.
    pub role_arn: ::Value<String>,
    /// Property `Stages`.
    pub stages: ::ValueList<self::pipeline::StageDeclaration>,
}

impl ::serde::Serialize for PipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArtifactStore", &self.artifact_store)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableInboundStageTransitions", &self.disable_inbound_stage_transitions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartExecutionOnUpdate", &self.restart_execution_on_update)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stages", &self.stages)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut artifact_store = None;
                let mut disable_inbound_stage_transitions = None;
                let mut name = None;
                let mut restart_execution_on_update = None;
                let mut role_arn = None;
                let mut stages = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ArtifactStore" => {
                            artifact_store = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DisableInboundStageTransitions" => {
                            disable_inbound_stage_transitions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestartExecutionOnUpdate" => {
                            restart_execution_on_update = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleArn" => {
                            role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Stages" => {
                            stages = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(PipelineProperties {
                    artifact_store: artifact_store.ok_or(::serde::de::Error::missing_field("ArtifactStore"))?,
                    disable_inbound_stage_transitions: disable_inbound_stage_transitions,
                    name: name,
                    restart_execution_on_update: restart_execution_on_update,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stages: stages.ok_or(::serde::de::Error::missing_field("Stages"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::CodePipeline::Pipeline";
    fn properties(&self) -> &PipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod custom_action_type {
    //! Property types for the `CustomActionType` resource.

    /// The [`AWS::CodePipeline::CustomActionType.ArtifactDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-artifactdetails.html) property type.
    #[derive(Debug)]
    pub struct ArtifactDetails {
        /// Property `MaximumCount`.
        pub maximum_count: ::Value<u32>,
        /// Property `MinimumCount`.
        pub minimum_count: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ArtifactDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumCount", &self.maximum_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumCount", &self.minimum_count)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArtifactDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArtifactDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArtifactDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArtifactDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_count = None;
                    let mut minimum_count = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumCount" => {
                                maximum_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinimumCount" => {
                                minimum_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ArtifactDetails {
                        maximum_count: maximum_count.ok_or(::serde::de::Error::missing_field("MaximumCount"))?,
                        minimum_count: minimum_count.ok_or(::serde::de::Error::missing_field("MinimumCount"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::CustomActionType.ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-configurationproperties.html) property type.
    #[derive(Debug)]
    pub struct ConfigurationProperties {
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `Key`.
        pub key: ::Value<bool>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Queryable`.
        pub queryable: Option<::Value<bool>>,
        /// Property `Required`.
        pub required: ::Value<bool>,
        /// Property `Secret`.
        pub secret: ::Value<bool>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigurationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queryable", &self.queryable)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", &self.required)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Secret", &self.secret)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description = None;
                    let mut key = None;
                    let mut name = None;
                    let mut queryable = None;
                    let mut required = None;
                    let mut secret = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Queryable" => {
                                queryable = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Required" => {
                                required = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Secret" => {
                                secret = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationProperties {
                        description: description,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        queryable: queryable,
                        required: required.ok_or(::serde::de::Error::missing_field("Required"))?,
                        secret: secret.ok_or(::serde::de::Error::missing_field("Secret"))?,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::CustomActionType.Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-settings.html) property type.
    #[derive(Debug)]
    pub struct Settings {
        /// Property `EntityUrlTemplate`.
        pub entity_url_template: Option<::Value<String>>,
        /// Property `ExecutionUrlTemplate`.
        pub execution_url_template: Option<::Value<String>>,
        /// Property `RevisionUrlTemplate`.
        pub revision_url_template: Option<::Value<String>>,
        /// Property `ThirdPartyConfigurationUrl`.
        pub third_party_configuration_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityUrlTemplate", &self.entity_url_template)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionUrlTemplate", &self.execution_url_template)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevisionUrlTemplate", &self.revision_url_template)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThirdPartyConfigurationUrl", &self.third_party_configuration_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_url_template = None;
                    let mut execution_url_template = None;
                    let mut revision_url_template = None;
                    let mut third_party_configuration_url = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityUrlTemplate" => {
                                entity_url_template = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExecutionUrlTemplate" => {
                                execution_url_template = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RevisionUrlTemplate" => {
                                revision_url_template = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThirdPartyConfigurationUrl" => {
                                third_party_configuration_url = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Settings {
                        entity_url_template: entity_url_template,
                        execution_url_template: execution_url_template,
                        revision_url_template: revision_url_template,
                        third_party_configuration_url: third_party_configuration_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod pipeline {
    //! Property types for the `Pipeline` resource.

    /// The [`AWS::CodePipeline::Pipeline.ActionDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions.html) property type.
    #[derive(Debug)]
    pub struct ActionDeclaration {
        /// Property `ActionTypeId`.
        pub action_type_id: ::Value<ActionTypeId>,
        /// Property `Configuration`.
        pub configuration: Option<::Value<::json::Value>>,
        /// Property `InputArtifacts`.
        pub input_artifacts: Option<::ValueList<InputArtifact>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `OutputArtifacts`.
        pub output_artifacts: Option<::ValueList<OutputArtifact>>,
        /// Property `RoleArn`.
        pub role_arn: Option<::Value<String>>,
        /// Property `RunOrder`.
        pub run_order: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ActionDeclaration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionTypeId", &self.action_type_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputArtifacts", &self.input_artifacts)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputArtifacts", &self.output_artifacts)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunOrder", &self.run_order)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionDeclaration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionDeclaration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionDeclaration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionDeclaration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_type_id = None;
                    let mut configuration = None;
                    let mut input_artifacts = None;
                    let mut name = None;
                    let mut output_artifacts = None;
                    let mut role_arn = None;
                    let mut run_order = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionTypeId" => {
                                action_type_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configuration" => {
                                configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InputArtifacts" => {
                                input_artifacts = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OutputArtifacts" => {
                                output_artifacts = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleArn" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RunOrder" => {
                                run_order = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionDeclaration {
                        action_type_id: action_type_id.ok_or(::serde::de::Error::missing_field("ActionTypeId"))?,
                        configuration: configuration,
                        input_artifacts: input_artifacts,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        output_artifacts: output_artifacts,
                        role_arn: role_arn,
                        run_order: run_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.ActionTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-actiontypeid.html) property type.
    #[derive(Debug)]
    pub struct ActionTypeId {
        /// Property `Category`.
        pub category: ::Value<String>,
        /// Property `Owner`.
        pub owner: ::Value<String>,
        /// Property `Provider`.
        pub provider: ::Value<String>,
        /// Property `Version`.
        pub version: ::Value<String>,
    }

    impl ::codec::SerializeValue for ActionTypeId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Category", &self.category)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", &self.owner)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", &self.provider)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionTypeId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionTypeId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionTypeId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionTypeId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut category = None;
                    let mut owner = None;
                    let mut provider = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Category" => {
                                category = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Owner" => {
                                owner = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Provider" => {
                                provider = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionTypeId {
                        category: category.ok_or(::serde::de::Error::missing_field("Category"))?,
                        owner: owner.ok_or(::serde::de::Error::missing_field("Owner"))?,
                        provider: provider.ok_or(::serde::de::Error::missing_field("Provider"))?,
                        version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.ArtifactStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore.html) property type.
    #[derive(Debug)]
    pub struct ArtifactStore {
        /// Property `EncryptionKey`.
        pub encryption_key: Option<::Value<EncryptionKey>>,
        /// Property `Location`.
        pub location: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ArtifactStore {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", &self.encryption_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArtifactStore {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArtifactStore, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArtifactStore;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArtifactStore")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_key = None;
                    let mut location = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionKey" => {
                                encryption_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Location" => {
                                location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ArtifactStore {
                        encryption_key: encryption_key,
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.BlockerDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-blockers.html) property type.
    #[derive(Debug)]
    pub struct BlockerDeclaration {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for BlockerDeclaration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockerDeclaration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockerDeclaration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockerDeclaration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockerDeclaration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockerDeclaration {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore-encryptionkey.html) property type.
    #[derive(Debug)]
    pub struct EncryptionKey {
        /// Property `Id`.
        pub id: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionKey {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.InputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-inputartifacts.html) property type.
    #[derive(Debug)]
    pub struct InputArtifact {
        /// Property `Name`.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputArtifact {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputArtifact {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputArtifact, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputArtifact;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputArtifact")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InputArtifact {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.OutputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-outputartifacts.html) property type.
    #[derive(Debug)]
    pub struct OutputArtifact {
        /// Property `Name`.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for OutputArtifact {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputArtifact {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputArtifact, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputArtifact;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputArtifact")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputArtifact {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.StageDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages.html) property type.
    #[derive(Debug)]
    pub struct StageDeclaration {
        /// Property `Actions`.
        pub actions: ::ValueList<ActionDeclaration>,
        /// Property `Blockers`.
        pub blockers: Option<::ValueList<BlockerDeclaration>>,
        /// Property `Name`.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for StageDeclaration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Blockers", &self.blockers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StageDeclaration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StageDeclaration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StageDeclaration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StageDeclaration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions = None;
                    let mut blockers = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Blockers" => {
                                blockers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StageDeclaration {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        blockers: blockers,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodePipeline::Pipeline.StageTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-disableinboundstagetransitions.html) property type.
    #[derive(Debug)]
    pub struct StageTransition {
        /// Property `Reason`.
        pub reason: ::Value<String>,
        /// Property `StageName`.
        pub stage_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for StageTransition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reason", &self.reason)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", &self.stage_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StageTransition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StageTransition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StageTransition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StageTransition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reason = None;
                    let mut stage_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Reason" => {
                                reason = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StageName" => {
                                stage_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StageTransition {
                        reason: reason.ok_or(::serde::de::Error::missing_field("Reason"))?,
                        stage_name: stage_name.ok_or(::serde::de::Error::missing_field("StageName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
