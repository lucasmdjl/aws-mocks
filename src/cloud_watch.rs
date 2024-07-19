/*
 * aws_mock - A mocking library for AWS.
 *
 * Copyright (C) 2024 Lucas M. de Jong Larrarte
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use aws_sdk_cloudwatch::operation::delete_alarms::{builders::*, *};
use aws_sdk_cloudwatch::operation::delete_anomaly_detector::{builders::*, *};
use aws_sdk_cloudwatch::operation::delete_dashboards::{builders::*, *};
use aws_sdk_cloudwatch::operation::delete_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::delete_metric_stream::{builders::*, *};
use aws_sdk_cloudwatch::operation::describe_alarm_history::{builders::*, *};
use aws_sdk_cloudwatch::operation::describe_alarms::{builders::*, *};
use aws_sdk_cloudwatch::operation::describe_alarms_for_metric::{builders::*, *};
use aws_sdk_cloudwatch::operation::describe_anomaly_detectors::{builders::*, *};
use aws_sdk_cloudwatch::operation::describe_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::disable_alarm_actions::{builders::*, *};
use aws_sdk_cloudwatch::operation::disable_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::enable_alarm_actions::{builders::*, *};
use aws_sdk_cloudwatch::operation::enable_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_dashboard::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_insight_rule_report::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_metric_data::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_metric_statistics::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_metric_stream::{builders::*, *};
use aws_sdk_cloudwatch::operation::get_metric_widget_image::{builders::*, *};
use aws_sdk_cloudwatch::operation::list_dashboards::{builders::*, *};
use aws_sdk_cloudwatch::operation::list_managed_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::list_metric_streams::{builders::*, *};
use aws_sdk_cloudwatch::operation::list_metrics::{builders::*, *};
use aws_sdk_cloudwatch::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_anomaly_detector::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_composite_alarm::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_dashboard::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_insight_rule::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_managed_insight_rules::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_metric_alarm::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_metric_data::{builders::*, *};
use aws_sdk_cloudwatch::operation::put_metric_stream::{builders::*, *};
use aws_sdk_cloudwatch::operation::set_alarm_state::{builders::*, *};
use aws_sdk_cloudwatch::operation::start_metric_streams::{builders::*, *};
use aws_sdk_cloudwatch::operation::stop_metric_streams::{builders::*, *};
use aws_sdk_cloudwatch::operation::tag_resource::{builders::*, *};
use aws_sdk_cloudwatch::operation::untag_resource::{builders::*, *};
use aws_sdk_cloudwatch::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_cloudwatch::Client;

pub use aws_sdk_cloudwatch::*;

pub struct CloudWatchClientImpl(Client);
impl CloudWatchClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CloudWatchClient {
    fn delete_alarms(&self, builder: DeleteAlarmsInputBuilder) -> impl Future<Output = Result<DeleteAlarmsOutput, SdkError<DeleteAlarmsError>>>;
    fn delete_anomaly_detector(&self, builder: DeleteAnomalyDetectorInputBuilder) -> impl Future<Output = Result<DeleteAnomalyDetectorOutput, SdkError<DeleteAnomalyDetectorError>>>;
    fn delete_dashboards(&self, builder: DeleteDashboardsInputBuilder) -> impl Future<Output = Result<DeleteDashboardsOutput, SdkError<DeleteDashboardsError>>>;
    fn delete_insight_rules(&self, builder: DeleteInsightRulesInputBuilder) -> impl Future<Output = Result<DeleteInsightRulesOutput, SdkError<DeleteInsightRulesError>>>;
    fn delete_metric_stream(&self, builder: DeleteMetricStreamInputBuilder) -> impl Future<Output = Result<DeleteMetricStreamOutput, SdkError<DeleteMetricStreamError>>>;
    fn describe_alarm_history(&self, builder: DescribeAlarmHistoryInputBuilder) -> impl Future<Output = Result<DescribeAlarmHistoryOutput, SdkError<DescribeAlarmHistoryError>>>;
    fn describe_alarms(&self, builder: DescribeAlarmsInputBuilder) -> impl Future<Output = Result<DescribeAlarmsOutput, SdkError<DescribeAlarmsError>>>;
    fn describe_alarms_for_metric(&self, builder: DescribeAlarmsForMetricInputBuilder) -> impl Future<Output = Result<DescribeAlarmsForMetricOutput, SdkError<DescribeAlarmsForMetricError>>>;
    fn describe_anomaly_detectors(&self, builder: DescribeAnomalyDetectorsInputBuilder) -> impl Future<Output = Result<DescribeAnomalyDetectorsOutput, SdkError<DescribeAnomalyDetectorsError>>>;
    fn describe_insight_rules(&self, builder: DescribeInsightRulesInputBuilder) -> impl Future<Output = Result<DescribeInsightRulesOutput, SdkError<DescribeInsightRulesError>>>;
    fn disable_alarm_actions(&self, builder: DisableAlarmActionsInputBuilder) -> impl Future<Output = Result<DisableAlarmActionsOutput, SdkError<DisableAlarmActionsError>>>;
    fn disable_insight_rules(&self, builder: DisableInsightRulesInputBuilder) -> impl Future<Output = Result<DisableInsightRulesOutput, SdkError<DisableInsightRulesError>>>;
    fn enable_alarm_actions(&self, builder: EnableAlarmActionsInputBuilder) -> impl Future<Output = Result<EnableAlarmActionsOutput, SdkError<EnableAlarmActionsError>>>;
    fn enable_insight_rules(&self, builder: EnableInsightRulesInputBuilder) -> impl Future<Output = Result<EnableInsightRulesOutput, SdkError<EnableInsightRulesError>>>;
    fn get_dashboard(&self, builder: GetDashboardInputBuilder) -> impl Future<Output = Result<GetDashboardOutput, SdkError<GetDashboardError>>>;
    fn get_insight_rule_report(&self, builder: GetInsightRuleReportInputBuilder) -> impl Future<Output = Result<GetInsightRuleReportOutput, SdkError<GetInsightRuleReportError>>>;
    fn get_metric_data(&self, builder: GetMetricDataInputBuilder) -> impl Future<Output = Result<GetMetricDataOutput, SdkError<GetMetricDataError>>>;
    fn get_metric_statistics(&self, builder: GetMetricStatisticsInputBuilder) -> impl Future<Output = Result<GetMetricStatisticsOutput, SdkError<GetMetricStatisticsError>>>;
    fn get_metric_stream(&self, builder: GetMetricStreamInputBuilder) -> impl Future<Output = Result<GetMetricStreamOutput, SdkError<GetMetricStreamError>>>;
    fn get_metric_widget_image(&self, builder: GetMetricWidgetImageInputBuilder) -> impl Future<Output = Result<GetMetricWidgetImageOutput, SdkError<GetMetricWidgetImageError>>>;
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>>;
    fn list_managed_insight_rules(&self, builder: ListManagedInsightRulesInputBuilder) -> impl Future<Output = Result<ListManagedInsightRulesOutput, SdkError<ListManagedInsightRulesError>>>;
    fn list_metric_streams(&self, builder: ListMetricStreamsInputBuilder) -> impl Future<Output = Result<ListMetricStreamsOutput, SdkError<ListMetricStreamsError>>>;
    fn list_metrics(&self, builder: ListMetricsInputBuilder) -> impl Future<Output = Result<ListMetricsOutput, SdkError<ListMetricsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn put_anomaly_detector(&self, builder: PutAnomalyDetectorInputBuilder) -> impl Future<Output = Result<PutAnomalyDetectorOutput, SdkError<PutAnomalyDetectorError>>>;
    fn put_composite_alarm(&self, builder: PutCompositeAlarmInputBuilder) -> impl Future<Output = Result<PutCompositeAlarmOutput, SdkError<PutCompositeAlarmError>>>;
    fn put_dashboard(&self, builder: PutDashboardInputBuilder) -> impl Future<Output = Result<PutDashboardOutput, SdkError<PutDashboardError>>>;
    fn put_insight_rule(&self, builder: PutInsightRuleInputBuilder) -> impl Future<Output = Result<PutInsightRuleOutput, SdkError<PutInsightRuleError>>>;
    fn put_managed_insight_rules(&self, builder: PutManagedInsightRulesInputBuilder) -> impl Future<Output = Result<PutManagedInsightRulesOutput, SdkError<PutManagedInsightRulesError>>>;
    fn put_metric_alarm(&self, builder: PutMetricAlarmInputBuilder) -> impl Future<Output = Result<PutMetricAlarmOutput, SdkError<PutMetricAlarmError>>>;
    fn put_metric_data(&self, builder: PutMetricDataInputBuilder) -> impl Future<Output = Result<PutMetricDataOutput, SdkError<PutMetricDataError>>>;
    fn put_metric_stream(&self, builder: PutMetricStreamInputBuilder) -> impl Future<Output = Result<PutMetricStreamOutput, SdkError<PutMetricStreamError>>>;
    fn set_alarm_state(&self, builder: SetAlarmStateInputBuilder) -> impl Future<Output = Result<SetAlarmStateOutput, SdkError<SetAlarmStateError>>>;
    fn start_metric_streams(&self, builder: StartMetricStreamsInputBuilder) -> impl Future<Output = Result<StartMetricStreamsOutput, SdkError<StartMetricStreamsError>>>;
    fn stop_metric_streams(&self, builder: StopMetricStreamsInputBuilder) -> impl Future<Output = Result<StopMetricStreamsOutput, SdkError<StopMetricStreamsError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
}
impl CloudWatchClient for CloudWatchClientImpl {
    fn delete_alarms(&self, builder: DeleteAlarmsInputBuilder) -> impl Future<Output = Result<DeleteAlarmsOutput, SdkError<DeleteAlarmsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_anomaly_detector(&self, builder: DeleteAnomalyDetectorInputBuilder) -> impl Future<Output = Result<DeleteAnomalyDetectorOutput, SdkError<DeleteAnomalyDetectorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dashboards(&self, builder: DeleteDashboardsInputBuilder) -> impl Future<Output = Result<DeleteDashboardsOutput, SdkError<DeleteDashboardsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_insight_rules(&self, builder: DeleteInsightRulesInputBuilder) -> impl Future<Output = Result<DeleteInsightRulesOutput, SdkError<DeleteInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_metric_stream(&self, builder: DeleteMetricStreamInputBuilder) -> impl Future<Output = Result<DeleteMetricStreamOutput, SdkError<DeleteMetricStreamError>>> {
        builder.send_with(&self.0)
    }
    fn describe_alarm_history(&self, builder: DescribeAlarmHistoryInputBuilder) -> impl Future<Output = Result<DescribeAlarmHistoryOutput, SdkError<DescribeAlarmHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_alarms(&self, builder: DescribeAlarmsInputBuilder) -> impl Future<Output = Result<DescribeAlarmsOutput, SdkError<DescribeAlarmsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_alarms_for_metric(&self, builder: DescribeAlarmsForMetricInputBuilder) -> impl Future<Output = Result<DescribeAlarmsForMetricOutput, SdkError<DescribeAlarmsForMetricError>>> {
        builder.send_with(&self.0)
    }
    fn describe_anomaly_detectors(&self, builder: DescribeAnomalyDetectorsInputBuilder) -> impl Future<Output = Result<DescribeAnomalyDetectorsOutput, SdkError<DescribeAnomalyDetectorsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_insight_rules(&self, builder: DescribeInsightRulesInputBuilder) -> impl Future<Output = Result<DescribeInsightRulesOutput, SdkError<DescribeInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn disable_alarm_actions(&self, builder: DisableAlarmActionsInputBuilder) -> impl Future<Output = Result<DisableAlarmActionsOutput, SdkError<DisableAlarmActionsError>>> {
        builder.send_with(&self.0)
    }
    fn disable_insight_rules(&self, builder: DisableInsightRulesInputBuilder) -> impl Future<Output = Result<DisableInsightRulesOutput, SdkError<DisableInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn enable_alarm_actions(&self, builder: EnableAlarmActionsInputBuilder) -> impl Future<Output = Result<EnableAlarmActionsOutput, SdkError<EnableAlarmActionsError>>> {
        builder.send_with(&self.0)
    }
    fn enable_insight_rules(&self, builder: EnableInsightRulesInputBuilder) -> impl Future<Output = Result<EnableInsightRulesOutput, SdkError<EnableInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn get_dashboard(&self, builder: GetDashboardInputBuilder) -> impl Future<Output = Result<GetDashboardOutput, SdkError<GetDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight_rule_report(&self, builder: GetInsightRuleReportInputBuilder) -> impl Future<Output = Result<GetInsightRuleReportOutput, SdkError<GetInsightRuleReportError>>> {
        builder.send_with(&self.0)
    }
    fn get_metric_data(&self, builder: GetMetricDataInputBuilder) -> impl Future<Output = Result<GetMetricDataOutput, SdkError<GetMetricDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_metric_statistics(&self, builder: GetMetricStatisticsInputBuilder) -> impl Future<Output = Result<GetMetricStatisticsOutput, SdkError<GetMetricStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_metric_stream(&self, builder: GetMetricStreamInputBuilder) -> impl Future<Output = Result<GetMetricStreamOutput, SdkError<GetMetricStreamError>>> {
        builder.send_with(&self.0)
    }
    fn get_metric_widget_image(&self, builder: GetMetricWidgetImageInputBuilder) -> impl Future<Output = Result<GetMetricWidgetImageOutput, SdkError<GetMetricWidgetImageError>>> {
        builder.send_with(&self.0)
    }
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>> {
        builder.send_with(&self.0)
    }
    fn list_managed_insight_rules(&self, builder: ListManagedInsightRulesInputBuilder) -> impl Future<Output = Result<ListManagedInsightRulesOutput, SdkError<ListManagedInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_metric_streams(&self, builder: ListMetricStreamsInputBuilder) -> impl Future<Output = Result<ListMetricStreamsOutput, SdkError<ListMetricStreamsError>>> {
        builder.send_with(&self.0)
    }
    fn list_metrics(&self, builder: ListMetricsInputBuilder) -> impl Future<Output = Result<ListMetricsOutput, SdkError<ListMetricsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn put_anomaly_detector(&self, builder: PutAnomalyDetectorInputBuilder) -> impl Future<Output = Result<PutAnomalyDetectorOutput, SdkError<PutAnomalyDetectorError>>> {
        builder.send_with(&self.0)
    }
    fn put_composite_alarm(&self, builder: PutCompositeAlarmInputBuilder) -> impl Future<Output = Result<PutCompositeAlarmOutput, SdkError<PutCompositeAlarmError>>> {
        builder.send_with(&self.0)
    }
    fn put_dashboard(&self, builder: PutDashboardInputBuilder) -> impl Future<Output = Result<PutDashboardOutput, SdkError<PutDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn put_insight_rule(&self, builder: PutInsightRuleInputBuilder) -> impl Future<Output = Result<PutInsightRuleOutput, SdkError<PutInsightRuleError>>> {
        builder.send_with(&self.0)
    }
    fn put_managed_insight_rules(&self, builder: PutManagedInsightRulesInputBuilder) -> impl Future<Output = Result<PutManagedInsightRulesOutput, SdkError<PutManagedInsightRulesError>>> {
        builder.send_with(&self.0)
    }
    fn put_metric_alarm(&self, builder: PutMetricAlarmInputBuilder) -> impl Future<Output = Result<PutMetricAlarmOutput, SdkError<PutMetricAlarmError>>> {
        builder.send_with(&self.0)
    }
    fn put_metric_data(&self, builder: PutMetricDataInputBuilder) -> impl Future<Output = Result<PutMetricDataOutput, SdkError<PutMetricDataError>>> {
        builder.send_with(&self.0)
    }
    fn put_metric_stream(&self, builder: PutMetricStreamInputBuilder) -> impl Future<Output = Result<PutMetricStreamOutput, SdkError<PutMetricStreamError>>> {
        builder.send_with(&self.0)
    }
    fn set_alarm_state(&self, builder: SetAlarmStateInputBuilder) -> impl Future<Output = Result<SetAlarmStateOutput, SdkError<SetAlarmStateError>>> {
        builder.send_with(&self.0)
    }
    fn start_metric_streams(&self, builder: StartMetricStreamsInputBuilder) -> impl Future<Output = Result<StartMetricStreamsOutput, SdkError<StartMetricStreamsError>>> {
        builder.send_with(&self.0)
    }
    fn stop_metric_streams(&self, builder: StopMetricStreamsInputBuilder) -> impl Future<Output = Result<StopMetricStreamsOutput, SdkError<StopMetricStreamsError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: CloudWatchClient> CloudWatchClient for &T {
    fn delete_alarms(&self, builder: DeleteAlarmsInputBuilder) -> impl Future<Output = Result<DeleteAlarmsOutput, SdkError<DeleteAlarmsError>>> {
        (*self).delete_alarms(builder)
    }
    fn delete_anomaly_detector(&self, builder: DeleteAnomalyDetectorInputBuilder) -> impl Future<Output = Result<DeleteAnomalyDetectorOutput, SdkError<DeleteAnomalyDetectorError>>> {
        (*self).delete_anomaly_detector(builder)
    }
    fn delete_dashboards(&self, builder: DeleteDashboardsInputBuilder) -> impl Future<Output = Result<DeleteDashboardsOutput, SdkError<DeleteDashboardsError>>> {
        (*self).delete_dashboards(builder)
    }
    fn delete_insight_rules(&self, builder: DeleteInsightRulesInputBuilder) -> impl Future<Output = Result<DeleteInsightRulesOutput, SdkError<DeleteInsightRulesError>>> {
        (*self).delete_insight_rules(builder)
    }
    fn delete_metric_stream(&self, builder: DeleteMetricStreamInputBuilder) -> impl Future<Output = Result<DeleteMetricStreamOutput, SdkError<DeleteMetricStreamError>>> {
        (*self).delete_metric_stream(builder)
    }
    fn describe_alarm_history(&self, builder: DescribeAlarmHistoryInputBuilder) -> impl Future<Output = Result<DescribeAlarmHistoryOutput, SdkError<DescribeAlarmHistoryError>>> {
        (*self).describe_alarm_history(builder)
    }
    fn describe_alarms(&self, builder: DescribeAlarmsInputBuilder) -> impl Future<Output = Result<DescribeAlarmsOutput, SdkError<DescribeAlarmsError>>> {
        (*self).describe_alarms(builder)
    }
    fn describe_alarms_for_metric(&self, builder: DescribeAlarmsForMetricInputBuilder) -> impl Future<Output = Result<DescribeAlarmsForMetricOutput, SdkError<DescribeAlarmsForMetricError>>> {
        (*self).describe_alarms_for_metric(builder)
    }
    fn describe_anomaly_detectors(&self, builder: DescribeAnomalyDetectorsInputBuilder) -> impl Future<Output = Result<DescribeAnomalyDetectorsOutput, SdkError<DescribeAnomalyDetectorsError>>> {
        (*self).describe_anomaly_detectors(builder)
    }
    fn describe_insight_rules(&self, builder: DescribeInsightRulesInputBuilder) -> impl Future<Output = Result<DescribeInsightRulesOutput, SdkError<DescribeInsightRulesError>>> {
        (*self).describe_insight_rules(builder)
    }
    fn disable_alarm_actions(&self, builder: DisableAlarmActionsInputBuilder) -> impl Future<Output = Result<DisableAlarmActionsOutput, SdkError<DisableAlarmActionsError>>> {
        (*self).disable_alarm_actions(builder)
    }
    fn disable_insight_rules(&self, builder: DisableInsightRulesInputBuilder) -> impl Future<Output = Result<DisableInsightRulesOutput, SdkError<DisableInsightRulesError>>> {
        (*self).disable_insight_rules(builder)
    }
    fn enable_alarm_actions(&self, builder: EnableAlarmActionsInputBuilder) -> impl Future<Output = Result<EnableAlarmActionsOutput, SdkError<EnableAlarmActionsError>>> {
        (*self).enable_alarm_actions(builder)
    }
    fn enable_insight_rules(&self, builder: EnableInsightRulesInputBuilder) -> impl Future<Output = Result<EnableInsightRulesOutput, SdkError<EnableInsightRulesError>>> {
        (*self).enable_insight_rules(builder)
    }
    fn get_dashboard(&self, builder: GetDashboardInputBuilder) -> impl Future<Output = Result<GetDashboardOutput, SdkError<GetDashboardError>>> {
        (*self).get_dashboard(builder)
    }
    fn get_insight_rule_report(&self, builder: GetInsightRuleReportInputBuilder) -> impl Future<Output = Result<GetInsightRuleReportOutput, SdkError<GetInsightRuleReportError>>> {
        (*self).get_insight_rule_report(builder)
    }
    fn get_metric_data(&self, builder: GetMetricDataInputBuilder) -> impl Future<Output = Result<GetMetricDataOutput, SdkError<GetMetricDataError>>> {
        (*self).get_metric_data(builder)
    }
    fn get_metric_statistics(&self, builder: GetMetricStatisticsInputBuilder) -> impl Future<Output = Result<GetMetricStatisticsOutput, SdkError<GetMetricStatisticsError>>> {
        (*self).get_metric_statistics(builder)
    }
    fn get_metric_stream(&self, builder: GetMetricStreamInputBuilder) -> impl Future<Output = Result<GetMetricStreamOutput, SdkError<GetMetricStreamError>>> {
        (*self).get_metric_stream(builder)
    }
    fn get_metric_widget_image(&self, builder: GetMetricWidgetImageInputBuilder) -> impl Future<Output = Result<GetMetricWidgetImageOutput, SdkError<GetMetricWidgetImageError>>> {
        (*self).get_metric_widget_image(builder)
    }
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>> {
        (*self).list_dashboards(builder)
    }
    fn list_managed_insight_rules(&self, builder: ListManagedInsightRulesInputBuilder) -> impl Future<Output = Result<ListManagedInsightRulesOutput, SdkError<ListManagedInsightRulesError>>> {
        (*self).list_managed_insight_rules(builder)
    }
    fn list_metric_streams(&self, builder: ListMetricStreamsInputBuilder) -> impl Future<Output = Result<ListMetricStreamsOutput, SdkError<ListMetricStreamsError>>> {
        (*self).list_metric_streams(builder)
    }
    fn list_metrics(&self, builder: ListMetricsInputBuilder) -> impl Future<Output = Result<ListMetricsOutput, SdkError<ListMetricsError>>> {
        (*self).list_metrics(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn put_anomaly_detector(&self, builder: PutAnomalyDetectorInputBuilder) -> impl Future<Output = Result<PutAnomalyDetectorOutput, SdkError<PutAnomalyDetectorError>>> {
        (*self).put_anomaly_detector(builder)
    }
    fn put_composite_alarm(&self, builder: PutCompositeAlarmInputBuilder) -> impl Future<Output = Result<PutCompositeAlarmOutput, SdkError<PutCompositeAlarmError>>> {
        (*self).put_composite_alarm(builder)
    }
    fn put_dashboard(&self, builder: PutDashboardInputBuilder) -> impl Future<Output = Result<PutDashboardOutput, SdkError<PutDashboardError>>> {
        (*self).put_dashboard(builder)
    }
    fn put_insight_rule(&self, builder: PutInsightRuleInputBuilder) -> impl Future<Output = Result<PutInsightRuleOutput, SdkError<PutInsightRuleError>>> {
        (*self).put_insight_rule(builder)
    }
    fn put_managed_insight_rules(&self, builder: PutManagedInsightRulesInputBuilder) -> impl Future<Output = Result<PutManagedInsightRulesOutput, SdkError<PutManagedInsightRulesError>>> {
        (*self).put_managed_insight_rules(builder)
    }
    fn put_metric_alarm(&self, builder: PutMetricAlarmInputBuilder) -> impl Future<Output = Result<PutMetricAlarmOutput, SdkError<PutMetricAlarmError>>> {
        (*self).put_metric_alarm(builder)
    }
    fn put_metric_data(&self, builder: PutMetricDataInputBuilder) -> impl Future<Output = Result<PutMetricDataOutput, SdkError<PutMetricDataError>>> {
        (*self).put_metric_data(builder)
    }
    fn put_metric_stream(&self, builder: PutMetricStreamInputBuilder) -> impl Future<Output = Result<PutMetricStreamOutput, SdkError<PutMetricStreamError>>> {
        (*self).put_metric_stream(builder)
    }
    fn set_alarm_state(&self, builder: SetAlarmStateInputBuilder) -> impl Future<Output = Result<SetAlarmStateOutput, SdkError<SetAlarmStateError>>> {
        (*self).set_alarm_state(builder)
    }
    fn start_metric_streams(&self, builder: StartMetricStreamsInputBuilder) -> impl Future<Output = Result<StartMetricStreamsOutput, SdkError<StartMetricStreamsError>>> {
        (*self).start_metric_streams(builder)
    }
    fn stop_metric_streams(&self, builder: StopMetricStreamsInputBuilder) -> impl Future<Output = Result<StopMetricStreamsOutput, SdkError<StopMetricStreamsError>>> {
        (*self).stop_metric_streams(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCloudWatchClient {}
    impl CloudWatchClient for edCloudWatchClient {
        async fn delete_alarms(&self, builder: DeleteAlarmsInputBuilder) -> Result<DeleteAlarmsOutput, SdkError<DeleteAlarmsError>>;
        async fn delete_anomaly_detector(&self, builder: DeleteAnomalyDetectorInputBuilder) -> Result<DeleteAnomalyDetectorOutput, SdkError<DeleteAnomalyDetectorError>>;
        async fn delete_dashboards(&self, builder: DeleteDashboardsInputBuilder) -> Result<DeleteDashboardsOutput, SdkError<DeleteDashboardsError>>;
        async fn delete_insight_rules(&self, builder: DeleteInsightRulesInputBuilder) -> Result<DeleteInsightRulesOutput, SdkError<DeleteInsightRulesError>>;
        async fn delete_metric_stream(&self, builder: DeleteMetricStreamInputBuilder) -> Result<DeleteMetricStreamOutput, SdkError<DeleteMetricStreamError>>;
        async fn describe_alarm_history(&self, builder: DescribeAlarmHistoryInputBuilder) -> Result<DescribeAlarmHistoryOutput, SdkError<DescribeAlarmHistoryError>>;
        async fn describe_alarms(&self, builder: DescribeAlarmsInputBuilder) -> Result<DescribeAlarmsOutput, SdkError<DescribeAlarmsError>>;
        async fn describe_alarms_for_metric(&self, builder: DescribeAlarmsForMetricInputBuilder) -> Result<DescribeAlarmsForMetricOutput, SdkError<DescribeAlarmsForMetricError>>;
        async fn describe_anomaly_detectors(&self, builder: DescribeAnomalyDetectorsInputBuilder) -> Result<DescribeAnomalyDetectorsOutput, SdkError<DescribeAnomalyDetectorsError>>;
        async fn describe_insight_rules(&self, builder: DescribeInsightRulesInputBuilder) -> Result<DescribeInsightRulesOutput, SdkError<DescribeInsightRulesError>>;
        async fn disable_alarm_actions(&self, builder: DisableAlarmActionsInputBuilder) -> Result<DisableAlarmActionsOutput, SdkError<DisableAlarmActionsError>>;
        async fn disable_insight_rules(&self, builder: DisableInsightRulesInputBuilder) -> Result<DisableInsightRulesOutput, SdkError<DisableInsightRulesError>>;
        async fn enable_alarm_actions(&self, builder: EnableAlarmActionsInputBuilder) -> Result<EnableAlarmActionsOutput, SdkError<EnableAlarmActionsError>>;
        async fn enable_insight_rules(&self, builder: EnableInsightRulesInputBuilder) -> Result<EnableInsightRulesOutput, SdkError<EnableInsightRulesError>>;
        async fn get_dashboard(&self, builder: GetDashboardInputBuilder) -> Result<GetDashboardOutput, SdkError<GetDashboardError>>;
        async fn get_insight_rule_report(&self, builder: GetInsightRuleReportInputBuilder) -> Result<GetInsightRuleReportOutput, SdkError<GetInsightRuleReportError>>;
        async fn get_metric_data(&self, builder: GetMetricDataInputBuilder) -> Result<GetMetricDataOutput, SdkError<GetMetricDataError>>;
        async fn get_metric_statistics(&self, builder: GetMetricStatisticsInputBuilder) -> Result<GetMetricStatisticsOutput, SdkError<GetMetricStatisticsError>>;
        async fn get_metric_stream(&self, builder: GetMetricStreamInputBuilder) -> Result<GetMetricStreamOutput, SdkError<GetMetricStreamError>>;
        async fn get_metric_widget_image(&self, builder: GetMetricWidgetImageInputBuilder) -> Result<GetMetricWidgetImageOutput, SdkError<GetMetricWidgetImageError>>;
        async fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> Result<ListDashboardsOutput, SdkError<ListDashboardsError>>;
        async fn list_managed_insight_rules(&self, builder: ListManagedInsightRulesInputBuilder) -> Result<ListManagedInsightRulesOutput, SdkError<ListManagedInsightRulesError>>;
        async fn list_metric_streams(&self, builder: ListMetricStreamsInputBuilder) -> Result<ListMetricStreamsOutput, SdkError<ListMetricStreamsError>>;
        async fn list_metrics(&self, builder: ListMetricsInputBuilder) -> Result<ListMetricsOutput, SdkError<ListMetricsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn put_anomaly_detector(&self, builder: PutAnomalyDetectorInputBuilder) -> Result<PutAnomalyDetectorOutput, SdkError<PutAnomalyDetectorError>>;
        async fn put_composite_alarm(&self, builder: PutCompositeAlarmInputBuilder) -> Result<PutCompositeAlarmOutput, SdkError<PutCompositeAlarmError>>;
        async fn put_dashboard(&self, builder: PutDashboardInputBuilder) -> Result<PutDashboardOutput, SdkError<PutDashboardError>>;
        async fn put_insight_rule(&self, builder: PutInsightRuleInputBuilder) -> Result<PutInsightRuleOutput, SdkError<PutInsightRuleError>>;
        async fn put_managed_insight_rules(&self, builder: PutManagedInsightRulesInputBuilder) -> Result<PutManagedInsightRulesOutput, SdkError<PutManagedInsightRulesError>>;
        async fn put_metric_alarm(&self, builder: PutMetricAlarmInputBuilder) -> Result<PutMetricAlarmOutput, SdkError<PutMetricAlarmError>>;
        async fn put_metric_data(&self, builder: PutMetricDataInputBuilder) -> Result<PutMetricDataOutput, SdkError<PutMetricDataError>>;
        async fn put_metric_stream(&self, builder: PutMetricStreamInputBuilder) -> Result<PutMetricStreamOutput, SdkError<PutMetricStreamError>>;
        async fn set_alarm_state(&self, builder: SetAlarmStateInputBuilder) -> Result<SetAlarmStateOutput, SdkError<SetAlarmStateError>>;
        async fn start_metric_streams(&self, builder: StartMetricStreamsInputBuilder) -> Result<StartMetricStreamsOutput, SdkError<StartMetricStreamsError>>;
        async fn stop_metric_streams(&self, builder: StopMetricStreamsInputBuilder) -> Result<StopMetricStreamsOutput, SdkError<StopMetricStreamsError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
    }
}
