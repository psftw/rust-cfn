//! Types for the `RDS` service.

/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
#[derive(Debug)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterProperties {
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    /// Property `BackupRetentionPeriod`.
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: u32,
    /// Property `DBClusterIdentifier`.
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: String,
    /// Property `DBClusterParameterGroupName`.
    #[serde(rename="DBClusterParameterGroupName")]
    pub db_cluster_parameter_group_name: String,
    /// Property `DBSubnetGroupName`.
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    pub engine: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    /// Property `MasterUserPassword`.
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: String,
    /// Property `MasterUsername`.
    #[serde(rename="MasterUsername")]
    pub master_username: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `PreferredBackupWindow`.
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: String,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `ReplicationSourceIdentifier`.
    #[serde(rename="ReplicationSourceIdentifier")]
    pub replication_source_identifier: String,
    /// Property `SnapshotIdentifier`.
    #[serde(rename="SnapshotIdentifier")]
    pub snapshot_identifier: String,
    /// Property `StorageEncrypted`.
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: bool,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
}

impl<'a> ::Resource<'a> for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::RDS::DBCluster";
    fn properties(&self) -> &DBClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBCluster {}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Family`.
    #[serde(rename="Family")]
    pub family: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    pub parameters: ::json::Value,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for DBClusterParameterGroup {
    type Properties = DBClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBClusterParameterGroup";
    fn properties(&self) -> &DBClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBClusterParameterGroup {}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource type.
#[derive(Debug)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBInstanceProperties {
    /// Property `AllocatedStorage`.
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: String,
    /// Property `AllowMajorVersionUpgrade`.
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: bool,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    /// Property `BackupRetentionPeriod`.
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: String,
    /// Property `CharacterSetName`.
    #[serde(rename="CharacterSetName")]
    pub character_set_name: String,
    /// Property `CopyTagsToSnapshot`.
    #[serde(rename="CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: bool,
    /// Property `DBClusterIdentifier`.
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: String,
    /// Property `DBInstanceClass`.
    #[serde(rename="DBInstanceClass")]
    pub db_instance_class: String,
    /// Property `DBInstanceIdentifier`.
    #[serde(rename="DBInstanceIdentifier")]
    pub db_instance_identifier: String,
    /// Property `DBName`.
    #[serde(rename="DBName")]
    pub db_name: String,
    /// Property `DBParameterGroupName`.
    #[serde(rename="DBParameterGroupName")]
    pub db_parameter_group_name: String,
    /// Property `DBSecurityGroups`.
    #[serde(rename="DBSecurityGroups")]
    pub db_security_groups: Vec<String>,
    /// Property `DBSnapshotIdentifier`.
    #[serde(rename="DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,
    /// Property `DBSubnetGroupName`.
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    /// Property `Domain`.
    #[serde(rename="Domain")]
    pub domain: String,
    /// Property `DomainIAMRoleName`.
    #[serde(rename="DomainIAMRoleName")]
    pub domain_iam_role_name: String,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    pub engine: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    /// Property `Iops`.
    #[serde(rename="Iops")]
    pub iops: u32,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    /// Property `LicenseModel`.
    #[serde(rename="LicenseModel")]
    pub license_model: String,
    /// Property `MasterUserPassword`.
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: String,
    /// Property `MasterUsername`.
    #[serde(rename="MasterUsername")]
    pub master_username: String,
    /// Property `MonitoringInterval`.
    #[serde(rename="MonitoringInterval")]
    pub monitoring_interval: u32,
    /// Property `MonitoringRoleArn`.
    #[serde(rename="MonitoringRoleArn")]
    pub monitoring_role_arn: String,
    /// Property `MultiAZ`.
    #[serde(rename="MultiAZ")]
    pub multi_az: bool,
    /// Property `OptionGroupName`.
    #[serde(rename="OptionGroupName")]
    pub option_group_name: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: String,
    /// Property `PreferredBackupWindow`.
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: String,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `PubliclyAccessible`.
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: bool,
    /// Property `SourceDBInstanceIdentifier`.
    #[serde(rename="SourceDBInstanceIdentifier")]
    pub source_db_instance_identifier: String,
    /// Property `SourceRegion`.
    #[serde(rename="SourceRegion")]
    pub source_region: String,
    /// Property `StorageEncrypted`.
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: bool,
    /// Property `StorageType`.
    #[serde(rename="StorageType")]
    pub storage_type: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `Timezone`.
    #[serde(rename="Timezone")]
    pub timezone: String,
    /// Property `VPCSecurityGroups`.
    #[serde(rename="VPCSecurityGroups")]
    pub vpc_security_groups: Vec<String>,
}

impl<'a> ::Resource<'a> for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::RDS::DBInstance";
    fn properties(&self) -> &DBInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBInstance {}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Family`.
    #[serde(rename="Family")]
    pub family: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for DBParameterGroup {
    type Properties = DBParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBParameterGroup";
    fn properties(&self) -> &DBParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBParameterGroup {}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSecurityGroupProperties {
    /// Property `DBSecurityGroupIngress`.
    #[serde(rename="DBSecurityGroupIngress")]
    pub db_security_group_ingress: Vec<self::db_security_group::Ingress>,
    /// Property `EC2VpcId`.
    #[serde(rename="EC2VpcId")]
    pub ec2_vpc_id: String,
    /// Property `GroupDescription`.
    #[serde(rename="GroupDescription")]
    pub group_description: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for DBSecurityGroup {
    type Properties = DBSecurityGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroup";
    fn properties(&self) -> &DBSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroup {}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSecurityGroupIngressProperties {
    /// Property `CIDRIP`.
    #[serde(rename="CIDRIP")]
    pub cidrip: String,
    /// Property `DBSecurityGroupName`.
    #[serde(rename="DBSecurityGroupName")]
    pub db_security_group_name: String,
    /// Property `EC2SecurityGroupId`.
    #[serde(rename="EC2SecurityGroupId")]
    pub ec2_security_group_id: String,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: String,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename="EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: String,
}

impl<'a> ::Resource<'a> for DBSecurityGroupIngress {
    type Properties = DBSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroupIngress";
    fn properties(&self) -> &DBSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroupIngress {}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource type.
#[derive(Debug)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSubnetGroupProperties {
    /// Property `DBSubnetGroupDescription`.
    #[serde(rename="DBSubnetGroupDescription")]
    pub db_subnet_group_description: String,
    /// Property `DBSubnetGroupName`.
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for DBSubnetGroup {
    type Properties = DBSubnetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSubnetGroup";
    fn properties(&self) -> &DBSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSubnetGroup {}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
#[derive(Debug)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    /// Property `Enabled`.
    #[serde(rename="Enabled")]
    pub enabled: bool,
    /// Property `EventCategories`.
    #[serde(rename="EventCategories")]
    pub event_categories: Vec<String>,
    /// Property `SnsTopicArn`.
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: String,
    /// Property `SourceIds`.
    #[serde(rename="SourceIds")]
    pub source_ids: Vec<String>,
    /// Property `SourceType`.
    #[serde(rename="SourceType")]
    pub source_type: String,
}

impl<'a> ::Resource<'a> for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::RDS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
#[derive(Debug)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionGroupProperties {
    /// Property `EngineName`.
    #[serde(rename="EngineName")]
    pub engine_name: String,
    /// Property `MajorEngineVersion`.
    #[serde(rename="MajorEngineVersion")]
    pub major_engine_version: String,
    /// Property `OptionConfigurations`.
    #[serde(rename="OptionConfigurations")]
    pub option_configurations: Vec<self::option_group::OptionConfiguration>,
    /// Property `OptionGroupDescription`.
    #[serde(rename="OptionGroupDescription")]
    pub option_group_description: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for OptionGroup {
    type Properties = OptionGroupProperties;
    const TYPE: &'static str = "AWS::RDS::OptionGroup";
    fn properties(&self) -> &OptionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OptionGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OptionGroup {}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

pub mod db_security_group {
    //! Property types for the `DBSecurityGroup` resource.

    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ingress {
        /// Property `CIDRIP`.
        #[serde(rename="CIDRIP")]
        pub cidrip: String,
        /// Property `EC2SecurityGroupId`.
        #[serde(rename="EC2SecurityGroupId")]
        pub ec2_security_group_id: String,
        /// Property `EC2SecurityGroupName`.
        #[serde(rename="EC2SecurityGroupName")]
        pub ec2_security_group_name: String,
        /// Property `EC2SecurityGroupOwnerId`.
        #[serde(rename="EC2SecurityGroupOwnerId")]
        pub ec2_security_group_owner_id: String,
    }
}

pub mod option_group {
    //! Property types for the `OptionGroup` resource.

    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionConfiguration {
        /// Property `DBSecurityGroupMemberships`.
        #[serde(rename="DBSecurityGroupMemberships")]
        pub db_security_group_memberships: Vec<String>,
        /// Property `OptionName`.
        #[serde(rename="OptionName")]
        pub option_name: String,
        /// Property `OptionSettings`.
        #[serde(rename="OptionSettings")]
        pub option_settings: OptionSetting,
        /// Property `OptionVersion`.
        #[serde(rename="OptionVersion")]
        pub option_version: String,
        /// Property `Port`.
        #[serde(rename="Port")]
        pub port: u32,
        /// Property `VpcSecurityGroupMemberships`.
        #[serde(rename="VpcSecurityGroupMemberships")]
        pub vpc_security_group_memberships: Vec<String>,
    }

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionSetting {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}
