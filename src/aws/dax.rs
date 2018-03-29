//! Types for the `DAX` service.

/// The [`AWS::DAX::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterProperties {
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    /// Property `ClusterName`.
    #[serde(rename="ClusterName")]
    pub cluster_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `IAMRoleARN`.
    #[serde(rename="IAMRoleARN")]
    pub iam_role_arn: String,
    /// Property `NodeType`.
    #[serde(rename="NodeType")]
    pub node_type: String,
    /// Property `NotificationTopicARN`.
    #[serde(rename="NotificationTopicARN")]
    pub notification_topic_arn: String,
    /// Property `ParameterGroupName`.
    #[serde(rename="ParameterGroupName")]
    pub parameter_group_name: String,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `ReplicationFactor`.
    #[serde(rename="ReplicationFactor")]
    pub replication_factor: u32,
    /// Property `SecurityGroupIds`.
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// Property `SubnetGroupName`.
    #[serde(rename="SubnetGroupName")]
    pub subnet_group_name: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::json::Value,
}

impl<'a> ::Resource<'a> for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::DAX::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::DAX::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-parametergroup.html) resource type.
#[derive(Debug)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `ParameterGroupName`.
    #[serde(rename="ParameterGroupName")]
    pub parameter_group_name: String,
    /// Property `ParameterNameValues`.
    #[serde(rename="ParameterNameValues")]
    pub parameter_name_values: ::json::Value,
}

impl<'a> ::Resource<'a> for ParameterGroup {
    type Properties = ParameterGroupProperties;
    const TYPE: &'static str = "AWS::DAX::ParameterGroup";
    fn properties(&self) -> &ParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ParameterGroup {}

impl From<ParameterGroupProperties> for ParameterGroup {
    fn from(properties: ParameterGroupProperties) -> ParameterGroup {
        ParameterGroup { properties }
    }
}

/// The [`AWS::DAX::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-subnetgroup.html) resource type.
#[derive(Debug)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetGroupProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `SubnetGroupName`.
    #[serde(rename="SubnetGroupName")]
    pub subnet_group_name: String,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
}

impl<'a> ::Resource<'a> for SubnetGroup {
    type Properties = SubnetGroupProperties;
    const TYPE: &'static str = "AWS::DAX::SubnetGroup";
    fn properties(&self) -> &SubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetGroup {}

impl From<SubnetGroupProperties> for SubnetGroup {
    fn from(properties: SubnetGroupProperties) -> SubnetGroup {
        SubnetGroup { properties }
    }
}
