//! Types for the `CloudWatch` service.

/// The [`AWS::CloudWatch::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html) resource type.
#[derive(Debug, Default)]
pub struct Alarm {
    properties: AlarmProperties
}

/// Properties for the `Alarm` resource.
#[derive(Debug, Default)]
pub struct AlarmProperties {
    /// Property [`ActionsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-actionsenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions_enabled: Option<::Value<bool>>,
    /// Property [`AlarmActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_actions: Option<::ValueList<String>>,
    /// Property [`AlarmDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alarm_description: Option<::Value<String>>,
    /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-alarmname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alarm_name: Option<::Value<String>>,
    /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-comparisonoperator).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub comparison_operator: ::Value<String>,
    /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-dimension).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dimensions: Option<::ValueList<self::alarm::Dimension>>,
    /// Property [`EvaluateLowSampleCountPercentile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-evaluatelowsamplecountpercentile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluate_low_sample_count_percentile: Option<::Value<String>>,
    /// Property [`EvaluationPeriods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-evaluationperiods).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluation_periods: ::Value<u32>,
    /// Property [`ExtendedStatistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-extendedstatistic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extended_statistic: Option<::Value<String>>,
    /// Property [`InsufficientDataActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-insufficientdataactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insufficient_data_actions: Option<::ValueList<String>>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-metricname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_name: ::Value<String>,
    /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-namespace).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub namespace: ::Value<String>,
    /// Property [`OKActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-okactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ok_actions: Option<::ValueList<String>>,
    /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-period).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub period: ::Value<u32>,
    /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-statistic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub statistic: Option<::Value<String>>,
    /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-threshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold: ::Value<f64>,
    /// Property [`TreatMissingData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-treatmissingdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treat_missing_data: Option<::Value<String>>,
    /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html#cfn-cloudwatch-alarms-unit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub unit: Option<::Value<String>>,
}

impl ::serde::Serialize for AlarmProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions_enabled) = self.actions_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionsEnabled", actions_enabled)?;
        }
        if let Some(ref alarm_actions) = self.alarm_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmActions", alarm_actions)?;
        }
        if let Some(ref alarm_description) = self.alarm_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmDescription", alarm_description)?;
        }
        if let Some(ref alarm_name) = self.alarm_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", alarm_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
        if let Some(ref dimensions) = self.dimensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
        }
        if let Some(ref evaluate_low_sample_count_percentile) = self.evaluate_low_sample_count_percentile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateLowSampleCountPercentile", evaluate_low_sample_count_percentile)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
        if let Some(ref extended_statistic) = self.extended_statistic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedStatistic", extended_statistic)?;
        }
        if let Some(ref insufficient_data_actions) = self.insufficient_data_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataActions", insufficient_data_actions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
        if let Some(ref ok_actions) = self.ok_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OKActions", ok_actions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
        if let Some(ref statistic) = self.statistic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", statistic)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
        if let Some(ref treat_missing_data) = self.treat_missing_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatMissingData", treat_missing_data)?;
        }
        if let Some(ref unit) = self.unit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AlarmProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AlarmProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AlarmProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions_enabled: Option<::Value<bool>> = None;
                let mut alarm_actions: Option<::ValueList<String>> = None;
                let mut alarm_description: Option<::Value<String>> = None;
                let mut alarm_name: Option<::Value<String>> = None;
                let mut comparison_operator: Option<::Value<String>> = None;
                let mut dimensions: Option<::ValueList<self::alarm::Dimension>> = None;
                let mut evaluate_low_sample_count_percentile: Option<::Value<String>> = None;
                let mut evaluation_periods: Option<::Value<u32>> = None;
                let mut extended_statistic: Option<::Value<String>> = None;
                let mut insufficient_data_actions: Option<::ValueList<String>> = None;
                let mut metric_name: Option<::Value<String>> = None;
                let mut namespace: Option<::Value<String>> = None;
                let mut ok_actions: Option<::ValueList<String>> = None;
                let mut period: Option<::Value<u32>> = None;
                let mut statistic: Option<::Value<String>> = None;
                let mut threshold: Option<::Value<f64>> = None;
                let mut treat_missing_data: Option<::Value<String>> = None;
                let mut unit: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionsEnabled" => {
                            actions_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmActions" => {
                            alarm_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmDescription" => {
                            alarm_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlarmName" => {
                            alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComparisonOperator" => {
                            comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Dimensions" => {
                            dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluateLowSampleCountPercentile" => {
                            evaluate_low_sample_count_percentile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluationPeriods" => {
                            evaluation_periods = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtendedStatistic" => {
                            extended_statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsufficientDataActions" => {
                            insufficient_data_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Namespace" => {
                            namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OKActions" => {
                            ok_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Period" => {
                            period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Statistic" => {
                            statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Threshold" => {
                            threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TreatMissingData" => {
                            treat_missing_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Unit" => {
                            unit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AlarmProperties {
                    actions_enabled: actions_enabled,
                    alarm_actions: alarm_actions,
                    alarm_description: alarm_description,
                    alarm_name: alarm_name,
                    comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                    dimensions: dimensions,
                    evaluate_low_sample_count_percentile: evaluate_low_sample_count_percentile,
                    evaluation_periods: evaluation_periods.ok_or(::serde::de::Error::missing_field("EvaluationPeriods"))?,
                    extended_statistic: extended_statistic,
                    insufficient_data_actions: insufficient_data_actions,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    ok_actions: ok_actions,
                    period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                    statistic: statistic,
                    threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                    treat_missing_data: treat_missing_data,
                    unit: unit,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Alarm {
    type Properties = AlarmProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Alarm";
    fn properties(&self) -> &AlarmProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AlarmProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alarm {}

impl From<AlarmProperties> for Alarm {
    fn from(properties: AlarmProperties) -> Alarm {
        Alarm { properties }
    }
}

/// The [`AWS::CloudWatch::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html) resource type.
#[derive(Debug, Default)]
pub struct Dashboard {
    properties: DashboardProperties
}

/// Properties for the `Dashboard` resource.
#[derive(Debug, Default)]
pub struct DashboardProperties {
    /// Property [`DashboardBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html#cfn-cloudwatch-dashboard-dashboardbody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_body: ::Value<String>,
    /// Property [`DashboardName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html#cfn-cloudwatch-dashboard-dashboardname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dashboard_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DashboardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardBody", &self.dashboard_body)?;
        if let Some(ref dashboard_name) = self.dashboard_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardName", dashboard_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DashboardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DashboardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DashboardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dashboard_body: Option<::Value<String>> = None;
                let mut dashboard_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DashboardBody" => {
                            dashboard_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardName" => {
                            dashboard_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DashboardProperties {
                    dashboard_body: dashboard_body.ok_or(::serde::de::Error::missing_field("DashboardBody"))?,
                    dashboard_name: dashboard_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dashboard {
    type Properties = DashboardProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Dashboard";
    fn properties(&self) -> &DashboardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DashboardProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dashboard {}

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties }
    }
}

pub mod alarm {
    //! Property types for the `Alarm` resource.

    /// The [`AWS::CloudWatch::Alarm.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html) property type.
    #[derive(Debug, Default)]
    pub struct Dimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html#cfn-cloudwatch-alarm-dimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html#cfn-cloudwatch-alarm-dimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Dimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
