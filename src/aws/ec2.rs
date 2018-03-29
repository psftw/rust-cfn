/// The [`AWS::EC2::CustomerGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-customer-gateway.html) resource type.
pub struct CustomerGateway {
    properties: CustomerGatewayProperties
}

/// Properties for the `CustomerGateway` resource.
#[derive(Serialize, Deserialize)]
pub struct CustomerGatewayProperties {
    #[serde(rename="BgpAsn")]
    pub bgp_asn: u32,
    #[serde(rename="IpAddress")]
    pub ip_address: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for CustomerGateway {
    type Properties = CustomerGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::CustomerGateway";
    fn properties(&self) -> &CustomerGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomerGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomerGateway {}

impl From<CustomerGatewayProperties> for CustomerGateway {
    fn from(properties: CustomerGatewayProperties) -> CustomerGateway {
        CustomerGateway { properties }
    }
}

/// The [`AWS::EC2::DHCPOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-dhcp-options.html) resource type.
pub struct DHCPOptions {
    properties: DHCPOptionsProperties
}

/// Properties for the `DHCPOptions` resource.
#[derive(Serialize, Deserialize)]
pub struct DHCPOptionsProperties {
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="DomainNameServers")]
    pub domain_name_servers: Vec<String>,
    #[serde(rename="NetbiosNameServers")]
    pub netbios_name_servers: Vec<String>,
    #[serde(rename="NetbiosNodeType")]
    pub netbios_node_type: u32,
    #[serde(rename="NtpServers")]
    pub ntp_servers: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for DHCPOptions {
    type Properties = DHCPOptionsProperties;
    const TYPE: &'static str = "AWS::EC2::DHCPOptions";
    fn properties(&self) -> &DHCPOptionsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DHCPOptionsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DHCPOptions {}

impl From<DHCPOptionsProperties> for DHCPOptions {
    fn from(properties: DHCPOptionsProperties) -> DHCPOptions {
        DHCPOptions { properties }
    }
}

/// The [`AWS::EC2::EIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-eip.html) resource type.
pub struct EIP {
    properties: EIPProperties
}

/// Properties for the `EIP` resource.
#[derive(Serialize, Deserialize)]
pub struct EIPProperties {
    #[serde(rename="Domain")]
    pub domain: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

impl<'a> ::Resource<'a> for EIP {
    type Properties = EIPProperties;
    const TYPE: &'static str = "AWS::EC2::EIP";
    fn properties(&self) -> &EIPProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EIPProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EIP {}

impl From<EIPProperties> for EIP {
    fn from(properties: EIPProperties) -> EIP {
        EIP { properties }
    }
}

/// The [`AWS::EC2::EIPAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-eip-association.html) resource type.
pub struct EIPAssociation {
    properties: EIPAssociationProperties
}

/// Properties for the `EIPAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct EIPAssociationProperties {
    #[serde(rename="AllocationId")]
    pub allocation_id: String,
    #[serde(rename="EIP")]
    pub eip: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="NetworkInterfaceId")]
    pub network_interface_id: String,
    #[serde(rename="PrivateIpAddress")]
    pub private_ip_address: String,
}

impl<'a> ::Resource<'a> for EIPAssociation {
    type Properties = EIPAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::EIPAssociation";
    fn properties(&self) -> &EIPAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EIPAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EIPAssociation {}

impl From<EIPAssociationProperties> for EIPAssociation {
    fn from(properties: EIPAssociationProperties) -> EIPAssociation {
        EIPAssociation { properties }
    }
}

/// The [`AWS::EC2::EgressOnlyInternetGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-egressonlyinternetgateway.html) resource type.
pub struct EgressOnlyInternetGateway {
    properties: EgressOnlyInternetGatewayProperties
}

/// Properties for the `EgressOnlyInternetGateway` resource.
#[derive(Serialize, Deserialize)]
pub struct EgressOnlyInternetGatewayProperties {
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for EgressOnlyInternetGateway {
    type Properties = EgressOnlyInternetGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::EgressOnlyInternetGateway";
    fn properties(&self) -> &EgressOnlyInternetGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EgressOnlyInternetGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EgressOnlyInternetGateway {}

impl From<EgressOnlyInternetGatewayProperties> for EgressOnlyInternetGateway {
    fn from(properties: EgressOnlyInternetGatewayProperties) -> EgressOnlyInternetGateway {
        EgressOnlyInternetGateway { properties }
    }
}

/// The [`AWS::EC2::FlowLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-flowlog.html) resource type.
pub struct FlowLog {
    properties: FlowLogProperties
}

/// Properties for the `FlowLog` resource.
#[derive(Serialize, Deserialize)]
pub struct FlowLogProperties {
    #[serde(rename="DeliverLogsPermissionArn")]
    pub deliver_logs_permission_arn: String,
    #[serde(rename="LogGroupName")]
    pub log_group_name: String,
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[serde(rename="ResourceType")]
    pub resource_type: String,
    #[serde(rename="TrafficType")]
    pub traffic_type: String,
}

impl<'a> ::Resource<'a> for FlowLog {
    type Properties = FlowLogProperties;
    const TYPE: &'static str = "AWS::EC2::FlowLog";
    fn properties(&self) -> &FlowLogProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowLogProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowLog {}

impl From<FlowLogProperties> for FlowLog {
    fn from(properties: FlowLogProperties) -> FlowLog {
        FlowLog { properties }
    }
}

/// The [`AWS::EC2::Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-host.html) resource type.
pub struct Host {
    properties: HostProperties
}

/// Properties for the `Host` resource.
#[derive(Serialize, Deserialize)]
pub struct HostProperties {
    #[serde(rename="AutoPlacement")]
    pub auto_placement: String,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
}

impl<'a> ::Resource<'a> for Host {
    type Properties = HostProperties;
    const TYPE: &'static str = "AWS::EC2::Host";
    fn properties(&self) -> &HostProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HostProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Host {}

impl From<HostProperties> for Host {
    fn from(properties: HostProperties) -> Host {
        Host { properties }
    }
}

/// The [`AWS::EC2::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance.html) resource type.
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceProperties {
    #[serde(rename="AdditionalInfo")]
    pub additional_info: String,
    #[serde(rename="Affinity")]
    pub affinity: String,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: Vec<self::instance::BlockDeviceMapping>,
    #[serde(rename="CreditSpecification")]
    pub credit_specification: self::instance::CreditSpecification,
    #[serde(rename="DisableApiTermination")]
    pub disable_api_termination: bool,
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: bool,
    #[serde(rename="ElasticGpuSpecifications")]
    pub elastic_gpu_specifications: Vec<self::instance::ElasticGpuSpecification>,
    #[serde(rename="HostId")]
    pub host_id: String,
    #[serde(rename="IamInstanceProfile")]
    pub iam_instance_profile: String,
    #[serde(rename="ImageId")]
    pub image_id: String,
    #[serde(rename="InstanceInitiatedShutdownBehavior")]
    pub instance_initiated_shutdown_behavior: String,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[serde(rename="Ipv6AddressCount")]
    pub ipv6_address_count: u32,
    #[serde(rename="Ipv6Addresses")]
    pub ipv6_addresses: Vec<self::instance::InstanceIpv6Address>,
    #[serde(rename="KernelId")]
    pub kernel_id: String,
    #[serde(rename="KeyName")]
    pub key_name: String,
    #[serde(rename="Monitoring")]
    pub monitoring: bool,
    #[serde(rename="NetworkInterfaces")]
    pub network_interfaces: Vec<self::instance::NetworkInterface>,
    #[serde(rename="PlacementGroupName")]
    pub placement_group_name: String,
    #[serde(rename="PrivateIpAddress")]
    pub private_ip_address: String,
    #[serde(rename="RamdiskId")]
    pub ramdisk_id: String,
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="SourceDestCheck")]
    pub source_dest_check: bool,
    #[serde(rename="SsmAssociations")]
    pub ssm_associations: Vec<self::instance::SsmAssociation>,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Tenancy")]
    pub tenancy: String,
    #[serde(rename="UserData")]
    pub user_data: String,
    #[serde(rename="Volumes")]
    pub volumes: Vec<self::instance::Volume>,
}

impl<'a> ::Resource<'a> for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::EC2::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::EC2::InternetGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-internetgateway.html) resource type.
pub struct InternetGateway {
    properties: InternetGatewayProperties
}

/// Properties for the `InternetGateway` resource.
#[derive(Serialize, Deserialize)]
pub struct InternetGatewayProperties {
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for InternetGateway {
    type Properties = InternetGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::InternetGateway";
    fn properties(&self) -> &InternetGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InternetGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InternetGateway {}

impl From<InternetGatewayProperties> for InternetGateway {
    fn from(properties: InternetGatewayProperties) -> InternetGateway {
        InternetGateway { properties }
    }
}

/// The [`AWS::EC2::NatGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-natgateway.html) resource type.
pub struct NatGateway {
    properties: NatGatewayProperties
}

/// Properties for the `NatGateway` resource.
#[derive(Serialize, Deserialize)]
pub struct NatGatewayProperties {
    #[serde(rename="AllocationId")]
    pub allocation_id: String,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for NatGateway {
    type Properties = NatGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::NatGateway";
    fn properties(&self) -> &NatGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NatGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NatGateway {}

impl From<NatGatewayProperties> for NatGateway {
    fn from(properties: NatGatewayProperties) -> NatGateway {
        NatGateway { properties }
    }
}

/// The [`AWS::EC2::NetworkAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-acl.html) resource type.
pub struct NetworkAcl {
    properties: NetworkAclProperties
}

/// Properties for the `NetworkAcl` resource.
#[derive(Serialize, Deserialize)]
pub struct NetworkAclProperties {
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for NetworkAcl {
    type Properties = NetworkAclProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkAcl";
    fn properties(&self) -> &NetworkAclProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkAclProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkAcl {}

impl From<NetworkAclProperties> for NetworkAcl {
    fn from(properties: NetworkAclProperties) -> NetworkAcl {
        NetworkAcl { properties }
    }
}

/// The [`AWS::EC2::NetworkAclEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-acl-entry.html) resource type.
pub struct NetworkAclEntry {
    properties: NetworkAclEntryProperties
}

/// Properties for the `NetworkAclEntry` resource.
#[derive(Serialize, Deserialize)]
pub struct NetworkAclEntryProperties {
    #[serde(rename="CidrBlock")]
    pub cidr_block: String,
    #[serde(rename="Egress")]
    pub egress: bool,
    #[serde(rename="Icmp")]
    pub icmp: self::network_acl_entry::Icmp,
    #[serde(rename="Ipv6CidrBlock")]
    pub ipv6_cidr_block: String,
    #[serde(rename="NetworkAclId")]
    pub network_acl_id: String,
    #[serde(rename="PortRange")]
    pub port_range: self::network_acl_entry::PortRange,
    #[serde(rename="Protocol")]
    pub protocol: u32,
    #[serde(rename="RuleAction")]
    pub rule_action: String,
    #[serde(rename="RuleNumber")]
    pub rule_number: u32,
}

impl<'a> ::Resource<'a> for NetworkAclEntry {
    type Properties = NetworkAclEntryProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkAclEntry";
    fn properties(&self) -> &NetworkAclEntryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkAclEntryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkAclEntry {}

impl From<NetworkAclEntryProperties> for NetworkAclEntry {
    fn from(properties: NetworkAclEntryProperties) -> NetworkAclEntry {
        NetworkAclEntry { properties }
    }
}

/// The [`AWS::EC2::NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-interface.html) resource type.
pub struct NetworkInterface {
    properties: NetworkInterfaceProperties
}

/// Properties for the `NetworkInterface` resource.
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfaceProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="GroupSet")]
    pub group_set: Vec<String>,
    #[serde(rename="InterfaceType")]
    pub interface_type: String,
    #[serde(rename="Ipv6AddressCount")]
    pub ipv6_address_count: u32,
    #[serde(rename="Ipv6Addresses")]
    pub ipv6_addresses: self::network_interface::InstanceIpv6Address,
    #[serde(rename="PrivateIpAddress")]
    pub private_ip_address: String,
    #[serde(rename="PrivateIpAddresses")]
    pub private_ip_addresses: Vec<self::network_interface::PrivateIpAddressSpecification>,
    #[serde(rename="SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: u32,
    #[serde(rename="SourceDestCheck")]
    pub source_dest_check: bool,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for NetworkInterface {
    type Properties = NetworkInterfaceProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterface";
    fn properties(&self) -> &NetworkInterfaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterface {}

impl From<NetworkInterfaceProperties> for NetworkInterface {
    fn from(properties: NetworkInterfaceProperties) -> NetworkInterface {
        NetworkInterface { properties }
    }
}

/// The [`AWS::EC2::NetworkInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-network-interface-attachment.html) resource type.
pub struct NetworkInterfaceAttachment {
    properties: NetworkInterfaceAttachmentProperties
}

/// Properties for the `NetworkInterfaceAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfaceAttachmentProperties {
    #[serde(rename="DeleteOnTermination")]
    pub delete_on_termination: bool,
    #[serde(rename="DeviceIndex")]
    pub device_index: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="NetworkInterfaceId")]
    pub network_interface_id: String,
}

impl<'a> ::Resource<'a> for NetworkInterfaceAttachment {
    type Properties = NetworkInterfaceAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterfaceAttachment";
    fn properties(&self) -> &NetworkInterfaceAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfaceAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterfaceAttachment {}

impl From<NetworkInterfaceAttachmentProperties> for NetworkInterfaceAttachment {
    fn from(properties: NetworkInterfaceAttachmentProperties) -> NetworkInterfaceAttachment {
        NetworkInterfaceAttachment { properties }
    }
}

/// The [`AWS::EC2::NetworkInterfacePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinterfacepermission.html) resource type.
pub struct NetworkInterfacePermission {
    properties: NetworkInterfacePermissionProperties
}

/// Properties for the `NetworkInterfacePermission` resource.
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfacePermissionProperties {
    #[serde(rename="AwsAccountId")]
    pub aws_account_id: String,
    #[serde(rename="NetworkInterfaceId")]
    pub network_interface_id: String,
    #[serde(rename="Permission")]
    pub permission: String,
}

impl<'a> ::Resource<'a> for NetworkInterfacePermission {
    type Properties = NetworkInterfacePermissionProperties;
    const TYPE: &'static str = "AWS::EC2::NetworkInterfacePermission";
    fn properties(&self) -> &NetworkInterfacePermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkInterfacePermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkInterfacePermission {}

impl From<NetworkInterfacePermissionProperties> for NetworkInterfacePermission {
    fn from(properties: NetworkInterfacePermissionProperties) -> NetworkInterfacePermission {
        NetworkInterfacePermission { properties }
    }
}

/// The [`AWS::EC2::PlacementGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-placementgroup.html) resource type.
pub struct PlacementGroup {
    properties: PlacementGroupProperties
}

/// Properties for the `PlacementGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct PlacementGroupProperties {
    #[serde(rename="Strategy")]
    pub strategy: String,
}

impl<'a> ::Resource<'a> for PlacementGroup {
    type Properties = PlacementGroupProperties;
    const TYPE: &'static str = "AWS::EC2::PlacementGroup";
    fn properties(&self) -> &PlacementGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlacementGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PlacementGroup {}

impl From<PlacementGroupProperties> for PlacementGroup {
    fn from(properties: PlacementGroupProperties) -> PlacementGroup {
        PlacementGroup { properties }
    }
}

/// The [`AWS::EC2::Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-route.html) resource type.
pub struct Route {
    properties: RouteProperties
}

/// Properties for the `Route` resource.
#[derive(Serialize, Deserialize)]
pub struct RouteProperties {
    #[serde(rename="DestinationCidrBlock")]
    pub destination_cidr_block: String,
    #[serde(rename="DestinationIpv6CidrBlock")]
    pub destination_ipv6_cidr_block: String,
    #[serde(rename="EgressOnlyInternetGatewayId")]
    pub egress_only_internet_gateway_id: String,
    #[serde(rename="GatewayId")]
    pub gateway_id: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="NatGatewayId")]
    pub nat_gateway_id: String,
    #[serde(rename="NetworkInterfaceId")]
    pub network_interface_id: String,
    #[serde(rename="RouteTableId")]
    pub route_table_id: String,
    #[serde(rename="VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: String,
}

impl<'a> ::Resource<'a> for Route {
    type Properties = RouteProperties;
    const TYPE: &'static str = "AWS::EC2::Route";
    fn properties(&self) -> &RouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Route {}

impl From<RouteProperties> for Route {
    fn from(properties: RouteProperties) -> Route {
        Route { properties }
    }
}

/// The [`AWS::EC2::RouteTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-route-table.html) resource type.
pub struct RouteTable {
    properties: RouteTableProperties
}

/// Properties for the `RouteTable` resource.
#[derive(Serialize, Deserialize)]
pub struct RouteTableProperties {
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for RouteTable {
    type Properties = RouteTableProperties;
    const TYPE: &'static str = "AWS::EC2::RouteTable";
    fn properties(&self) -> &RouteTableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteTableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RouteTable {}

impl From<RouteTableProperties> for RouteTable {
    fn from(properties: RouteTableProperties) -> RouteTable {
        RouteTable { properties }
    }
}

/// The [`AWS::EC2::SecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group.html) resource type.
pub struct SecurityGroup {
    properties: SecurityGroupProperties
}

/// Properties for the `SecurityGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupProperties {
    #[serde(rename="GroupDescription")]
    pub group_description: String,
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="SecurityGroupEgress")]
    pub security_group_egress: Vec<self::security_group::Egress>,
    #[serde(rename="SecurityGroupIngress")]
    pub security_group_ingress: Vec<self::security_group::Ingress>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for SecurityGroup {
    type Properties = SecurityGroupProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroup";
    fn properties(&self) -> &SecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroup {}

impl From<SecurityGroupProperties> for SecurityGroup {
    fn from(properties: SecurityGroupProperties) -> SecurityGroup {
        SecurityGroup { properties }
    }
}

/// The [`AWS::EC2::SecurityGroupEgress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-security-group-egress.html) resource type.
pub struct SecurityGroupEgress {
    properties: SecurityGroupEgressProperties
}

/// Properties for the `SecurityGroupEgress` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupEgressProperties {
    #[serde(rename="CidrIp")]
    pub cidr_ip: String,
    #[serde(rename="CidrIpv6")]
    pub cidr_ipv6: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="DestinationPrefixListId")]
    pub destination_prefix_list_id: String,
    #[serde(rename="DestinationSecurityGroupId")]
    pub destination_security_group_id: String,
    #[serde(rename="FromPort")]
    pub from_port: u32,
    #[serde(rename="GroupId")]
    pub group_id: String,
    #[serde(rename="IpProtocol")]
    pub ip_protocol: String,
    #[serde(rename="ToPort")]
    pub to_port: u32,
}

impl<'a> ::Resource<'a> for SecurityGroupEgress {
    type Properties = SecurityGroupEgressProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroupEgress";
    fn properties(&self) -> &SecurityGroupEgressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupEgressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroupEgress {}

impl From<SecurityGroupEgressProperties> for SecurityGroupEgress {
    fn from(properties: SecurityGroupEgressProperties) -> SecurityGroupEgress {
        SecurityGroupEgress { properties }
    }
}

/// The [`AWS::EC2::SecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-ingress.html) resource type.
pub struct SecurityGroupIngress {
    properties: SecurityGroupIngressProperties
}

/// Properties for the `SecurityGroupIngress` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupIngressProperties {
    #[serde(rename="CidrIp")]
    pub cidr_ip: String,
    #[serde(rename="CidrIpv6")]
    pub cidr_ipv6: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="FromPort")]
    pub from_port: u32,
    #[serde(rename="GroupId")]
    pub group_id: String,
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="IpProtocol")]
    pub ip_protocol: String,
    #[serde(rename="SourceSecurityGroupId")]
    pub source_security_group_id: String,
    #[serde(rename="SourceSecurityGroupName")]
    pub source_security_group_name: String,
    #[serde(rename="SourceSecurityGroupOwnerId")]
    pub source_security_group_owner_id: String,
    #[serde(rename="ToPort")]
    pub to_port: u32,
}

impl<'a> ::Resource<'a> for SecurityGroupIngress {
    type Properties = SecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::EC2::SecurityGroupIngress";
    fn properties(&self) -> &SecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroupIngress {}

impl From<SecurityGroupIngressProperties> for SecurityGroupIngress {
    fn from(properties: SecurityGroupIngressProperties) -> SecurityGroupIngress {
        SecurityGroupIngress { properties }
    }
}

/// The [`AWS::EC2::SpotFleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-spotfleet.html) resource type.
pub struct SpotFleet {
    properties: SpotFleetProperties
}

/// Properties for the `SpotFleet` resource.
#[derive(Serialize, Deserialize)]
pub struct SpotFleetProperties {
    #[serde(rename="SpotFleetRequestConfigData")]
    pub spot_fleet_request_config_data: self::spot_fleet::SpotFleetRequestConfigData,
}

impl<'a> ::Resource<'a> for SpotFleet {
    type Properties = SpotFleetProperties;
    const TYPE: &'static str = "AWS::EC2::SpotFleet";
    fn properties(&self) -> &SpotFleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SpotFleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SpotFleet {}

impl From<SpotFleetProperties> for SpotFleet {
    fn from(properties: SpotFleetProperties) -> SpotFleet {
        SpotFleet { properties }
    }
}

/// The [`AWS::EC2::Subnet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet.html) resource type.
pub struct Subnet {
    properties: SubnetProperties
}

/// Properties for the `Subnet` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetProperties {
    #[serde(rename="AssignIpv6AddressOnCreation")]
    pub assign_ipv6_address_on_creation: bool,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="CidrBlock")]
    pub cidr_block: String,
    #[serde(rename="Ipv6CidrBlock")]
    pub ipv6_cidr_block: String,
    #[serde(rename="MapPublicIpOnLaunch")]
    pub map_public_ip_on_launch: bool,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for Subnet {
    type Properties = SubnetProperties;
    const TYPE: &'static str = "AWS::EC2::Subnet";
    fn properties(&self) -> &SubnetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Subnet {}

impl From<SubnetProperties> for Subnet {
    fn from(properties: SubnetProperties) -> Subnet {
        Subnet { properties }
    }
}

/// The [`AWS::EC2::SubnetCidrBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnetcidrblock.html) resource type.
pub struct SubnetCidrBlock {
    properties: SubnetCidrBlockProperties
}

/// Properties for the `SubnetCidrBlock` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetCidrBlockProperties {
    #[serde(rename="Ipv6CidrBlock")]
    pub ipv6_cidr_block: String,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
}

impl<'a> ::Resource<'a> for SubnetCidrBlock {
    type Properties = SubnetCidrBlockProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetCidrBlock";
    fn properties(&self) -> &SubnetCidrBlockProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetCidrBlockProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetCidrBlock {}

impl From<SubnetCidrBlockProperties> for SubnetCidrBlock {
    fn from(properties: SubnetCidrBlockProperties) -> SubnetCidrBlock {
        SubnetCidrBlock { properties }
    }
}

/// The [`AWS::EC2::SubnetNetworkAclAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet-network-acl-assoc.html) resource type.
pub struct SubnetNetworkAclAssociation {
    properties: SubnetNetworkAclAssociationProperties
}

/// Properties for the `SubnetNetworkAclAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetNetworkAclAssociationProperties {
    #[serde(rename="NetworkAclId")]
    pub network_acl_id: String,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
}

impl<'a> ::Resource<'a> for SubnetNetworkAclAssociation {
    type Properties = SubnetNetworkAclAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetNetworkAclAssociation";
    fn properties(&self) -> &SubnetNetworkAclAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetNetworkAclAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetNetworkAclAssociation {}

impl From<SubnetNetworkAclAssociationProperties> for SubnetNetworkAclAssociation {
    fn from(properties: SubnetNetworkAclAssociationProperties) -> SubnetNetworkAclAssociation {
        SubnetNetworkAclAssociation { properties }
    }
}

/// The [`AWS::EC2::SubnetRouteTableAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet-route-table-assoc.html) resource type.
pub struct SubnetRouteTableAssociation {
    properties: SubnetRouteTableAssociationProperties
}

/// Properties for the `SubnetRouteTableAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetRouteTableAssociationProperties {
    #[serde(rename="RouteTableId")]
    pub route_table_id: String,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
}

impl<'a> ::Resource<'a> for SubnetRouteTableAssociation {
    type Properties = SubnetRouteTableAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::SubnetRouteTableAssociation";
    fn properties(&self) -> &SubnetRouteTableAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetRouteTableAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetRouteTableAssociation {}

impl From<SubnetRouteTableAssociationProperties> for SubnetRouteTableAssociation {
    fn from(properties: SubnetRouteTableAssociationProperties) -> SubnetRouteTableAssociation {
        SubnetRouteTableAssociation { properties }
    }
}

/// The [`AWS::EC2::TrunkInterfaceAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trunkinterfaceassociation.html) resource type.
pub struct TrunkInterfaceAssociation {
    properties: TrunkInterfaceAssociationProperties
}

/// Properties for the `TrunkInterfaceAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct TrunkInterfaceAssociationProperties {
    #[serde(rename="BranchInterfaceId")]
    pub branch_interface_id: String,
    #[serde(rename="GREKey")]
    pub gre_key: u32,
    #[serde(rename="TrunkInterfaceId")]
    pub trunk_interface_id: String,
    #[serde(rename="VLANId")]
    pub vlan_id: u32,
}

impl<'a> ::Resource<'a> for TrunkInterfaceAssociation {
    type Properties = TrunkInterfaceAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::TrunkInterfaceAssociation";
    fn properties(&self) -> &TrunkInterfaceAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrunkInterfaceAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrunkInterfaceAssociation {}

impl From<TrunkInterfaceAssociationProperties> for TrunkInterfaceAssociation {
    fn from(properties: TrunkInterfaceAssociationProperties) -> TrunkInterfaceAssociation {
        TrunkInterfaceAssociation { properties }
    }
}

/// The [`AWS::EC2::VPC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc.html) resource type.
pub struct VPC {
    properties: VPCProperties
}

/// Properties for the `VPC` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCProperties {
    #[serde(rename="CidrBlock")]
    pub cidr_block: String,
    #[serde(rename="EnableDnsHostnames")]
    pub enable_dns_hostnames: bool,
    #[serde(rename="EnableDnsSupport")]
    pub enable_dns_support: bool,
    #[serde(rename="InstanceTenancy")]
    pub instance_tenancy: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for VPC {
    type Properties = VPCProperties;
    const TYPE: &'static str = "AWS::EC2::VPC";
    fn properties(&self) -> &VPCProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPC {}

impl From<VPCProperties> for VPC {
    fn from(properties: VPCProperties) -> VPC {
        VPC { properties }
    }
}

/// The [`AWS::EC2::VPCCidrBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpccidrblock.html) resource type.
pub struct VPCCidrBlock {
    properties: VPCCidrBlockProperties
}

/// Properties for the `VPCCidrBlock` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCCidrBlockProperties {
    #[serde(rename="AmazonProvidedIpv6CidrBlock")]
    pub amazon_provided_ipv6_cidr_block: bool,
    #[serde(rename="CidrBlock")]
    pub cidr_block: String,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for VPCCidrBlock {
    type Properties = VPCCidrBlockProperties;
    const TYPE: &'static str = "AWS::EC2::VPCCidrBlock";
    fn properties(&self) -> &VPCCidrBlockProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCCidrBlockProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCCidrBlock {}

impl From<VPCCidrBlockProperties> for VPCCidrBlock {
    fn from(properties: VPCCidrBlockProperties) -> VPCCidrBlock {
        VPCCidrBlock { properties }
    }
}

/// The [`AWS::EC2::VPCDHCPOptionsAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc-dhcp-options-assoc.html) resource type.
pub struct VPCDHCPOptionsAssociation {
    properties: VPCDHCPOptionsAssociationProperties
}

/// Properties for the `VPCDHCPOptionsAssociation` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCDHCPOptionsAssociationProperties {
    #[serde(rename="DhcpOptionsId")]
    pub dhcp_options_id: String,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for VPCDHCPOptionsAssociation {
    type Properties = VPCDHCPOptionsAssociationProperties;
    const TYPE: &'static str = "AWS::EC2::VPCDHCPOptionsAssociation";
    fn properties(&self) -> &VPCDHCPOptionsAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCDHCPOptionsAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCDHCPOptionsAssociation {}

impl From<VPCDHCPOptionsAssociationProperties> for VPCDHCPOptionsAssociation {
    fn from(properties: VPCDHCPOptionsAssociationProperties) -> VPCDHCPOptionsAssociation {
        VPCDHCPOptionsAssociation { properties }
    }
}

/// The [`AWS::EC2::VPCEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpoint.html) resource type.
pub struct VPCEndpoint {
    properties: VPCEndpointProperties
}

/// Properties for the `VPCEndpoint` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCEndpointProperties {
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    #[serde(rename="RouteTableIds")]
    pub route_table_ids: Vec<String>,
    #[serde(rename="ServiceName")]
    pub service_name: String,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for VPCEndpoint {
    type Properties = VPCEndpointProperties;
    const TYPE: &'static str = "AWS::EC2::VPCEndpoint";
    fn properties(&self) -> &VPCEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCEndpoint {}

impl From<VPCEndpointProperties> for VPCEndpoint {
    fn from(properties: VPCEndpointProperties) -> VPCEndpoint {
        VPCEndpoint { properties }
    }
}

/// The [`AWS::EC2::VPCGatewayAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc-gateway-attachment.html) resource type.
pub struct VPCGatewayAttachment {
    properties: VPCGatewayAttachmentProperties
}

/// Properties for the `VPCGatewayAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCGatewayAttachmentProperties {
    #[serde(rename="InternetGatewayId")]
    pub internet_gateway_id: String,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
    #[serde(rename="VpnGatewayId")]
    pub vpn_gateway_id: String,
}

impl<'a> ::Resource<'a> for VPCGatewayAttachment {
    type Properties = VPCGatewayAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::VPCGatewayAttachment";
    fn properties(&self) -> &VPCGatewayAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCGatewayAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCGatewayAttachment {}

impl From<VPCGatewayAttachmentProperties> for VPCGatewayAttachment {
    fn from(properties: VPCGatewayAttachmentProperties) -> VPCGatewayAttachment {
        VPCGatewayAttachment { properties }
    }
}

/// The [`AWS::EC2::VPCPeeringConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcpeeringconnection.html) resource type.
pub struct VPCPeeringConnection {
    properties: VPCPeeringConnectionProperties
}

/// Properties for the `VPCPeeringConnection` resource.
#[derive(Serialize, Deserialize)]
pub struct VPCPeeringConnectionProperties {
    #[serde(rename="PeerOwnerId")]
    pub peer_owner_id: String,
    #[serde(rename="PeerRoleArn")]
    pub peer_role_arn: String,
    #[serde(rename="PeerVpcId")]
    pub peer_vpc_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for VPCPeeringConnection {
    type Properties = VPCPeeringConnectionProperties;
    const TYPE: &'static str = "AWS::EC2::VPCPeeringConnection";
    fn properties(&self) -> &VPCPeeringConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPCPeeringConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPCPeeringConnection {}

impl From<VPCPeeringConnectionProperties> for VPCPeeringConnection {
    fn from(properties: VPCPeeringConnectionProperties) -> VPCPeeringConnection {
        VPCPeeringConnection { properties }
    }
}

/// The [`AWS::EC2::VPNConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-connection.html) resource type.
pub struct VPNConnection {
    properties: VPNConnectionProperties
}

/// Properties for the `VPNConnection` resource.
#[derive(Serialize, Deserialize)]
pub struct VPNConnectionProperties {
    #[serde(rename="CustomerGatewayId")]
    pub customer_gateway_id: String,
    #[serde(rename="StaticRoutesOnly")]
    pub static_routes_only: bool,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Type")]
    pub type_: String,
    #[serde(rename="VpnGatewayId")]
    pub vpn_gateway_id: String,
    #[serde(rename="VpnTunnelOptionsSpecifications")]
    pub vpn_tunnel_options_specifications: Vec<self::vpn_connection::VpnTunnelOptionsSpecification>,
}

impl<'a> ::Resource<'a> for VPNConnection {
    type Properties = VPNConnectionProperties;
    const TYPE: &'static str = "AWS::EC2::VPNConnection";
    fn properties(&self) -> &VPNConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNConnection {}

impl From<VPNConnectionProperties> for VPNConnection {
    fn from(properties: VPNConnectionProperties) -> VPNConnection {
        VPNConnection { properties }
    }
}

/// The [`AWS::EC2::VPNConnectionRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-connection-route.html) resource type.
pub struct VPNConnectionRoute {
    properties: VPNConnectionRouteProperties
}

/// Properties for the `VPNConnectionRoute` resource.
#[derive(Serialize, Deserialize)]
pub struct VPNConnectionRouteProperties {
    #[serde(rename="DestinationCidrBlock")]
    pub destination_cidr_block: String,
    #[serde(rename="VpnConnectionId")]
    pub vpn_connection_id: String,
}

impl<'a> ::Resource<'a> for VPNConnectionRoute {
    type Properties = VPNConnectionRouteProperties;
    const TYPE: &'static str = "AWS::EC2::VPNConnectionRoute";
    fn properties(&self) -> &VPNConnectionRouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNConnectionRouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNConnectionRoute {}

impl From<VPNConnectionRouteProperties> for VPNConnectionRoute {
    fn from(properties: VPNConnectionRouteProperties) -> VPNConnectionRoute {
        VPNConnectionRoute { properties }
    }
}

/// The [`AWS::EC2::VPNGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-gateway.html) resource type.
pub struct VPNGateway {
    properties: VPNGatewayProperties
}

/// Properties for the `VPNGateway` resource.
#[derive(Serialize, Deserialize)]
pub struct VPNGatewayProperties {
    #[serde(rename="AmazonSideAsn")]
    pub amazon_side_asn: u64,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for VPNGateway {
    type Properties = VPNGatewayProperties;
    const TYPE: &'static str = "AWS::EC2::VPNGateway";
    fn properties(&self) -> &VPNGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNGateway {}

impl From<VPNGatewayProperties> for VPNGateway {
    fn from(properties: VPNGatewayProperties) -> VPNGateway {
        VPNGateway { properties }
    }
}

/// The [`AWS::EC2::VPNGatewayRoutePropagation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpn-gatewayrouteprop.html) resource type.
pub struct VPNGatewayRoutePropagation {
    properties: VPNGatewayRoutePropagationProperties
}

/// Properties for the `VPNGatewayRoutePropagation` resource.
#[derive(Serialize, Deserialize)]
pub struct VPNGatewayRoutePropagationProperties {
    #[serde(rename="RouteTableIds")]
    pub route_table_ids: Vec<String>,
    #[serde(rename="VpnGatewayId")]
    pub vpn_gateway_id: String,
}

impl<'a> ::Resource<'a> for VPNGatewayRoutePropagation {
    type Properties = VPNGatewayRoutePropagationProperties;
    const TYPE: &'static str = "AWS::EC2::VPNGatewayRoutePropagation";
    fn properties(&self) -> &VPNGatewayRoutePropagationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VPNGatewayRoutePropagationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VPNGatewayRoutePropagation {}

impl From<VPNGatewayRoutePropagationProperties> for VPNGatewayRoutePropagation {
    fn from(properties: VPNGatewayRoutePropagationProperties) -> VPNGatewayRoutePropagation {
        VPNGatewayRoutePropagation { properties }
    }
}

/// The [`AWS::EC2::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ebs-volume.html) resource type.
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename="AutoEnableIO")]
    pub auto_enable_io: bool,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="Encrypted")]
    pub encrypted: bool,
    #[serde(rename="Iops")]
    pub iops: u32,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="Size")]
    pub size: u32,
    #[serde(rename="SnapshotId")]
    pub snapshot_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VolumeType")]
    pub volume_type: String,
}

impl<'a> ::Resource<'a> for Volume {
    type Properties = VolumeProperties;
    const TYPE: &'static str = "AWS::EC2::Volume";
    fn properties(&self) -> &VolumeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Volume {}

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

/// The [`AWS::EC2::VolumeAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ebs-volumeattachment.html) resource type.
pub struct VolumeAttachment {
    properties: VolumeAttachmentProperties
}

/// Properties for the `VolumeAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct VolumeAttachmentProperties {
    #[serde(rename="Device")]
    pub device: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="VolumeId")]
    pub volume_id: String,
}

impl<'a> ::Resource<'a> for VolumeAttachment {
    type Properties = VolumeAttachmentProperties;
    const TYPE: &'static str = "AWS::EC2::VolumeAttachment";
    fn properties(&self) -> &VolumeAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VolumeAttachment {}

impl From<VolumeAttachmentProperties> for VolumeAttachment {
    fn from(properties: VolumeAttachmentProperties) -> VolumeAttachment {
        VolumeAttachment { properties }
    }
}

pub mod instance {
    /// The [`AWS::EC2::Instance.AssociationParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociations-associationparameters.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AssociationParameter {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: Vec<String>,
    }

    /// The [`AWS::EC2::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-mapping.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        #[serde(rename="DeviceName")]
        pub device_name: String,
        #[serde(rename="Ebs")]
        pub ebs: Ebs,
        #[serde(rename="NoDevice")]
        pub no_device: NoDevice,
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
    }

    /// The [`AWS::EC2::Instance.CreditSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-creditspecification.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct CreditSpecification {
        #[serde(rename="CPUCredits")]
        pub cpu_credits: String,
    }

    /// The [`AWS::EC2::Instance.Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-blockdev-template.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Ebs {
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Encrypted")]
        pub encrypted: bool,
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    /// The [`AWS::EC2::Instance.ElasticGpuSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-elasticgpuspecification.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ElasticGpuSpecification {
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::EC2::Instance.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-instanceipv6address.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        #[serde(rename="Ipv6Address")]
        pub ipv6_address: String,
    }

    /// The [`AWS::EC2::Instance.NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-iface-embedded.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct NetworkInterface {
        #[serde(rename="AssociatePublicIpAddress")]
        pub associate_public_ip_address: bool,
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="DeviceIndex")]
        pub device_index: String,
        #[serde(rename="GroupSet")]
        pub group_set: Vec<String>,
        #[serde(rename="Ipv6AddressCount")]
        pub ipv6_address_count: u32,
        #[serde(rename="Ipv6Addresses")]
        pub ipv6_addresses: Vec<InstanceIpv6Address>,
        #[serde(rename="NetworkInterfaceId")]
        pub network_interface_id: String,
        #[serde(rename="PrivateIpAddress")]
        pub private_ip_address: String,
        #[serde(rename="PrivateIpAddresses")]
        pub private_ip_addresses: Vec<PrivateIpAddressSpecification>,
        #[serde(rename="SecondaryPrivateIpAddressCount")]
        pub secondary_private_ip_address_count: u32,
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }

    /// The [`AWS::EC2::Instance.NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-nodevice.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct NoDevice {
    }

    /// The [`AWS::EC2::Instance.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        #[serde(rename="Primary")]
        pub primary: bool,
        #[serde(rename="PrivateIpAddress")]
        pub private_ip_address: String,
    }

    /// The [`AWS::EC2::Instance.SsmAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociations.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SsmAssociation {
        #[serde(rename="AssociationParameters")]
        pub association_parameters: Vec<AssociationParameter>,
        #[serde(rename="DocumentName")]
        pub document_name: String,
    }

    /// The [`AWS::EC2::Instance.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-mount-point.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Volume {
        #[serde(rename="Device")]
        pub device: String,
        #[serde(rename="VolumeId")]
        pub volume_id: String,
    }

}

pub mod network_acl_entry {
    /// The [`AWS::EC2::NetworkAclEntry.Icmp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-icmp.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Icmp {
        #[serde(rename="Code")]
        pub code: u32,
        #[serde(rename="Type")]
        pub type_: u32,
    }

    /// The [`AWS::EC2::NetworkAclEntry.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-portrange.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PortRange {
        #[serde(rename="From")]
        pub from: u32,
        #[serde(rename="To")]
        pub to: u32,
    }

}

pub mod network_interface {
    /// The [`AWS::EC2::NetworkInterface.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-instanceipv6address.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        #[serde(rename="Ipv6Address")]
        pub ipv6_address: String,
    }

    /// The [`AWS::EC2::NetworkInterface.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-network-interface-privateipspec.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        #[serde(rename="Primary")]
        pub primary: bool,
        #[serde(rename="PrivateIpAddress")]
        pub private_ip_address: String,
    }

}

pub mod security_group {
    /// The [`AWS::EC2::SecurityGroup.Egress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Egress {
        #[serde(rename="CidrIp")]
        pub cidr_ip: String,
        #[serde(rename="CidrIpv6")]
        pub cidr_ipv6: String,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="DestinationPrefixListId")]
        pub destination_prefix_list_id: String,
        #[serde(rename="DestinationSecurityGroupId")]
        pub destination_security_group_id: String,
        #[serde(rename="FromPort")]
        pub from_port: u32,
        #[serde(rename="IpProtocol")]
        pub ip_protocol: String,
        #[serde(rename="ToPort")]
        pub to_port: u32,
    }

    /// The [`AWS::EC2::SecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Ingress {
        #[serde(rename="CidrIp")]
        pub cidr_ip: String,
        #[serde(rename="CidrIpv6")]
        pub cidr_ipv6: String,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="FromPort")]
        pub from_port: u32,
        #[serde(rename="IpProtocol")]
        pub ip_protocol: String,
        #[serde(rename="SourceSecurityGroupId")]
        pub source_security_group_id: String,
        #[serde(rename="SourceSecurityGroupName")]
        pub source_security_group_name: String,
        #[serde(rename="SourceSecurityGroupOwnerId")]
        pub source_security_group_owner_id: String,
        #[serde(rename="ToPort")]
        pub to_port: u32,
    }

}

pub mod spot_fleet {
    /// The [`AWS::EC2::SpotFleet.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        #[serde(rename="DeviceName")]
        pub device_name: String,
        #[serde(rename="Ebs")]
        pub ebs: EbsBlockDevice,
        #[serde(rename="NoDevice")]
        pub no_device: String,
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
    }

    /// The [`AWS::EC2::SpotFleet.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-blockdevicemappings-ebs.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Encrypted")]
        pub encrypted: bool,
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    /// The [`AWS::EC2::SpotFleet.GroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-securitygroups.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct GroupIdentifier {
        #[serde(rename="GroupId")]
        pub group_id: String,
    }

    /// The [`AWS::EC2::SpotFleet.IamInstanceProfileSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-iaminstanceprofile.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct IamInstanceProfileSpecification {
        #[serde(rename="Arn")]
        pub arn: String,
    }

    /// The [`AWS::EC2::SpotFleet.InstanceIpv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instanceipv6address.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct InstanceIpv6Address {
        #[serde(rename="Ipv6Address")]
        pub ipv6_address: String,
    }

    /// The [`AWS::EC2::SpotFleet.InstanceNetworkInterfaceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct InstanceNetworkInterfaceSpecification {
        #[serde(rename="AssociatePublicIpAddress")]
        pub associate_public_ip_address: bool,
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="DeviceIndex")]
        pub device_index: u32,
        #[serde(rename="Groups")]
        pub groups: Vec<String>,
        #[serde(rename="Ipv6AddressCount")]
        pub ipv6_address_count: u32,
        #[serde(rename="Ipv6Addresses")]
        pub ipv6_addresses: Vec<InstanceIpv6Address>,
        #[serde(rename="NetworkInterfaceId")]
        pub network_interface_id: String,
        #[serde(rename="PrivateIpAddresses")]
        pub private_ip_addresses: Vec<PrivateIpAddressSpecification>,
        #[serde(rename="SecondaryPrivateIpAddressCount")]
        pub secondary_private_ip_address_count: u32,
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }

    /// The [`AWS::EC2::SpotFleet.PrivateIpAddressSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-networkinterfaces-privateipaddresses.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PrivateIpAddressSpecification {
        #[serde(rename="Primary")]
        pub primary: bool,
        #[serde(rename="PrivateIpAddress")]
        pub private_ip_address: String,
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetLaunchSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SpotFleetLaunchSpecification {
        #[serde(rename="BlockDeviceMappings")]
        pub block_device_mappings: Vec<BlockDeviceMapping>,
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
        #[serde(rename="IamInstanceProfile")]
        pub iam_instance_profile: IamInstanceProfileSpecification,
        #[serde(rename="ImageId")]
        pub image_id: String,
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        #[serde(rename="KernelId")]
        pub kernel_id: String,
        #[serde(rename="KeyName")]
        pub key_name: String,
        #[serde(rename="Monitoring")]
        pub monitoring: SpotFleetMonitoring,
        #[serde(rename="NetworkInterfaces")]
        pub network_interfaces: Vec<InstanceNetworkInterfaceSpecification>,
        #[serde(rename="Placement")]
        pub placement: SpotPlacement,
        #[serde(rename="RamdiskId")]
        pub ramdisk_id: String,
        #[serde(rename="SecurityGroups")]
        pub security_groups: Vec<GroupIdentifier>,
        #[serde(rename="SpotPrice")]
        pub spot_price: String,
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
        #[serde(rename="TagSpecifications")]
        pub tag_specifications: Vec<SpotFleetTagSpecification>,
        #[serde(rename="UserData")]
        pub user_data: String,
        #[serde(rename="WeightedCapacity")]
        pub weighted_capacity: f64,
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-monitoring.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SpotFleetMonitoring {
        #[serde(rename="Enabled")]
        pub enabled: bool,
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetRequestConfigData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SpotFleetRequestConfigData {
        #[serde(rename="AllocationStrategy")]
        pub allocation_strategy: String,
        #[serde(rename="ExcessCapacityTerminationPolicy")]
        pub excess_capacity_termination_policy: String,
        #[serde(rename="IamFleetRole")]
        pub iam_fleet_role: String,
        #[serde(rename="LaunchSpecifications")]
        pub launch_specifications: Vec<SpotFleetLaunchSpecification>,
        #[serde(rename="ReplaceUnhealthyInstances")]
        pub replace_unhealthy_instances: bool,
        #[serde(rename="SpotPrice")]
        pub spot_price: String,
        #[serde(rename="TargetCapacity")]
        pub target_capacity: u32,
        #[serde(rename="TerminateInstancesWithExpiration")]
        pub terminate_instances_with_expiration: bool,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="ValidFrom")]
        pub valid_from: String,
        #[serde(rename="ValidUntil")]
        pub valid_until: String,
    }

    /// The [`AWS::EC2::SpotFleet.SpotFleetTagSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-tagspecifications.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SpotFleetTagSpecification {
        #[serde(rename="ResourceType")]
        pub resource_type: String,
    }

    /// The [`AWS::EC2::SpotFleet.SpotPlacement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata-launchspecifications-placement.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SpotPlacement {
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
        #[serde(rename="GroupName")]
        pub group_name: String,
    }

}

pub mod vpn_connection {
    /// The [`AWS::EC2::VPNConnection.VpnTunnelOptionsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-vpntunneloptionsspecification.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct VpnTunnelOptionsSpecification {
        #[serde(rename="PreSharedKey")]
        pub pre_shared_key: String,
        #[serde(rename="TunnelInsideCidr")]
        pub tunnel_inside_cidr: String,
    }

}

