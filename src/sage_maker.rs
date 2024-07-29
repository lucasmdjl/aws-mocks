/*
 * aws-mocks - A mocking library for AWS.
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
use aws_sdk_sagemaker::operation::add_association::{builders::*, *};
use aws_sdk_sagemaker::operation::add_tags::{builders::*, *};
use aws_sdk_sagemaker::operation::associate_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::batch_describe_model_package::{builders::*, *};
use aws_sdk_sagemaker::operation::create_action::{builders::*, *};
use aws_sdk_sagemaker::operation::create_algorithm::{builders::*, *};
use aws_sdk_sagemaker::operation::create_app::{builders::*, *};
use aws_sdk_sagemaker::operation::create_app_image_config::{builders::*, *};
use aws_sdk_sagemaker::operation::create_artifact::{builders::*, *};
use aws_sdk_sagemaker::operation::create_auto_ml_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_auto_ml_job_v2::{builders::*, *};
use aws_sdk_sagemaker::operation::create_cluster::{builders::*, *};
use aws_sdk_sagemaker::operation::create_code_repository::{builders::*, *};
use aws_sdk_sagemaker::operation::create_compilation_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_context::{builders::*, *};
use aws_sdk_sagemaker::operation::create_data_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::create_device_fleet::{builders::*, *};
use aws_sdk_sagemaker::operation::create_domain::{builders::*, *};
use aws_sdk_sagemaker::operation::create_edge_deployment_plan::{builders::*, *};
use aws_sdk_sagemaker::operation::create_edge_deployment_stage::{builders::*, *};
use aws_sdk_sagemaker::operation::create_edge_packaging_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_endpoint::{builders::*, *};
use aws_sdk_sagemaker::operation::create_endpoint_config::{builders::*, *};
use aws_sdk_sagemaker::operation::create_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::create_feature_group::{builders::*, *};
use aws_sdk_sagemaker::operation::create_flow_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::create_hub::{builders::*, *};
use aws_sdk_sagemaker::operation::create_hub_content_reference::{builders::*, *};
use aws_sdk_sagemaker::operation::create_human_task_ui::{builders::*, *};
use aws_sdk_sagemaker::operation::create_hyper_parameter_tuning_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_image::{builders::*, *};
use aws_sdk_sagemaker::operation::create_image_version::{builders::*, *};
use aws_sdk_sagemaker::operation::create_inference_component::{builders::*, *};
use aws_sdk_sagemaker::operation::create_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::create_inference_recommendations_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_labeling_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_bias_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_card::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_card_export_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_explainability_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_package::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_package_group::{builders::*, *};
use aws_sdk_sagemaker::operation::create_model_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::create_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::create_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::create_notebook_instance_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::create_optimization_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_pipeline::{builders::*, *};
use aws_sdk_sagemaker::operation::create_presigned_domain_url::{builders::*, *};
use aws_sdk_sagemaker::operation::create_presigned_mlflow_tracking_server_url::{builders::*, *};
use aws_sdk_sagemaker::operation::create_presigned_notebook_instance_url::{builders::*, *};
use aws_sdk_sagemaker::operation::create_processing_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_project::{builders::*, *};
use aws_sdk_sagemaker::operation::create_space::{builders::*, *};
use aws_sdk_sagemaker::operation::create_studio_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::create_training_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_transform_job::{builders::*, *};
use aws_sdk_sagemaker::operation::create_trial::{builders::*, *};
use aws_sdk_sagemaker::operation::create_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::create_user_profile::{builders::*, *};
use aws_sdk_sagemaker::operation::create_workforce::{builders::*, *};
use aws_sdk_sagemaker::operation::create_workteam::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_action::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_algorithm::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_app::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_app_image_config::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_artifact::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_association::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_cluster::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_code_repository::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_compilation_job::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_context::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_data_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_device_fleet::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_domain::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_edge_deployment_plan::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_edge_deployment_stage::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_endpoint::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_endpoint_config::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_feature_group::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_flow_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_hub::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_hub_content::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_hub_content_reference::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_human_task_ui::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_hyper_parameter_tuning_job::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_image::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_image_version::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_inference_component::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_bias_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_card::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_explainability_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_package::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_package_group::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_package_group_policy::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_model_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_notebook_instance_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_optimization_job::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_pipeline::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_project::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_space::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_studio_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_tags::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_trial::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_user_profile::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_workforce::{builders::*, *};
use aws_sdk_sagemaker::operation::delete_workteam::{builders::*, *};
use aws_sdk_sagemaker::operation::deregister_devices::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_action::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_algorithm::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_app::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_app_image_config::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_artifact::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_auto_ml_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_auto_ml_job_v2::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_cluster::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_cluster_node::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_code_repository::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_compilation_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_context::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_data_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_device::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_device_fleet::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_domain::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_edge_deployment_plan::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_edge_packaging_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_endpoint::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_endpoint_config::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_feature_group::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_feature_metadata::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_flow_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_hub::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_hub_content::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_human_task_ui::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_hyper_parameter_tuning_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_image::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_image_version::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_inference_component::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_inference_recommendations_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_labeling_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_lineage_group::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_bias_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_card::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_card_export_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_explainability_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_package::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_package_group::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_model_quality_job_definition::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_notebook_instance_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_optimization_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_pipeline::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_pipeline_definition_for_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_pipeline_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_processing_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_project::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_space::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_studio_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_subscribed_workteam::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_training_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_transform_job::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_trial::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_user_profile::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_workforce::{builders::*, *};
use aws_sdk_sagemaker::operation::describe_workteam::{builders::*, *};
use aws_sdk_sagemaker::operation::disable_sagemaker_servicecatalog_portfolio::{builders::*, *};
use aws_sdk_sagemaker::operation::disassociate_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::enable_sagemaker_servicecatalog_portfolio::{builders::*, *};
use aws_sdk_sagemaker::operation::get_device_fleet_report::{builders::*, *};
use aws_sdk_sagemaker::operation::get_lineage_group_policy::{builders::*, *};
use aws_sdk_sagemaker::operation::get_model_package_group_policy::{builders::*, *};
use aws_sdk_sagemaker::operation::get_sagemaker_servicecatalog_portfolio_status::{builders::*, *};
use aws_sdk_sagemaker::operation::get_scaling_configuration_recommendation::{builders::*, *};
use aws_sdk_sagemaker::operation::get_search_suggestions::{builders::*, *};
use aws_sdk_sagemaker::operation::import_hub_content::{builders::*, *};
use aws_sdk_sagemaker::operation::list_actions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_algorithms::{builders::*, *};
use aws_sdk_sagemaker::operation::list_aliases::{builders::*, *};
use aws_sdk_sagemaker::operation::list_app_image_configs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_apps::{builders::*, *};
use aws_sdk_sagemaker::operation::list_artifacts::{builders::*, *};
use aws_sdk_sagemaker::operation::list_associations::{builders::*, *};
use aws_sdk_sagemaker::operation::list_auto_ml_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_candidates_for_auto_ml_job::{builders::*, *};
use aws_sdk_sagemaker::operation::list_cluster_nodes::{builders::*, *};
use aws_sdk_sagemaker::operation::list_clusters::{builders::*, *};
use aws_sdk_sagemaker::operation::list_code_repositories::{builders::*, *};
use aws_sdk_sagemaker::operation::list_compilation_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_contexts::{builders::*, *};
use aws_sdk_sagemaker::operation::list_data_quality_job_definitions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_device_fleets::{builders::*, *};
use aws_sdk_sagemaker::operation::list_devices::{builders::*, *};
use aws_sdk_sagemaker::operation::list_domains::{builders::*, *};
use aws_sdk_sagemaker::operation::list_edge_deployment_plans::{builders::*, *};
use aws_sdk_sagemaker::operation::list_edge_packaging_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_endpoint_configs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_endpoints::{builders::*, *};
use aws_sdk_sagemaker::operation::list_experiments::{builders::*, *};
use aws_sdk_sagemaker::operation::list_feature_groups::{builders::*, *};
use aws_sdk_sagemaker::operation::list_flow_definitions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_hub_content_versions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_hub_contents::{builders::*, *};
use aws_sdk_sagemaker::operation::list_hubs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_human_task_uis::{builders::*, *};
use aws_sdk_sagemaker::operation::list_hyper_parameter_tuning_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_image_versions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_images::{builders::*, *};
use aws_sdk_sagemaker::operation::list_inference_components::{builders::*, *};
use aws_sdk_sagemaker::operation::list_inference_experiments::{builders::*, *};
use aws_sdk_sagemaker::operation::list_inference_recommendations_job_steps::{builders::*, *};
use aws_sdk_sagemaker::operation::list_inference_recommendations_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_labeling_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_labeling_jobs_for_workteam::{builders::*, *};
use aws_sdk_sagemaker::operation::list_lineage_groups::{builders::*, *};
use aws_sdk_sagemaker::operation::list_mlflow_tracking_servers::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_bias_job_definitions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_card_export_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_card_versions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_cards::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_explainability_job_definitions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_metadata::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_package_groups::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_packages::{builders::*, *};
use aws_sdk_sagemaker::operation::list_model_quality_job_definitions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_models::{builders::*, *};
use aws_sdk_sagemaker::operation::list_monitoring_alert_history::{builders::*, *};
use aws_sdk_sagemaker::operation::list_monitoring_alerts::{builders::*, *};
use aws_sdk_sagemaker::operation::list_monitoring_executions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_monitoring_schedules::{builders::*, *};
use aws_sdk_sagemaker::operation::list_notebook_instance_lifecycle_configs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_notebook_instances::{builders::*, *};
use aws_sdk_sagemaker::operation::list_optimization_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_pipeline_execution_steps::{builders::*, *};
use aws_sdk_sagemaker::operation::list_pipeline_executions::{builders::*, *};
use aws_sdk_sagemaker::operation::list_pipeline_parameters_for_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::list_pipelines::{builders::*, *};
use aws_sdk_sagemaker::operation::list_processing_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_projects::{builders::*, *};
use aws_sdk_sagemaker::operation::list_resource_catalogs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_spaces::{builders::*, *};
use aws_sdk_sagemaker::operation::list_stage_devices::{builders::*, *};
use aws_sdk_sagemaker::operation::list_studio_lifecycle_configs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_subscribed_workteams::{builders::*, *};
use aws_sdk_sagemaker::operation::list_tags::{builders::*, *};
use aws_sdk_sagemaker::operation::list_training_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_training_jobs_for_hyper_parameter_tuning_job::{builders::*, *};
use aws_sdk_sagemaker::operation::list_transform_jobs::{builders::*, *};
use aws_sdk_sagemaker::operation::list_trial_components::{builders::*, *};
use aws_sdk_sagemaker::operation::list_trials::{builders::*, *};
use aws_sdk_sagemaker::operation::list_user_profiles::{builders::*, *};
use aws_sdk_sagemaker::operation::list_workforces::{builders::*, *};
use aws_sdk_sagemaker::operation::list_workteams::{builders::*, *};
use aws_sdk_sagemaker::operation::put_model_package_group_policy::{builders::*, *};
use aws_sdk_sagemaker::operation::query_lineage::{builders::*, *};
use aws_sdk_sagemaker::operation::register_devices::{builders::*, *};
use aws_sdk_sagemaker::operation::render_ui_template::{builders::*, *};
use aws_sdk_sagemaker::operation::retry_pipeline_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::search::{builders::*, *};
use aws_sdk_sagemaker::operation::send_pipeline_execution_step_failure::{builders::*, *};
use aws_sdk_sagemaker::operation::send_pipeline_execution_step_success::{builders::*, *};
use aws_sdk_sagemaker::operation::start_edge_deployment_stage::{builders::*, *};
use aws_sdk_sagemaker::operation::start_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::start_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::start_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::start_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::start_pipeline_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_auto_ml_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_compilation_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_edge_deployment_stage::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_edge_packaging_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_hyper_parameter_tuning_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_inference_recommendations_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_labeling_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_optimization_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_pipeline_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_processing_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_training_job::{builders::*, *};
use aws_sdk_sagemaker::operation::stop_transform_job::{builders::*, *};
use aws_sdk_sagemaker::operation::update_action::{builders::*, *};
use aws_sdk_sagemaker::operation::update_app_image_config::{builders::*, *};
use aws_sdk_sagemaker::operation::update_artifact::{builders::*, *};
use aws_sdk_sagemaker::operation::update_cluster::{builders::*, *};
use aws_sdk_sagemaker::operation::update_cluster_software::{builders::*, *};
use aws_sdk_sagemaker::operation::update_code_repository::{builders::*, *};
use aws_sdk_sagemaker::operation::update_context::{builders::*, *};
use aws_sdk_sagemaker::operation::update_device_fleet::{builders::*, *};
use aws_sdk_sagemaker::operation::update_devices::{builders::*, *};
use aws_sdk_sagemaker::operation::update_domain::{builders::*, *};
use aws_sdk_sagemaker::operation::update_endpoint::{builders::*, *};
use aws_sdk_sagemaker::operation::update_endpoint_weights_and_capacities::{builders::*, *};
use aws_sdk_sagemaker::operation::update_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::update_feature_group::{builders::*, *};
use aws_sdk_sagemaker::operation::update_feature_metadata::{builders::*, *};
use aws_sdk_sagemaker::operation::update_hub::{builders::*, *};
use aws_sdk_sagemaker::operation::update_image::{builders::*, *};
use aws_sdk_sagemaker::operation::update_image_version::{builders::*, *};
use aws_sdk_sagemaker::operation::update_inference_component::{builders::*, *};
use aws_sdk_sagemaker::operation::update_inference_component_runtime_config::{builders::*, *};
use aws_sdk_sagemaker::operation::update_inference_experiment::{builders::*, *};
use aws_sdk_sagemaker::operation::update_mlflow_tracking_server::{builders::*, *};
use aws_sdk_sagemaker::operation::update_model_card::{builders::*, *};
use aws_sdk_sagemaker::operation::update_model_package::{builders::*, *};
use aws_sdk_sagemaker::operation::update_monitoring_alert::{builders::*, *};
use aws_sdk_sagemaker::operation::update_monitoring_schedule::{builders::*, *};
use aws_sdk_sagemaker::operation::update_notebook_instance::{builders::*, *};
use aws_sdk_sagemaker::operation::update_notebook_instance_lifecycle_config::{builders::*, *};
use aws_sdk_sagemaker::operation::update_pipeline::{builders::*, *};
use aws_sdk_sagemaker::operation::update_pipeline_execution::{builders::*, *};
use aws_sdk_sagemaker::operation::update_project::{builders::*, *};
use aws_sdk_sagemaker::operation::update_space::{builders::*, *};
use aws_sdk_sagemaker::operation::update_training_job::{builders::*, *};
use aws_sdk_sagemaker::operation::update_trial::{builders::*, *};
use aws_sdk_sagemaker::operation::update_trial_component::{builders::*, *};
use aws_sdk_sagemaker::operation::update_user_profile::{builders::*, *};
use aws_sdk_sagemaker::operation::update_workforce::{builders::*, *};
use aws_sdk_sagemaker::operation::update_workteam::{builders::*, *};
use aws_sdk_sagemaker::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_sagemaker::Client;
use std::ops::Deref;

pub use aws_sdk_sagemaker::*;

pub struct SageMakerClientImpl(Client);
impl SageMakerClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait SageMakerClient {
    fn add_association(&self, builder: AddAssociationInputBuilder) -> impl Future<Output = Result<AddAssociationOutput, SdkError<AddAssociationError>>>;
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>>;
    fn associate_trial_component(&self, builder: AssociateTrialComponentInputBuilder) -> impl Future<Output = Result<AssociateTrialComponentOutput, SdkError<AssociateTrialComponentError>>>;
    fn batch_describe_model_package(&self, builder: BatchDescribeModelPackageInputBuilder) -> impl Future<Output = Result<BatchDescribeModelPackageOutput, SdkError<BatchDescribeModelPackageError>>>;
    fn create_action(&self, builder: CreateActionInputBuilder) -> impl Future<Output = Result<CreateActionOutput, SdkError<CreateActionError>>>;
    fn create_algorithm(&self, builder: CreateAlgorithmInputBuilder) -> impl Future<Output = Result<CreateAlgorithmOutput, SdkError<CreateAlgorithmError>>>;
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>>;
    fn create_app_image_config(&self, builder: CreateAppImageConfigInputBuilder) -> impl Future<Output = Result<CreateAppImageConfigOutput, SdkError<CreateAppImageConfigError>>>;
    fn create_artifact(&self, builder: CreateArtifactInputBuilder) -> impl Future<Output = Result<CreateArtifactOutput, SdkError<CreateArtifactError>>>;
    fn create_auto_ml_job(&self, builder: CreateAutoMlJobInputBuilder) -> impl Future<Output = Result<CreateAutoMlJobOutput, SdkError<CreateAutoMLJobError>>>;
    fn create_auto_ml_job_v2(&self, builder: CreateAutoMlJobV2InputBuilder) -> impl Future<Output = Result<CreateAutoMlJobV2Output, SdkError<CreateAutoMLJobV2Error>>>;
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>>;
    fn create_code_repository(&self, builder: CreateCodeRepositoryInputBuilder) -> impl Future<Output = Result<CreateCodeRepositoryOutput, SdkError<CreateCodeRepositoryError>>>;
    fn create_compilation_job(&self, builder: CreateCompilationJobInputBuilder) -> impl Future<Output = Result<CreateCompilationJobOutput, SdkError<CreateCompilationJobError>>>;
    fn create_context(&self, builder: CreateContextInputBuilder) -> impl Future<Output = Result<CreateContextOutput, SdkError<CreateContextError>>>;
    fn create_data_quality_job_definition(&self, builder: CreateDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateDataQualityJobDefinitionOutput, SdkError<CreateDataQualityJobDefinitionError>>>;
    fn create_device_fleet(&self, builder: CreateDeviceFleetInputBuilder) -> impl Future<Output = Result<CreateDeviceFleetOutput, SdkError<CreateDeviceFleetError>>>;
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>>;
    fn create_edge_deployment_plan(&self, builder: CreateEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentPlanOutput, SdkError<CreateEdgeDeploymentPlanError>>>;
    fn create_edge_deployment_stage(&self, builder: CreateEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentStageOutput, SdkError<CreateEdgeDeploymentStageError>>>;
    fn create_edge_packaging_job(&self, builder: CreateEdgePackagingJobInputBuilder) -> impl Future<Output = Result<CreateEdgePackagingJobOutput, SdkError<CreateEdgePackagingJobError>>>;
    fn create_endpoint(&self, builder: CreateEndpointInputBuilder) -> impl Future<Output = Result<CreateEndpointOutput, SdkError<CreateEndpointError>>>;
    fn create_endpoint_config(&self, builder: CreateEndpointConfigInputBuilder) -> impl Future<Output = Result<CreateEndpointConfigOutput, SdkError<CreateEndpointConfigError>>>;
    fn create_experiment(&self, builder: CreateExperimentInputBuilder) -> impl Future<Output = Result<CreateExperimentOutput, SdkError<CreateExperimentError>>>;
    fn create_feature_group(&self, builder: CreateFeatureGroupInputBuilder) -> impl Future<Output = Result<CreateFeatureGroupOutput, SdkError<CreateFeatureGroupError>>>;
    fn create_flow_definition(&self, builder: CreateFlowDefinitionInputBuilder) -> impl Future<Output = Result<CreateFlowDefinitionOutput, SdkError<CreateFlowDefinitionError>>>;
    fn create_hub(&self, builder: CreateHubInputBuilder) -> impl Future<Output = Result<CreateHubOutput, SdkError<CreateHubError>>>;
    fn create_hub_content_reference(&self, builder: CreateHubContentReferenceInputBuilder) -> impl Future<Output = Result<CreateHubContentReferenceOutput, SdkError<CreateHubContentReferenceError>>>;
    fn create_human_task_ui(&self, builder: CreateHumanTaskUiInputBuilder) -> impl Future<Output = Result<CreateHumanTaskUiOutput, SdkError<CreateHumanTaskUiError>>>;
    fn create_hyper_parameter_tuning_job(&self, builder: CreateHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<CreateHyperParameterTuningJobOutput, SdkError<CreateHyperParameterTuningJobError>>>;
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>>;
    fn create_image_version(&self, builder: CreateImageVersionInputBuilder) -> impl Future<Output = Result<CreateImageVersionOutput, SdkError<CreateImageVersionError>>>;
    fn create_inference_component(&self, builder: CreateInferenceComponentInputBuilder) -> impl Future<Output = Result<CreateInferenceComponentOutput, SdkError<CreateInferenceComponentError>>>;
    fn create_inference_experiment(&self, builder: CreateInferenceExperimentInputBuilder) -> impl Future<Output = Result<CreateInferenceExperimentOutput, SdkError<CreateInferenceExperimentError>>>;
    fn create_inference_recommendations_job(&self, builder: CreateInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<CreateInferenceRecommendationsJobOutput, SdkError<CreateInferenceRecommendationsJobError>>>;
    fn create_labeling_job(&self, builder: CreateLabelingJobInputBuilder) -> impl Future<Output = Result<CreateLabelingJobOutput, SdkError<CreateLabelingJobError>>>;
    fn create_mlflow_tracking_server(&self, builder: CreateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<CreateMlflowTrackingServerOutput, SdkError<CreateMlflowTrackingServerError>>>;
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>>;
    fn create_model_bias_job_definition(&self, builder: CreateModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelBiasJobDefinitionOutput, SdkError<CreateModelBiasJobDefinitionError>>>;
    fn create_model_card(&self, builder: CreateModelCardInputBuilder) -> impl Future<Output = Result<CreateModelCardOutput, SdkError<CreateModelCardError>>>;
    fn create_model_card_export_job(&self, builder: CreateModelCardExportJobInputBuilder) -> impl Future<Output = Result<CreateModelCardExportJobOutput, SdkError<CreateModelCardExportJobError>>>;
    fn create_model_explainability_job_definition(&self, builder: CreateModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelExplainabilityJobDefinitionOutput, SdkError<CreateModelExplainabilityJobDefinitionError>>>;
    fn create_model_package(&self, builder: CreateModelPackageInputBuilder) -> impl Future<Output = Result<CreateModelPackageOutput, SdkError<CreateModelPackageError>>>;
    fn create_model_package_group(&self, builder: CreateModelPackageGroupInputBuilder) -> impl Future<Output = Result<CreateModelPackageGroupOutput, SdkError<CreateModelPackageGroupError>>>;
    fn create_model_quality_job_definition(&self, builder: CreateModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelQualityJobDefinitionOutput, SdkError<CreateModelQualityJobDefinitionError>>>;
    fn create_monitoring_schedule(&self, builder: CreateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<CreateMonitoringScheduleOutput, SdkError<CreateMonitoringScheduleError>>>;
    fn create_notebook_instance(&self, builder: CreateNotebookInstanceInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceOutput, SdkError<CreateNotebookInstanceError>>>;
    fn create_notebook_instance_lifecycle_config(&self, builder: CreateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceLifecycleConfigOutput, SdkError<CreateNotebookInstanceLifecycleConfigError>>>;
    fn create_optimization_job(&self, builder: CreateOptimizationJobInputBuilder) -> impl Future<Output = Result<CreateOptimizationJobOutput, SdkError<CreateOptimizationJobError>>>;
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>>;
    fn create_presigned_domain_url(&self, builder: CreatePresignedDomainUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedDomainUrlOutput, SdkError<CreatePresignedDomainUrlError>>>;
    fn create_presigned_mlflow_tracking_server_url(&self, builder: CreatePresignedMlflowTrackingServerUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedMlflowTrackingServerUrlOutput, SdkError<CreatePresignedMlflowTrackingServerUrlError>>>;
    fn create_presigned_notebook_instance_url(&self, builder: CreatePresignedNotebookInstanceUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookInstanceUrlOutput, SdkError<CreatePresignedNotebookInstanceUrlError>>>;
    fn create_processing_job(&self, builder: CreateProcessingJobInputBuilder) -> impl Future<Output = Result<CreateProcessingJobOutput, SdkError<CreateProcessingJobError>>>;
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>>;
    fn create_space(&self, builder: CreateSpaceInputBuilder) -> impl Future<Output = Result<CreateSpaceOutput, SdkError<CreateSpaceError>>>;
    fn create_studio_lifecycle_config(&self, builder: CreateStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateStudioLifecycleConfigOutput, SdkError<CreateStudioLifecycleConfigError>>>;
    fn create_training_job(&self, builder: CreateTrainingJobInputBuilder) -> impl Future<Output = Result<CreateTrainingJobOutput, SdkError<CreateTrainingJobError>>>;
    fn create_transform_job(&self, builder: CreateTransformJobInputBuilder) -> impl Future<Output = Result<CreateTransformJobOutput, SdkError<CreateTransformJobError>>>;
    fn create_trial(&self, builder: CreateTrialInputBuilder) -> impl Future<Output = Result<CreateTrialOutput, SdkError<CreateTrialError>>>;
    fn create_trial_component(&self, builder: CreateTrialComponentInputBuilder) -> impl Future<Output = Result<CreateTrialComponentOutput, SdkError<CreateTrialComponentError>>>;
    fn create_user_profile(&self, builder: CreateUserProfileInputBuilder) -> impl Future<Output = Result<CreateUserProfileOutput, SdkError<CreateUserProfileError>>>;
    fn create_workforce(&self, builder: CreateWorkforceInputBuilder) -> impl Future<Output = Result<CreateWorkforceOutput, SdkError<CreateWorkforceError>>>;
    fn create_workteam(&self, builder: CreateWorkteamInputBuilder) -> impl Future<Output = Result<CreateWorkteamOutput, SdkError<CreateWorkteamError>>>;
    fn delete_action(&self, builder: DeleteActionInputBuilder) -> impl Future<Output = Result<DeleteActionOutput, SdkError<DeleteActionError>>>;
    fn delete_algorithm(&self, builder: DeleteAlgorithmInputBuilder) -> impl Future<Output = Result<DeleteAlgorithmOutput, SdkError<DeleteAlgorithmError>>>;
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>>;
    fn delete_app_image_config(&self, builder: DeleteAppImageConfigInputBuilder) -> impl Future<Output = Result<DeleteAppImageConfigOutput, SdkError<DeleteAppImageConfigError>>>;
    fn delete_artifact(&self, builder: DeleteArtifactInputBuilder) -> impl Future<Output = Result<DeleteArtifactOutput, SdkError<DeleteArtifactError>>>;
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>>;
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>>;
    fn delete_code_repository(&self, builder: DeleteCodeRepositoryInputBuilder) -> impl Future<Output = Result<DeleteCodeRepositoryOutput, SdkError<DeleteCodeRepositoryError>>>;
    fn delete_compilation_job(&self, builder: DeleteCompilationJobInputBuilder) -> impl Future<Output = Result<DeleteCompilationJobOutput, SdkError<DeleteCompilationJobError>>>;
    fn delete_context(&self, builder: DeleteContextInputBuilder) -> impl Future<Output = Result<DeleteContextOutput, SdkError<DeleteContextError>>>;
    fn delete_data_quality_job_definition(&self, builder: DeleteDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteDataQualityJobDefinitionOutput, SdkError<DeleteDataQualityJobDefinitionError>>>;
    fn delete_device_fleet(&self, builder: DeleteDeviceFleetInputBuilder) -> impl Future<Output = Result<DeleteDeviceFleetOutput, SdkError<DeleteDeviceFleetError>>>;
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>>;
    fn delete_edge_deployment_plan(&self, builder: DeleteEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentPlanOutput, SdkError<DeleteEdgeDeploymentPlanError>>>;
    fn delete_edge_deployment_stage(&self, builder: DeleteEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentStageOutput, SdkError<DeleteEdgeDeploymentStageError>>>;
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>>;
    fn delete_endpoint_config(&self, builder: DeleteEndpointConfigInputBuilder) -> impl Future<Output = Result<DeleteEndpointConfigOutput, SdkError<DeleteEndpointConfigError>>>;
    fn delete_experiment(&self, builder: DeleteExperimentInputBuilder) -> impl Future<Output = Result<DeleteExperimentOutput, SdkError<DeleteExperimentError>>>;
    fn delete_feature_group(&self, builder: DeleteFeatureGroupInputBuilder) -> impl Future<Output = Result<DeleteFeatureGroupOutput, SdkError<DeleteFeatureGroupError>>>;
    fn delete_flow_definition(&self, builder: DeleteFlowDefinitionInputBuilder) -> impl Future<Output = Result<DeleteFlowDefinitionOutput, SdkError<DeleteFlowDefinitionError>>>;
    fn delete_hub(&self, builder: DeleteHubInputBuilder) -> impl Future<Output = Result<DeleteHubOutput, SdkError<DeleteHubError>>>;
    fn delete_hub_content(&self, builder: DeleteHubContentInputBuilder) -> impl Future<Output = Result<DeleteHubContentOutput, SdkError<DeleteHubContentError>>>;
    fn delete_hub_content_reference(&self, builder: DeleteHubContentReferenceInputBuilder) -> impl Future<Output = Result<DeleteHubContentReferenceOutput, SdkError<DeleteHubContentReferenceError>>>;
    fn delete_human_task_ui(&self, builder: DeleteHumanTaskUiInputBuilder) -> impl Future<Output = Result<DeleteHumanTaskUiOutput, SdkError<DeleteHumanTaskUiError>>>;
    fn delete_hyper_parameter_tuning_job(&self, builder: DeleteHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DeleteHyperParameterTuningJobOutput, SdkError<DeleteHyperParameterTuningJobError>>>;
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>>;
    fn delete_image_version(&self, builder: DeleteImageVersionInputBuilder) -> impl Future<Output = Result<DeleteImageVersionOutput, SdkError<DeleteImageVersionError>>>;
    fn delete_inference_component(&self, builder: DeleteInferenceComponentInputBuilder) -> impl Future<Output = Result<DeleteInferenceComponentOutput, SdkError<DeleteInferenceComponentError>>>;
    fn delete_inference_experiment(&self, builder: DeleteInferenceExperimentInputBuilder) -> impl Future<Output = Result<DeleteInferenceExperimentOutput, SdkError<DeleteInferenceExperimentError>>>;
    fn delete_mlflow_tracking_server(&self, builder: DeleteMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DeleteMlflowTrackingServerOutput, SdkError<DeleteMlflowTrackingServerError>>>;
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>>;
    fn delete_model_bias_job_definition(&self, builder: DeleteModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelBiasJobDefinitionOutput, SdkError<DeleteModelBiasJobDefinitionError>>>;
    fn delete_model_card(&self, builder: DeleteModelCardInputBuilder) -> impl Future<Output = Result<DeleteModelCardOutput, SdkError<DeleteModelCardError>>>;
    fn delete_model_explainability_job_definition(&self, builder: DeleteModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelExplainabilityJobDefinitionOutput, SdkError<DeleteModelExplainabilityJobDefinitionError>>>;
    fn delete_model_package(&self, builder: DeleteModelPackageInputBuilder) -> impl Future<Output = Result<DeleteModelPackageOutput, SdkError<DeleteModelPackageError>>>;
    fn delete_model_package_group(&self, builder: DeleteModelPackageGroupInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupOutput, SdkError<DeleteModelPackageGroupError>>>;
    fn delete_model_package_group_policy(&self, builder: DeleteModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupPolicyOutput, SdkError<DeleteModelPackageGroupPolicyError>>>;
    fn delete_model_quality_job_definition(&self, builder: DeleteModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelQualityJobDefinitionOutput, SdkError<DeleteModelQualityJobDefinitionError>>>;
    fn delete_monitoring_schedule(&self, builder: DeleteMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DeleteMonitoringScheduleOutput, SdkError<DeleteMonitoringScheduleError>>>;
    fn delete_notebook_instance(&self, builder: DeleteNotebookInstanceInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceOutput, SdkError<DeleteNotebookInstanceError>>>;
    fn delete_notebook_instance_lifecycle_config(&self, builder: DeleteNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceLifecycleConfigOutput, SdkError<DeleteNotebookInstanceLifecycleConfigError>>>;
    fn delete_optimization_job(&self, builder: DeleteOptimizationJobInputBuilder) -> impl Future<Output = Result<DeleteOptimizationJobOutput, SdkError<DeleteOptimizationJobError>>>;
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>>;
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>>;
    fn delete_space(&self, builder: DeleteSpaceInputBuilder) -> impl Future<Output = Result<DeleteSpaceOutput, SdkError<DeleteSpaceError>>>;
    fn delete_studio_lifecycle_config(&self, builder: DeleteStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteStudioLifecycleConfigOutput, SdkError<DeleteStudioLifecycleConfigError>>>;
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>>;
    fn delete_trial(&self, builder: DeleteTrialInputBuilder) -> impl Future<Output = Result<DeleteTrialOutput, SdkError<DeleteTrialError>>>;
    fn delete_trial_component(&self, builder: DeleteTrialComponentInputBuilder) -> impl Future<Output = Result<DeleteTrialComponentOutput, SdkError<DeleteTrialComponentError>>>;
    fn delete_user_profile(&self, builder: DeleteUserProfileInputBuilder) -> impl Future<Output = Result<DeleteUserProfileOutput, SdkError<DeleteUserProfileError>>>;
    fn delete_workforce(&self, builder: DeleteWorkforceInputBuilder) -> impl Future<Output = Result<DeleteWorkforceOutput, SdkError<DeleteWorkforceError>>>;
    fn delete_workteam(&self, builder: DeleteWorkteamInputBuilder) -> impl Future<Output = Result<DeleteWorkteamOutput, SdkError<DeleteWorkteamError>>>;
    fn deregister_devices(&self, builder: DeregisterDevicesInputBuilder) -> impl Future<Output = Result<DeregisterDevicesOutput, SdkError<DeregisterDevicesError>>>;
    fn describe_action(&self, builder: DescribeActionInputBuilder) -> impl Future<Output = Result<DescribeActionOutput, SdkError<DescribeActionError>>>;
    fn describe_algorithm(&self, builder: DescribeAlgorithmInputBuilder) -> impl Future<Output = Result<DescribeAlgorithmOutput, SdkError<DescribeAlgorithmError>>>;
    fn describe_app(&self, builder: DescribeAppInputBuilder) -> impl Future<Output = Result<DescribeAppOutput, SdkError<DescribeAppError>>>;
    fn describe_app_image_config(&self, builder: DescribeAppImageConfigInputBuilder) -> impl Future<Output = Result<DescribeAppImageConfigOutput, SdkError<DescribeAppImageConfigError>>>;
    fn describe_artifact(&self, builder: DescribeArtifactInputBuilder) -> impl Future<Output = Result<DescribeArtifactOutput, SdkError<DescribeArtifactError>>>;
    fn describe_auto_ml_job(&self, builder: DescribeAutoMlJobInputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobOutput, SdkError<DescribeAutoMLJobError>>>;
    fn describe_auto_ml_job_v2(&self, builder: DescribeAutoMlJobV2InputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobV2Output, SdkError<DescribeAutoMLJobV2Error>>>;
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>>;
    fn describe_cluster_node(&self, builder: DescribeClusterNodeInputBuilder) -> impl Future<Output = Result<DescribeClusterNodeOutput, SdkError<DescribeClusterNodeError>>>;
    fn describe_code_repository(&self, builder: DescribeCodeRepositoryInputBuilder) -> impl Future<Output = Result<DescribeCodeRepositoryOutput, SdkError<DescribeCodeRepositoryError>>>;
    fn describe_compilation_job(&self, builder: DescribeCompilationJobInputBuilder) -> impl Future<Output = Result<DescribeCompilationJobOutput, SdkError<DescribeCompilationJobError>>>;
    fn describe_context(&self, builder: DescribeContextInputBuilder) -> impl Future<Output = Result<DescribeContextOutput, SdkError<DescribeContextError>>>;
    fn describe_data_quality_job_definition(&self, builder: DescribeDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDataQualityJobDefinitionOutput, SdkError<DescribeDataQualityJobDefinitionError>>>;
    fn describe_device(&self, builder: DescribeDeviceInputBuilder) -> impl Future<Output = Result<DescribeDeviceOutput, SdkError<DescribeDeviceError>>>;
    fn describe_device_fleet(&self, builder: DescribeDeviceFleetInputBuilder) -> impl Future<Output = Result<DescribeDeviceFleetOutput, SdkError<DescribeDeviceFleetError>>>;
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>>;
    fn describe_edge_deployment_plan(&self, builder: DescribeEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DescribeEdgeDeploymentPlanOutput, SdkError<DescribeEdgeDeploymentPlanError>>>;
    fn describe_edge_packaging_job(&self, builder: DescribeEdgePackagingJobInputBuilder) -> impl Future<Output = Result<DescribeEdgePackagingJobOutput, SdkError<DescribeEdgePackagingJobError>>>;
    fn describe_endpoint(&self, builder: DescribeEndpointInputBuilder) -> impl Future<Output = Result<DescribeEndpointOutput, SdkError<DescribeEndpointError>>>;
    fn describe_endpoint_config(&self, builder: DescribeEndpointConfigInputBuilder) -> impl Future<Output = Result<DescribeEndpointConfigOutput, SdkError<DescribeEndpointConfigError>>>;
    fn describe_experiment(&self, builder: DescribeExperimentInputBuilder) -> impl Future<Output = Result<DescribeExperimentOutput, SdkError<DescribeExperimentError>>>;
    fn describe_feature_group(&self, builder: DescribeFeatureGroupInputBuilder) -> impl Future<Output = Result<DescribeFeatureGroupOutput, SdkError<DescribeFeatureGroupError>>>;
    fn describe_feature_metadata(&self, builder: DescribeFeatureMetadataInputBuilder) -> impl Future<Output = Result<DescribeFeatureMetadataOutput, SdkError<DescribeFeatureMetadataError>>>;
    fn describe_flow_definition(&self, builder: DescribeFlowDefinitionInputBuilder) -> impl Future<Output = Result<DescribeFlowDefinitionOutput, SdkError<DescribeFlowDefinitionError>>>;
    fn describe_hub(&self, builder: DescribeHubInputBuilder) -> impl Future<Output = Result<DescribeHubOutput, SdkError<DescribeHubError>>>;
    fn describe_hub_content(&self, builder: DescribeHubContentInputBuilder) -> impl Future<Output = Result<DescribeHubContentOutput, SdkError<DescribeHubContentError>>>;
    fn describe_human_task_ui(&self, builder: DescribeHumanTaskUiInputBuilder) -> impl Future<Output = Result<DescribeHumanTaskUiOutput, SdkError<DescribeHumanTaskUiError>>>;
    fn describe_hyper_parameter_tuning_job(&self, builder: DescribeHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DescribeHyperParameterTuningJobOutput, SdkError<DescribeHyperParameterTuningJobError>>>;
    fn describe_image(&self, builder: DescribeImageInputBuilder) -> impl Future<Output = Result<DescribeImageOutput, SdkError<DescribeImageError>>>;
    fn describe_image_version(&self, builder: DescribeImageVersionInputBuilder) -> impl Future<Output = Result<DescribeImageVersionOutput, SdkError<DescribeImageVersionError>>>;
    fn describe_inference_component(&self, builder: DescribeInferenceComponentInputBuilder) -> impl Future<Output = Result<DescribeInferenceComponentOutput, SdkError<DescribeInferenceComponentError>>>;
    fn describe_inference_experiment(&self, builder: DescribeInferenceExperimentInputBuilder) -> impl Future<Output = Result<DescribeInferenceExperimentOutput, SdkError<DescribeInferenceExperimentError>>>;
    fn describe_inference_recommendations_job(&self, builder: DescribeInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<DescribeInferenceRecommendationsJobOutput, SdkError<DescribeInferenceRecommendationsJobError>>>;
    fn describe_labeling_job(&self, builder: DescribeLabelingJobInputBuilder) -> impl Future<Output = Result<DescribeLabelingJobOutput, SdkError<DescribeLabelingJobError>>>;
    fn describe_lineage_group(&self, builder: DescribeLineageGroupInputBuilder) -> impl Future<Output = Result<DescribeLineageGroupOutput, SdkError<DescribeLineageGroupError>>>;
    fn describe_mlflow_tracking_server(&self, builder: DescribeMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DescribeMlflowTrackingServerOutput, SdkError<DescribeMlflowTrackingServerError>>>;
    fn describe_model(&self, builder: DescribeModelInputBuilder) -> impl Future<Output = Result<DescribeModelOutput, SdkError<DescribeModelError>>>;
    fn describe_model_bias_job_definition(&self, builder: DescribeModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelBiasJobDefinitionOutput, SdkError<DescribeModelBiasJobDefinitionError>>>;
    fn describe_model_card(&self, builder: DescribeModelCardInputBuilder) -> impl Future<Output = Result<DescribeModelCardOutput, SdkError<DescribeModelCardError>>>;
    fn describe_model_card_export_job(&self, builder: DescribeModelCardExportJobInputBuilder) -> impl Future<Output = Result<DescribeModelCardExportJobOutput, SdkError<DescribeModelCardExportJobError>>>;
    fn describe_model_explainability_job_definition(&self, builder: DescribeModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelExplainabilityJobDefinitionOutput, SdkError<DescribeModelExplainabilityJobDefinitionError>>>;
    fn describe_model_package(&self, builder: DescribeModelPackageInputBuilder) -> impl Future<Output = Result<DescribeModelPackageOutput, SdkError<DescribeModelPackageError>>>;
    fn describe_model_package_group(&self, builder: DescribeModelPackageGroupInputBuilder) -> impl Future<Output = Result<DescribeModelPackageGroupOutput, SdkError<DescribeModelPackageGroupError>>>;
    fn describe_model_quality_job_definition(&self, builder: DescribeModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelQualityJobDefinitionOutput, SdkError<DescribeModelQualityJobDefinitionError>>>;
    fn describe_monitoring_schedule(&self, builder: DescribeMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DescribeMonitoringScheduleOutput, SdkError<DescribeMonitoringScheduleError>>>;
    fn describe_notebook_instance(&self, builder: DescribeNotebookInstanceInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceOutput, SdkError<DescribeNotebookInstanceError>>>;
    fn describe_notebook_instance_lifecycle_config(&self, builder: DescribeNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceLifecycleConfigOutput, SdkError<DescribeNotebookInstanceLifecycleConfigError>>>;
    fn describe_optimization_job(&self, builder: DescribeOptimizationJobInputBuilder) -> impl Future<Output = Result<DescribeOptimizationJobOutput, SdkError<DescribeOptimizationJobError>>>;
    fn describe_pipeline(&self, builder: DescribePipelineInputBuilder) -> impl Future<Output = Result<DescribePipelineOutput, SdkError<DescribePipelineError>>>;
    fn describe_pipeline_definition_for_execution(&self, builder: DescribePipelineDefinitionForExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineDefinitionForExecutionOutput, SdkError<DescribePipelineDefinitionForExecutionError>>>;
    fn describe_pipeline_execution(&self, builder: DescribePipelineExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineExecutionOutput, SdkError<DescribePipelineExecutionError>>>;
    fn describe_processing_job(&self, builder: DescribeProcessingJobInputBuilder) -> impl Future<Output = Result<DescribeProcessingJobOutput, SdkError<DescribeProcessingJobError>>>;
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>>;
    fn describe_space(&self, builder: DescribeSpaceInputBuilder) -> impl Future<Output = Result<DescribeSpaceOutput, SdkError<DescribeSpaceError>>>;
    fn describe_studio_lifecycle_config(&self, builder: DescribeStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeStudioLifecycleConfigOutput, SdkError<DescribeStudioLifecycleConfigError>>>;
    fn describe_subscribed_workteam(&self, builder: DescribeSubscribedWorkteamInputBuilder) -> impl Future<Output = Result<DescribeSubscribedWorkteamOutput, SdkError<DescribeSubscribedWorkteamError>>>;
    fn describe_training_job(&self, builder: DescribeTrainingJobInputBuilder) -> impl Future<Output = Result<DescribeTrainingJobOutput, SdkError<DescribeTrainingJobError>>>;
    fn describe_transform_job(&self, builder: DescribeTransformJobInputBuilder) -> impl Future<Output = Result<DescribeTransformJobOutput, SdkError<DescribeTransformJobError>>>;
    fn describe_trial(&self, builder: DescribeTrialInputBuilder) -> impl Future<Output = Result<DescribeTrialOutput, SdkError<DescribeTrialError>>>;
    fn describe_trial_component(&self, builder: DescribeTrialComponentInputBuilder) -> impl Future<Output = Result<DescribeTrialComponentOutput, SdkError<DescribeTrialComponentError>>>;
    fn describe_user_profile(&self, builder: DescribeUserProfileInputBuilder) -> impl Future<Output = Result<DescribeUserProfileOutput, SdkError<DescribeUserProfileError>>>;
    fn describe_workforce(&self, builder: DescribeWorkforceInputBuilder) -> impl Future<Output = Result<DescribeWorkforceOutput, SdkError<DescribeWorkforceError>>>;
    fn describe_workteam(&self, builder: DescribeWorkteamInputBuilder) -> impl Future<Output = Result<DescribeWorkteamOutput, SdkError<DescribeWorkteamError>>>;
    fn disable_sagemaker_servicecatalog_portfolio(&self, builder: DisableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<DisableSagemakerServicecatalogPortfolioOutput, SdkError<DisableSagemakerServicecatalogPortfolioError>>>;
    fn disassociate_trial_component(&self, builder: DisassociateTrialComponentInputBuilder) -> impl Future<Output = Result<DisassociateTrialComponentOutput, SdkError<DisassociateTrialComponentError>>>;
    fn enable_sagemaker_servicecatalog_portfolio(&self, builder: EnableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<EnableSagemakerServicecatalogPortfolioOutput, SdkError<EnableSagemakerServicecatalogPortfolioError>>>;
    fn get_device_fleet_report(&self, builder: GetDeviceFleetReportInputBuilder) -> impl Future<Output = Result<GetDeviceFleetReportOutput, SdkError<GetDeviceFleetReportError>>>;
    fn get_lineage_group_policy(&self, builder: GetLineageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetLineageGroupPolicyOutput, SdkError<GetLineageGroupPolicyError>>>;
    fn get_model_package_group_policy(&self, builder: GetModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetModelPackageGroupPolicyOutput, SdkError<GetModelPackageGroupPolicyError>>>;
    fn get_sagemaker_servicecatalog_portfolio_status(&self, builder: GetSagemakerServicecatalogPortfolioStatusInputBuilder) -> impl Future<Output = Result<GetSagemakerServicecatalogPortfolioStatusOutput, SdkError<GetSagemakerServicecatalogPortfolioStatusError>>>;
    fn get_scaling_configuration_recommendation(&self, builder: GetScalingConfigurationRecommendationInputBuilder) -> impl Future<Output = Result<GetScalingConfigurationRecommendationOutput, SdkError<GetScalingConfigurationRecommendationError>>>;
    fn get_search_suggestions(&self, builder: GetSearchSuggestionsInputBuilder) -> impl Future<Output = Result<GetSearchSuggestionsOutput, SdkError<GetSearchSuggestionsError>>>;
    fn import_hub_content(&self, builder: ImportHubContentInputBuilder) -> impl Future<Output = Result<ImportHubContentOutput, SdkError<ImportHubContentError>>>;
    fn list_actions(&self, builder: ListActionsInputBuilder) -> impl Future<Output = Result<ListActionsOutput, SdkError<ListActionsError>>>;
    fn list_algorithms(&self, builder: ListAlgorithmsInputBuilder) -> impl Future<Output = Result<ListAlgorithmsOutput, SdkError<ListAlgorithmsError>>>;
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>>;
    fn list_app_image_configs(&self, builder: ListAppImageConfigsInputBuilder) -> impl Future<Output = Result<ListAppImageConfigsOutput, SdkError<ListAppImageConfigsError>>>;
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>>;
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>>;
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>>;
    fn list_auto_ml_jobs(&self, builder: ListAutoMlJobsInputBuilder) -> impl Future<Output = Result<ListAutoMlJobsOutput, SdkError<ListAutoMLJobsError>>>;
    fn list_candidates_for_auto_ml_job(&self, builder: ListCandidatesForAutoMlJobInputBuilder) -> impl Future<Output = Result<ListCandidatesForAutoMlJobOutput, SdkError<ListCandidatesForAutoMLJobError>>>;
    fn list_cluster_nodes(&self, builder: ListClusterNodesInputBuilder) -> impl Future<Output = Result<ListClusterNodesOutput, SdkError<ListClusterNodesError>>>;
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>>;
    fn list_code_repositories(&self, builder: ListCodeRepositoriesInputBuilder) -> impl Future<Output = Result<ListCodeRepositoriesOutput, SdkError<ListCodeRepositoriesError>>>;
    fn list_compilation_jobs(&self, builder: ListCompilationJobsInputBuilder) -> impl Future<Output = Result<ListCompilationJobsOutput, SdkError<ListCompilationJobsError>>>;
    fn list_contexts(&self, builder: ListContextsInputBuilder) -> impl Future<Output = Result<ListContextsOutput, SdkError<ListContextsError>>>;
    fn list_data_quality_job_definitions(&self, builder: ListDataQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListDataQualityJobDefinitionsOutput, SdkError<ListDataQualityJobDefinitionsError>>>;
    fn list_device_fleets(&self, builder: ListDeviceFleetsInputBuilder) -> impl Future<Output = Result<ListDeviceFleetsOutput, SdkError<ListDeviceFleetsError>>>;
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>>;
    fn list_domains(&self, builder: ListDomainsInputBuilder) -> impl Future<Output = Result<ListDomainsOutput, SdkError<ListDomainsError>>>;
    fn list_edge_deployment_plans(&self, builder: ListEdgeDeploymentPlansInputBuilder) -> impl Future<Output = Result<ListEdgeDeploymentPlansOutput, SdkError<ListEdgeDeploymentPlansError>>>;
    fn list_edge_packaging_jobs(&self, builder: ListEdgePackagingJobsInputBuilder) -> impl Future<Output = Result<ListEdgePackagingJobsOutput, SdkError<ListEdgePackagingJobsError>>>;
    fn list_endpoint_configs(&self, builder: ListEndpointConfigsInputBuilder) -> impl Future<Output = Result<ListEndpointConfigsOutput, SdkError<ListEndpointConfigsError>>>;
    fn list_endpoints(&self, builder: ListEndpointsInputBuilder) -> impl Future<Output = Result<ListEndpointsOutput, SdkError<ListEndpointsError>>>;
    fn list_experiments(&self, builder: ListExperimentsInputBuilder) -> impl Future<Output = Result<ListExperimentsOutput, SdkError<ListExperimentsError>>>;
    fn list_feature_groups(&self, builder: ListFeatureGroupsInputBuilder) -> impl Future<Output = Result<ListFeatureGroupsOutput, SdkError<ListFeatureGroupsError>>>;
    fn list_flow_definitions(&self, builder: ListFlowDefinitionsInputBuilder) -> impl Future<Output = Result<ListFlowDefinitionsOutput, SdkError<ListFlowDefinitionsError>>>;
    fn list_hub_content_versions(&self, builder: ListHubContentVersionsInputBuilder) -> impl Future<Output = Result<ListHubContentVersionsOutput, SdkError<ListHubContentVersionsError>>>;
    fn list_hub_contents(&self, builder: ListHubContentsInputBuilder) -> impl Future<Output = Result<ListHubContentsOutput, SdkError<ListHubContentsError>>>;
    fn list_hubs(&self, builder: ListHubsInputBuilder) -> impl Future<Output = Result<ListHubsOutput, SdkError<ListHubsError>>>;
    fn list_human_task_uis(&self, builder: ListHumanTaskUisInputBuilder) -> impl Future<Output = Result<ListHumanTaskUisOutput, SdkError<ListHumanTaskUisError>>>;
    fn list_hyper_parameter_tuning_jobs(&self, builder: ListHyperParameterTuningJobsInputBuilder) -> impl Future<Output = Result<ListHyperParameterTuningJobsOutput, SdkError<ListHyperParameterTuningJobsError>>>;
    fn list_image_versions(&self, builder: ListImageVersionsInputBuilder) -> impl Future<Output = Result<ListImageVersionsOutput, SdkError<ListImageVersionsError>>>;
    fn list_images(&self, builder: ListImagesInputBuilder) -> impl Future<Output = Result<ListImagesOutput, SdkError<ListImagesError>>>;
    fn list_inference_components(&self, builder: ListInferenceComponentsInputBuilder) -> impl Future<Output = Result<ListInferenceComponentsOutput, SdkError<ListInferenceComponentsError>>>;
    fn list_inference_experiments(&self, builder: ListInferenceExperimentsInputBuilder) -> impl Future<Output = Result<ListInferenceExperimentsOutput, SdkError<ListInferenceExperimentsError>>>;
    fn list_inference_recommendations_job_steps(&self, builder: ListInferenceRecommendationsJobStepsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobStepsOutput, SdkError<ListInferenceRecommendationsJobStepsError>>>;
    fn list_inference_recommendations_jobs(&self, builder: ListInferenceRecommendationsJobsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobsOutput, SdkError<ListInferenceRecommendationsJobsError>>>;
    fn list_labeling_jobs(&self, builder: ListLabelingJobsInputBuilder) -> impl Future<Output = Result<ListLabelingJobsOutput, SdkError<ListLabelingJobsError>>>;
    fn list_labeling_jobs_for_workteam(&self, builder: ListLabelingJobsForWorkteamInputBuilder) -> impl Future<Output = Result<ListLabelingJobsForWorkteamOutput, SdkError<ListLabelingJobsForWorkteamError>>>;
    fn list_lineage_groups(&self, builder: ListLineageGroupsInputBuilder) -> impl Future<Output = Result<ListLineageGroupsOutput, SdkError<ListLineageGroupsError>>>;
    fn list_mlflow_tracking_servers(&self, builder: ListMlflowTrackingServersInputBuilder) -> impl Future<Output = Result<ListMlflowTrackingServersOutput, SdkError<ListMlflowTrackingServersError>>>;
    fn list_model_bias_job_definitions(&self, builder: ListModelBiasJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelBiasJobDefinitionsOutput, SdkError<ListModelBiasJobDefinitionsError>>>;
    fn list_model_card_export_jobs(&self, builder: ListModelCardExportJobsInputBuilder) -> impl Future<Output = Result<ListModelCardExportJobsOutput, SdkError<ListModelCardExportJobsError>>>;
    fn list_model_card_versions(&self, builder: ListModelCardVersionsInputBuilder) -> impl Future<Output = Result<ListModelCardVersionsOutput, SdkError<ListModelCardVersionsError>>>;
    fn list_model_cards(&self, builder: ListModelCardsInputBuilder) -> impl Future<Output = Result<ListModelCardsOutput, SdkError<ListModelCardsError>>>;
    fn list_model_explainability_job_definitions(&self, builder: ListModelExplainabilityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelExplainabilityJobDefinitionsOutput, SdkError<ListModelExplainabilityJobDefinitionsError>>>;
    fn list_model_metadata(&self, builder: ListModelMetadataInputBuilder) -> impl Future<Output = Result<ListModelMetadataOutput, SdkError<ListModelMetadataError>>>;
    fn list_model_package_groups(&self, builder: ListModelPackageGroupsInputBuilder) -> impl Future<Output = Result<ListModelPackageGroupsOutput, SdkError<ListModelPackageGroupsError>>>;
    fn list_model_packages(&self, builder: ListModelPackagesInputBuilder) -> impl Future<Output = Result<ListModelPackagesOutput, SdkError<ListModelPackagesError>>>;
    fn list_model_quality_job_definitions(&self, builder: ListModelQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelQualityJobDefinitionsOutput, SdkError<ListModelQualityJobDefinitionsError>>>;
    fn list_models(&self, builder: ListModelsInputBuilder) -> impl Future<Output = Result<ListModelsOutput, SdkError<ListModelsError>>>;
    fn list_monitoring_alert_history(&self, builder: ListMonitoringAlertHistoryInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertHistoryOutput, SdkError<ListMonitoringAlertHistoryError>>>;
    fn list_monitoring_alerts(&self, builder: ListMonitoringAlertsInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertsOutput, SdkError<ListMonitoringAlertsError>>>;
    fn list_monitoring_executions(&self, builder: ListMonitoringExecutionsInputBuilder) -> impl Future<Output = Result<ListMonitoringExecutionsOutput, SdkError<ListMonitoringExecutionsError>>>;
    fn list_monitoring_schedules(&self, builder: ListMonitoringSchedulesInputBuilder) -> impl Future<Output = Result<ListMonitoringSchedulesOutput, SdkError<ListMonitoringSchedulesError>>>;
    fn list_notebook_instance_lifecycle_configs(&self, builder: ListNotebookInstanceLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListNotebookInstanceLifecycleConfigsOutput, SdkError<ListNotebookInstanceLifecycleConfigsError>>>;
    fn list_notebook_instances(&self, builder: ListNotebookInstancesInputBuilder) -> impl Future<Output = Result<ListNotebookInstancesOutput, SdkError<ListNotebookInstancesError>>>;
    fn list_optimization_jobs(&self, builder: ListOptimizationJobsInputBuilder) -> impl Future<Output = Result<ListOptimizationJobsOutput, SdkError<ListOptimizationJobsError>>>;
    fn list_pipeline_execution_steps(&self, builder: ListPipelineExecutionStepsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionStepsOutput, SdkError<ListPipelineExecutionStepsError>>>;
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>>;
    fn list_pipeline_parameters_for_execution(&self, builder: ListPipelineParametersForExecutionInputBuilder) -> impl Future<Output = Result<ListPipelineParametersForExecutionOutput, SdkError<ListPipelineParametersForExecutionError>>>;
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>>;
    fn list_processing_jobs(&self, builder: ListProcessingJobsInputBuilder) -> impl Future<Output = Result<ListProcessingJobsOutput, SdkError<ListProcessingJobsError>>>;
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>>;
    fn list_resource_catalogs(&self, builder: ListResourceCatalogsInputBuilder) -> impl Future<Output = Result<ListResourceCatalogsOutput, SdkError<ListResourceCatalogsError>>>;
    fn list_spaces(&self, builder: ListSpacesInputBuilder) -> impl Future<Output = Result<ListSpacesOutput, SdkError<ListSpacesError>>>;
    fn list_stage_devices(&self, builder: ListStageDevicesInputBuilder) -> impl Future<Output = Result<ListStageDevicesOutput, SdkError<ListStageDevicesError>>>;
    fn list_studio_lifecycle_configs(&self, builder: ListStudioLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListStudioLifecycleConfigsOutput, SdkError<ListStudioLifecycleConfigsError>>>;
    fn list_subscribed_workteams(&self, builder: ListSubscribedWorkteamsInputBuilder) -> impl Future<Output = Result<ListSubscribedWorkteamsOutput, SdkError<ListSubscribedWorkteamsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn list_training_jobs(&self, builder: ListTrainingJobsInputBuilder) -> impl Future<Output = Result<ListTrainingJobsOutput, SdkError<ListTrainingJobsError>>>;
    fn list_training_jobs_for_hyper_parameter_tuning_job(&self, builder: ListTrainingJobsForHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<ListTrainingJobsForHyperParameterTuningJobOutput, SdkError<ListTrainingJobsForHyperParameterTuningJobError>>>;
    fn list_transform_jobs(&self, builder: ListTransformJobsInputBuilder) -> impl Future<Output = Result<ListTransformJobsOutput, SdkError<ListTransformJobsError>>>;
    fn list_trial_components(&self, builder: ListTrialComponentsInputBuilder) -> impl Future<Output = Result<ListTrialComponentsOutput, SdkError<ListTrialComponentsError>>>;
    fn list_trials(&self, builder: ListTrialsInputBuilder) -> impl Future<Output = Result<ListTrialsOutput, SdkError<ListTrialsError>>>;
    fn list_user_profiles(&self, builder: ListUserProfilesInputBuilder) -> impl Future<Output = Result<ListUserProfilesOutput, SdkError<ListUserProfilesError>>>;
    fn list_workforces(&self, builder: ListWorkforcesInputBuilder) -> impl Future<Output = Result<ListWorkforcesOutput, SdkError<ListWorkforcesError>>>;
    fn list_workteams(&self, builder: ListWorkteamsInputBuilder) -> impl Future<Output = Result<ListWorkteamsOutput, SdkError<ListWorkteamsError>>>;
    fn put_model_package_group_policy(&self, builder: PutModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<PutModelPackageGroupPolicyOutput, SdkError<PutModelPackageGroupPolicyError>>>;
    fn query_lineage(&self, builder: QueryLineageInputBuilder) -> impl Future<Output = Result<QueryLineageOutput, SdkError<QueryLineageError>>>;
    fn register_devices(&self, builder: RegisterDevicesInputBuilder) -> impl Future<Output = Result<RegisterDevicesOutput, SdkError<RegisterDevicesError>>>;
    fn render_ui_template(&self, builder: RenderUiTemplateInputBuilder) -> impl Future<Output = Result<RenderUiTemplateOutput, SdkError<RenderUiTemplateError>>>;
    fn retry_pipeline_execution(&self, builder: RetryPipelineExecutionInputBuilder) -> impl Future<Output = Result<RetryPipelineExecutionOutput, SdkError<RetryPipelineExecutionError>>>;
    fn search(&self, builder: SearchInputBuilder) -> impl Future<Output = Result<SearchOutput, SdkError<SearchError>>>;
    fn send_pipeline_execution_step_failure(&self, builder: SendPipelineExecutionStepFailureInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepFailureOutput, SdkError<SendPipelineExecutionStepFailureError>>>;
    fn send_pipeline_execution_step_success(&self, builder: SendPipelineExecutionStepSuccessInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepSuccessOutput, SdkError<SendPipelineExecutionStepSuccessError>>>;
    fn start_edge_deployment_stage(&self, builder: StartEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StartEdgeDeploymentStageOutput, SdkError<StartEdgeDeploymentStageError>>>;
    fn start_inference_experiment(&self, builder: StartInferenceExperimentInputBuilder) -> impl Future<Output = Result<StartInferenceExperimentOutput, SdkError<StartInferenceExperimentError>>>;
    fn start_mlflow_tracking_server(&self, builder: StartMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StartMlflowTrackingServerOutput, SdkError<StartMlflowTrackingServerError>>>;
    fn start_monitoring_schedule(&self, builder: StartMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StartMonitoringScheduleOutput, SdkError<StartMonitoringScheduleError>>>;
    fn start_notebook_instance(&self, builder: StartNotebookInstanceInputBuilder) -> impl Future<Output = Result<StartNotebookInstanceOutput, SdkError<StartNotebookInstanceError>>>;
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>>;
    fn stop_auto_ml_job(&self, builder: StopAutoMlJobInputBuilder) -> impl Future<Output = Result<StopAutoMlJobOutput, SdkError<StopAutoMLJobError>>>;
    fn stop_compilation_job(&self, builder: StopCompilationJobInputBuilder) -> impl Future<Output = Result<StopCompilationJobOutput, SdkError<StopCompilationJobError>>>;
    fn stop_edge_deployment_stage(&self, builder: StopEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StopEdgeDeploymentStageOutput, SdkError<StopEdgeDeploymentStageError>>>;
    fn stop_edge_packaging_job(&self, builder: StopEdgePackagingJobInputBuilder) -> impl Future<Output = Result<StopEdgePackagingJobOutput, SdkError<StopEdgePackagingJobError>>>;
    fn stop_hyper_parameter_tuning_job(&self, builder: StopHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<StopHyperParameterTuningJobOutput, SdkError<StopHyperParameterTuningJobError>>>;
    fn stop_inference_experiment(&self, builder: StopInferenceExperimentInputBuilder) -> impl Future<Output = Result<StopInferenceExperimentOutput, SdkError<StopInferenceExperimentError>>>;
    fn stop_inference_recommendations_job(&self, builder: StopInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<StopInferenceRecommendationsJobOutput, SdkError<StopInferenceRecommendationsJobError>>>;
    fn stop_labeling_job(&self, builder: StopLabelingJobInputBuilder) -> impl Future<Output = Result<StopLabelingJobOutput, SdkError<StopLabelingJobError>>>;
    fn stop_mlflow_tracking_server(&self, builder: StopMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StopMlflowTrackingServerOutput, SdkError<StopMlflowTrackingServerError>>>;
    fn stop_monitoring_schedule(&self, builder: StopMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StopMonitoringScheduleOutput, SdkError<StopMonitoringScheduleError>>>;
    fn stop_notebook_instance(&self, builder: StopNotebookInstanceInputBuilder) -> impl Future<Output = Result<StopNotebookInstanceOutput, SdkError<StopNotebookInstanceError>>>;
    fn stop_optimization_job(&self, builder: StopOptimizationJobInputBuilder) -> impl Future<Output = Result<StopOptimizationJobOutput, SdkError<StopOptimizationJobError>>>;
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>>;
    fn stop_processing_job(&self, builder: StopProcessingJobInputBuilder) -> impl Future<Output = Result<StopProcessingJobOutput, SdkError<StopProcessingJobError>>>;
    fn stop_training_job(&self, builder: StopTrainingJobInputBuilder) -> impl Future<Output = Result<StopTrainingJobOutput, SdkError<StopTrainingJobError>>>;
    fn stop_transform_job(&self, builder: StopTransformJobInputBuilder) -> impl Future<Output = Result<StopTransformJobOutput, SdkError<StopTransformJobError>>>;
    fn update_action(&self, builder: UpdateActionInputBuilder) -> impl Future<Output = Result<UpdateActionOutput, SdkError<UpdateActionError>>>;
    fn update_app_image_config(&self, builder: UpdateAppImageConfigInputBuilder) -> impl Future<Output = Result<UpdateAppImageConfigOutput, SdkError<UpdateAppImageConfigError>>>;
    fn update_artifact(&self, builder: UpdateArtifactInputBuilder) -> impl Future<Output = Result<UpdateArtifactOutput, SdkError<UpdateArtifactError>>>;
    fn update_cluster(&self, builder: UpdateClusterInputBuilder) -> impl Future<Output = Result<UpdateClusterOutput, SdkError<UpdateClusterError>>>;
    fn update_cluster_software(&self, builder: UpdateClusterSoftwareInputBuilder) -> impl Future<Output = Result<UpdateClusterSoftwareOutput, SdkError<UpdateClusterSoftwareError>>>;
    fn update_code_repository(&self, builder: UpdateCodeRepositoryInputBuilder) -> impl Future<Output = Result<UpdateCodeRepositoryOutput, SdkError<UpdateCodeRepositoryError>>>;
    fn update_context(&self, builder: UpdateContextInputBuilder) -> impl Future<Output = Result<UpdateContextOutput, SdkError<UpdateContextError>>>;
    fn update_device_fleet(&self, builder: UpdateDeviceFleetInputBuilder) -> impl Future<Output = Result<UpdateDeviceFleetOutput, SdkError<UpdateDeviceFleetError>>>;
    fn update_devices(&self, builder: UpdateDevicesInputBuilder) -> impl Future<Output = Result<UpdateDevicesOutput, SdkError<UpdateDevicesError>>>;
    fn update_domain(&self, builder: UpdateDomainInputBuilder) -> impl Future<Output = Result<UpdateDomainOutput, SdkError<UpdateDomainError>>>;
    fn update_endpoint(&self, builder: UpdateEndpointInputBuilder) -> impl Future<Output = Result<UpdateEndpointOutput, SdkError<UpdateEndpointError>>>;
    fn update_endpoint_weights_and_capacities(&self, builder: UpdateEndpointWeightsAndCapacitiesInputBuilder) -> impl Future<Output = Result<UpdateEndpointWeightsAndCapacitiesOutput, SdkError<UpdateEndpointWeightsAndCapacitiesError>>>;
    fn update_experiment(&self, builder: UpdateExperimentInputBuilder) -> impl Future<Output = Result<UpdateExperimentOutput, SdkError<UpdateExperimentError>>>;
    fn update_feature_group(&self, builder: UpdateFeatureGroupInputBuilder) -> impl Future<Output = Result<UpdateFeatureGroupOutput, SdkError<UpdateFeatureGroupError>>>;
    fn update_feature_metadata(&self, builder: UpdateFeatureMetadataInputBuilder) -> impl Future<Output = Result<UpdateFeatureMetadataOutput, SdkError<UpdateFeatureMetadataError>>>;
    fn update_hub(&self, builder: UpdateHubInputBuilder) -> impl Future<Output = Result<UpdateHubOutput, SdkError<UpdateHubError>>>;
    fn update_image(&self, builder: UpdateImageInputBuilder) -> impl Future<Output = Result<UpdateImageOutput, SdkError<UpdateImageError>>>;
    fn update_image_version(&self, builder: UpdateImageVersionInputBuilder) -> impl Future<Output = Result<UpdateImageVersionOutput, SdkError<UpdateImageVersionError>>>;
    fn update_inference_component(&self, builder: UpdateInferenceComponentInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentOutput, SdkError<UpdateInferenceComponentError>>>;
    fn update_inference_component_runtime_config(&self, builder: UpdateInferenceComponentRuntimeConfigInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentRuntimeConfigOutput, SdkError<UpdateInferenceComponentRuntimeConfigError>>>;
    fn update_inference_experiment(&self, builder: UpdateInferenceExperimentInputBuilder) -> impl Future<Output = Result<UpdateInferenceExperimentOutput, SdkError<UpdateInferenceExperimentError>>>;
    fn update_mlflow_tracking_server(&self, builder: UpdateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<UpdateMlflowTrackingServerOutput, SdkError<UpdateMlflowTrackingServerError>>>;
    fn update_model_card(&self, builder: UpdateModelCardInputBuilder) -> impl Future<Output = Result<UpdateModelCardOutput, SdkError<UpdateModelCardError>>>;
    fn update_model_package(&self, builder: UpdateModelPackageInputBuilder) -> impl Future<Output = Result<UpdateModelPackageOutput, SdkError<UpdateModelPackageError>>>;
    fn update_monitoring_alert(&self, builder: UpdateMonitoringAlertInputBuilder) -> impl Future<Output = Result<UpdateMonitoringAlertOutput, SdkError<UpdateMonitoringAlertError>>>;
    fn update_monitoring_schedule(&self, builder: UpdateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<UpdateMonitoringScheduleOutput, SdkError<UpdateMonitoringScheduleError>>>;
    fn update_notebook_instance(&self, builder: UpdateNotebookInstanceInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceOutput, SdkError<UpdateNotebookInstanceError>>>;
    fn update_notebook_instance_lifecycle_config(&self, builder: UpdateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceLifecycleConfigOutput, SdkError<UpdateNotebookInstanceLifecycleConfigError>>>;
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>>;
    fn update_pipeline_execution(&self, builder: UpdatePipelineExecutionInputBuilder) -> impl Future<Output = Result<UpdatePipelineExecutionOutput, SdkError<UpdatePipelineExecutionError>>>;
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>>;
    fn update_space(&self, builder: UpdateSpaceInputBuilder) -> impl Future<Output = Result<UpdateSpaceOutput, SdkError<UpdateSpaceError>>>;
    fn update_training_job(&self, builder: UpdateTrainingJobInputBuilder) -> impl Future<Output = Result<UpdateTrainingJobOutput, SdkError<UpdateTrainingJobError>>>;
    fn update_trial(&self, builder: UpdateTrialInputBuilder) -> impl Future<Output = Result<UpdateTrialOutput, SdkError<UpdateTrialError>>>;
    fn update_trial_component(&self, builder: UpdateTrialComponentInputBuilder) -> impl Future<Output = Result<UpdateTrialComponentOutput, SdkError<UpdateTrialComponentError>>>;
    fn update_user_profile(&self, builder: UpdateUserProfileInputBuilder) -> impl Future<Output = Result<UpdateUserProfileOutput, SdkError<UpdateUserProfileError>>>;
    fn update_workforce(&self, builder: UpdateWorkforceInputBuilder) -> impl Future<Output = Result<UpdateWorkforceOutput, SdkError<UpdateWorkforceError>>>;
    fn update_workteam(&self, builder: UpdateWorkteamInputBuilder) -> impl Future<Output = Result<UpdateWorkteamOutput, SdkError<UpdateWorkteamError>>>;
}
impl SageMakerClient for SageMakerClientImpl {
    fn add_association(&self, builder: AddAssociationInputBuilder) -> impl Future<Output = Result<AddAssociationOutput, SdkError<AddAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        builder.send_with(&self.0)
    }
    fn associate_trial_component(&self, builder: AssociateTrialComponentInputBuilder) -> impl Future<Output = Result<AssociateTrialComponentOutput, SdkError<AssociateTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn batch_describe_model_package(&self, builder: BatchDescribeModelPackageInputBuilder) -> impl Future<Output = Result<BatchDescribeModelPackageOutput, SdkError<BatchDescribeModelPackageError>>> {
        builder.send_with(&self.0)
    }
    fn create_action(&self, builder: CreateActionInputBuilder) -> impl Future<Output = Result<CreateActionOutput, SdkError<CreateActionError>>> {
        builder.send_with(&self.0)
    }
    fn create_algorithm(&self, builder: CreateAlgorithmInputBuilder) -> impl Future<Output = Result<CreateAlgorithmOutput, SdkError<CreateAlgorithmError>>> {
        builder.send_with(&self.0)
    }
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>> {
        builder.send_with(&self.0)
    }
    fn create_app_image_config(&self, builder: CreateAppImageConfigInputBuilder) -> impl Future<Output = Result<CreateAppImageConfigOutput, SdkError<CreateAppImageConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_artifact(&self, builder: CreateArtifactInputBuilder) -> impl Future<Output = Result<CreateArtifactOutput, SdkError<CreateArtifactError>>> {
        builder.send_with(&self.0)
    }
    fn create_auto_ml_job(&self, builder: CreateAutoMlJobInputBuilder) -> impl Future<Output = Result<CreateAutoMlJobOutput, SdkError<CreateAutoMLJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_auto_ml_job_v2(&self, builder: CreateAutoMlJobV2InputBuilder) -> impl Future<Output = Result<CreateAutoMlJobV2Output, SdkError<CreateAutoMLJobV2Error>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_code_repository(&self, builder: CreateCodeRepositoryInputBuilder) -> impl Future<Output = Result<CreateCodeRepositoryOutput, SdkError<CreateCodeRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn create_compilation_job(&self, builder: CreateCompilationJobInputBuilder) -> impl Future<Output = Result<CreateCompilationJobOutput, SdkError<CreateCompilationJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_context(&self, builder: CreateContextInputBuilder) -> impl Future<Output = Result<CreateContextOutput, SdkError<CreateContextError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_quality_job_definition(&self, builder: CreateDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateDataQualityJobDefinitionOutput, SdkError<CreateDataQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_device_fleet(&self, builder: CreateDeviceFleetInputBuilder) -> impl Future<Output = Result<CreateDeviceFleetOutput, SdkError<CreateDeviceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>> {
        builder.send_with(&self.0)
    }
    fn create_edge_deployment_plan(&self, builder: CreateEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentPlanOutput, SdkError<CreateEdgeDeploymentPlanError>>> {
        builder.send_with(&self.0)
    }
    fn create_edge_deployment_stage(&self, builder: CreateEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentStageOutput, SdkError<CreateEdgeDeploymentStageError>>> {
        builder.send_with(&self.0)
    }
    fn create_edge_packaging_job(&self, builder: CreateEdgePackagingJobInputBuilder) -> impl Future<Output = Result<CreateEdgePackagingJobOutput, SdkError<CreateEdgePackagingJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_endpoint(&self, builder: CreateEndpointInputBuilder) -> impl Future<Output = Result<CreateEndpointOutput, SdkError<CreateEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_endpoint_config(&self, builder: CreateEndpointConfigInputBuilder) -> impl Future<Output = Result<CreateEndpointConfigOutput, SdkError<CreateEndpointConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_experiment(&self, builder: CreateExperimentInputBuilder) -> impl Future<Output = Result<CreateExperimentOutput, SdkError<CreateExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn create_feature_group(&self, builder: CreateFeatureGroupInputBuilder) -> impl Future<Output = Result<CreateFeatureGroupOutput, SdkError<CreateFeatureGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_flow_definition(&self, builder: CreateFlowDefinitionInputBuilder) -> impl Future<Output = Result<CreateFlowDefinitionOutput, SdkError<CreateFlowDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_hub(&self, builder: CreateHubInputBuilder) -> impl Future<Output = Result<CreateHubOutput, SdkError<CreateHubError>>> {
        builder.send_with(&self.0)
    }
    fn create_hub_content_reference(&self, builder: CreateHubContentReferenceInputBuilder) -> impl Future<Output = Result<CreateHubContentReferenceOutput, SdkError<CreateHubContentReferenceError>>> {
        builder.send_with(&self.0)
    }
    fn create_human_task_ui(&self, builder: CreateHumanTaskUiInputBuilder) -> impl Future<Output = Result<CreateHumanTaskUiOutput, SdkError<CreateHumanTaskUiError>>> {
        builder.send_with(&self.0)
    }
    fn create_hyper_parameter_tuning_job(&self, builder: CreateHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<CreateHyperParameterTuningJobOutput, SdkError<CreateHyperParameterTuningJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>> {
        builder.send_with(&self.0)
    }
    fn create_image_version(&self, builder: CreateImageVersionInputBuilder) -> impl Future<Output = Result<CreateImageVersionOutput, SdkError<CreateImageVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_inference_component(&self, builder: CreateInferenceComponentInputBuilder) -> impl Future<Output = Result<CreateInferenceComponentOutput, SdkError<CreateInferenceComponentError>>> {
        builder.send_with(&self.0)
    }
    fn create_inference_experiment(&self, builder: CreateInferenceExperimentInputBuilder) -> impl Future<Output = Result<CreateInferenceExperimentOutput, SdkError<CreateInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn create_inference_recommendations_job(&self, builder: CreateInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<CreateInferenceRecommendationsJobOutput, SdkError<CreateInferenceRecommendationsJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_labeling_job(&self, builder: CreateLabelingJobInputBuilder) -> impl Future<Output = Result<CreateLabelingJobOutput, SdkError<CreateLabelingJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_mlflow_tracking_server(&self, builder: CreateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<CreateMlflowTrackingServerOutput, SdkError<CreateMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_bias_job_definition(&self, builder: CreateModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelBiasJobDefinitionOutput, SdkError<CreateModelBiasJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_card(&self, builder: CreateModelCardInputBuilder) -> impl Future<Output = Result<CreateModelCardOutput, SdkError<CreateModelCardError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_card_export_job(&self, builder: CreateModelCardExportJobInputBuilder) -> impl Future<Output = Result<CreateModelCardExportJobOutput, SdkError<CreateModelCardExportJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_explainability_job_definition(&self, builder: CreateModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelExplainabilityJobDefinitionOutput, SdkError<CreateModelExplainabilityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_package(&self, builder: CreateModelPackageInputBuilder) -> impl Future<Output = Result<CreateModelPackageOutput, SdkError<CreateModelPackageError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_package_group(&self, builder: CreateModelPackageGroupInputBuilder) -> impl Future<Output = Result<CreateModelPackageGroupOutput, SdkError<CreateModelPackageGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_model_quality_job_definition(&self, builder: CreateModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelQualityJobDefinitionOutput, SdkError<CreateModelQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_monitoring_schedule(&self, builder: CreateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<CreateMonitoringScheduleOutput, SdkError<CreateMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn create_notebook_instance(&self, builder: CreateNotebookInstanceInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceOutput, SdkError<CreateNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn create_notebook_instance_lifecycle_config(&self, builder: CreateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceLifecycleConfigOutput, SdkError<CreateNotebookInstanceLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_optimization_job(&self, builder: CreateOptimizationJobInputBuilder) -> impl Future<Output = Result<CreateOptimizationJobOutput, SdkError<CreateOptimizationJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn create_presigned_domain_url(&self, builder: CreatePresignedDomainUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedDomainUrlOutput, SdkError<CreatePresignedDomainUrlError>>> {
        builder.send_with(&self.0)
    }
    fn create_presigned_mlflow_tracking_server_url(&self, builder: CreatePresignedMlflowTrackingServerUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedMlflowTrackingServerUrlOutput, SdkError<CreatePresignedMlflowTrackingServerUrlError>>> {
        builder.send_with(&self.0)
    }
    fn create_presigned_notebook_instance_url(&self, builder: CreatePresignedNotebookInstanceUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookInstanceUrlOutput, SdkError<CreatePresignedNotebookInstanceUrlError>>> {
        builder.send_with(&self.0)
    }
    fn create_processing_job(&self, builder: CreateProcessingJobInputBuilder) -> impl Future<Output = Result<CreateProcessingJobOutput, SdkError<CreateProcessingJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn create_space(&self, builder: CreateSpaceInputBuilder) -> impl Future<Output = Result<CreateSpaceOutput, SdkError<CreateSpaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_studio_lifecycle_config(&self, builder: CreateStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateStudioLifecycleConfigOutput, SdkError<CreateStudioLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_training_job(&self, builder: CreateTrainingJobInputBuilder) -> impl Future<Output = Result<CreateTrainingJobOutput, SdkError<CreateTrainingJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_transform_job(&self, builder: CreateTransformJobInputBuilder) -> impl Future<Output = Result<CreateTransformJobOutput, SdkError<CreateTransformJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_trial(&self, builder: CreateTrialInputBuilder) -> impl Future<Output = Result<CreateTrialOutput, SdkError<CreateTrialError>>> {
        builder.send_with(&self.0)
    }
    fn create_trial_component(&self, builder: CreateTrialComponentInputBuilder) -> impl Future<Output = Result<CreateTrialComponentOutput, SdkError<CreateTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_profile(&self, builder: CreateUserProfileInputBuilder) -> impl Future<Output = Result<CreateUserProfileOutput, SdkError<CreateUserProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_workforce(&self, builder: CreateWorkforceInputBuilder) -> impl Future<Output = Result<CreateWorkforceOutput, SdkError<CreateWorkforceError>>> {
        builder.send_with(&self.0)
    }
    fn create_workteam(&self, builder: CreateWorkteamInputBuilder) -> impl Future<Output = Result<CreateWorkteamOutput, SdkError<CreateWorkteamError>>> {
        builder.send_with(&self.0)
    }
    fn delete_action(&self, builder: DeleteActionInputBuilder) -> impl Future<Output = Result<DeleteActionOutput, SdkError<DeleteActionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_algorithm(&self, builder: DeleteAlgorithmInputBuilder) -> impl Future<Output = Result<DeleteAlgorithmOutput, SdkError<DeleteAlgorithmError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app_image_config(&self, builder: DeleteAppImageConfigInputBuilder) -> impl Future<Output = Result<DeleteAppImageConfigOutput, SdkError<DeleteAppImageConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_artifact(&self, builder: DeleteArtifactInputBuilder) -> impl Future<Output = Result<DeleteArtifactOutput, SdkError<DeleteArtifactError>>> {
        builder.send_with(&self.0)
    }
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_code_repository(&self, builder: DeleteCodeRepositoryInputBuilder) -> impl Future<Output = Result<DeleteCodeRepositoryOutput, SdkError<DeleteCodeRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_compilation_job(&self, builder: DeleteCompilationJobInputBuilder) -> impl Future<Output = Result<DeleteCompilationJobOutput, SdkError<DeleteCompilationJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_context(&self, builder: DeleteContextInputBuilder) -> impl Future<Output = Result<DeleteContextOutput, SdkError<DeleteContextError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_quality_job_definition(&self, builder: DeleteDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteDataQualityJobDefinitionOutput, SdkError<DeleteDataQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_device_fleet(&self, builder: DeleteDeviceFleetInputBuilder) -> impl Future<Output = Result<DeleteDeviceFleetOutput, SdkError<DeleteDeviceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>> {
        builder.send_with(&self.0)
    }
    fn delete_edge_deployment_plan(&self, builder: DeleteEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentPlanOutput, SdkError<DeleteEdgeDeploymentPlanError>>> {
        builder.send_with(&self.0)
    }
    fn delete_edge_deployment_stage(&self, builder: DeleteEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentStageOutput, SdkError<DeleteEdgeDeploymentStageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_endpoint_config(&self, builder: DeleteEndpointConfigInputBuilder) -> impl Future<Output = Result<DeleteEndpointConfigOutput, SdkError<DeleteEndpointConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_experiment(&self, builder: DeleteExperimentInputBuilder) -> impl Future<Output = Result<DeleteExperimentOutput, SdkError<DeleteExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_feature_group(&self, builder: DeleteFeatureGroupInputBuilder) -> impl Future<Output = Result<DeleteFeatureGroupOutput, SdkError<DeleteFeatureGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_flow_definition(&self, builder: DeleteFlowDefinitionInputBuilder) -> impl Future<Output = Result<DeleteFlowDefinitionOutput, SdkError<DeleteFlowDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hub(&self, builder: DeleteHubInputBuilder) -> impl Future<Output = Result<DeleteHubOutput, SdkError<DeleteHubError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hub_content(&self, builder: DeleteHubContentInputBuilder) -> impl Future<Output = Result<DeleteHubContentOutput, SdkError<DeleteHubContentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hub_content_reference(&self, builder: DeleteHubContentReferenceInputBuilder) -> impl Future<Output = Result<DeleteHubContentReferenceOutput, SdkError<DeleteHubContentReferenceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_human_task_ui(&self, builder: DeleteHumanTaskUiInputBuilder) -> impl Future<Output = Result<DeleteHumanTaskUiOutput, SdkError<DeleteHumanTaskUiError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hyper_parameter_tuning_job(&self, builder: DeleteHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DeleteHyperParameterTuningJobOutput, SdkError<DeleteHyperParameterTuningJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_image_version(&self, builder: DeleteImageVersionInputBuilder) -> impl Future<Output = Result<DeleteImageVersionOutput, SdkError<DeleteImageVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_inference_component(&self, builder: DeleteInferenceComponentInputBuilder) -> impl Future<Output = Result<DeleteInferenceComponentOutput, SdkError<DeleteInferenceComponentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_inference_experiment(&self, builder: DeleteInferenceExperimentInputBuilder) -> impl Future<Output = Result<DeleteInferenceExperimentOutput, SdkError<DeleteInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_mlflow_tracking_server(&self, builder: DeleteMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DeleteMlflowTrackingServerOutput, SdkError<DeleteMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_bias_job_definition(&self, builder: DeleteModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelBiasJobDefinitionOutput, SdkError<DeleteModelBiasJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_card(&self, builder: DeleteModelCardInputBuilder) -> impl Future<Output = Result<DeleteModelCardOutput, SdkError<DeleteModelCardError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_explainability_job_definition(&self, builder: DeleteModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelExplainabilityJobDefinitionOutput, SdkError<DeleteModelExplainabilityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_package(&self, builder: DeleteModelPackageInputBuilder) -> impl Future<Output = Result<DeleteModelPackageOutput, SdkError<DeleteModelPackageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_package_group(&self, builder: DeleteModelPackageGroupInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupOutput, SdkError<DeleteModelPackageGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_package_group_policy(&self, builder: DeleteModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupPolicyOutput, SdkError<DeleteModelPackageGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model_quality_job_definition(&self, builder: DeleteModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelQualityJobDefinitionOutput, SdkError<DeleteModelQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_monitoring_schedule(&self, builder: DeleteMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DeleteMonitoringScheduleOutput, SdkError<DeleteMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_notebook_instance(&self, builder: DeleteNotebookInstanceInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceOutput, SdkError<DeleteNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_notebook_instance_lifecycle_config(&self, builder: DeleteNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceLifecycleConfigOutput, SdkError<DeleteNotebookInstanceLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_optimization_job(&self, builder: DeleteOptimizationJobInputBuilder) -> impl Future<Output = Result<DeleteOptimizationJobOutput, SdkError<DeleteOptimizationJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_space(&self, builder: DeleteSpaceInputBuilder) -> impl Future<Output = Result<DeleteSpaceOutput, SdkError<DeleteSpaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_studio_lifecycle_config(&self, builder: DeleteStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteStudioLifecycleConfigOutput, SdkError<DeleteStudioLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_trial(&self, builder: DeleteTrialInputBuilder) -> impl Future<Output = Result<DeleteTrialOutput, SdkError<DeleteTrialError>>> {
        builder.send_with(&self.0)
    }
    fn delete_trial_component(&self, builder: DeleteTrialComponentInputBuilder) -> impl Future<Output = Result<DeleteTrialComponentOutput, SdkError<DeleteTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_profile(&self, builder: DeleteUserProfileInputBuilder) -> impl Future<Output = Result<DeleteUserProfileOutput, SdkError<DeleteUserProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_workforce(&self, builder: DeleteWorkforceInputBuilder) -> impl Future<Output = Result<DeleteWorkforceOutput, SdkError<DeleteWorkforceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_workteam(&self, builder: DeleteWorkteamInputBuilder) -> impl Future<Output = Result<DeleteWorkteamOutput, SdkError<DeleteWorkteamError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_devices(&self, builder: DeregisterDevicesInputBuilder) -> impl Future<Output = Result<DeregisterDevicesOutput, SdkError<DeregisterDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_action(&self, builder: DescribeActionInputBuilder) -> impl Future<Output = Result<DescribeActionOutput, SdkError<DescribeActionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_algorithm(&self, builder: DescribeAlgorithmInputBuilder) -> impl Future<Output = Result<DescribeAlgorithmOutput, SdkError<DescribeAlgorithmError>>> {
        builder.send_with(&self.0)
    }
    fn describe_app(&self, builder: DescribeAppInputBuilder) -> impl Future<Output = Result<DescribeAppOutput, SdkError<DescribeAppError>>> {
        builder.send_with(&self.0)
    }
    fn describe_app_image_config(&self, builder: DescribeAppImageConfigInputBuilder) -> impl Future<Output = Result<DescribeAppImageConfigOutput, SdkError<DescribeAppImageConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_artifact(&self, builder: DescribeArtifactInputBuilder) -> impl Future<Output = Result<DescribeArtifactOutput, SdkError<DescribeArtifactError>>> {
        builder.send_with(&self.0)
    }
    fn describe_auto_ml_job(&self, builder: DescribeAutoMlJobInputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobOutput, SdkError<DescribeAutoMLJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_auto_ml_job_v2(&self, builder: DescribeAutoMlJobV2InputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobV2Output, SdkError<DescribeAutoMLJobV2Error>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_node(&self, builder: DescribeClusterNodeInputBuilder) -> impl Future<Output = Result<DescribeClusterNodeOutput, SdkError<DescribeClusterNodeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_code_repository(&self, builder: DescribeCodeRepositoryInputBuilder) -> impl Future<Output = Result<DescribeCodeRepositoryOutput, SdkError<DescribeCodeRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_compilation_job(&self, builder: DescribeCompilationJobInputBuilder) -> impl Future<Output = Result<DescribeCompilationJobOutput, SdkError<DescribeCompilationJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_context(&self, builder: DescribeContextInputBuilder) -> impl Future<Output = Result<DescribeContextOutput, SdkError<DescribeContextError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_quality_job_definition(&self, builder: DescribeDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDataQualityJobDefinitionOutput, SdkError<DescribeDataQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_device(&self, builder: DescribeDeviceInputBuilder) -> impl Future<Output = Result<DescribeDeviceOutput, SdkError<DescribeDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_device_fleet(&self, builder: DescribeDeviceFleetInputBuilder) -> impl Future<Output = Result<DescribeDeviceFleetOutput, SdkError<DescribeDeviceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>> {
        builder.send_with(&self.0)
    }
    fn describe_edge_deployment_plan(&self, builder: DescribeEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DescribeEdgeDeploymentPlanOutput, SdkError<DescribeEdgeDeploymentPlanError>>> {
        builder.send_with(&self.0)
    }
    fn describe_edge_packaging_job(&self, builder: DescribeEdgePackagingJobInputBuilder) -> impl Future<Output = Result<DescribeEdgePackagingJobOutput, SdkError<DescribeEdgePackagingJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoint(&self, builder: DescribeEndpointInputBuilder) -> impl Future<Output = Result<DescribeEndpointOutput, SdkError<DescribeEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoint_config(&self, builder: DescribeEndpointConfigInputBuilder) -> impl Future<Output = Result<DescribeEndpointConfigOutput, SdkError<DescribeEndpointConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_experiment(&self, builder: DescribeExperimentInputBuilder) -> impl Future<Output = Result<DescribeExperimentOutput, SdkError<DescribeExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_feature_group(&self, builder: DescribeFeatureGroupInputBuilder) -> impl Future<Output = Result<DescribeFeatureGroupOutput, SdkError<DescribeFeatureGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_feature_metadata(&self, builder: DescribeFeatureMetadataInputBuilder) -> impl Future<Output = Result<DescribeFeatureMetadataOutput, SdkError<DescribeFeatureMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn describe_flow_definition(&self, builder: DescribeFlowDefinitionInputBuilder) -> impl Future<Output = Result<DescribeFlowDefinitionOutput, SdkError<DescribeFlowDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hub(&self, builder: DescribeHubInputBuilder) -> impl Future<Output = Result<DescribeHubOutput, SdkError<DescribeHubError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hub_content(&self, builder: DescribeHubContentInputBuilder) -> impl Future<Output = Result<DescribeHubContentOutput, SdkError<DescribeHubContentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_human_task_ui(&self, builder: DescribeHumanTaskUiInputBuilder) -> impl Future<Output = Result<DescribeHumanTaskUiOutput, SdkError<DescribeHumanTaskUiError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hyper_parameter_tuning_job(&self, builder: DescribeHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DescribeHyperParameterTuningJobOutput, SdkError<DescribeHyperParameterTuningJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_image(&self, builder: DescribeImageInputBuilder) -> impl Future<Output = Result<DescribeImageOutput, SdkError<DescribeImageError>>> {
        builder.send_with(&self.0)
    }
    fn describe_image_version(&self, builder: DescribeImageVersionInputBuilder) -> impl Future<Output = Result<DescribeImageVersionOutput, SdkError<DescribeImageVersionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inference_component(&self, builder: DescribeInferenceComponentInputBuilder) -> impl Future<Output = Result<DescribeInferenceComponentOutput, SdkError<DescribeInferenceComponentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inference_experiment(&self, builder: DescribeInferenceExperimentInputBuilder) -> impl Future<Output = Result<DescribeInferenceExperimentOutput, SdkError<DescribeInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inference_recommendations_job(&self, builder: DescribeInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<DescribeInferenceRecommendationsJobOutput, SdkError<DescribeInferenceRecommendationsJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_labeling_job(&self, builder: DescribeLabelingJobInputBuilder) -> impl Future<Output = Result<DescribeLabelingJobOutput, SdkError<DescribeLabelingJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_lineage_group(&self, builder: DescribeLineageGroupInputBuilder) -> impl Future<Output = Result<DescribeLineageGroupOutput, SdkError<DescribeLineageGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_mlflow_tracking_server(&self, builder: DescribeMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DescribeMlflowTrackingServerOutput, SdkError<DescribeMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model(&self, builder: DescribeModelInputBuilder) -> impl Future<Output = Result<DescribeModelOutput, SdkError<DescribeModelError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_bias_job_definition(&self, builder: DescribeModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelBiasJobDefinitionOutput, SdkError<DescribeModelBiasJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_card(&self, builder: DescribeModelCardInputBuilder) -> impl Future<Output = Result<DescribeModelCardOutput, SdkError<DescribeModelCardError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_card_export_job(&self, builder: DescribeModelCardExportJobInputBuilder) -> impl Future<Output = Result<DescribeModelCardExportJobOutput, SdkError<DescribeModelCardExportJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_explainability_job_definition(&self, builder: DescribeModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelExplainabilityJobDefinitionOutput, SdkError<DescribeModelExplainabilityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_package(&self, builder: DescribeModelPackageInputBuilder) -> impl Future<Output = Result<DescribeModelPackageOutput, SdkError<DescribeModelPackageError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_package_group(&self, builder: DescribeModelPackageGroupInputBuilder) -> impl Future<Output = Result<DescribeModelPackageGroupOutput, SdkError<DescribeModelPackageGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_model_quality_job_definition(&self, builder: DescribeModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelQualityJobDefinitionOutput, SdkError<DescribeModelQualityJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_monitoring_schedule(&self, builder: DescribeMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DescribeMonitoringScheduleOutput, SdkError<DescribeMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn describe_notebook_instance(&self, builder: DescribeNotebookInstanceInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceOutput, SdkError<DescribeNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_notebook_instance_lifecycle_config(&self, builder: DescribeNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceLifecycleConfigOutput, SdkError<DescribeNotebookInstanceLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_optimization_job(&self, builder: DescribeOptimizationJobInputBuilder) -> impl Future<Output = Result<DescribeOptimizationJobOutput, SdkError<DescribeOptimizationJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pipeline(&self, builder: DescribePipelineInputBuilder) -> impl Future<Output = Result<DescribePipelineOutput, SdkError<DescribePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pipeline_definition_for_execution(&self, builder: DescribePipelineDefinitionForExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineDefinitionForExecutionOutput, SdkError<DescribePipelineDefinitionForExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pipeline_execution(&self, builder: DescribePipelineExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineExecutionOutput, SdkError<DescribePipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_processing_job(&self, builder: DescribeProcessingJobInputBuilder) -> impl Future<Output = Result<DescribeProcessingJobOutput, SdkError<DescribeProcessingJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>> {
        builder.send_with(&self.0)
    }
    fn describe_space(&self, builder: DescribeSpaceInputBuilder) -> impl Future<Output = Result<DescribeSpaceOutput, SdkError<DescribeSpaceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_studio_lifecycle_config(&self, builder: DescribeStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeStudioLifecycleConfigOutput, SdkError<DescribeStudioLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_subscribed_workteam(&self, builder: DescribeSubscribedWorkteamInputBuilder) -> impl Future<Output = Result<DescribeSubscribedWorkteamOutput, SdkError<DescribeSubscribedWorkteamError>>> {
        builder.send_with(&self.0)
    }
    fn describe_training_job(&self, builder: DescribeTrainingJobInputBuilder) -> impl Future<Output = Result<DescribeTrainingJobOutput, SdkError<DescribeTrainingJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transform_job(&self, builder: DescribeTransformJobInputBuilder) -> impl Future<Output = Result<DescribeTransformJobOutput, SdkError<DescribeTransformJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_trial(&self, builder: DescribeTrialInputBuilder) -> impl Future<Output = Result<DescribeTrialOutput, SdkError<DescribeTrialError>>> {
        builder.send_with(&self.0)
    }
    fn describe_trial_component(&self, builder: DescribeTrialComponentInputBuilder) -> impl Future<Output = Result<DescribeTrialComponentOutput, SdkError<DescribeTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_profile(&self, builder: DescribeUserProfileInputBuilder) -> impl Future<Output = Result<DescribeUserProfileOutput, SdkError<DescribeUserProfileError>>> {
        builder.send_with(&self.0)
    }
    fn describe_workforce(&self, builder: DescribeWorkforceInputBuilder) -> impl Future<Output = Result<DescribeWorkforceOutput, SdkError<DescribeWorkforceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_workteam(&self, builder: DescribeWorkteamInputBuilder) -> impl Future<Output = Result<DescribeWorkteamOutput, SdkError<DescribeWorkteamError>>> {
        builder.send_with(&self.0)
    }
    fn disable_sagemaker_servicecatalog_portfolio(&self, builder: DisableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<DisableSagemakerServicecatalogPortfolioOutput, SdkError<DisableSagemakerServicecatalogPortfolioError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_trial_component(&self, builder: DisassociateTrialComponentInputBuilder) -> impl Future<Output = Result<DisassociateTrialComponentOutput, SdkError<DisassociateTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn enable_sagemaker_servicecatalog_portfolio(&self, builder: EnableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<EnableSagemakerServicecatalogPortfolioOutput, SdkError<EnableSagemakerServicecatalogPortfolioError>>> {
        builder.send_with(&self.0)
    }
    fn get_device_fleet_report(&self, builder: GetDeviceFleetReportInputBuilder) -> impl Future<Output = Result<GetDeviceFleetReportOutput, SdkError<GetDeviceFleetReportError>>> {
        builder.send_with(&self.0)
    }
    fn get_lineage_group_policy(&self, builder: GetLineageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetLineageGroupPolicyOutput, SdkError<GetLineageGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_model_package_group_policy(&self, builder: GetModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetModelPackageGroupPolicyOutput, SdkError<GetModelPackageGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_sagemaker_servicecatalog_portfolio_status(&self, builder: GetSagemakerServicecatalogPortfolioStatusInputBuilder) -> impl Future<Output = Result<GetSagemakerServicecatalogPortfolioStatusOutput, SdkError<GetSagemakerServicecatalogPortfolioStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_scaling_configuration_recommendation(&self, builder: GetScalingConfigurationRecommendationInputBuilder) -> impl Future<Output = Result<GetScalingConfigurationRecommendationOutput, SdkError<GetScalingConfigurationRecommendationError>>> {
        builder.send_with(&self.0)
    }
    fn get_search_suggestions(&self, builder: GetSearchSuggestionsInputBuilder) -> impl Future<Output = Result<GetSearchSuggestionsOutput, SdkError<GetSearchSuggestionsError>>> {
        builder.send_with(&self.0)
    }
    fn import_hub_content(&self, builder: ImportHubContentInputBuilder) -> impl Future<Output = Result<ImportHubContentOutput, SdkError<ImportHubContentError>>> {
        builder.send_with(&self.0)
    }
    fn list_actions(&self, builder: ListActionsInputBuilder) -> impl Future<Output = Result<ListActionsOutput, SdkError<ListActionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_algorithms(&self, builder: ListAlgorithmsInputBuilder) -> impl Future<Output = Result<ListAlgorithmsOutput, SdkError<ListAlgorithmsError>>> {
        builder.send_with(&self.0)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_app_image_configs(&self, builder: ListAppImageConfigsInputBuilder) -> impl Future<Output = Result<ListAppImageConfigsOutput, SdkError<ListAppImageConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>> {
        builder.send_with(&self.0)
    }
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>> {
        builder.send_with(&self.0)
    }
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_auto_ml_jobs(&self, builder: ListAutoMlJobsInputBuilder) -> impl Future<Output = Result<ListAutoMlJobsOutput, SdkError<ListAutoMLJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_candidates_for_auto_ml_job(&self, builder: ListCandidatesForAutoMlJobInputBuilder) -> impl Future<Output = Result<ListCandidatesForAutoMlJobOutput, SdkError<ListCandidatesForAutoMLJobError>>> {
        builder.send_with(&self.0)
    }
    fn list_cluster_nodes(&self, builder: ListClusterNodesInputBuilder) -> impl Future<Output = Result<ListClusterNodesOutput, SdkError<ListClusterNodesError>>> {
        builder.send_with(&self.0)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        builder.send_with(&self.0)
    }
    fn list_code_repositories(&self, builder: ListCodeRepositoriesInputBuilder) -> impl Future<Output = Result<ListCodeRepositoriesOutput, SdkError<ListCodeRepositoriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_compilation_jobs(&self, builder: ListCompilationJobsInputBuilder) -> impl Future<Output = Result<ListCompilationJobsOutput, SdkError<ListCompilationJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_contexts(&self, builder: ListContextsInputBuilder) -> impl Future<Output = Result<ListContextsOutput, SdkError<ListContextsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_quality_job_definitions(&self, builder: ListDataQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListDataQualityJobDefinitionsOutput, SdkError<ListDataQualityJobDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_device_fleets(&self, builder: ListDeviceFleetsInputBuilder) -> impl Future<Output = Result<ListDeviceFleetsOutput, SdkError<ListDeviceFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domains(&self, builder: ListDomainsInputBuilder) -> impl Future<Output = Result<ListDomainsOutput, SdkError<ListDomainsError>>> {
        builder.send_with(&self.0)
    }
    fn list_edge_deployment_plans(&self, builder: ListEdgeDeploymentPlansInputBuilder) -> impl Future<Output = Result<ListEdgeDeploymentPlansOutput, SdkError<ListEdgeDeploymentPlansError>>> {
        builder.send_with(&self.0)
    }
    fn list_edge_packaging_jobs(&self, builder: ListEdgePackagingJobsInputBuilder) -> impl Future<Output = Result<ListEdgePackagingJobsOutput, SdkError<ListEdgePackagingJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_endpoint_configs(&self, builder: ListEndpointConfigsInputBuilder) -> impl Future<Output = Result<ListEndpointConfigsOutput, SdkError<ListEndpointConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_endpoints(&self, builder: ListEndpointsInputBuilder) -> impl Future<Output = Result<ListEndpointsOutput, SdkError<ListEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn list_experiments(&self, builder: ListExperimentsInputBuilder) -> impl Future<Output = Result<ListExperimentsOutput, SdkError<ListExperimentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_feature_groups(&self, builder: ListFeatureGroupsInputBuilder) -> impl Future<Output = Result<ListFeatureGroupsOutput, SdkError<ListFeatureGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_flow_definitions(&self, builder: ListFlowDefinitionsInputBuilder) -> impl Future<Output = Result<ListFlowDefinitionsOutput, SdkError<ListFlowDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_hub_content_versions(&self, builder: ListHubContentVersionsInputBuilder) -> impl Future<Output = Result<ListHubContentVersionsOutput, SdkError<ListHubContentVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_hub_contents(&self, builder: ListHubContentsInputBuilder) -> impl Future<Output = Result<ListHubContentsOutput, SdkError<ListHubContentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_hubs(&self, builder: ListHubsInputBuilder) -> impl Future<Output = Result<ListHubsOutput, SdkError<ListHubsError>>> {
        builder.send_with(&self.0)
    }
    fn list_human_task_uis(&self, builder: ListHumanTaskUisInputBuilder) -> impl Future<Output = Result<ListHumanTaskUisOutput, SdkError<ListHumanTaskUisError>>> {
        builder.send_with(&self.0)
    }
    fn list_hyper_parameter_tuning_jobs(&self, builder: ListHyperParameterTuningJobsInputBuilder) -> impl Future<Output = Result<ListHyperParameterTuningJobsOutput, SdkError<ListHyperParameterTuningJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_image_versions(&self, builder: ListImageVersionsInputBuilder) -> impl Future<Output = Result<ListImageVersionsOutput, SdkError<ListImageVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_images(&self, builder: ListImagesInputBuilder) -> impl Future<Output = Result<ListImagesOutput, SdkError<ListImagesError>>> {
        builder.send_with(&self.0)
    }
    fn list_inference_components(&self, builder: ListInferenceComponentsInputBuilder) -> impl Future<Output = Result<ListInferenceComponentsOutput, SdkError<ListInferenceComponentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_inference_experiments(&self, builder: ListInferenceExperimentsInputBuilder) -> impl Future<Output = Result<ListInferenceExperimentsOutput, SdkError<ListInferenceExperimentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_inference_recommendations_job_steps(&self, builder: ListInferenceRecommendationsJobStepsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobStepsOutput, SdkError<ListInferenceRecommendationsJobStepsError>>> {
        builder.send_with(&self.0)
    }
    fn list_inference_recommendations_jobs(&self, builder: ListInferenceRecommendationsJobsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobsOutput, SdkError<ListInferenceRecommendationsJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_labeling_jobs(&self, builder: ListLabelingJobsInputBuilder) -> impl Future<Output = Result<ListLabelingJobsOutput, SdkError<ListLabelingJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_labeling_jobs_for_workteam(&self, builder: ListLabelingJobsForWorkteamInputBuilder) -> impl Future<Output = Result<ListLabelingJobsForWorkteamOutput, SdkError<ListLabelingJobsForWorkteamError>>> {
        builder.send_with(&self.0)
    }
    fn list_lineage_groups(&self, builder: ListLineageGroupsInputBuilder) -> impl Future<Output = Result<ListLineageGroupsOutput, SdkError<ListLineageGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_mlflow_tracking_servers(&self, builder: ListMlflowTrackingServersInputBuilder) -> impl Future<Output = Result<ListMlflowTrackingServersOutput, SdkError<ListMlflowTrackingServersError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_bias_job_definitions(&self, builder: ListModelBiasJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelBiasJobDefinitionsOutput, SdkError<ListModelBiasJobDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_card_export_jobs(&self, builder: ListModelCardExportJobsInputBuilder) -> impl Future<Output = Result<ListModelCardExportJobsOutput, SdkError<ListModelCardExportJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_card_versions(&self, builder: ListModelCardVersionsInputBuilder) -> impl Future<Output = Result<ListModelCardVersionsOutput, SdkError<ListModelCardVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_cards(&self, builder: ListModelCardsInputBuilder) -> impl Future<Output = Result<ListModelCardsOutput, SdkError<ListModelCardsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_explainability_job_definitions(&self, builder: ListModelExplainabilityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelExplainabilityJobDefinitionsOutput, SdkError<ListModelExplainabilityJobDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_metadata(&self, builder: ListModelMetadataInputBuilder) -> impl Future<Output = Result<ListModelMetadataOutput, SdkError<ListModelMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_package_groups(&self, builder: ListModelPackageGroupsInputBuilder) -> impl Future<Output = Result<ListModelPackageGroupsOutput, SdkError<ListModelPackageGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_packages(&self, builder: ListModelPackagesInputBuilder) -> impl Future<Output = Result<ListModelPackagesOutput, SdkError<ListModelPackagesError>>> {
        builder.send_with(&self.0)
    }
    fn list_model_quality_job_definitions(&self, builder: ListModelQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelQualityJobDefinitionsOutput, SdkError<ListModelQualityJobDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_models(&self, builder: ListModelsInputBuilder) -> impl Future<Output = Result<ListModelsOutput, SdkError<ListModelsError>>> {
        builder.send_with(&self.0)
    }
    fn list_monitoring_alert_history(&self, builder: ListMonitoringAlertHistoryInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertHistoryOutput, SdkError<ListMonitoringAlertHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn list_monitoring_alerts(&self, builder: ListMonitoringAlertsInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertsOutput, SdkError<ListMonitoringAlertsError>>> {
        builder.send_with(&self.0)
    }
    fn list_monitoring_executions(&self, builder: ListMonitoringExecutionsInputBuilder) -> impl Future<Output = Result<ListMonitoringExecutionsOutput, SdkError<ListMonitoringExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_monitoring_schedules(&self, builder: ListMonitoringSchedulesInputBuilder) -> impl Future<Output = Result<ListMonitoringSchedulesOutput, SdkError<ListMonitoringSchedulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_notebook_instance_lifecycle_configs(&self, builder: ListNotebookInstanceLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListNotebookInstanceLifecycleConfigsOutput, SdkError<ListNotebookInstanceLifecycleConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_notebook_instances(&self, builder: ListNotebookInstancesInputBuilder) -> impl Future<Output = Result<ListNotebookInstancesOutput, SdkError<ListNotebookInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn list_optimization_jobs(&self, builder: ListOptimizationJobsInputBuilder) -> impl Future<Output = Result<ListOptimizationJobsOutput, SdkError<ListOptimizationJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipeline_execution_steps(&self, builder: ListPipelineExecutionStepsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionStepsOutput, SdkError<ListPipelineExecutionStepsError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipeline_parameters_for_execution(&self, builder: ListPipelineParametersForExecutionInputBuilder) -> impl Future<Output = Result<ListPipelineParametersForExecutionOutput, SdkError<ListPipelineParametersForExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        builder.send_with(&self.0)
    }
    fn list_processing_jobs(&self, builder: ListProcessingJobsInputBuilder) -> impl Future<Output = Result<ListProcessingJobsOutput, SdkError<ListProcessingJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_catalogs(&self, builder: ListResourceCatalogsInputBuilder) -> impl Future<Output = Result<ListResourceCatalogsOutput, SdkError<ListResourceCatalogsError>>> {
        builder.send_with(&self.0)
    }
    fn list_spaces(&self, builder: ListSpacesInputBuilder) -> impl Future<Output = Result<ListSpacesOutput, SdkError<ListSpacesError>>> {
        builder.send_with(&self.0)
    }
    fn list_stage_devices(&self, builder: ListStageDevicesInputBuilder) -> impl Future<Output = Result<ListStageDevicesOutput, SdkError<ListStageDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn list_studio_lifecycle_configs(&self, builder: ListStudioLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListStudioLifecycleConfigsOutput, SdkError<ListStudioLifecycleConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_subscribed_workteams(&self, builder: ListSubscribedWorkteamsInputBuilder) -> impl Future<Output = Result<ListSubscribedWorkteamsOutput, SdkError<ListSubscribedWorkteamsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_training_jobs(&self, builder: ListTrainingJobsInputBuilder) -> impl Future<Output = Result<ListTrainingJobsOutput, SdkError<ListTrainingJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_training_jobs_for_hyper_parameter_tuning_job(&self, builder: ListTrainingJobsForHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<ListTrainingJobsForHyperParameterTuningJobOutput, SdkError<ListTrainingJobsForHyperParameterTuningJobError>>> {
        builder.send_with(&self.0)
    }
    fn list_transform_jobs(&self, builder: ListTransformJobsInputBuilder) -> impl Future<Output = Result<ListTransformJobsOutput, SdkError<ListTransformJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_trial_components(&self, builder: ListTrialComponentsInputBuilder) -> impl Future<Output = Result<ListTrialComponentsOutput, SdkError<ListTrialComponentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_trials(&self, builder: ListTrialsInputBuilder) -> impl Future<Output = Result<ListTrialsOutput, SdkError<ListTrialsError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_profiles(&self, builder: ListUserProfilesInputBuilder) -> impl Future<Output = Result<ListUserProfilesOutput, SdkError<ListUserProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_workforces(&self, builder: ListWorkforcesInputBuilder) -> impl Future<Output = Result<ListWorkforcesOutput, SdkError<ListWorkforcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_workteams(&self, builder: ListWorkteamsInputBuilder) -> impl Future<Output = Result<ListWorkteamsOutput, SdkError<ListWorkteamsError>>> {
        builder.send_with(&self.0)
    }
    fn put_model_package_group_policy(&self, builder: PutModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<PutModelPackageGroupPolicyOutput, SdkError<PutModelPackageGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn query_lineage(&self, builder: QueryLineageInputBuilder) -> impl Future<Output = Result<QueryLineageOutput, SdkError<QueryLineageError>>> {
        builder.send_with(&self.0)
    }
    fn register_devices(&self, builder: RegisterDevicesInputBuilder) -> impl Future<Output = Result<RegisterDevicesOutput, SdkError<RegisterDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn render_ui_template(&self, builder: RenderUiTemplateInputBuilder) -> impl Future<Output = Result<RenderUiTemplateOutput, SdkError<RenderUiTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn retry_pipeline_execution(&self, builder: RetryPipelineExecutionInputBuilder) -> impl Future<Output = Result<RetryPipelineExecutionOutput, SdkError<RetryPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn search(&self, builder: SearchInputBuilder) -> impl Future<Output = Result<SearchOutput, SdkError<SearchError>>> {
        builder.send_with(&self.0)
    }
    fn send_pipeline_execution_step_failure(&self, builder: SendPipelineExecutionStepFailureInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepFailureOutput, SdkError<SendPipelineExecutionStepFailureError>>> {
        builder.send_with(&self.0)
    }
    fn send_pipeline_execution_step_success(&self, builder: SendPipelineExecutionStepSuccessInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepSuccessOutput, SdkError<SendPipelineExecutionStepSuccessError>>> {
        builder.send_with(&self.0)
    }
    fn start_edge_deployment_stage(&self, builder: StartEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StartEdgeDeploymentStageOutput, SdkError<StartEdgeDeploymentStageError>>> {
        builder.send_with(&self.0)
    }
    fn start_inference_experiment(&self, builder: StartInferenceExperimentInputBuilder) -> impl Future<Output = Result<StartInferenceExperimentOutput, SdkError<StartInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn start_mlflow_tracking_server(&self, builder: StartMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StartMlflowTrackingServerOutput, SdkError<StartMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn start_monitoring_schedule(&self, builder: StartMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StartMonitoringScheduleOutput, SdkError<StartMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn start_notebook_instance(&self, builder: StartNotebookInstanceInputBuilder) -> impl Future<Output = Result<StartNotebookInstanceOutput, SdkError<StartNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_auto_ml_job(&self, builder: StopAutoMlJobInputBuilder) -> impl Future<Output = Result<StopAutoMlJobOutput, SdkError<StopAutoMLJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_compilation_job(&self, builder: StopCompilationJobInputBuilder) -> impl Future<Output = Result<StopCompilationJobOutput, SdkError<StopCompilationJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_edge_deployment_stage(&self, builder: StopEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StopEdgeDeploymentStageOutput, SdkError<StopEdgeDeploymentStageError>>> {
        builder.send_with(&self.0)
    }
    fn stop_edge_packaging_job(&self, builder: StopEdgePackagingJobInputBuilder) -> impl Future<Output = Result<StopEdgePackagingJobOutput, SdkError<StopEdgePackagingJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_hyper_parameter_tuning_job(&self, builder: StopHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<StopHyperParameterTuningJobOutput, SdkError<StopHyperParameterTuningJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_inference_experiment(&self, builder: StopInferenceExperimentInputBuilder) -> impl Future<Output = Result<StopInferenceExperimentOutput, SdkError<StopInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn stop_inference_recommendations_job(&self, builder: StopInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<StopInferenceRecommendationsJobOutput, SdkError<StopInferenceRecommendationsJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_labeling_job(&self, builder: StopLabelingJobInputBuilder) -> impl Future<Output = Result<StopLabelingJobOutput, SdkError<StopLabelingJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_mlflow_tracking_server(&self, builder: StopMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StopMlflowTrackingServerOutput, SdkError<StopMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn stop_monitoring_schedule(&self, builder: StopMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StopMonitoringScheduleOutput, SdkError<StopMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn stop_notebook_instance(&self, builder: StopNotebookInstanceInputBuilder) -> impl Future<Output = Result<StopNotebookInstanceOutput, SdkError<StopNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn stop_optimization_job(&self, builder: StopOptimizationJobInputBuilder) -> impl Future<Output = Result<StopOptimizationJobOutput, SdkError<StopOptimizationJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_processing_job(&self, builder: StopProcessingJobInputBuilder) -> impl Future<Output = Result<StopProcessingJobOutput, SdkError<StopProcessingJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_training_job(&self, builder: StopTrainingJobInputBuilder) -> impl Future<Output = Result<StopTrainingJobOutput, SdkError<StopTrainingJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_transform_job(&self, builder: StopTransformJobInputBuilder) -> impl Future<Output = Result<StopTransformJobOutput, SdkError<StopTransformJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_action(&self, builder: UpdateActionInputBuilder) -> impl Future<Output = Result<UpdateActionOutput, SdkError<UpdateActionError>>> {
        builder.send_with(&self.0)
    }
    fn update_app_image_config(&self, builder: UpdateAppImageConfigInputBuilder) -> impl Future<Output = Result<UpdateAppImageConfigOutput, SdkError<UpdateAppImageConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_artifact(&self, builder: UpdateArtifactInputBuilder) -> impl Future<Output = Result<UpdateArtifactOutput, SdkError<UpdateArtifactError>>> {
        builder.send_with(&self.0)
    }
    fn update_cluster(&self, builder: UpdateClusterInputBuilder) -> impl Future<Output = Result<UpdateClusterOutput, SdkError<UpdateClusterError>>> {
        builder.send_with(&self.0)
    }
    fn update_cluster_software(&self, builder: UpdateClusterSoftwareInputBuilder) -> impl Future<Output = Result<UpdateClusterSoftwareOutput, SdkError<UpdateClusterSoftwareError>>> {
        builder.send_with(&self.0)
    }
    fn update_code_repository(&self, builder: UpdateCodeRepositoryInputBuilder) -> impl Future<Output = Result<UpdateCodeRepositoryOutput, SdkError<UpdateCodeRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn update_context(&self, builder: UpdateContextInputBuilder) -> impl Future<Output = Result<UpdateContextOutput, SdkError<UpdateContextError>>> {
        builder.send_with(&self.0)
    }
    fn update_device_fleet(&self, builder: UpdateDeviceFleetInputBuilder) -> impl Future<Output = Result<UpdateDeviceFleetOutput, SdkError<UpdateDeviceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn update_devices(&self, builder: UpdateDevicesInputBuilder) -> impl Future<Output = Result<UpdateDevicesOutput, SdkError<UpdateDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain(&self, builder: UpdateDomainInputBuilder) -> impl Future<Output = Result<UpdateDomainOutput, SdkError<UpdateDomainError>>> {
        builder.send_with(&self.0)
    }
    fn update_endpoint(&self, builder: UpdateEndpointInputBuilder) -> impl Future<Output = Result<UpdateEndpointOutput, SdkError<UpdateEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn update_endpoint_weights_and_capacities(&self, builder: UpdateEndpointWeightsAndCapacitiesInputBuilder) -> impl Future<Output = Result<UpdateEndpointWeightsAndCapacitiesOutput, SdkError<UpdateEndpointWeightsAndCapacitiesError>>> {
        builder.send_with(&self.0)
    }
    fn update_experiment(&self, builder: UpdateExperimentInputBuilder) -> impl Future<Output = Result<UpdateExperimentOutput, SdkError<UpdateExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn update_feature_group(&self, builder: UpdateFeatureGroupInputBuilder) -> impl Future<Output = Result<UpdateFeatureGroupOutput, SdkError<UpdateFeatureGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_feature_metadata(&self, builder: UpdateFeatureMetadataInputBuilder) -> impl Future<Output = Result<UpdateFeatureMetadataOutput, SdkError<UpdateFeatureMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn update_hub(&self, builder: UpdateHubInputBuilder) -> impl Future<Output = Result<UpdateHubOutput, SdkError<UpdateHubError>>> {
        builder.send_with(&self.0)
    }
    fn update_image(&self, builder: UpdateImageInputBuilder) -> impl Future<Output = Result<UpdateImageOutput, SdkError<UpdateImageError>>> {
        builder.send_with(&self.0)
    }
    fn update_image_version(&self, builder: UpdateImageVersionInputBuilder) -> impl Future<Output = Result<UpdateImageVersionOutput, SdkError<UpdateImageVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_inference_component(&self, builder: UpdateInferenceComponentInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentOutput, SdkError<UpdateInferenceComponentError>>> {
        builder.send_with(&self.0)
    }
    fn update_inference_component_runtime_config(&self, builder: UpdateInferenceComponentRuntimeConfigInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentRuntimeConfigOutput, SdkError<UpdateInferenceComponentRuntimeConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_inference_experiment(&self, builder: UpdateInferenceExperimentInputBuilder) -> impl Future<Output = Result<UpdateInferenceExperimentOutput, SdkError<UpdateInferenceExperimentError>>> {
        builder.send_with(&self.0)
    }
    fn update_mlflow_tracking_server(&self, builder: UpdateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<UpdateMlflowTrackingServerOutput, SdkError<UpdateMlflowTrackingServerError>>> {
        builder.send_with(&self.0)
    }
    fn update_model_card(&self, builder: UpdateModelCardInputBuilder) -> impl Future<Output = Result<UpdateModelCardOutput, SdkError<UpdateModelCardError>>> {
        builder.send_with(&self.0)
    }
    fn update_model_package(&self, builder: UpdateModelPackageInputBuilder) -> impl Future<Output = Result<UpdateModelPackageOutput, SdkError<UpdateModelPackageError>>> {
        builder.send_with(&self.0)
    }
    fn update_monitoring_alert(&self, builder: UpdateMonitoringAlertInputBuilder) -> impl Future<Output = Result<UpdateMonitoringAlertOutput, SdkError<UpdateMonitoringAlertError>>> {
        builder.send_with(&self.0)
    }
    fn update_monitoring_schedule(&self, builder: UpdateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<UpdateMonitoringScheduleOutput, SdkError<UpdateMonitoringScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn update_notebook_instance(&self, builder: UpdateNotebookInstanceInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceOutput, SdkError<UpdateNotebookInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn update_notebook_instance_lifecycle_config(&self, builder: UpdateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceLifecycleConfigOutput, SdkError<UpdateNotebookInstanceLifecycleConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn update_pipeline_execution(&self, builder: UpdatePipelineExecutionInputBuilder) -> impl Future<Output = Result<UpdatePipelineExecutionOutput, SdkError<UpdatePipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn update_space(&self, builder: UpdateSpaceInputBuilder) -> impl Future<Output = Result<UpdateSpaceOutput, SdkError<UpdateSpaceError>>> {
        builder.send_with(&self.0)
    }
    fn update_training_job(&self, builder: UpdateTrainingJobInputBuilder) -> impl Future<Output = Result<UpdateTrainingJobOutput, SdkError<UpdateTrainingJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_trial(&self, builder: UpdateTrialInputBuilder) -> impl Future<Output = Result<UpdateTrialOutput, SdkError<UpdateTrialError>>> {
        builder.send_with(&self.0)
    }
    fn update_trial_component(&self, builder: UpdateTrialComponentInputBuilder) -> impl Future<Output = Result<UpdateTrialComponentOutput, SdkError<UpdateTrialComponentError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_profile(&self, builder: UpdateUserProfileInputBuilder) -> impl Future<Output = Result<UpdateUserProfileOutput, SdkError<UpdateUserProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_workforce(&self, builder: UpdateWorkforceInputBuilder) -> impl Future<Output = Result<UpdateWorkforceOutput, SdkError<UpdateWorkforceError>>> {
        builder.send_with(&self.0)
    }
    fn update_workteam(&self, builder: UpdateWorkteamInputBuilder) -> impl Future<Output = Result<UpdateWorkteamOutput, SdkError<UpdateWorkteamError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> SageMakerClient for T
where T: Deref,
      T::Target: SageMakerClient {
    fn add_association(&self, builder: AddAssociationInputBuilder) -> impl Future<Output = Result<AddAssociationOutput, SdkError<AddAssociationError>>> {
        self.deref().add_association(builder)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        self.deref().add_tags(builder)
    }
    fn associate_trial_component(&self, builder: AssociateTrialComponentInputBuilder) -> impl Future<Output = Result<AssociateTrialComponentOutput, SdkError<AssociateTrialComponentError>>> {
        self.deref().associate_trial_component(builder)
    }
    fn batch_describe_model_package(&self, builder: BatchDescribeModelPackageInputBuilder) -> impl Future<Output = Result<BatchDescribeModelPackageOutput, SdkError<BatchDescribeModelPackageError>>> {
        self.deref().batch_describe_model_package(builder)
    }
    fn create_action(&self, builder: CreateActionInputBuilder) -> impl Future<Output = Result<CreateActionOutput, SdkError<CreateActionError>>> {
        self.deref().create_action(builder)
    }
    fn create_algorithm(&self, builder: CreateAlgorithmInputBuilder) -> impl Future<Output = Result<CreateAlgorithmOutput, SdkError<CreateAlgorithmError>>> {
        self.deref().create_algorithm(builder)
    }
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>> {
        self.deref().create_app(builder)
    }
    fn create_app_image_config(&self, builder: CreateAppImageConfigInputBuilder) -> impl Future<Output = Result<CreateAppImageConfigOutput, SdkError<CreateAppImageConfigError>>> {
        self.deref().create_app_image_config(builder)
    }
    fn create_artifact(&self, builder: CreateArtifactInputBuilder) -> impl Future<Output = Result<CreateArtifactOutput, SdkError<CreateArtifactError>>> {
        self.deref().create_artifact(builder)
    }
    fn create_auto_ml_job(&self, builder: CreateAutoMlJobInputBuilder) -> impl Future<Output = Result<CreateAutoMlJobOutput, SdkError<CreateAutoMLJobError>>> {
        self.deref().create_auto_ml_job(builder)
    }
    fn create_auto_ml_job_v2(&self, builder: CreateAutoMlJobV2InputBuilder) -> impl Future<Output = Result<CreateAutoMlJobV2Output, SdkError<CreateAutoMLJobV2Error>>> {
        self.deref().create_auto_ml_job_v2(builder)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        self.deref().create_cluster(builder)
    }
    fn create_code_repository(&self, builder: CreateCodeRepositoryInputBuilder) -> impl Future<Output = Result<CreateCodeRepositoryOutput, SdkError<CreateCodeRepositoryError>>> {
        self.deref().create_code_repository(builder)
    }
    fn create_compilation_job(&self, builder: CreateCompilationJobInputBuilder) -> impl Future<Output = Result<CreateCompilationJobOutput, SdkError<CreateCompilationJobError>>> {
        self.deref().create_compilation_job(builder)
    }
    fn create_context(&self, builder: CreateContextInputBuilder) -> impl Future<Output = Result<CreateContextOutput, SdkError<CreateContextError>>> {
        self.deref().create_context(builder)
    }
    fn create_data_quality_job_definition(&self, builder: CreateDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateDataQualityJobDefinitionOutput, SdkError<CreateDataQualityJobDefinitionError>>> {
        self.deref().create_data_quality_job_definition(builder)
    }
    fn create_device_fleet(&self, builder: CreateDeviceFleetInputBuilder) -> impl Future<Output = Result<CreateDeviceFleetOutput, SdkError<CreateDeviceFleetError>>> {
        self.deref().create_device_fleet(builder)
    }
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>> {
        self.deref().create_domain(builder)
    }
    fn create_edge_deployment_plan(&self, builder: CreateEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentPlanOutput, SdkError<CreateEdgeDeploymentPlanError>>> {
        self.deref().create_edge_deployment_plan(builder)
    }
    fn create_edge_deployment_stage(&self, builder: CreateEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<CreateEdgeDeploymentStageOutput, SdkError<CreateEdgeDeploymentStageError>>> {
        self.deref().create_edge_deployment_stage(builder)
    }
    fn create_edge_packaging_job(&self, builder: CreateEdgePackagingJobInputBuilder) -> impl Future<Output = Result<CreateEdgePackagingJobOutput, SdkError<CreateEdgePackagingJobError>>> {
        self.deref().create_edge_packaging_job(builder)
    }
    fn create_endpoint(&self, builder: CreateEndpointInputBuilder) -> impl Future<Output = Result<CreateEndpointOutput, SdkError<CreateEndpointError>>> {
        self.deref().create_endpoint(builder)
    }
    fn create_endpoint_config(&self, builder: CreateEndpointConfigInputBuilder) -> impl Future<Output = Result<CreateEndpointConfigOutput, SdkError<CreateEndpointConfigError>>> {
        self.deref().create_endpoint_config(builder)
    }
    fn create_experiment(&self, builder: CreateExperimentInputBuilder) -> impl Future<Output = Result<CreateExperimentOutput, SdkError<CreateExperimentError>>> {
        self.deref().create_experiment(builder)
    }
    fn create_feature_group(&self, builder: CreateFeatureGroupInputBuilder) -> impl Future<Output = Result<CreateFeatureGroupOutput, SdkError<CreateFeatureGroupError>>> {
        self.deref().create_feature_group(builder)
    }
    fn create_flow_definition(&self, builder: CreateFlowDefinitionInputBuilder) -> impl Future<Output = Result<CreateFlowDefinitionOutput, SdkError<CreateFlowDefinitionError>>> {
        self.deref().create_flow_definition(builder)
    }
    fn create_hub(&self, builder: CreateHubInputBuilder) -> impl Future<Output = Result<CreateHubOutput, SdkError<CreateHubError>>> {
        self.deref().create_hub(builder)
    }
    fn create_hub_content_reference(&self, builder: CreateHubContentReferenceInputBuilder) -> impl Future<Output = Result<CreateHubContentReferenceOutput, SdkError<CreateHubContentReferenceError>>> {
        self.deref().create_hub_content_reference(builder)
    }
    fn create_human_task_ui(&self, builder: CreateHumanTaskUiInputBuilder) -> impl Future<Output = Result<CreateHumanTaskUiOutput, SdkError<CreateHumanTaskUiError>>> {
        self.deref().create_human_task_ui(builder)
    }
    fn create_hyper_parameter_tuning_job(&self, builder: CreateHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<CreateHyperParameterTuningJobOutput, SdkError<CreateHyperParameterTuningJobError>>> {
        self.deref().create_hyper_parameter_tuning_job(builder)
    }
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>> {
        self.deref().create_image(builder)
    }
    fn create_image_version(&self, builder: CreateImageVersionInputBuilder) -> impl Future<Output = Result<CreateImageVersionOutput, SdkError<CreateImageVersionError>>> {
        self.deref().create_image_version(builder)
    }
    fn create_inference_component(&self, builder: CreateInferenceComponentInputBuilder) -> impl Future<Output = Result<CreateInferenceComponentOutput, SdkError<CreateInferenceComponentError>>> {
        self.deref().create_inference_component(builder)
    }
    fn create_inference_experiment(&self, builder: CreateInferenceExperimentInputBuilder) -> impl Future<Output = Result<CreateInferenceExperimentOutput, SdkError<CreateInferenceExperimentError>>> {
        self.deref().create_inference_experiment(builder)
    }
    fn create_inference_recommendations_job(&self, builder: CreateInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<CreateInferenceRecommendationsJobOutput, SdkError<CreateInferenceRecommendationsJobError>>> {
        self.deref().create_inference_recommendations_job(builder)
    }
    fn create_labeling_job(&self, builder: CreateLabelingJobInputBuilder) -> impl Future<Output = Result<CreateLabelingJobOutput, SdkError<CreateLabelingJobError>>> {
        self.deref().create_labeling_job(builder)
    }
    fn create_mlflow_tracking_server(&self, builder: CreateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<CreateMlflowTrackingServerOutput, SdkError<CreateMlflowTrackingServerError>>> {
        self.deref().create_mlflow_tracking_server(builder)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        self.deref().create_model(builder)
    }
    fn create_model_bias_job_definition(&self, builder: CreateModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelBiasJobDefinitionOutput, SdkError<CreateModelBiasJobDefinitionError>>> {
        self.deref().create_model_bias_job_definition(builder)
    }
    fn create_model_card(&self, builder: CreateModelCardInputBuilder) -> impl Future<Output = Result<CreateModelCardOutput, SdkError<CreateModelCardError>>> {
        self.deref().create_model_card(builder)
    }
    fn create_model_card_export_job(&self, builder: CreateModelCardExportJobInputBuilder) -> impl Future<Output = Result<CreateModelCardExportJobOutput, SdkError<CreateModelCardExportJobError>>> {
        self.deref().create_model_card_export_job(builder)
    }
    fn create_model_explainability_job_definition(&self, builder: CreateModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelExplainabilityJobDefinitionOutput, SdkError<CreateModelExplainabilityJobDefinitionError>>> {
        self.deref().create_model_explainability_job_definition(builder)
    }
    fn create_model_package(&self, builder: CreateModelPackageInputBuilder) -> impl Future<Output = Result<CreateModelPackageOutput, SdkError<CreateModelPackageError>>> {
        self.deref().create_model_package(builder)
    }
    fn create_model_package_group(&self, builder: CreateModelPackageGroupInputBuilder) -> impl Future<Output = Result<CreateModelPackageGroupOutput, SdkError<CreateModelPackageGroupError>>> {
        self.deref().create_model_package_group(builder)
    }
    fn create_model_quality_job_definition(&self, builder: CreateModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<CreateModelQualityJobDefinitionOutput, SdkError<CreateModelQualityJobDefinitionError>>> {
        self.deref().create_model_quality_job_definition(builder)
    }
    fn create_monitoring_schedule(&self, builder: CreateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<CreateMonitoringScheduleOutput, SdkError<CreateMonitoringScheduleError>>> {
        self.deref().create_monitoring_schedule(builder)
    }
    fn create_notebook_instance(&self, builder: CreateNotebookInstanceInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceOutput, SdkError<CreateNotebookInstanceError>>> {
        self.deref().create_notebook_instance(builder)
    }
    fn create_notebook_instance_lifecycle_config(&self, builder: CreateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateNotebookInstanceLifecycleConfigOutput, SdkError<CreateNotebookInstanceLifecycleConfigError>>> {
        self.deref().create_notebook_instance_lifecycle_config(builder)
    }
    fn create_optimization_job(&self, builder: CreateOptimizationJobInputBuilder) -> impl Future<Output = Result<CreateOptimizationJobOutput, SdkError<CreateOptimizationJobError>>> {
        self.deref().create_optimization_job(builder)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        self.deref().create_pipeline(builder)
    }
    fn create_presigned_domain_url(&self, builder: CreatePresignedDomainUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedDomainUrlOutput, SdkError<CreatePresignedDomainUrlError>>> {
        self.deref().create_presigned_domain_url(builder)
    }
    fn create_presigned_mlflow_tracking_server_url(&self, builder: CreatePresignedMlflowTrackingServerUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedMlflowTrackingServerUrlOutput, SdkError<CreatePresignedMlflowTrackingServerUrlError>>> {
        self.deref().create_presigned_mlflow_tracking_server_url(builder)
    }
    fn create_presigned_notebook_instance_url(&self, builder: CreatePresignedNotebookInstanceUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookInstanceUrlOutput, SdkError<CreatePresignedNotebookInstanceUrlError>>> {
        self.deref().create_presigned_notebook_instance_url(builder)
    }
    fn create_processing_job(&self, builder: CreateProcessingJobInputBuilder) -> impl Future<Output = Result<CreateProcessingJobOutput, SdkError<CreateProcessingJobError>>> {
        self.deref().create_processing_job(builder)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        self.deref().create_project(builder)
    }
    fn create_space(&self, builder: CreateSpaceInputBuilder) -> impl Future<Output = Result<CreateSpaceOutput, SdkError<CreateSpaceError>>> {
        self.deref().create_space(builder)
    }
    fn create_studio_lifecycle_config(&self, builder: CreateStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<CreateStudioLifecycleConfigOutput, SdkError<CreateStudioLifecycleConfigError>>> {
        self.deref().create_studio_lifecycle_config(builder)
    }
    fn create_training_job(&self, builder: CreateTrainingJobInputBuilder) -> impl Future<Output = Result<CreateTrainingJobOutput, SdkError<CreateTrainingJobError>>> {
        self.deref().create_training_job(builder)
    }
    fn create_transform_job(&self, builder: CreateTransformJobInputBuilder) -> impl Future<Output = Result<CreateTransformJobOutput, SdkError<CreateTransformJobError>>> {
        self.deref().create_transform_job(builder)
    }
    fn create_trial(&self, builder: CreateTrialInputBuilder) -> impl Future<Output = Result<CreateTrialOutput, SdkError<CreateTrialError>>> {
        self.deref().create_trial(builder)
    }
    fn create_trial_component(&self, builder: CreateTrialComponentInputBuilder) -> impl Future<Output = Result<CreateTrialComponentOutput, SdkError<CreateTrialComponentError>>> {
        self.deref().create_trial_component(builder)
    }
    fn create_user_profile(&self, builder: CreateUserProfileInputBuilder) -> impl Future<Output = Result<CreateUserProfileOutput, SdkError<CreateUserProfileError>>> {
        self.deref().create_user_profile(builder)
    }
    fn create_workforce(&self, builder: CreateWorkforceInputBuilder) -> impl Future<Output = Result<CreateWorkforceOutput, SdkError<CreateWorkforceError>>> {
        self.deref().create_workforce(builder)
    }
    fn create_workteam(&self, builder: CreateWorkteamInputBuilder) -> impl Future<Output = Result<CreateWorkteamOutput, SdkError<CreateWorkteamError>>> {
        self.deref().create_workteam(builder)
    }
    fn delete_action(&self, builder: DeleteActionInputBuilder) -> impl Future<Output = Result<DeleteActionOutput, SdkError<DeleteActionError>>> {
        self.deref().delete_action(builder)
    }
    fn delete_algorithm(&self, builder: DeleteAlgorithmInputBuilder) -> impl Future<Output = Result<DeleteAlgorithmOutput, SdkError<DeleteAlgorithmError>>> {
        self.deref().delete_algorithm(builder)
    }
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>> {
        self.deref().delete_app(builder)
    }
    fn delete_app_image_config(&self, builder: DeleteAppImageConfigInputBuilder) -> impl Future<Output = Result<DeleteAppImageConfigOutput, SdkError<DeleteAppImageConfigError>>> {
        self.deref().delete_app_image_config(builder)
    }
    fn delete_artifact(&self, builder: DeleteArtifactInputBuilder) -> impl Future<Output = Result<DeleteArtifactOutput, SdkError<DeleteArtifactError>>> {
        self.deref().delete_artifact(builder)
    }
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>> {
        self.deref().delete_association(builder)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        self.deref().delete_cluster(builder)
    }
    fn delete_code_repository(&self, builder: DeleteCodeRepositoryInputBuilder) -> impl Future<Output = Result<DeleteCodeRepositoryOutput, SdkError<DeleteCodeRepositoryError>>> {
        self.deref().delete_code_repository(builder)
    }
    fn delete_compilation_job(&self, builder: DeleteCompilationJobInputBuilder) -> impl Future<Output = Result<DeleteCompilationJobOutput, SdkError<DeleteCompilationJobError>>> {
        self.deref().delete_compilation_job(builder)
    }
    fn delete_context(&self, builder: DeleteContextInputBuilder) -> impl Future<Output = Result<DeleteContextOutput, SdkError<DeleteContextError>>> {
        self.deref().delete_context(builder)
    }
    fn delete_data_quality_job_definition(&self, builder: DeleteDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteDataQualityJobDefinitionOutput, SdkError<DeleteDataQualityJobDefinitionError>>> {
        self.deref().delete_data_quality_job_definition(builder)
    }
    fn delete_device_fleet(&self, builder: DeleteDeviceFleetInputBuilder) -> impl Future<Output = Result<DeleteDeviceFleetOutput, SdkError<DeleteDeviceFleetError>>> {
        self.deref().delete_device_fleet(builder)
    }
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>> {
        self.deref().delete_domain(builder)
    }
    fn delete_edge_deployment_plan(&self, builder: DeleteEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentPlanOutput, SdkError<DeleteEdgeDeploymentPlanError>>> {
        self.deref().delete_edge_deployment_plan(builder)
    }
    fn delete_edge_deployment_stage(&self, builder: DeleteEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<DeleteEdgeDeploymentStageOutput, SdkError<DeleteEdgeDeploymentStageError>>> {
        self.deref().delete_edge_deployment_stage(builder)
    }
    fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> impl Future<Output = Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>> {
        self.deref().delete_endpoint(builder)
    }
    fn delete_endpoint_config(&self, builder: DeleteEndpointConfigInputBuilder) -> impl Future<Output = Result<DeleteEndpointConfigOutput, SdkError<DeleteEndpointConfigError>>> {
        self.deref().delete_endpoint_config(builder)
    }
    fn delete_experiment(&self, builder: DeleteExperimentInputBuilder) -> impl Future<Output = Result<DeleteExperimentOutput, SdkError<DeleteExperimentError>>> {
        self.deref().delete_experiment(builder)
    }
    fn delete_feature_group(&self, builder: DeleteFeatureGroupInputBuilder) -> impl Future<Output = Result<DeleteFeatureGroupOutput, SdkError<DeleteFeatureGroupError>>> {
        self.deref().delete_feature_group(builder)
    }
    fn delete_flow_definition(&self, builder: DeleteFlowDefinitionInputBuilder) -> impl Future<Output = Result<DeleteFlowDefinitionOutput, SdkError<DeleteFlowDefinitionError>>> {
        self.deref().delete_flow_definition(builder)
    }
    fn delete_hub(&self, builder: DeleteHubInputBuilder) -> impl Future<Output = Result<DeleteHubOutput, SdkError<DeleteHubError>>> {
        self.deref().delete_hub(builder)
    }
    fn delete_hub_content(&self, builder: DeleteHubContentInputBuilder) -> impl Future<Output = Result<DeleteHubContentOutput, SdkError<DeleteHubContentError>>> {
        self.deref().delete_hub_content(builder)
    }
    fn delete_hub_content_reference(&self, builder: DeleteHubContentReferenceInputBuilder) -> impl Future<Output = Result<DeleteHubContentReferenceOutput, SdkError<DeleteHubContentReferenceError>>> {
        self.deref().delete_hub_content_reference(builder)
    }
    fn delete_human_task_ui(&self, builder: DeleteHumanTaskUiInputBuilder) -> impl Future<Output = Result<DeleteHumanTaskUiOutput, SdkError<DeleteHumanTaskUiError>>> {
        self.deref().delete_human_task_ui(builder)
    }
    fn delete_hyper_parameter_tuning_job(&self, builder: DeleteHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DeleteHyperParameterTuningJobOutput, SdkError<DeleteHyperParameterTuningJobError>>> {
        self.deref().delete_hyper_parameter_tuning_job(builder)
    }
    fn delete_image(&self, builder: DeleteImageInputBuilder) -> impl Future<Output = Result<DeleteImageOutput, SdkError<DeleteImageError>>> {
        self.deref().delete_image(builder)
    }
    fn delete_image_version(&self, builder: DeleteImageVersionInputBuilder) -> impl Future<Output = Result<DeleteImageVersionOutput, SdkError<DeleteImageVersionError>>> {
        self.deref().delete_image_version(builder)
    }
    fn delete_inference_component(&self, builder: DeleteInferenceComponentInputBuilder) -> impl Future<Output = Result<DeleteInferenceComponentOutput, SdkError<DeleteInferenceComponentError>>> {
        self.deref().delete_inference_component(builder)
    }
    fn delete_inference_experiment(&self, builder: DeleteInferenceExperimentInputBuilder) -> impl Future<Output = Result<DeleteInferenceExperimentOutput, SdkError<DeleteInferenceExperimentError>>> {
        self.deref().delete_inference_experiment(builder)
    }
    fn delete_mlflow_tracking_server(&self, builder: DeleteMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DeleteMlflowTrackingServerOutput, SdkError<DeleteMlflowTrackingServerError>>> {
        self.deref().delete_mlflow_tracking_server(builder)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        self.deref().delete_model(builder)
    }
    fn delete_model_bias_job_definition(&self, builder: DeleteModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelBiasJobDefinitionOutput, SdkError<DeleteModelBiasJobDefinitionError>>> {
        self.deref().delete_model_bias_job_definition(builder)
    }
    fn delete_model_card(&self, builder: DeleteModelCardInputBuilder) -> impl Future<Output = Result<DeleteModelCardOutput, SdkError<DeleteModelCardError>>> {
        self.deref().delete_model_card(builder)
    }
    fn delete_model_explainability_job_definition(&self, builder: DeleteModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelExplainabilityJobDefinitionOutput, SdkError<DeleteModelExplainabilityJobDefinitionError>>> {
        self.deref().delete_model_explainability_job_definition(builder)
    }
    fn delete_model_package(&self, builder: DeleteModelPackageInputBuilder) -> impl Future<Output = Result<DeleteModelPackageOutput, SdkError<DeleteModelPackageError>>> {
        self.deref().delete_model_package(builder)
    }
    fn delete_model_package_group(&self, builder: DeleteModelPackageGroupInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupOutput, SdkError<DeleteModelPackageGroupError>>> {
        self.deref().delete_model_package_group(builder)
    }
    fn delete_model_package_group_policy(&self, builder: DeleteModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteModelPackageGroupPolicyOutput, SdkError<DeleteModelPackageGroupPolicyError>>> {
        self.deref().delete_model_package_group_policy(builder)
    }
    fn delete_model_quality_job_definition(&self, builder: DeleteModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DeleteModelQualityJobDefinitionOutput, SdkError<DeleteModelQualityJobDefinitionError>>> {
        self.deref().delete_model_quality_job_definition(builder)
    }
    fn delete_monitoring_schedule(&self, builder: DeleteMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DeleteMonitoringScheduleOutput, SdkError<DeleteMonitoringScheduleError>>> {
        self.deref().delete_monitoring_schedule(builder)
    }
    fn delete_notebook_instance(&self, builder: DeleteNotebookInstanceInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceOutput, SdkError<DeleteNotebookInstanceError>>> {
        self.deref().delete_notebook_instance(builder)
    }
    fn delete_notebook_instance_lifecycle_config(&self, builder: DeleteNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteNotebookInstanceLifecycleConfigOutput, SdkError<DeleteNotebookInstanceLifecycleConfigError>>> {
        self.deref().delete_notebook_instance_lifecycle_config(builder)
    }
    fn delete_optimization_job(&self, builder: DeleteOptimizationJobInputBuilder) -> impl Future<Output = Result<DeleteOptimizationJobOutput, SdkError<DeleteOptimizationJobError>>> {
        self.deref().delete_optimization_job(builder)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        self.deref().delete_pipeline(builder)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        self.deref().delete_project(builder)
    }
    fn delete_space(&self, builder: DeleteSpaceInputBuilder) -> impl Future<Output = Result<DeleteSpaceOutput, SdkError<DeleteSpaceError>>> {
        self.deref().delete_space(builder)
    }
    fn delete_studio_lifecycle_config(&self, builder: DeleteStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DeleteStudioLifecycleConfigOutput, SdkError<DeleteStudioLifecycleConfigError>>> {
        self.deref().delete_studio_lifecycle_config(builder)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        self.deref().delete_tags(builder)
    }
    fn delete_trial(&self, builder: DeleteTrialInputBuilder) -> impl Future<Output = Result<DeleteTrialOutput, SdkError<DeleteTrialError>>> {
        self.deref().delete_trial(builder)
    }
    fn delete_trial_component(&self, builder: DeleteTrialComponentInputBuilder) -> impl Future<Output = Result<DeleteTrialComponentOutput, SdkError<DeleteTrialComponentError>>> {
        self.deref().delete_trial_component(builder)
    }
    fn delete_user_profile(&self, builder: DeleteUserProfileInputBuilder) -> impl Future<Output = Result<DeleteUserProfileOutput, SdkError<DeleteUserProfileError>>> {
        self.deref().delete_user_profile(builder)
    }
    fn delete_workforce(&self, builder: DeleteWorkforceInputBuilder) -> impl Future<Output = Result<DeleteWorkforceOutput, SdkError<DeleteWorkforceError>>> {
        self.deref().delete_workforce(builder)
    }
    fn delete_workteam(&self, builder: DeleteWorkteamInputBuilder) -> impl Future<Output = Result<DeleteWorkteamOutput, SdkError<DeleteWorkteamError>>> {
        self.deref().delete_workteam(builder)
    }
    fn deregister_devices(&self, builder: DeregisterDevicesInputBuilder) -> impl Future<Output = Result<DeregisterDevicesOutput, SdkError<DeregisterDevicesError>>> {
        self.deref().deregister_devices(builder)
    }
    fn describe_action(&self, builder: DescribeActionInputBuilder) -> impl Future<Output = Result<DescribeActionOutput, SdkError<DescribeActionError>>> {
        self.deref().describe_action(builder)
    }
    fn describe_algorithm(&self, builder: DescribeAlgorithmInputBuilder) -> impl Future<Output = Result<DescribeAlgorithmOutput, SdkError<DescribeAlgorithmError>>> {
        self.deref().describe_algorithm(builder)
    }
    fn describe_app(&self, builder: DescribeAppInputBuilder) -> impl Future<Output = Result<DescribeAppOutput, SdkError<DescribeAppError>>> {
        self.deref().describe_app(builder)
    }
    fn describe_app_image_config(&self, builder: DescribeAppImageConfigInputBuilder) -> impl Future<Output = Result<DescribeAppImageConfigOutput, SdkError<DescribeAppImageConfigError>>> {
        self.deref().describe_app_image_config(builder)
    }
    fn describe_artifact(&self, builder: DescribeArtifactInputBuilder) -> impl Future<Output = Result<DescribeArtifactOutput, SdkError<DescribeArtifactError>>> {
        self.deref().describe_artifact(builder)
    }
    fn describe_auto_ml_job(&self, builder: DescribeAutoMlJobInputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobOutput, SdkError<DescribeAutoMLJobError>>> {
        self.deref().describe_auto_ml_job(builder)
    }
    fn describe_auto_ml_job_v2(&self, builder: DescribeAutoMlJobV2InputBuilder) -> impl Future<Output = Result<DescribeAutoMlJobV2Output, SdkError<DescribeAutoMLJobV2Error>>> {
        self.deref().describe_auto_ml_job_v2(builder)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        self.deref().describe_cluster(builder)
    }
    fn describe_cluster_node(&self, builder: DescribeClusterNodeInputBuilder) -> impl Future<Output = Result<DescribeClusterNodeOutput, SdkError<DescribeClusterNodeError>>> {
        self.deref().describe_cluster_node(builder)
    }
    fn describe_code_repository(&self, builder: DescribeCodeRepositoryInputBuilder) -> impl Future<Output = Result<DescribeCodeRepositoryOutput, SdkError<DescribeCodeRepositoryError>>> {
        self.deref().describe_code_repository(builder)
    }
    fn describe_compilation_job(&self, builder: DescribeCompilationJobInputBuilder) -> impl Future<Output = Result<DescribeCompilationJobOutput, SdkError<DescribeCompilationJobError>>> {
        self.deref().describe_compilation_job(builder)
    }
    fn describe_context(&self, builder: DescribeContextInputBuilder) -> impl Future<Output = Result<DescribeContextOutput, SdkError<DescribeContextError>>> {
        self.deref().describe_context(builder)
    }
    fn describe_data_quality_job_definition(&self, builder: DescribeDataQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDataQualityJobDefinitionOutput, SdkError<DescribeDataQualityJobDefinitionError>>> {
        self.deref().describe_data_quality_job_definition(builder)
    }
    fn describe_device(&self, builder: DescribeDeviceInputBuilder) -> impl Future<Output = Result<DescribeDeviceOutput, SdkError<DescribeDeviceError>>> {
        self.deref().describe_device(builder)
    }
    fn describe_device_fleet(&self, builder: DescribeDeviceFleetInputBuilder) -> impl Future<Output = Result<DescribeDeviceFleetOutput, SdkError<DescribeDeviceFleetError>>> {
        self.deref().describe_device_fleet(builder)
    }
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>> {
        self.deref().describe_domain(builder)
    }
    fn describe_edge_deployment_plan(&self, builder: DescribeEdgeDeploymentPlanInputBuilder) -> impl Future<Output = Result<DescribeEdgeDeploymentPlanOutput, SdkError<DescribeEdgeDeploymentPlanError>>> {
        self.deref().describe_edge_deployment_plan(builder)
    }
    fn describe_edge_packaging_job(&self, builder: DescribeEdgePackagingJobInputBuilder) -> impl Future<Output = Result<DescribeEdgePackagingJobOutput, SdkError<DescribeEdgePackagingJobError>>> {
        self.deref().describe_edge_packaging_job(builder)
    }
    fn describe_endpoint(&self, builder: DescribeEndpointInputBuilder) -> impl Future<Output = Result<DescribeEndpointOutput, SdkError<DescribeEndpointError>>> {
        self.deref().describe_endpoint(builder)
    }
    fn describe_endpoint_config(&self, builder: DescribeEndpointConfigInputBuilder) -> impl Future<Output = Result<DescribeEndpointConfigOutput, SdkError<DescribeEndpointConfigError>>> {
        self.deref().describe_endpoint_config(builder)
    }
    fn describe_experiment(&self, builder: DescribeExperimentInputBuilder) -> impl Future<Output = Result<DescribeExperimentOutput, SdkError<DescribeExperimentError>>> {
        self.deref().describe_experiment(builder)
    }
    fn describe_feature_group(&self, builder: DescribeFeatureGroupInputBuilder) -> impl Future<Output = Result<DescribeFeatureGroupOutput, SdkError<DescribeFeatureGroupError>>> {
        self.deref().describe_feature_group(builder)
    }
    fn describe_feature_metadata(&self, builder: DescribeFeatureMetadataInputBuilder) -> impl Future<Output = Result<DescribeFeatureMetadataOutput, SdkError<DescribeFeatureMetadataError>>> {
        self.deref().describe_feature_metadata(builder)
    }
    fn describe_flow_definition(&self, builder: DescribeFlowDefinitionInputBuilder) -> impl Future<Output = Result<DescribeFlowDefinitionOutput, SdkError<DescribeFlowDefinitionError>>> {
        self.deref().describe_flow_definition(builder)
    }
    fn describe_hub(&self, builder: DescribeHubInputBuilder) -> impl Future<Output = Result<DescribeHubOutput, SdkError<DescribeHubError>>> {
        self.deref().describe_hub(builder)
    }
    fn describe_hub_content(&self, builder: DescribeHubContentInputBuilder) -> impl Future<Output = Result<DescribeHubContentOutput, SdkError<DescribeHubContentError>>> {
        self.deref().describe_hub_content(builder)
    }
    fn describe_human_task_ui(&self, builder: DescribeHumanTaskUiInputBuilder) -> impl Future<Output = Result<DescribeHumanTaskUiOutput, SdkError<DescribeHumanTaskUiError>>> {
        self.deref().describe_human_task_ui(builder)
    }
    fn describe_hyper_parameter_tuning_job(&self, builder: DescribeHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<DescribeHyperParameterTuningJobOutput, SdkError<DescribeHyperParameterTuningJobError>>> {
        self.deref().describe_hyper_parameter_tuning_job(builder)
    }
    fn describe_image(&self, builder: DescribeImageInputBuilder) -> impl Future<Output = Result<DescribeImageOutput, SdkError<DescribeImageError>>> {
        self.deref().describe_image(builder)
    }
    fn describe_image_version(&self, builder: DescribeImageVersionInputBuilder) -> impl Future<Output = Result<DescribeImageVersionOutput, SdkError<DescribeImageVersionError>>> {
        self.deref().describe_image_version(builder)
    }
    fn describe_inference_component(&self, builder: DescribeInferenceComponentInputBuilder) -> impl Future<Output = Result<DescribeInferenceComponentOutput, SdkError<DescribeInferenceComponentError>>> {
        self.deref().describe_inference_component(builder)
    }
    fn describe_inference_experiment(&self, builder: DescribeInferenceExperimentInputBuilder) -> impl Future<Output = Result<DescribeInferenceExperimentOutput, SdkError<DescribeInferenceExperimentError>>> {
        self.deref().describe_inference_experiment(builder)
    }
    fn describe_inference_recommendations_job(&self, builder: DescribeInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<DescribeInferenceRecommendationsJobOutput, SdkError<DescribeInferenceRecommendationsJobError>>> {
        self.deref().describe_inference_recommendations_job(builder)
    }
    fn describe_labeling_job(&self, builder: DescribeLabelingJobInputBuilder) -> impl Future<Output = Result<DescribeLabelingJobOutput, SdkError<DescribeLabelingJobError>>> {
        self.deref().describe_labeling_job(builder)
    }
    fn describe_lineage_group(&self, builder: DescribeLineageGroupInputBuilder) -> impl Future<Output = Result<DescribeLineageGroupOutput, SdkError<DescribeLineageGroupError>>> {
        self.deref().describe_lineage_group(builder)
    }
    fn describe_mlflow_tracking_server(&self, builder: DescribeMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<DescribeMlflowTrackingServerOutput, SdkError<DescribeMlflowTrackingServerError>>> {
        self.deref().describe_mlflow_tracking_server(builder)
    }
    fn describe_model(&self, builder: DescribeModelInputBuilder) -> impl Future<Output = Result<DescribeModelOutput, SdkError<DescribeModelError>>> {
        self.deref().describe_model(builder)
    }
    fn describe_model_bias_job_definition(&self, builder: DescribeModelBiasJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelBiasJobDefinitionOutput, SdkError<DescribeModelBiasJobDefinitionError>>> {
        self.deref().describe_model_bias_job_definition(builder)
    }
    fn describe_model_card(&self, builder: DescribeModelCardInputBuilder) -> impl Future<Output = Result<DescribeModelCardOutput, SdkError<DescribeModelCardError>>> {
        self.deref().describe_model_card(builder)
    }
    fn describe_model_card_export_job(&self, builder: DescribeModelCardExportJobInputBuilder) -> impl Future<Output = Result<DescribeModelCardExportJobOutput, SdkError<DescribeModelCardExportJobError>>> {
        self.deref().describe_model_card_export_job(builder)
    }
    fn describe_model_explainability_job_definition(&self, builder: DescribeModelExplainabilityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelExplainabilityJobDefinitionOutput, SdkError<DescribeModelExplainabilityJobDefinitionError>>> {
        self.deref().describe_model_explainability_job_definition(builder)
    }
    fn describe_model_package(&self, builder: DescribeModelPackageInputBuilder) -> impl Future<Output = Result<DescribeModelPackageOutput, SdkError<DescribeModelPackageError>>> {
        self.deref().describe_model_package(builder)
    }
    fn describe_model_package_group(&self, builder: DescribeModelPackageGroupInputBuilder) -> impl Future<Output = Result<DescribeModelPackageGroupOutput, SdkError<DescribeModelPackageGroupError>>> {
        self.deref().describe_model_package_group(builder)
    }
    fn describe_model_quality_job_definition(&self, builder: DescribeModelQualityJobDefinitionInputBuilder) -> impl Future<Output = Result<DescribeModelQualityJobDefinitionOutput, SdkError<DescribeModelQualityJobDefinitionError>>> {
        self.deref().describe_model_quality_job_definition(builder)
    }
    fn describe_monitoring_schedule(&self, builder: DescribeMonitoringScheduleInputBuilder) -> impl Future<Output = Result<DescribeMonitoringScheduleOutput, SdkError<DescribeMonitoringScheduleError>>> {
        self.deref().describe_monitoring_schedule(builder)
    }
    fn describe_notebook_instance(&self, builder: DescribeNotebookInstanceInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceOutput, SdkError<DescribeNotebookInstanceError>>> {
        self.deref().describe_notebook_instance(builder)
    }
    fn describe_notebook_instance_lifecycle_config(&self, builder: DescribeNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeNotebookInstanceLifecycleConfigOutput, SdkError<DescribeNotebookInstanceLifecycleConfigError>>> {
        self.deref().describe_notebook_instance_lifecycle_config(builder)
    }
    fn describe_optimization_job(&self, builder: DescribeOptimizationJobInputBuilder) -> impl Future<Output = Result<DescribeOptimizationJobOutput, SdkError<DescribeOptimizationJobError>>> {
        self.deref().describe_optimization_job(builder)
    }
    fn describe_pipeline(&self, builder: DescribePipelineInputBuilder) -> impl Future<Output = Result<DescribePipelineOutput, SdkError<DescribePipelineError>>> {
        self.deref().describe_pipeline(builder)
    }
    fn describe_pipeline_definition_for_execution(&self, builder: DescribePipelineDefinitionForExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineDefinitionForExecutionOutput, SdkError<DescribePipelineDefinitionForExecutionError>>> {
        self.deref().describe_pipeline_definition_for_execution(builder)
    }
    fn describe_pipeline_execution(&self, builder: DescribePipelineExecutionInputBuilder) -> impl Future<Output = Result<DescribePipelineExecutionOutput, SdkError<DescribePipelineExecutionError>>> {
        self.deref().describe_pipeline_execution(builder)
    }
    fn describe_processing_job(&self, builder: DescribeProcessingJobInputBuilder) -> impl Future<Output = Result<DescribeProcessingJobOutput, SdkError<DescribeProcessingJobError>>> {
        self.deref().describe_processing_job(builder)
    }
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>> {
        self.deref().describe_project(builder)
    }
    fn describe_space(&self, builder: DescribeSpaceInputBuilder) -> impl Future<Output = Result<DescribeSpaceOutput, SdkError<DescribeSpaceError>>> {
        self.deref().describe_space(builder)
    }
    fn describe_studio_lifecycle_config(&self, builder: DescribeStudioLifecycleConfigInputBuilder) -> impl Future<Output = Result<DescribeStudioLifecycleConfigOutput, SdkError<DescribeStudioLifecycleConfigError>>> {
        self.deref().describe_studio_lifecycle_config(builder)
    }
    fn describe_subscribed_workteam(&self, builder: DescribeSubscribedWorkteamInputBuilder) -> impl Future<Output = Result<DescribeSubscribedWorkteamOutput, SdkError<DescribeSubscribedWorkteamError>>> {
        self.deref().describe_subscribed_workteam(builder)
    }
    fn describe_training_job(&self, builder: DescribeTrainingJobInputBuilder) -> impl Future<Output = Result<DescribeTrainingJobOutput, SdkError<DescribeTrainingJobError>>> {
        self.deref().describe_training_job(builder)
    }
    fn describe_transform_job(&self, builder: DescribeTransformJobInputBuilder) -> impl Future<Output = Result<DescribeTransformJobOutput, SdkError<DescribeTransformJobError>>> {
        self.deref().describe_transform_job(builder)
    }
    fn describe_trial(&self, builder: DescribeTrialInputBuilder) -> impl Future<Output = Result<DescribeTrialOutput, SdkError<DescribeTrialError>>> {
        self.deref().describe_trial(builder)
    }
    fn describe_trial_component(&self, builder: DescribeTrialComponentInputBuilder) -> impl Future<Output = Result<DescribeTrialComponentOutput, SdkError<DescribeTrialComponentError>>> {
        self.deref().describe_trial_component(builder)
    }
    fn describe_user_profile(&self, builder: DescribeUserProfileInputBuilder) -> impl Future<Output = Result<DescribeUserProfileOutput, SdkError<DescribeUserProfileError>>> {
        self.deref().describe_user_profile(builder)
    }
    fn describe_workforce(&self, builder: DescribeWorkforceInputBuilder) -> impl Future<Output = Result<DescribeWorkforceOutput, SdkError<DescribeWorkforceError>>> {
        self.deref().describe_workforce(builder)
    }
    fn describe_workteam(&self, builder: DescribeWorkteamInputBuilder) -> impl Future<Output = Result<DescribeWorkteamOutput, SdkError<DescribeWorkteamError>>> {
        self.deref().describe_workteam(builder)
    }
    fn disable_sagemaker_servicecatalog_portfolio(&self, builder: DisableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<DisableSagemakerServicecatalogPortfolioOutput, SdkError<DisableSagemakerServicecatalogPortfolioError>>> {
        self.deref().disable_sagemaker_servicecatalog_portfolio(builder)
    }
    fn disassociate_trial_component(&self, builder: DisassociateTrialComponentInputBuilder) -> impl Future<Output = Result<DisassociateTrialComponentOutput, SdkError<DisassociateTrialComponentError>>> {
        self.deref().disassociate_trial_component(builder)
    }
    fn enable_sagemaker_servicecatalog_portfolio(&self, builder: EnableSagemakerServicecatalogPortfolioInputBuilder) -> impl Future<Output = Result<EnableSagemakerServicecatalogPortfolioOutput, SdkError<EnableSagemakerServicecatalogPortfolioError>>> {
        self.deref().enable_sagemaker_servicecatalog_portfolio(builder)
    }
    fn get_device_fleet_report(&self, builder: GetDeviceFleetReportInputBuilder) -> impl Future<Output = Result<GetDeviceFleetReportOutput, SdkError<GetDeviceFleetReportError>>> {
        self.deref().get_device_fleet_report(builder)
    }
    fn get_lineage_group_policy(&self, builder: GetLineageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetLineageGroupPolicyOutput, SdkError<GetLineageGroupPolicyError>>> {
        self.deref().get_lineage_group_policy(builder)
    }
    fn get_model_package_group_policy(&self, builder: GetModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<GetModelPackageGroupPolicyOutput, SdkError<GetModelPackageGroupPolicyError>>> {
        self.deref().get_model_package_group_policy(builder)
    }
    fn get_sagemaker_servicecatalog_portfolio_status(&self, builder: GetSagemakerServicecatalogPortfolioStatusInputBuilder) -> impl Future<Output = Result<GetSagemakerServicecatalogPortfolioStatusOutput, SdkError<GetSagemakerServicecatalogPortfolioStatusError>>> {
        self.deref().get_sagemaker_servicecatalog_portfolio_status(builder)
    }
    fn get_scaling_configuration_recommendation(&self, builder: GetScalingConfigurationRecommendationInputBuilder) -> impl Future<Output = Result<GetScalingConfigurationRecommendationOutput, SdkError<GetScalingConfigurationRecommendationError>>> {
        self.deref().get_scaling_configuration_recommendation(builder)
    }
    fn get_search_suggestions(&self, builder: GetSearchSuggestionsInputBuilder) -> impl Future<Output = Result<GetSearchSuggestionsOutput, SdkError<GetSearchSuggestionsError>>> {
        self.deref().get_search_suggestions(builder)
    }
    fn import_hub_content(&self, builder: ImportHubContentInputBuilder) -> impl Future<Output = Result<ImportHubContentOutput, SdkError<ImportHubContentError>>> {
        self.deref().import_hub_content(builder)
    }
    fn list_actions(&self, builder: ListActionsInputBuilder) -> impl Future<Output = Result<ListActionsOutput, SdkError<ListActionsError>>> {
        self.deref().list_actions(builder)
    }
    fn list_algorithms(&self, builder: ListAlgorithmsInputBuilder) -> impl Future<Output = Result<ListAlgorithmsOutput, SdkError<ListAlgorithmsError>>> {
        self.deref().list_algorithms(builder)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        self.deref().list_aliases(builder)
    }
    fn list_app_image_configs(&self, builder: ListAppImageConfigsInputBuilder) -> impl Future<Output = Result<ListAppImageConfigsOutput, SdkError<ListAppImageConfigsError>>> {
        self.deref().list_app_image_configs(builder)
    }
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>> {
        self.deref().list_apps(builder)
    }
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>> {
        self.deref().list_artifacts(builder)
    }
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>> {
        self.deref().list_associations(builder)
    }
    fn list_auto_ml_jobs(&self, builder: ListAutoMlJobsInputBuilder) -> impl Future<Output = Result<ListAutoMlJobsOutput, SdkError<ListAutoMLJobsError>>> {
        self.deref().list_auto_ml_jobs(builder)
    }
    fn list_candidates_for_auto_ml_job(&self, builder: ListCandidatesForAutoMlJobInputBuilder) -> impl Future<Output = Result<ListCandidatesForAutoMlJobOutput, SdkError<ListCandidatesForAutoMLJobError>>> {
        self.deref().list_candidates_for_auto_ml_job(builder)
    }
    fn list_cluster_nodes(&self, builder: ListClusterNodesInputBuilder) -> impl Future<Output = Result<ListClusterNodesOutput, SdkError<ListClusterNodesError>>> {
        self.deref().list_cluster_nodes(builder)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        self.deref().list_clusters(builder)
    }
    fn list_code_repositories(&self, builder: ListCodeRepositoriesInputBuilder) -> impl Future<Output = Result<ListCodeRepositoriesOutput, SdkError<ListCodeRepositoriesError>>> {
        self.deref().list_code_repositories(builder)
    }
    fn list_compilation_jobs(&self, builder: ListCompilationJobsInputBuilder) -> impl Future<Output = Result<ListCompilationJobsOutput, SdkError<ListCompilationJobsError>>> {
        self.deref().list_compilation_jobs(builder)
    }
    fn list_contexts(&self, builder: ListContextsInputBuilder) -> impl Future<Output = Result<ListContextsOutput, SdkError<ListContextsError>>> {
        self.deref().list_contexts(builder)
    }
    fn list_data_quality_job_definitions(&self, builder: ListDataQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListDataQualityJobDefinitionsOutput, SdkError<ListDataQualityJobDefinitionsError>>> {
        self.deref().list_data_quality_job_definitions(builder)
    }
    fn list_device_fleets(&self, builder: ListDeviceFleetsInputBuilder) -> impl Future<Output = Result<ListDeviceFleetsOutput, SdkError<ListDeviceFleetsError>>> {
        self.deref().list_device_fleets(builder)
    }
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>> {
        self.deref().list_devices(builder)
    }
    fn list_domains(&self, builder: ListDomainsInputBuilder) -> impl Future<Output = Result<ListDomainsOutput, SdkError<ListDomainsError>>> {
        self.deref().list_domains(builder)
    }
    fn list_edge_deployment_plans(&self, builder: ListEdgeDeploymentPlansInputBuilder) -> impl Future<Output = Result<ListEdgeDeploymentPlansOutput, SdkError<ListEdgeDeploymentPlansError>>> {
        self.deref().list_edge_deployment_plans(builder)
    }
    fn list_edge_packaging_jobs(&self, builder: ListEdgePackagingJobsInputBuilder) -> impl Future<Output = Result<ListEdgePackagingJobsOutput, SdkError<ListEdgePackagingJobsError>>> {
        self.deref().list_edge_packaging_jobs(builder)
    }
    fn list_endpoint_configs(&self, builder: ListEndpointConfigsInputBuilder) -> impl Future<Output = Result<ListEndpointConfigsOutput, SdkError<ListEndpointConfigsError>>> {
        self.deref().list_endpoint_configs(builder)
    }
    fn list_endpoints(&self, builder: ListEndpointsInputBuilder) -> impl Future<Output = Result<ListEndpointsOutput, SdkError<ListEndpointsError>>> {
        self.deref().list_endpoints(builder)
    }
    fn list_experiments(&self, builder: ListExperimentsInputBuilder) -> impl Future<Output = Result<ListExperimentsOutput, SdkError<ListExperimentsError>>> {
        self.deref().list_experiments(builder)
    }
    fn list_feature_groups(&self, builder: ListFeatureGroupsInputBuilder) -> impl Future<Output = Result<ListFeatureGroupsOutput, SdkError<ListFeatureGroupsError>>> {
        self.deref().list_feature_groups(builder)
    }
    fn list_flow_definitions(&self, builder: ListFlowDefinitionsInputBuilder) -> impl Future<Output = Result<ListFlowDefinitionsOutput, SdkError<ListFlowDefinitionsError>>> {
        self.deref().list_flow_definitions(builder)
    }
    fn list_hub_content_versions(&self, builder: ListHubContentVersionsInputBuilder) -> impl Future<Output = Result<ListHubContentVersionsOutput, SdkError<ListHubContentVersionsError>>> {
        self.deref().list_hub_content_versions(builder)
    }
    fn list_hub_contents(&self, builder: ListHubContentsInputBuilder) -> impl Future<Output = Result<ListHubContentsOutput, SdkError<ListHubContentsError>>> {
        self.deref().list_hub_contents(builder)
    }
    fn list_hubs(&self, builder: ListHubsInputBuilder) -> impl Future<Output = Result<ListHubsOutput, SdkError<ListHubsError>>> {
        self.deref().list_hubs(builder)
    }
    fn list_human_task_uis(&self, builder: ListHumanTaskUisInputBuilder) -> impl Future<Output = Result<ListHumanTaskUisOutput, SdkError<ListHumanTaskUisError>>> {
        self.deref().list_human_task_uis(builder)
    }
    fn list_hyper_parameter_tuning_jobs(&self, builder: ListHyperParameterTuningJobsInputBuilder) -> impl Future<Output = Result<ListHyperParameterTuningJobsOutput, SdkError<ListHyperParameterTuningJobsError>>> {
        self.deref().list_hyper_parameter_tuning_jobs(builder)
    }
    fn list_image_versions(&self, builder: ListImageVersionsInputBuilder) -> impl Future<Output = Result<ListImageVersionsOutput, SdkError<ListImageVersionsError>>> {
        self.deref().list_image_versions(builder)
    }
    fn list_images(&self, builder: ListImagesInputBuilder) -> impl Future<Output = Result<ListImagesOutput, SdkError<ListImagesError>>> {
        self.deref().list_images(builder)
    }
    fn list_inference_components(&self, builder: ListInferenceComponentsInputBuilder) -> impl Future<Output = Result<ListInferenceComponentsOutput, SdkError<ListInferenceComponentsError>>> {
        self.deref().list_inference_components(builder)
    }
    fn list_inference_experiments(&self, builder: ListInferenceExperimentsInputBuilder) -> impl Future<Output = Result<ListInferenceExperimentsOutput, SdkError<ListInferenceExperimentsError>>> {
        self.deref().list_inference_experiments(builder)
    }
    fn list_inference_recommendations_job_steps(&self, builder: ListInferenceRecommendationsJobStepsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobStepsOutput, SdkError<ListInferenceRecommendationsJobStepsError>>> {
        self.deref().list_inference_recommendations_job_steps(builder)
    }
    fn list_inference_recommendations_jobs(&self, builder: ListInferenceRecommendationsJobsInputBuilder) -> impl Future<Output = Result<ListInferenceRecommendationsJobsOutput, SdkError<ListInferenceRecommendationsJobsError>>> {
        self.deref().list_inference_recommendations_jobs(builder)
    }
    fn list_labeling_jobs(&self, builder: ListLabelingJobsInputBuilder) -> impl Future<Output = Result<ListLabelingJobsOutput, SdkError<ListLabelingJobsError>>> {
        self.deref().list_labeling_jobs(builder)
    }
    fn list_labeling_jobs_for_workteam(&self, builder: ListLabelingJobsForWorkteamInputBuilder) -> impl Future<Output = Result<ListLabelingJobsForWorkteamOutput, SdkError<ListLabelingJobsForWorkteamError>>> {
        self.deref().list_labeling_jobs_for_workteam(builder)
    }
    fn list_lineage_groups(&self, builder: ListLineageGroupsInputBuilder) -> impl Future<Output = Result<ListLineageGroupsOutput, SdkError<ListLineageGroupsError>>> {
        self.deref().list_lineage_groups(builder)
    }
    fn list_mlflow_tracking_servers(&self, builder: ListMlflowTrackingServersInputBuilder) -> impl Future<Output = Result<ListMlflowTrackingServersOutput, SdkError<ListMlflowTrackingServersError>>> {
        self.deref().list_mlflow_tracking_servers(builder)
    }
    fn list_model_bias_job_definitions(&self, builder: ListModelBiasJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelBiasJobDefinitionsOutput, SdkError<ListModelBiasJobDefinitionsError>>> {
        self.deref().list_model_bias_job_definitions(builder)
    }
    fn list_model_card_export_jobs(&self, builder: ListModelCardExportJobsInputBuilder) -> impl Future<Output = Result<ListModelCardExportJobsOutput, SdkError<ListModelCardExportJobsError>>> {
        self.deref().list_model_card_export_jobs(builder)
    }
    fn list_model_card_versions(&self, builder: ListModelCardVersionsInputBuilder) -> impl Future<Output = Result<ListModelCardVersionsOutput, SdkError<ListModelCardVersionsError>>> {
        self.deref().list_model_card_versions(builder)
    }
    fn list_model_cards(&self, builder: ListModelCardsInputBuilder) -> impl Future<Output = Result<ListModelCardsOutput, SdkError<ListModelCardsError>>> {
        self.deref().list_model_cards(builder)
    }
    fn list_model_explainability_job_definitions(&self, builder: ListModelExplainabilityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelExplainabilityJobDefinitionsOutput, SdkError<ListModelExplainabilityJobDefinitionsError>>> {
        self.deref().list_model_explainability_job_definitions(builder)
    }
    fn list_model_metadata(&self, builder: ListModelMetadataInputBuilder) -> impl Future<Output = Result<ListModelMetadataOutput, SdkError<ListModelMetadataError>>> {
        self.deref().list_model_metadata(builder)
    }
    fn list_model_package_groups(&self, builder: ListModelPackageGroupsInputBuilder) -> impl Future<Output = Result<ListModelPackageGroupsOutput, SdkError<ListModelPackageGroupsError>>> {
        self.deref().list_model_package_groups(builder)
    }
    fn list_model_packages(&self, builder: ListModelPackagesInputBuilder) -> impl Future<Output = Result<ListModelPackagesOutput, SdkError<ListModelPackagesError>>> {
        self.deref().list_model_packages(builder)
    }
    fn list_model_quality_job_definitions(&self, builder: ListModelQualityJobDefinitionsInputBuilder) -> impl Future<Output = Result<ListModelQualityJobDefinitionsOutput, SdkError<ListModelQualityJobDefinitionsError>>> {
        self.deref().list_model_quality_job_definitions(builder)
    }
    fn list_models(&self, builder: ListModelsInputBuilder) -> impl Future<Output = Result<ListModelsOutput, SdkError<ListModelsError>>> {
        self.deref().list_models(builder)
    }
    fn list_monitoring_alert_history(&self, builder: ListMonitoringAlertHistoryInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertHistoryOutput, SdkError<ListMonitoringAlertHistoryError>>> {
        self.deref().list_monitoring_alert_history(builder)
    }
    fn list_monitoring_alerts(&self, builder: ListMonitoringAlertsInputBuilder) -> impl Future<Output = Result<ListMonitoringAlertsOutput, SdkError<ListMonitoringAlertsError>>> {
        self.deref().list_monitoring_alerts(builder)
    }
    fn list_monitoring_executions(&self, builder: ListMonitoringExecutionsInputBuilder) -> impl Future<Output = Result<ListMonitoringExecutionsOutput, SdkError<ListMonitoringExecutionsError>>> {
        self.deref().list_monitoring_executions(builder)
    }
    fn list_monitoring_schedules(&self, builder: ListMonitoringSchedulesInputBuilder) -> impl Future<Output = Result<ListMonitoringSchedulesOutput, SdkError<ListMonitoringSchedulesError>>> {
        self.deref().list_monitoring_schedules(builder)
    }
    fn list_notebook_instance_lifecycle_configs(&self, builder: ListNotebookInstanceLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListNotebookInstanceLifecycleConfigsOutput, SdkError<ListNotebookInstanceLifecycleConfigsError>>> {
        self.deref().list_notebook_instance_lifecycle_configs(builder)
    }
    fn list_notebook_instances(&self, builder: ListNotebookInstancesInputBuilder) -> impl Future<Output = Result<ListNotebookInstancesOutput, SdkError<ListNotebookInstancesError>>> {
        self.deref().list_notebook_instances(builder)
    }
    fn list_optimization_jobs(&self, builder: ListOptimizationJobsInputBuilder) -> impl Future<Output = Result<ListOptimizationJobsOutput, SdkError<ListOptimizationJobsError>>> {
        self.deref().list_optimization_jobs(builder)
    }
    fn list_pipeline_execution_steps(&self, builder: ListPipelineExecutionStepsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionStepsOutput, SdkError<ListPipelineExecutionStepsError>>> {
        self.deref().list_pipeline_execution_steps(builder)
    }
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>> {
        self.deref().list_pipeline_executions(builder)
    }
    fn list_pipeline_parameters_for_execution(&self, builder: ListPipelineParametersForExecutionInputBuilder) -> impl Future<Output = Result<ListPipelineParametersForExecutionOutput, SdkError<ListPipelineParametersForExecutionError>>> {
        self.deref().list_pipeline_parameters_for_execution(builder)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        self.deref().list_pipelines(builder)
    }
    fn list_processing_jobs(&self, builder: ListProcessingJobsInputBuilder) -> impl Future<Output = Result<ListProcessingJobsOutput, SdkError<ListProcessingJobsError>>> {
        self.deref().list_processing_jobs(builder)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        self.deref().list_projects(builder)
    }
    fn list_resource_catalogs(&self, builder: ListResourceCatalogsInputBuilder) -> impl Future<Output = Result<ListResourceCatalogsOutput, SdkError<ListResourceCatalogsError>>> {
        self.deref().list_resource_catalogs(builder)
    }
    fn list_spaces(&self, builder: ListSpacesInputBuilder) -> impl Future<Output = Result<ListSpacesOutput, SdkError<ListSpacesError>>> {
        self.deref().list_spaces(builder)
    }
    fn list_stage_devices(&self, builder: ListStageDevicesInputBuilder) -> impl Future<Output = Result<ListStageDevicesOutput, SdkError<ListStageDevicesError>>> {
        self.deref().list_stage_devices(builder)
    }
    fn list_studio_lifecycle_configs(&self, builder: ListStudioLifecycleConfigsInputBuilder) -> impl Future<Output = Result<ListStudioLifecycleConfigsOutput, SdkError<ListStudioLifecycleConfigsError>>> {
        self.deref().list_studio_lifecycle_configs(builder)
    }
    fn list_subscribed_workteams(&self, builder: ListSubscribedWorkteamsInputBuilder) -> impl Future<Output = Result<ListSubscribedWorkteamsOutput, SdkError<ListSubscribedWorkteamsError>>> {
        self.deref().list_subscribed_workteams(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        self.deref().list_tags(builder)
    }
    fn list_training_jobs(&self, builder: ListTrainingJobsInputBuilder) -> impl Future<Output = Result<ListTrainingJobsOutput, SdkError<ListTrainingJobsError>>> {
        self.deref().list_training_jobs(builder)
    }
    fn list_training_jobs_for_hyper_parameter_tuning_job(&self, builder: ListTrainingJobsForHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<ListTrainingJobsForHyperParameterTuningJobOutput, SdkError<ListTrainingJobsForHyperParameterTuningJobError>>> {
        self.deref().list_training_jobs_for_hyper_parameter_tuning_job(builder)
    }
    fn list_transform_jobs(&self, builder: ListTransformJobsInputBuilder) -> impl Future<Output = Result<ListTransformJobsOutput, SdkError<ListTransformJobsError>>> {
        self.deref().list_transform_jobs(builder)
    }
    fn list_trial_components(&self, builder: ListTrialComponentsInputBuilder) -> impl Future<Output = Result<ListTrialComponentsOutput, SdkError<ListTrialComponentsError>>> {
        self.deref().list_trial_components(builder)
    }
    fn list_trials(&self, builder: ListTrialsInputBuilder) -> impl Future<Output = Result<ListTrialsOutput, SdkError<ListTrialsError>>> {
        self.deref().list_trials(builder)
    }
    fn list_user_profiles(&self, builder: ListUserProfilesInputBuilder) -> impl Future<Output = Result<ListUserProfilesOutput, SdkError<ListUserProfilesError>>> {
        self.deref().list_user_profiles(builder)
    }
    fn list_workforces(&self, builder: ListWorkforcesInputBuilder) -> impl Future<Output = Result<ListWorkforcesOutput, SdkError<ListWorkforcesError>>> {
        self.deref().list_workforces(builder)
    }
    fn list_workteams(&self, builder: ListWorkteamsInputBuilder) -> impl Future<Output = Result<ListWorkteamsOutput, SdkError<ListWorkteamsError>>> {
        self.deref().list_workteams(builder)
    }
    fn put_model_package_group_policy(&self, builder: PutModelPackageGroupPolicyInputBuilder) -> impl Future<Output = Result<PutModelPackageGroupPolicyOutput, SdkError<PutModelPackageGroupPolicyError>>> {
        self.deref().put_model_package_group_policy(builder)
    }
    fn query_lineage(&self, builder: QueryLineageInputBuilder) -> impl Future<Output = Result<QueryLineageOutput, SdkError<QueryLineageError>>> {
        self.deref().query_lineage(builder)
    }
    fn register_devices(&self, builder: RegisterDevicesInputBuilder) -> impl Future<Output = Result<RegisterDevicesOutput, SdkError<RegisterDevicesError>>> {
        self.deref().register_devices(builder)
    }
    fn render_ui_template(&self, builder: RenderUiTemplateInputBuilder) -> impl Future<Output = Result<RenderUiTemplateOutput, SdkError<RenderUiTemplateError>>> {
        self.deref().render_ui_template(builder)
    }
    fn retry_pipeline_execution(&self, builder: RetryPipelineExecutionInputBuilder) -> impl Future<Output = Result<RetryPipelineExecutionOutput, SdkError<RetryPipelineExecutionError>>> {
        self.deref().retry_pipeline_execution(builder)
    }
    fn search(&self, builder: SearchInputBuilder) -> impl Future<Output = Result<SearchOutput, SdkError<SearchError>>> {
        self.deref().search(builder)
    }
    fn send_pipeline_execution_step_failure(&self, builder: SendPipelineExecutionStepFailureInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepFailureOutput, SdkError<SendPipelineExecutionStepFailureError>>> {
        self.deref().send_pipeline_execution_step_failure(builder)
    }
    fn send_pipeline_execution_step_success(&self, builder: SendPipelineExecutionStepSuccessInputBuilder) -> impl Future<Output = Result<SendPipelineExecutionStepSuccessOutput, SdkError<SendPipelineExecutionStepSuccessError>>> {
        self.deref().send_pipeline_execution_step_success(builder)
    }
    fn start_edge_deployment_stage(&self, builder: StartEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StartEdgeDeploymentStageOutput, SdkError<StartEdgeDeploymentStageError>>> {
        self.deref().start_edge_deployment_stage(builder)
    }
    fn start_inference_experiment(&self, builder: StartInferenceExperimentInputBuilder) -> impl Future<Output = Result<StartInferenceExperimentOutput, SdkError<StartInferenceExperimentError>>> {
        self.deref().start_inference_experiment(builder)
    }
    fn start_mlflow_tracking_server(&self, builder: StartMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StartMlflowTrackingServerOutput, SdkError<StartMlflowTrackingServerError>>> {
        self.deref().start_mlflow_tracking_server(builder)
    }
    fn start_monitoring_schedule(&self, builder: StartMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StartMonitoringScheduleOutput, SdkError<StartMonitoringScheduleError>>> {
        self.deref().start_monitoring_schedule(builder)
    }
    fn start_notebook_instance(&self, builder: StartNotebookInstanceInputBuilder) -> impl Future<Output = Result<StartNotebookInstanceOutput, SdkError<StartNotebookInstanceError>>> {
        self.deref().start_notebook_instance(builder)
    }
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>> {
        self.deref().start_pipeline_execution(builder)
    }
    fn stop_auto_ml_job(&self, builder: StopAutoMlJobInputBuilder) -> impl Future<Output = Result<StopAutoMlJobOutput, SdkError<StopAutoMLJobError>>> {
        self.deref().stop_auto_ml_job(builder)
    }
    fn stop_compilation_job(&self, builder: StopCompilationJobInputBuilder) -> impl Future<Output = Result<StopCompilationJobOutput, SdkError<StopCompilationJobError>>> {
        self.deref().stop_compilation_job(builder)
    }
    fn stop_edge_deployment_stage(&self, builder: StopEdgeDeploymentStageInputBuilder) -> impl Future<Output = Result<StopEdgeDeploymentStageOutput, SdkError<StopEdgeDeploymentStageError>>> {
        self.deref().stop_edge_deployment_stage(builder)
    }
    fn stop_edge_packaging_job(&self, builder: StopEdgePackagingJobInputBuilder) -> impl Future<Output = Result<StopEdgePackagingJobOutput, SdkError<StopEdgePackagingJobError>>> {
        self.deref().stop_edge_packaging_job(builder)
    }
    fn stop_hyper_parameter_tuning_job(&self, builder: StopHyperParameterTuningJobInputBuilder) -> impl Future<Output = Result<StopHyperParameterTuningJobOutput, SdkError<StopHyperParameterTuningJobError>>> {
        self.deref().stop_hyper_parameter_tuning_job(builder)
    }
    fn stop_inference_experiment(&self, builder: StopInferenceExperimentInputBuilder) -> impl Future<Output = Result<StopInferenceExperimentOutput, SdkError<StopInferenceExperimentError>>> {
        self.deref().stop_inference_experiment(builder)
    }
    fn stop_inference_recommendations_job(&self, builder: StopInferenceRecommendationsJobInputBuilder) -> impl Future<Output = Result<StopInferenceRecommendationsJobOutput, SdkError<StopInferenceRecommendationsJobError>>> {
        self.deref().stop_inference_recommendations_job(builder)
    }
    fn stop_labeling_job(&self, builder: StopLabelingJobInputBuilder) -> impl Future<Output = Result<StopLabelingJobOutput, SdkError<StopLabelingJobError>>> {
        self.deref().stop_labeling_job(builder)
    }
    fn stop_mlflow_tracking_server(&self, builder: StopMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<StopMlflowTrackingServerOutput, SdkError<StopMlflowTrackingServerError>>> {
        self.deref().stop_mlflow_tracking_server(builder)
    }
    fn stop_monitoring_schedule(&self, builder: StopMonitoringScheduleInputBuilder) -> impl Future<Output = Result<StopMonitoringScheduleOutput, SdkError<StopMonitoringScheduleError>>> {
        self.deref().stop_monitoring_schedule(builder)
    }
    fn stop_notebook_instance(&self, builder: StopNotebookInstanceInputBuilder) -> impl Future<Output = Result<StopNotebookInstanceOutput, SdkError<StopNotebookInstanceError>>> {
        self.deref().stop_notebook_instance(builder)
    }
    fn stop_optimization_job(&self, builder: StopOptimizationJobInputBuilder) -> impl Future<Output = Result<StopOptimizationJobOutput, SdkError<StopOptimizationJobError>>> {
        self.deref().stop_optimization_job(builder)
    }
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>> {
        self.deref().stop_pipeline_execution(builder)
    }
    fn stop_processing_job(&self, builder: StopProcessingJobInputBuilder) -> impl Future<Output = Result<StopProcessingJobOutput, SdkError<StopProcessingJobError>>> {
        self.deref().stop_processing_job(builder)
    }
    fn stop_training_job(&self, builder: StopTrainingJobInputBuilder) -> impl Future<Output = Result<StopTrainingJobOutput, SdkError<StopTrainingJobError>>> {
        self.deref().stop_training_job(builder)
    }
    fn stop_transform_job(&self, builder: StopTransformJobInputBuilder) -> impl Future<Output = Result<StopTransformJobOutput, SdkError<StopTransformJobError>>> {
        self.deref().stop_transform_job(builder)
    }
    fn update_action(&self, builder: UpdateActionInputBuilder) -> impl Future<Output = Result<UpdateActionOutput, SdkError<UpdateActionError>>> {
        self.deref().update_action(builder)
    }
    fn update_app_image_config(&self, builder: UpdateAppImageConfigInputBuilder) -> impl Future<Output = Result<UpdateAppImageConfigOutput, SdkError<UpdateAppImageConfigError>>> {
        self.deref().update_app_image_config(builder)
    }
    fn update_artifact(&self, builder: UpdateArtifactInputBuilder) -> impl Future<Output = Result<UpdateArtifactOutput, SdkError<UpdateArtifactError>>> {
        self.deref().update_artifact(builder)
    }
    fn update_cluster(&self, builder: UpdateClusterInputBuilder) -> impl Future<Output = Result<UpdateClusterOutput, SdkError<UpdateClusterError>>> {
        self.deref().update_cluster(builder)
    }
    fn update_cluster_software(&self, builder: UpdateClusterSoftwareInputBuilder) -> impl Future<Output = Result<UpdateClusterSoftwareOutput, SdkError<UpdateClusterSoftwareError>>> {
        self.deref().update_cluster_software(builder)
    }
    fn update_code_repository(&self, builder: UpdateCodeRepositoryInputBuilder) -> impl Future<Output = Result<UpdateCodeRepositoryOutput, SdkError<UpdateCodeRepositoryError>>> {
        self.deref().update_code_repository(builder)
    }
    fn update_context(&self, builder: UpdateContextInputBuilder) -> impl Future<Output = Result<UpdateContextOutput, SdkError<UpdateContextError>>> {
        self.deref().update_context(builder)
    }
    fn update_device_fleet(&self, builder: UpdateDeviceFleetInputBuilder) -> impl Future<Output = Result<UpdateDeviceFleetOutput, SdkError<UpdateDeviceFleetError>>> {
        self.deref().update_device_fleet(builder)
    }
    fn update_devices(&self, builder: UpdateDevicesInputBuilder) -> impl Future<Output = Result<UpdateDevicesOutput, SdkError<UpdateDevicesError>>> {
        self.deref().update_devices(builder)
    }
    fn update_domain(&self, builder: UpdateDomainInputBuilder) -> impl Future<Output = Result<UpdateDomainOutput, SdkError<UpdateDomainError>>> {
        self.deref().update_domain(builder)
    }
    fn update_endpoint(&self, builder: UpdateEndpointInputBuilder) -> impl Future<Output = Result<UpdateEndpointOutput, SdkError<UpdateEndpointError>>> {
        self.deref().update_endpoint(builder)
    }
    fn update_endpoint_weights_and_capacities(&self, builder: UpdateEndpointWeightsAndCapacitiesInputBuilder) -> impl Future<Output = Result<UpdateEndpointWeightsAndCapacitiesOutput, SdkError<UpdateEndpointWeightsAndCapacitiesError>>> {
        self.deref().update_endpoint_weights_and_capacities(builder)
    }
    fn update_experiment(&self, builder: UpdateExperimentInputBuilder) -> impl Future<Output = Result<UpdateExperimentOutput, SdkError<UpdateExperimentError>>> {
        self.deref().update_experiment(builder)
    }
    fn update_feature_group(&self, builder: UpdateFeatureGroupInputBuilder) -> impl Future<Output = Result<UpdateFeatureGroupOutput, SdkError<UpdateFeatureGroupError>>> {
        self.deref().update_feature_group(builder)
    }
    fn update_feature_metadata(&self, builder: UpdateFeatureMetadataInputBuilder) -> impl Future<Output = Result<UpdateFeatureMetadataOutput, SdkError<UpdateFeatureMetadataError>>> {
        self.deref().update_feature_metadata(builder)
    }
    fn update_hub(&self, builder: UpdateHubInputBuilder) -> impl Future<Output = Result<UpdateHubOutput, SdkError<UpdateHubError>>> {
        self.deref().update_hub(builder)
    }
    fn update_image(&self, builder: UpdateImageInputBuilder) -> impl Future<Output = Result<UpdateImageOutput, SdkError<UpdateImageError>>> {
        self.deref().update_image(builder)
    }
    fn update_image_version(&self, builder: UpdateImageVersionInputBuilder) -> impl Future<Output = Result<UpdateImageVersionOutput, SdkError<UpdateImageVersionError>>> {
        self.deref().update_image_version(builder)
    }
    fn update_inference_component(&self, builder: UpdateInferenceComponentInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentOutput, SdkError<UpdateInferenceComponentError>>> {
        self.deref().update_inference_component(builder)
    }
    fn update_inference_component_runtime_config(&self, builder: UpdateInferenceComponentRuntimeConfigInputBuilder) -> impl Future<Output = Result<UpdateInferenceComponentRuntimeConfigOutput, SdkError<UpdateInferenceComponentRuntimeConfigError>>> {
        self.deref().update_inference_component_runtime_config(builder)
    }
    fn update_inference_experiment(&self, builder: UpdateInferenceExperimentInputBuilder) -> impl Future<Output = Result<UpdateInferenceExperimentOutput, SdkError<UpdateInferenceExperimentError>>> {
        self.deref().update_inference_experiment(builder)
    }
    fn update_mlflow_tracking_server(&self, builder: UpdateMlflowTrackingServerInputBuilder) -> impl Future<Output = Result<UpdateMlflowTrackingServerOutput, SdkError<UpdateMlflowTrackingServerError>>> {
        self.deref().update_mlflow_tracking_server(builder)
    }
    fn update_model_card(&self, builder: UpdateModelCardInputBuilder) -> impl Future<Output = Result<UpdateModelCardOutput, SdkError<UpdateModelCardError>>> {
        self.deref().update_model_card(builder)
    }
    fn update_model_package(&self, builder: UpdateModelPackageInputBuilder) -> impl Future<Output = Result<UpdateModelPackageOutput, SdkError<UpdateModelPackageError>>> {
        self.deref().update_model_package(builder)
    }
    fn update_monitoring_alert(&self, builder: UpdateMonitoringAlertInputBuilder) -> impl Future<Output = Result<UpdateMonitoringAlertOutput, SdkError<UpdateMonitoringAlertError>>> {
        self.deref().update_monitoring_alert(builder)
    }
    fn update_monitoring_schedule(&self, builder: UpdateMonitoringScheduleInputBuilder) -> impl Future<Output = Result<UpdateMonitoringScheduleOutput, SdkError<UpdateMonitoringScheduleError>>> {
        self.deref().update_monitoring_schedule(builder)
    }
    fn update_notebook_instance(&self, builder: UpdateNotebookInstanceInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceOutput, SdkError<UpdateNotebookInstanceError>>> {
        self.deref().update_notebook_instance(builder)
    }
    fn update_notebook_instance_lifecycle_config(&self, builder: UpdateNotebookInstanceLifecycleConfigInputBuilder) -> impl Future<Output = Result<UpdateNotebookInstanceLifecycleConfigOutput, SdkError<UpdateNotebookInstanceLifecycleConfigError>>> {
        self.deref().update_notebook_instance_lifecycle_config(builder)
    }
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>> {
        self.deref().update_pipeline(builder)
    }
    fn update_pipeline_execution(&self, builder: UpdatePipelineExecutionInputBuilder) -> impl Future<Output = Result<UpdatePipelineExecutionOutput, SdkError<UpdatePipelineExecutionError>>> {
        self.deref().update_pipeline_execution(builder)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        self.deref().update_project(builder)
    }
    fn update_space(&self, builder: UpdateSpaceInputBuilder) -> impl Future<Output = Result<UpdateSpaceOutput, SdkError<UpdateSpaceError>>> {
        self.deref().update_space(builder)
    }
    fn update_training_job(&self, builder: UpdateTrainingJobInputBuilder) -> impl Future<Output = Result<UpdateTrainingJobOutput, SdkError<UpdateTrainingJobError>>> {
        self.deref().update_training_job(builder)
    }
    fn update_trial(&self, builder: UpdateTrialInputBuilder) -> impl Future<Output = Result<UpdateTrialOutput, SdkError<UpdateTrialError>>> {
        self.deref().update_trial(builder)
    }
    fn update_trial_component(&self, builder: UpdateTrialComponentInputBuilder) -> impl Future<Output = Result<UpdateTrialComponentOutput, SdkError<UpdateTrialComponentError>>> {
        self.deref().update_trial_component(builder)
    }
    fn update_user_profile(&self, builder: UpdateUserProfileInputBuilder) -> impl Future<Output = Result<UpdateUserProfileOutput, SdkError<UpdateUserProfileError>>> {
        self.deref().update_user_profile(builder)
    }
    fn update_workforce(&self, builder: UpdateWorkforceInputBuilder) -> impl Future<Output = Result<UpdateWorkforceOutput, SdkError<UpdateWorkforceError>>> {
        self.deref().update_workforce(builder)
    }
    fn update_workteam(&self, builder: UpdateWorkteamInputBuilder) -> impl Future<Output = Result<UpdateWorkteamOutput, SdkError<UpdateWorkteamError>>> {
        self.deref().update_workteam(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edSageMakerClient {}
    impl SageMakerClient for edSageMakerClient {
        async fn add_association(&self, builder: AddAssociationInputBuilder) -> Result<AddAssociationOutput, SdkError<AddAssociationError>>;
        async fn add_tags(&self, builder: AddTagsInputBuilder) -> Result<AddTagsOutput, SdkError<AddTagsError>>;
        async fn associate_trial_component(&self, builder: AssociateTrialComponentInputBuilder) -> Result<AssociateTrialComponentOutput, SdkError<AssociateTrialComponentError>>;
        async fn batch_describe_model_package(&self, builder: BatchDescribeModelPackageInputBuilder) -> Result<BatchDescribeModelPackageOutput, SdkError<BatchDescribeModelPackageError>>;
        async fn create_action(&self, builder: CreateActionInputBuilder) -> Result<CreateActionOutput, SdkError<CreateActionError>>;
        async fn create_algorithm(&self, builder: CreateAlgorithmInputBuilder) -> Result<CreateAlgorithmOutput, SdkError<CreateAlgorithmError>>;
        async fn create_app(&self, builder: CreateAppInputBuilder) -> Result<CreateAppOutput, SdkError<CreateAppError>>;
        async fn create_app_image_config(&self, builder: CreateAppImageConfigInputBuilder) -> Result<CreateAppImageConfigOutput, SdkError<CreateAppImageConfigError>>;
        async fn create_artifact(&self, builder: CreateArtifactInputBuilder) -> Result<CreateArtifactOutput, SdkError<CreateArtifactError>>;
        async fn create_auto_ml_job(&self, builder: CreateAutoMlJobInputBuilder) -> Result<CreateAutoMlJobOutput, SdkError<CreateAutoMLJobError>>;
        async fn create_auto_ml_job_v2(&self, builder: CreateAutoMlJobV2InputBuilder) -> Result<CreateAutoMlJobV2Output, SdkError<CreateAutoMLJobV2Error>>;
        async fn create_cluster(&self, builder: CreateClusterInputBuilder) -> Result<CreateClusterOutput, SdkError<CreateClusterError>>;
        async fn create_code_repository(&self, builder: CreateCodeRepositoryInputBuilder) -> Result<CreateCodeRepositoryOutput, SdkError<CreateCodeRepositoryError>>;
        async fn create_compilation_job(&self, builder: CreateCompilationJobInputBuilder) -> Result<CreateCompilationJobOutput, SdkError<CreateCompilationJobError>>;
        async fn create_context(&self, builder: CreateContextInputBuilder) -> Result<CreateContextOutput, SdkError<CreateContextError>>;
        async fn create_data_quality_job_definition(&self, builder: CreateDataQualityJobDefinitionInputBuilder) -> Result<CreateDataQualityJobDefinitionOutput, SdkError<CreateDataQualityJobDefinitionError>>;
        async fn create_device_fleet(&self, builder: CreateDeviceFleetInputBuilder) -> Result<CreateDeviceFleetOutput, SdkError<CreateDeviceFleetError>>;
        async fn create_domain(&self, builder: CreateDomainInputBuilder) -> Result<CreateDomainOutput, SdkError<CreateDomainError>>;
        async fn create_edge_deployment_plan(&self, builder: CreateEdgeDeploymentPlanInputBuilder) -> Result<CreateEdgeDeploymentPlanOutput, SdkError<CreateEdgeDeploymentPlanError>>;
        async fn create_edge_deployment_stage(&self, builder: CreateEdgeDeploymentStageInputBuilder) -> Result<CreateEdgeDeploymentStageOutput, SdkError<CreateEdgeDeploymentStageError>>;
        async fn create_edge_packaging_job(&self, builder: CreateEdgePackagingJobInputBuilder) -> Result<CreateEdgePackagingJobOutput, SdkError<CreateEdgePackagingJobError>>;
        async fn create_endpoint(&self, builder: CreateEndpointInputBuilder) -> Result<CreateEndpointOutput, SdkError<CreateEndpointError>>;
        async fn create_endpoint_config(&self, builder: CreateEndpointConfigInputBuilder) -> Result<CreateEndpointConfigOutput, SdkError<CreateEndpointConfigError>>;
        async fn create_experiment(&self, builder: CreateExperimentInputBuilder) -> Result<CreateExperimentOutput, SdkError<CreateExperimentError>>;
        async fn create_feature_group(&self, builder: CreateFeatureGroupInputBuilder) -> Result<CreateFeatureGroupOutput, SdkError<CreateFeatureGroupError>>;
        async fn create_flow_definition(&self, builder: CreateFlowDefinitionInputBuilder) -> Result<CreateFlowDefinitionOutput, SdkError<CreateFlowDefinitionError>>;
        async fn create_hub(&self, builder: CreateHubInputBuilder) -> Result<CreateHubOutput, SdkError<CreateHubError>>;
        async fn create_hub_content_reference(&self, builder: CreateHubContentReferenceInputBuilder) -> Result<CreateHubContentReferenceOutput, SdkError<CreateHubContentReferenceError>>;
        async fn create_human_task_ui(&self, builder: CreateHumanTaskUiInputBuilder) -> Result<CreateHumanTaskUiOutput, SdkError<CreateHumanTaskUiError>>;
        async fn create_hyper_parameter_tuning_job(&self, builder: CreateHyperParameterTuningJobInputBuilder) -> Result<CreateHyperParameterTuningJobOutput, SdkError<CreateHyperParameterTuningJobError>>;
        async fn create_image(&self, builder: CreateImageInputBuilder) -> Result<CreateImageOutput, SdkError<CreateImageError>>;
        async fn create_image_version(&self, builder: CreateImageVersionInputBuilder) -> Result<CreateImageVersionOutput, SdkError<CreateImageVersionError>>;
        async fn create_inference_component(&self, builder: CreateInferenceComponentInputBuilder) -> Result<CreateInferenceComponentOutput, SdkError<CreateInferenceComponentError>>;
        async fn create_inference_experiment(&self, builder: CreateInferenceExperimentInputBuilder) -> Result<CreateInferenceExperimentOutput, SdkError<CreateInferenceExperimentError>>;
        async fn create_inference_recommendations_job(&self, builder: CreateInferenceRecommendationsJobInputBuilder) -> Result<CreateInferenceRecommendationsJobOutput, SdkError<CreateInferenceRecommendationsJobError>>;
        async fn create_labeling_job(&self, builder: CreateLabelingJobInputBuilder) -> Result<CreateLabelingJobOutput, SdkError<CreateLabelingJobError>>;
        async fn create_mlflow_tracking_server(&self, builder: CreateMlflowTrackingServerInputBuilder) -> Result<CreateMlflowTrackingServerOutput, SdkError<CreateMlflowTrackingServerError>>;
        async fn create_model(&self, builder: CreateModelInputBuilder) -> Result<CreateModelOutput, SdkError<CreateModelError>>;
        async fn create_model_bias_job_definition(&self, builder: CreateModelBiasJobDefinitionInputBuilder) -> Result<CreateModelBiasJobDefinitionOutput, SdkError<CreateModelBiasJobDefinitionError>>;
        async fn create_model_card(&self, builder: CreateModelCardInputBuilder) -> Result<CreateModelCardOutput, SdkError<CreateModelCardError>>;
        async fn create_model_card_export_job(&self, builder: CreateModelCardExportJobInputBuilder) -> Result<CreateModelCardExportJobOutput, SdkError<CreateModelCardExportJobError>>;
        async fn create_model_explainability_job_definition(&self, builder: CreateModelExplainabilityJobDefinitionInputBuilder) -> Result<CreateModelExplainabilityJobDefinitionOutput, SdkError<CreateModelExplainabilityJobDefinitionError>>;
        async fn create_model_package(&self, builder: CreateModelPackageInputBuilder) -> Result<CreateModelPackageOutput, SdkError<CreateModelPackageError>>;
        async fn create_model_package_group(&self, builder: CreateModelPackageGroupInputBuilder) -> Result<CreateModelPackageGroupOutput, SdkError<CreateModelPackageGroupError>>;
        async fn create_model_quality_job_definition(&self, builder: CreateModelQualityJobDefinitionInputBuilder) -> Result<CreateModelQualityJobDefinitionOutput, SdkError<CreateModelQualityJobDefinitionError>>;
        async fn create_monitoring_schedule(&self, builder: CreateMonitoringScheduleInputBuilder) -> Result<CreateMonitoringScheduleOutput, SdkError<CreateMonitoringScheduleError>>;
        async fn create_notebook_instance(&self, builder: CreateNotebookInstanceInputBuilder) -> Result<CreateNotebookInstanceOutput, SdkError<CreateNotebookInstanceError>>;
        async fn create_notebook_instance_lifecycle_config(&self, builder: CreateNotebookInstanceLifecycleConfigInputBuilder) -> Result<CreateNotebookInstanceLifecycleConfigOutput, SdkError<CreateNotebookInstanceLifecycleConfigError>>;
        async fn create_optimization_job(&self, builder: CreateOptimizationJobInputBuilder) -> Result<CreateOptimizationJobOutput, SdkError<CreateOptimizationJobError>>;
        async fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> Result<CreatePipelineOutput, SdkError<CreatePipelineError>>;
        async fn create_presigned_domain_url(&self, builder: CreatePresignedDomainUrlInputBuilder) -> Result<CreatePresignedDomainUrlOutput, SdkError<CreatePresignedDomainUrlError>>;
        async fn create_presigned_mlflow_tracking_server_url(&self, builder: CreatePresignedMlflowTrackingServerUrlInputBuilder) -> Result<CreatePresignedMlflowTrackingServerUrlOutput, SdkError<CreatePresignedMlflowTrackingServerUrlError>>;
        async fn create_presigned_notebook_instance_url(&self, builder: CreatePresignedNotebookInstanceUrlInputBuilder) -> Result<CreatePresignedNotebookInstanceUrlOutput, SdkError<CreatePresignedNotebookInstanceUrlError>>;
        async fn create_processing_job(&self, builder: CreateProcessingJobInputBuilder) -> Result<CreateProcessingJobOutput, SdkError<CreateProcessingJobError>>;
        async fn create_project(&self, builder: CreateProjectInputBuilder) -> Result<CreateProjectOutput, SdkError<CreateProjectError>>;
        async fn create_space(&self, builder: CreateSpaceInputBuilder) -> Result<CreateSpaceOutput, SdkError<CreateSpaceError>>;
        async fn create_studio_lifecycle_config(&self, builder: CreateStudioLifecycleConfigInputBuilder) -> Result<CreateStudioLifecycleConfigOutput, SdkError<CreateStudioLifecycleConfigError>>;
        async fn create_training_job(&self, builder: CreateTrainingJobInputBuilder) -> Result<CreateTrainingJobOutput, SdkError<CreateTrainingJobError>>;
        async fn create_transform_job(&self, builder: CreateTransformJobInputBuilder) -> Result<CreateTransformJobOutput, SdkError<CreateTransformJobError>>;
        async fn create_trial(&self, builder: CreateTrialInputBuilder) -> Result<CreateTrialOutput, SdkError<CreateTrialError>>;
        async fn create_trial_component(&self, builder: CreateTrialComponentInputBuilder) -> Result<CreateTrialComponentOutput, SdkError<CreateTrialComponentError>>;
        async fn create_user_profile(&self, builder: CreateUserProfileInputBuilder) -> Result<CreateUserProfileOutput, SdkError<CreateUserProfileError>>;
        async fn create_workforce(&self, builder: CreateWorkforceInputBuilder) -> Result<CreateWorkforceOutput, SdkError<CreateWorkforceError>>;
        async fn create_workteam(&self, builder: CreateWorkteamInputBuilder) -> Result<CreateWorkteamOutput, SdkError<CreateWorkteamError>>;
        async fn delete_action(&self, builder: DeleteActionInputBuilder) -> Result<DeleteActionOutput, SdkError<DeleteActionError>>;
        async fn delete_algorithm(&self, builder: DeleteAlgorithmInputBuilder) -> Result<DeleteAlgorithmOutput, SdkError<DeleteAlgorithmError>>;
        async fn delete_app(&self, builder: DeleteAppInputBuilder) -> Result<DeleteAppOutput, SdkError<DeleteAppError>>;
        async fn delete_app_image_config(&self, builder: DeleteAppImageConfigInputBuilder) -> Result<DeleteAppImageConfigOutput, SdkError<DeleteAppImageConfigError>>;
        async fn delete_artifact(&self, builder: DeleteArtifactInputBuilder) -> Result<DeleteArtifactOutput, SdkError<DeleteArtifactError>>;
        async fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>;
        async fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> Result<DeleteClusterOutput, SdkError<DeleteClusterError>>;
        async fn delete_code_repository(&self, builder: DeleteCodeRepositoryInputBuilder) -> Result<DeleteCodeRepositoryOutput, SdkError<DeleteCodeRepositoryError>>;
        async fn delete_compilation_job(&self, builder: DeleteCompilationJobInputBuilder) -> Result<DeleteCompilationJobOutput, SdkError<DeleteCompilationJobError>>;
        async fn delete_context(&self, builder: DeleteContextInputBuilder) -> Result<DeleteContextOutput, SdkError<DeleteContextError>>;
        async fn delete_data_quality_job_definition(&self, builder: DeleteDataQualityJobDefinitionInputBuilder) -> Result<DeleteDataQualityJobDefinitionOutput, SdkError<DeleteDataQualityJobDefinitionError>>;
        async fn delete_device_fleet(&self, builder: DeleteDeviceFleetInputBuilder) -> Result<DeleteDeviceFleetOutput, SdkError<DeleteDeviceFleetError>>;
        async fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> Result<DeleteDomainOutput, SdkError<DeleteDomainError>>;
        async fn delete_edge_deployment_plan(&self, builder: DeleteEdgeDeploymentPlanInputBuilder) -> Result<DeleteEdgeDeploymentPlanOutput, SdkError<DeleteEdgeDeploymentPlanError>>;
        async fn delete_edge_deployment_stage(&self, builder: DeleteEdgeDeploymentStageInputBuilder) -> Result<DeleteEdgeDeploymentStageOutput, SdkError<DeleteEdgeDeploymentStageError>>;
        async fn delete_endpoint(&self, builder: DeleteEndpointInputBuilder) -> Result<DeleteEndpointOutput, SdkError<DeleteEndpointError>>;
        async fn delete_endpoint_config(&self, builder: DeleteEndpointConfigInputBuilder) -> Result<DeleteEndpointConfigOutput, SdkError<DeleteEndpointConfigError>>;
        async fn delete_experiment(&self, builder: DeleteExperimentInputBuilder) -> Result<DeleteExperimentOutput, SdkError<DeleteExperimentError>>;
        async fn delete_feature_group(&self, builder: DeleteFeatureGroupInputBuilder) -> Result<DeleteFeatureGroupOutput, SdkError<DeleteFeatureGroupError>>;
        async fn delete_flow_definition(&self, builder: DeleteFlowDefinitionInputBuilder) -> Result<DeleteFlowDefinitionOutput, SdkError<DeleteFlowDefinitionError>>;
        async fn delete_hub(&self, builder: DeleteHubInputBuilder) -> Result<DeleteHubOutput, SdkError<DeleteHubError>>;
        async fn delete_hub_content(&self, builder: DeleteHubContentInputBuilder) -> Result<DeleteHubContentOutput, SdkError<DeleteHubContentError>>;
        async fn delete_hub_content_reference(&self, builder: DeleteHubContentReferenceInputBuilder) -> Result<DeleteHubContentReferenceOutput, SdkError<DeleteHubContentReferenceError>>;
        async fn delete_human_task_ui(&self, builder: DeleteHumanTaskUiInputBuilder) -> Result<DeleteHumanTaskUiOutput, SdkError<DeleteHumanTaskUiError>>;
        async fn delete_hyper_parameter_tuning_job(&self, builder: DeleteHyperParameterTuningJobInputBuilder) -> Result<DeleteHyperParameterTuningJobOutput, SdkError<DeleteHyperParameterTuningJobError>>;
        async fn delete_image(&self, builder: DeleteImageInputBuilder) -> Result<DeleteImageOutput, SdkError<DeleteImageError>>;
        async fn delete_image_version(&self, builder: DeleteImageVersionInputBuilder) -> Result<DeleteImageVersionOutput, SdkError<DeleteImageVersionError>>;
        async fn delete_inference_component(&self, builder: DeleteInferenceComponentInputBuilder) -> Result<DeleteInferenceComponentOutput, SdkError<DeleteInferenceComponentError>>;
        async fn delete_inference_experiment(&self, builder: DeleteInferenceExperimentInputBuilder) -> Result<DeleteInferenceExperimentOutput, SdkError<DeleteInferenceExperimentError>>;
        async fn delete_mlflow_tracking_server(&self, builder: DeleteMlflowTrackingServerInputBuilder) -> Result<DeleteMlflowTrackingServerOutput, SdkError<DeleteMlflowTrackingServerError>>;
        async fn delete_model(&self, builder: DeleteModelInputBuilder) -> Result<DeleteModelOutput, SdkError<DeleteModelError>>;
        async fn delete_model_bias_job_definition(&self, builder: DeleteModelBiasJobDefinitionInputBuilder) -> Result<DeleteModelBiasJobDefinitionOutput, SdkError<DeleteModelBiasJobDefinitionError>>;
        async fn delete_model_card(&self, builder: DeleteModelCardInputBuilder) -> Result<DeleteModelCardOutput, SdkError<DeleteModelCardError>>;
        async fn delete_model_explainability_job_definition(&self, builder: DeleteModelExplainabilityJobDefinitionInputBuilder) -> Result<DeleteModelExplainabilityJobDefinitionOutput, SdkError<DeleteModelExplainabilityJobDefinitionError>>;
        async fn delete_model_package(&self, builder: DeleteModelPackageInputBuilder) -> Result<DeleteModelPackageOutput, SdkError<DeleteModelPackageError>>;
        async fn delete_model_package_group(&self, builder: DeleteModelPackageGroupInputBuilder) -> Result<DeleteModelPackageGroupOutput, SdkError<DeleteModelPackageGroupError>>;
        async fn delete_model_package_group_policy(&self, builder: DeleteModelPackageGroupPolicyInputBuilder) -> Result<DeleteModelPackageGroupPolicyOutput, SdkError<DeleteModelPackageGroupPolicyError>>;
        async fn delete_model_quality_job_definition(&self, builder: DeleteModelQualityJobDefinitionInputBuilder) -> Result<DeleteModelQualityJobDefinitionOutput, SdkError<DeleteModelQualityJobDefinitionError>>;
        async fn delete_monitoring_schedule(&self, builder: DeleteMonitoringScheduleInputBuilder) -> Result<DeleteMonitoringScheduleOutput, SdkError<DeleteMonitoringScheduleError>>;
        async fn delete_notebook_instance(&self, builder: DeleteNotebookInstanceInputBuilder) -> Result<DeleteNotebookInstanceOutput, SdkError<DeleteNotebookInstanceError>>;
        async fn delete_notebook_instance_lifecycle_config(&self, builder: DeleteNotebookInstanceLifecycleConfigInputBuilder) -> Result<DeleteNotebookInstanceLifecycleConfigOutput, SdkError<DeleteNotebookInstanceLifecycleConfigError>>;
        async fn delete_optimization_job(&self, builder: DeleteOptimizationJobInputBuilder) -> Result<DeleteOptimizationJobOutput, SdkError<DeleteOptimizationJobError>>;
        async fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> Result<DeletePipelineOutput, SdkError<DeletePipelineError>>;
        async fn delete_project(&self, builder: DeleteProjectInputBuilder) -> Result<DeleteProjectOutput, SdkError<DeleteProjectError>>;
        async fn delete_space(&self, builder: DeleteSpaceInputBuilder) -> Result<DeleteSpaceOutput, SdkError<DeleteSpaceError>>;
        async fn delete_studio_lifecycle_config(&self, builder: DeleteStudioLifecycleConfigInputBuilder) -> Result<DeleteStudioLifecycleConfigOutput, SdkError<DeleteStudioLifecycleConfigError>>;
        async fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> Result<DeleteTagsOutput, SdkError<DeleteTagsError>>;
        async fn delete_trial(&self, builder: DeleteTrialInputBuilder) -> Result<DeleteTrialOutput, SdkError<DeleteTrialError>>;
        async fn delete_trial_component(&self, builder: DeleteTrialComponentInputBuilder) -> Result<DeleteTrialComponentOutput, SdkError<DeleteTrialComponentError>>;
        async fn delete_user_profile(&self, builder: DeleteUserProfileInputBuilder) -> Result<DeleteUserProfileOutput, SdkError<DeleteUserProfileError>>;
        async fn delete_workforce(&self, builder: DeleteWorkforceInputBuilder) -> Result<DeleteWorkforceOutput, SdkError<DeleteWorkforceError>>;
        async fn delete_workteam(&self, builder: DeleteWorkteamInputBuilder) -> Result<DeleteWorkteamOutput, SdkError<DeleteWorkteamError>>;
        async fn deregister_devices(&self, builder: DeregisterDevicesInputBuilder) -> Result<DeregisterDevicesOutput, SdkError<DeregisterDevicesError>>;
        async fn describe_action(&self, builder: DescribeActionInputBuilder) -> Result<DescribeActionOutput, SdkError<DescribeActionError>>;
        async fn describe_algorithm(&self, builder: DescribeAlgorithmInputBuilder) -> Result<DescribeAlgorithmOutput, SdkError<DescribeAlgorithmError>>;
        async fn describe_app(&self, builder: DescribeAppInputBuilder) -> Result<DescribeAppOutput, SdkError<DescribeAppError>>;
        async fn describe_app_image_config(&self, builder: DescribeAppImageConfigInputBuilder) -> Result<DescribeAppImageConfigOutput, SdkError<DescribeAppImageConfigError>>;
        async fn describe_artifact(&self, builder: DescribeArtifactInputBuilder) -> Result<DescribeArtifactOutput, SdkError<DescribeArtifactError>>;
        async fn describe_auto_ml_job(&self, builder: DescribeAutoMlJobInputBuilder) -> Result<DescribeAutoMlJobOutput, SdkError<DescribeAutoMLJobError>>;
        async fn describe_auto_ml_job_v2(&self, builder: DescribeAutoMlJobV2InputBuilder) -> Result<DescribeAutoMlJobV2Output, SdkError<DescribeAutoMLJobV2Error>>;
        async fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> Result<DescribeClusterOutput, SdkError<DescribeClusterError>>;
        async fn describe_cluster_node(&self, builder: DescribeClusterNodeInputBuilder) -> Result<DescribeClusterNodeOutput, SdkError<DescribeClusterNodeError>>;
        async fn describe_code_repository(&self, builder: DescribeCodeRepositoryInputBuilder) -> Result<DescribeCodeRepositoryOutput, SdkError<DescribeCodeRepositoryError>>;
        async fn describe_compilation_job(&self, builder: DescribeCompilationJobInputBuilder) -> Result<DescribeCompilationJobOutput, SdkError<DescribeCompilationJobError>>;
        async fn describe_context(&self, builder: DescribeContextInputBuilder) -> Result<DescribeContextOutput, SdkError<DescribeContextError>>;
        async fn describe_data_quality_job_definition(&self, builder: DescribeDataQualityJobDefinitionInputBuilder) -> Result<DescribeDataQualityJobDefinitionOutput, SdkError<DescribeDataQualityJobDefinitionError>>;
        async fn describe_device(&self, builder: DescribeDeviceInputBuilder) -> Result<DescribeDeviceOutput, SdkError<DescribeDeviceError>>;
        async fn describe_device_fleet(&self, builder: DescribeDeviceFleetInputBuilder) -> Result<DescribeDeviceFleetOutput, SdkError<DescribeDeviceFleetError>>;
        async fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> Result<DescribeDomainOutput, SdkError<DescribeDomainError>>;
        async fn describe_edge_deployment_plan(&self, builder: DescribeEdgeDeploymentPlanInputBuilder) -> Result<DescribeEdgeDeploymentPlanOutput, SdkError<DescribeEdgeDeploymentPlanError>>;
        async fn describe_edge_packaging_job(&self, builder: DescribeEdgePackagingJobInputBuilder) -> Result<DescribeEdgePackagingJobOutput, SdkError<DescribeEdgePackagingJobError>>;
        async fn describe_endpoint(&self, builder: DescribeEndpointInputBuilder) -> Result<DescribeEndpointOutput, SdkError<DescribeEndpointError>>;
        async fn describe_endpoint_config(&self, builder: DescribeEndpointConfigInputBuilder) -> Result<DescribeEndpointConfigOutput, SdkError<DescribeEndpointConfigError>>;
        async fn describe_experiment(&self, builder: DescribeExperimentInputBuilder) -> Result<DescribeExperimentOutput, SdkError<DescribeExperimentError>>;
        async fn describe_feature_group(&self, builder: DescribeFeatureGroupInputBuilder) -> Result<DescribeFeatureGroupOutput, SdkError<DescribeFeatureGroupError>>;
        async fn describe_feature_metadata(&self, builder: DescribeFeatureMetadataInputBuilder) -> Result<DescribeFeatureMetadataOutput, SdkError<DescribeFeatureMetadataError>>;
        async fn describe_flow_definition(&self, builder: DescribeFlowDefinitionInputBuilder) -> Result<DescribeFlowDefinitionOutput, SdkError<DescribeFlowDefinitionError>>;
        async fn describe_hub(&self, builder: DescribeHubInputBuilder) -> Result<DescribeHubOutput, SdkError<DescribeHubError>>;
        async fn describe_hub_content(&self, builder: DescribeHubContentInputBuilder) -> Result<DescribeHubContentOutput, SdkError<DescribeHubContentError>>;
        async fn describe_human_task_ui(&self, builder: DescribeHumanTaskUiInputBuilder) -> Result<DescribeHumanTaskUiOutput, SdkError<DescribeHumanTaskUiError>>;
        async fn describe_hyper_parameter_tuning_job(&self, builder: DescribeHyperParameterTuningJobInputBuilder) -> Result<DescribeHyperParameterTuningJobOutput, SdkError<DescribeHyperParameterTuningJobError>>;
        async fn describe_image(&self, builder: DescribeImageInputBuilder) -> Result<DescribeImageOutput, SdkError<DescribeImageError>>;
        async fn describe_image_version(&self, builder: DescribeImageVersionInputBuilder) -> Result<DescribeImageVersionOutput, SdkError<DescribeImageVersionError>>;
        async fn describe_inference_component(&self, builder: DescribeInferenceComponentInputBuilder) -> Result<DescribeInferenceComponentOutput, SdkError<DescribeInferenceComponentError>>;
        async fn describe_inference_experiment(&self, builder: DescribeInferenceExperimentInputBuilder) -> Result<DescribeInferenceExperimentOutput, SdkError<DescribeInferenceExperimentError>>;
        async fn describe_inference_recommendations_job(&self, builder: DescribeInferenceRecommendationsJobInputBuilder) -> Result<DescribeInferenceRecommendationsJobOutput, SdkError<DescribeInferenceRecommendationsJobError>>;
        async fn describe_labeling_job(&self, builder: DescribeLabelingJobInputBuilder) -> Result<DescribeLabelingJobOutput, SdkError<DescribeLabelingJobError>>;
        async fn describe_lineage_group(&self, builder: DescribeLineageGroupInputBuilder) -> Result<DescribeLineageGroupOutput, SdkError<DescribeLineageGroupError>>;
        async fn describe_mlflow_tracking_server(&self, builder: DescribeMlflowTrackingServerInputBuilder) -> Result<DescribeMlflowTrackingServerOutput, SdkError<DescribeMlflowTrackingServerError>>;
        async fn describe_model(&self, builder: DescribeModelInputBuilder) -> Result<DescribeModelOutput, SdkError<DescribeModelError>>;
        async fn describe_model_bias_job_definition(&self, builder: DescribeModelBiasJobDefinitionInputBuilder) -> Result<DescribeModelBiasJobDefinitionOutput, SdkError<DescribeModelBiasJobDefinitionError>>;
        async fn describe_model_card(&self, builder: DescribeModelCardInputBuilder) -> Result<DescribeModelCardOutput, SdkError<DescribeModelCardError>>;
        async fn describe_model_card_export_job(&self, builder: DescribeModelCardExportJobInputBuilder) -> Result<DescribeModelCardExportJobOutput, SdkError<DescribeModelCardExportJobError>>;
        async fn describe_model_explainability_job_definition(&self, builder: DescribeModelExplainabilityJobDefinitionInputBuilder) -> Result<DescribeModelExplainabilityJobDefinitionOutput, SdkError<DescribeModelExplainabilityJobDefinitionError>>;
        async fn describe_model_package(&self, builder: DescribeModelPackageInputBuilder) -> Result<DescribeModelPackageOutput, SdkError<DescribeModelPackageError>>;
        async fn describe_model_package_group(&self, builder: DescribeModelPackageGroupInputBuilder) -> Result<DescribeModelPackageGroupOutput, SdkError<DescribeModelPackageGroupError>>;
        async fn describe_model_quality_job_definition(&self, builder: DescribeModelQualityJobDefinitionInputBuilder) -> Result<DescribeModelQualityJobDefinitionOutput, SdkError<DescribeModelQualityJobDefinitionError>>;
        async fn describe_monitoring_schedule(&self, builder: DescribeMonitoringScheduleInputBuilder) -> Result<DescribeMonitoringScheduleOutput, SdkError<DescribeMonitoringScheduleError>>;
        async fn describe_notebook_instance(&self, builder: DescribeNotebookInstanceInputBuilder) -> Result<DescribeNotebookInstanceOutput, SdkError<DescribeNotebookInstanceError>>;
        async fn describe_notebook_instance_lifecycle_config(&self, builder: DescribeNotebookInstanceLifecycleConfigInputBuilder) -> Result<DescribeNotebookInstanceLifecycleConfigOutput, SdkError<DescribeNotebookInstanceLifecycleConfigError>>;
        async fn describe_optimization_job(&self, builder: DescribeOptimizationJobInputBuilder) -> Result<DescribeOptimizationJobOutput, SdkError<DescribeOptimizationJobError>>;
        async fn describe_pipeline(&self, builder: DescribePipelineInputBuilder) -> Result<DescribePipelineOutput, SdkError<DescribePipelineError>>;
        async fn describe_pipeline_definition_for_execution(&self, builder: DescribePipelineDefinitionForExecutionInputBuilder) -> Result<DescribePipelineDefinitionForExecutionOutput, SdkError<DescribePipelineDefinitionForExecutionError>>;
        async fn describe_pipeline_execution(&self, builder: DescribePipelineExecutionInputBuilder) -> Result<DescribePipelineExecutionOutput, SdkError<DescribePipelineExecutionError>>;
        async fn describe_processing_job(&self, builder: DescribeProcessingJobInputBuilder) -> Result<DescribeProcessingJobOutput, SdkError<DescribeProcessingJobError>>;
        async fn describe_project(&self, builder: DescribeProjectInputBuilder) -> Result<DescribeProjectOutput, SdkError<DescribeProjectError>>;
        async fn describe_space(&self, builder: DescribeSpaceInputBuilder) -> Result<DescribeSpaceOutput, SdkError<DescribeSpaceError>>;
        async fn describe_studio_lifecycle_config(&self, builder: DescribeStudioLifecycleConfigInputBuilder) -> Result<DescribeStudioLifecycleConfigOutput, SdkError<DescribeStudioLifecycleConfigError>>;
        async fn describe_subscribed_workteam(&self, builder: DescribeSubscribedWorkteamInputBuilder) -> Result<DescribeSubscribedWorkteamOutput, SdkError<DescribeSubscribedWorkteamError>>;
        async fn describe_training_job(&self, builder: DescribeTrainingJobInputBuilder) -> Result<DescribeTrainingJobOutput, SdkError<DescribeTrainingJobError>>;
        async fn describe_transform_job(&self, builder: DescribeTransformJobInputBuilder) -> Result<DescribeTransformJobOutput, SdkError<DescribeTransformJobError>>;
        async fn describe_trial(&self, builder: DescribeTrialInputBuilder) -> Result<DescribeTrialOutput, SdkError<DescribeTrialError>>;
        async fn describe_trial_component(&self, builder: DescribeTrialComponentInputBuilder) -> Result<DescribeTrialComponentOutput, SdkError<DescribeTrialComponentError>>;
        async fn describe_user_profile(&self, builder: DescribeUserProfileInputBuilder) -> Result<DescribeUserProfileOutput, SdkError<DescribeUserProfileError>>;
        async fn describe_workforce(&self, builder: DescribeWorkforceInputBuilder) -> Result<DescribeWorkforceOutput, SdkError<DescribeWorkforceError>>;
        async fn describe_workteam(&self, builder: DescribeWorkteamInputBuilder) -> Result<DescribeWorkteamOutput, SdkError<DescribeWorkteamError>>;
        async fn disable_sagemaker_servicecatalog_portfolio(&self, builder: DisableSagemakerServicecatalogPortfolioInputBuilder) -> Result<DisableSagemakerServicecatalogPortfolioOutput, SdkError<DisableSagemakerServicecatalogPortfolioError>>;
        async fn disassociate_trial_component(&self, builder: DisassociateTrialComponentInputBuilder) -> Result<DisassociateTrialComponentOutput, SdkError<DisassociateTrialComponentError>>;
        async fn enable_sagemaker_servicecatalog_portfolio(&self, builder: EnableSagemakerServicecatalogPortfolioInputBuilder) -> Result<EnableSagemakerServicecatalogPortfolioOutput, SdkError<EnableSagemakerServicecatalogPortfolioError>>;
        async fn get_device_fleet_report(&self, builder: GetDeviceFleetReportInputBuilder) -> Result<GetDeviceFleetReportOutput, SdkError<GetDeviceFleetReportError>>;
        async fn get_lineage_group_policy(&self, builder: GetLineageGroupPolicyInputBuilder) -> Result<GetLineageGroupPolicyOutput, SdkError<GetLineageGroupPolicyError>>;
        async fn get_model_package_group_policy(&self, builder: GetModelPackageGroupPolicyInputBuilder) -> Result<GetModelPackageGroupPolicyOutput, SdkError<GetModelPackageGroupPolicyError>>;
        async fn get_sagemaker_servicecatalog_portfolio_status(&self, builder: GetSagemakerServicecatalogPortfolioStatusInputBuilder) -> Result<GetSagemakerServicecatalogPortfolioStatusOutput, SdkError<GetSagemakerServicecatalogPortfolioStatusError>>;
        async fn get_scaling_configuration_recommendation(&self, builder: GetScalingConfigurationRecommendationInputBuilder) -> Result<GetScalingConfigurationRecommendationOutput, SdkError<GetScalingConfigurationRecommendationError>>;
        async fn get_search_suggestions(&self, builder: GetSearchSuggestionsInputBuilder) -> Result<GetSearchSuggestionsOutput, SdkError<GetSearchSuggestionsError>>;
        async fn import_hub_content(&self, builder: ImportHubContentInputBuilder) -> Result<ImportHubContentOutput, SdkError<ImportHubContentError>>;
        async fn list_actions(&self, builder: ListActionsInputBuilder) -> Result<ListActionsOutput, SdkError<ListActionsError>>;
        async fn list_algorithms(&self, builder: ListAlgorithmsInputBuilder) -> Result<ListAlgorithmsOutput, SdkError<ListAlgorithmsError>>;
        async fn list_aliases(&self, builder: ListAliasesInputBuilder) -> Result<ListAliasesOutput, SdkError<ListAliasesError>>;
        async fn list_app_image_configs(&self, builder: ListAppImageConfigsInputBuilder) -> Result<ListAppImageConfigsOutput, SdkError<ListAppImageConfigsError>>;
        async fn list_apps(&self, builder: ListAppsInputBuilder) -> Result<ListAppsOutput, SdkError<ListAppsError>>;
        async fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> Result<ListArtifactsOutput, SdkError<ListArtifactsError>>;
        async fn list_associations(&self, builder: ListAssociationsInputBuilder) -> Result<ListAssociationsOutput, SdkError<ListAssociationsError>>;
        async fn list_auto_ml_jobs(&self, builder: ListAutoMlJobsInputBuilder) -> Result<ListAutoMlJobsOutput, SdkError<ListAutoMLJobsError>>;
        async fn list_candidates_for_auto_ml_job(&self, builder: ListCandidatesForAutoMlJobInputBuilder) -> Result<ListCandidatesForAutoMlJobOutput, SdkError<ListCandidatesForAutoMLJobError>>;
        async fn list_cluster_nodes(&self, builder: ListClusterNodesInputBuilder) -> Result<ListClusterNodesOutput, SdkError<ListClusterNodesError>>;
        async fn list_clusters(&self, builder: ListClustersInputBuilder) -> Result<ListClustersOutput, SdkError<ListClustersError>>;
        async fn list_code_repositories(&self, builder: ListCodeRepositoriesInputBuilder) -> Result<ListCodeRepositoriesOutput, SdkError<ListCodeRepositoriesError>>;
        async fn list_compilation_jobs(&self, builder: ListCompilationJobsInputBuilder) -> Result<ListCompilationJobsOutput, SdkError<ListCompilationJobsError>>;
        async fn list_contexts(&self, builder: ListContextsInputBuilder) -> Result<ListContextsOutput, SdkError<ListContextsError>>;
        async fn list_data_quality_job_definitions(&self, builder: ListDataQualityJobDefinitionsInputBuilder) -> Result<ListDataQualityJobDefinitionsOutput, SdkError<ListDataQualityJobDefinitionsError>>;
        async fn list_device_fleets(&self, builder: ListDeviceFleetsInputBuilder) -> Result<ListDeviceFleetsOutput, SdkError<ListDeviceFleetsError>>;
        async fn list_devices(&self, builder: ListDevicesInputBuilder) -> Result<ListDevicesOutput, SdkError<ListDevicesError>>;
        async fn list_domains(&self, builder: ListDomainsInputBuilder) -> Result<ListDomainsOutput, SdkError<ListDomainsError>>;
        async fn list_edge_deployment_plans(&self, builder: ListEdgeDeploymentPlansInputBuilder) -> Result<ListEdgeDeploymentPlansOutput, SdkError<ListEdgeDeploymentPlansError>>;
        async fn list_edge_packaging_jobs(&self, builder: ListEdgePackagingJobsInputBuilder) -> Result<ListEdgePackagingJobsOutput, SdkError<ListEdgePackagingJobsError>>;
        async fn list_endpoint_configs(&self, builder: ListEndpointConfigsInputBuilder) -> Result<ListEndpointConfigsOutput, SdkError<ListEndpointConfigsError>>;
        async fn list_endpoints(&self, builder: ListEndpointsInputBuilder) -> Result<ListEndpointsOutput, SdkError<ListEndpointsError>>;
        async fn list_experiments(&self, builder: ListExperimentsInputBuilder) -> Result<ListExperimentsOutput, SdkError<ListExperimentsError>>;
        async fn list_feature_groups(&self, builder: ListFeatureGroupsInputBuilder) -> Result<ListFeatureGroupsOutput, SdkError<ListFeatureGroupsError>>;
        async fn list_flow_definitions(&self, builder: ListFlowDefinitionsInputBuilder) -> Result<ListFlowDefinitionsOutput, SdkError<ListFlowDefinitionsError>>;
        async fn list_hub_content_versions(&self, builder: ListHubContentVersionsInputBuilder) -> Result<ListHubContentVersionsOutput, SdkError<ListHubContentVersionsError>>;
        async fn list_hub_contents(&self, builder: ListHubContentsInputBuilder) -> Result<ListHubContentsOutput, SdkError<ListHubContentsError>>;
        async fn list_hubs(&self, builder: ListHubsInputBuilder) -> Result<ListHubsOutput, SdkError<ListHubsError>>;
        async fn list_human_task_uis(&self, builder: ListHumanTaskUisInputBuilder) -> Result<ListHumanTaskUisOutput, SdkError<ListHumanTaskUisError>>;
        async fn list_hyper_parameter_tuning_jobs(&self, builder: ListHyperParameterTuningJobsInputBuilder) -> Result<ListHyperParameterTuningJobsOutput, SdkError<ListHyperParameterTuningJobsError>>;
        async fn list_image_versions(&self, builder: ListImageVersionsInputBuilder) -> Result<ListImageVersionsOutput, SdkError<ListImageVersionsError>>;
        async fn list_images(&self, builder: ListImagesInputBuilder) -> Result<ListImagesOutput, SdkError<ListImagesError>>;
        async fn list_inference_components(&self, builder: ListInferenceComponentsInputBuilder) -> Result<ListInferenceComponentsOutput, SdkError<ListInferenceComponentsError>>;
        async fn list_inference_experiments(&self, builder: ListInferenceExperimentsInputBuilder) -> Result<ListInferenceExperimentsOutput, SdkError<ListInferenceExperimentsError>>;
        async fn list_inference_recommendations_job_steps(&self, builder: ListInferenceRecommendationsJobStepsInputBuilder) -> Result<ListInferenceRecommendationsJobStepsOutput, SdkError<ListInferenceRecommendationsJobStepsError>>;
        async fn list_inference_recommendations_jobs(&self, builder: ListInferenceRecommendationsJobsInputBuilder) -> Result<ListInferenceRecommendationsJobsOutput, SdkError<ListInferenceRecommendationsJobsError>>;
        async fn list_labeling_jobs(&self, builder: ListLabelingJobsInputBuilder) -> Result<ListLabelingJobsOutput, SdkError<ListLabelingJobsError>>;
        async fn list_labeling_jobs_for_workteam(&self, builder: ListLabelingJobsForWorkteamInputBuilder) -> Result<ListLabelingJobsForWorkteamOutput, SdkError<ListLabelingJobsForWorkteamError>>;
        async fn list_lineage_groups(&self, builder: ListLineageGroupsInputBuilder) -> Result<ListLineageGroupsOutput, SdkError<ListLineageGroupsError>>;
        async fn list_mlflow_tracking_servers(&self, builder: ListMlflowTrackingServersInputBuilder) -> Result<ListMlflowTrackingServersOutput, SdkError<ListMlflowTrackingServersError>>;
        async fn list_model_bias_job_definitions(&self, builder: ListModelBiasJobDefinitionsInputBuilder) -> Result<ListModelBiasJobDefinitionsOutput, SdkError<ListModelBiasJobDefinitionsError>>;
        async fn list_model_card_export_jobs(&self, builder: ListModelCardExportJobsInputBuilder) -> Result<ListModelCardExportJobsOutput, SdkError<ListModelCardExportJobsError>>;
        async fn list_model_card_versions(&self, builder: ListModelCardVersionsInputBuilder) -> Result<ListModelCardVersionsOutput, SdkError<ListModelCardVersionsError>>;
        async fn list_model_cards(&self, builder: ListModelCardsInputBuilder) -> Result<ListModelCardsOutput, SdkError<ListModelCardsError>>;
        async fn list_model_explainability_job_definitions(&self, builder: ListModelExplainabilityJobDefinitionsInputBuilder) -> Result<ListModelExplainabilityJobDefinitionsOutput, SdkError<ListModelExplainabilityJobDefinitionsError>>;
        async fn list_model_metadata(&self, builder: ListModelMetadataInputBuilder) -> Result<ListModelMetadataOutput, SdkError<ListModelMetadataError>>;
        async fn list_model_package_groups(&self, builder: ListModelPackageGroupsInputBuilder) -> Result<ListModelPackageGroupsOutput, SdkError<ListModelPackageGroupsError>>;
        async fn list_model_packages(&self, builder: ListModelPackagesInputBuilder) -> Result<ListModelPackagesOutput, SdkError<ListModelPackagesError>>;
        async fn list_model_quality_job_definitions(&self, builder: ListModelQualityJobDefinitionsInputBuilder) -> Result<ListModelQualityJobDefinitionsOutput, SdkError<ListModelQualityJobDefinitionsError>>;
        async fn list_models(&self, builder: ListModelsInputBuilder) -> Result<ListModelsOutput, SdkError<ListModelsError>>;
        async fn list_monitoring_alert_history(&self, builder: ListMonitoringAlertHistoryInputBuilder) -> Result<ListMonitoringAlertHistoryOutput, SdkError<ListMonitoringAlertHistoryError>>;
        async fn list_monitoring_alerts(&self, builder: ListMonitoringAlertsInputBuilder) -> Result<ListMonitoringAlertsOutput, SdkError<ListMonitoringAlertsError>>;
        async fn list_monitoring_executions(&self, builder: ListMonitoringExecutionsInputBuilder) -> Result<ListMonitoringExecutionsOutput, SdkError<ListMonitoringExecutionsError>>;
        async fn list_monitoring_schedules(&self, builder: ListMonitoringSchedulesInputBuilder) -> Result<ListMonitoringSchedulesOutput, SdkError<ListMonitoringSchedulesError>>;
        async fn list_notebook_instance_lifecycle_configs(&self, builder: ListNotebookInstanceLifecycleConfigsInputBuilder) -> Result<ListNotebookInstanceLifecycleConfigsOutput, SdkError<ListNotebookInstanceLifecycleConfigsError>>;
        async fn list_notebook_instances(&self, builder: ListNotebookInstancesInputBuilder) -> Result<ListNotebookInstancesOutput, SdkError<ListNotebookInstancesError>>;
        async fn list_optimization_jobs(&self, builder: ListOptimizationJobsInputBuilder) -> Result<ListOptimizationJobsOutput, SdkError<ListOptimizationJobsError>>;
        async fn list_pipeline_execution_steps(&self, builder: ListPipelineExecutionStepsInputBuilder) -> Result<ListPipelineExecutionStepsOutput, SdkError<ListPipelineExecutionStepsError>>;
        async fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>;
        async fn list_pipeline_parameters_for_execution(&self, builder: ListPipelineParametersForExecutionInputBuilder) -> Result<ListPipelineParametersForExecutionOutput, SdkError<ListPipelineParametersForExecutionError>>;
        async fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> Result<ListPipelinesOutput, SdkError<ListPipelinesError>>;
        async fn list_processing_jobs(&self, builder: ListProcessingJobsInputBuilder) -> Result<ListProcessingJobsOutput, SdkError<ListProcessingJobsError>>;
        async fn list_projects(&self, builder: ListProjectsInputBuilder) -> Result<ListProjectsOutput, SdkError<ListProjectsError>>;
        async fn list_resource_catalogs(&self, builder: ListResourceCatalogsInputBuilder) -> Result<ListResourceCatalogsOutput, SdkError<ListResourceCatalogsError>>;
        async fn list_spaces(&self, builder: ListSpacesInputBuilder) -> Result<ListSpacesOutput, SdkError<ListSpacesError>>;
        async fn list_stage_devices(&self, builder: ListStageDevicesInputBuilder) -> Result<ListStageDevicesOutput, SdkError<ListStageDevicesError>>;
        async fn list_studio_lifecycle_configs(&self, builder: ListStudioLifecycleConfigsInputBuilder) -> Result<ListStudioLifecycleConfigsOutput, SdkError<ListStudioLifecycleConfigsError>>;
        async fn list_subscribed_workteams(&self, builder: ListSubscribedWorkteamsInputBuilder) -> Result<ListSubscribedWorkteamsOutput, SdkError<ListSubscribedWorkteamsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn list_training_jobs(&self, builder: ListTrainingJobsInputBuilder) -> Result<ListTrainingJobsOutput, SdkError<ListTrainingJobsError>>;
        async fn list_training_jobs_for_hyper_parameter_tuning_job(&self, builder: ListTrainingJobsForHyperParameterTuningJobInputBuilder) -> Result<ListTrainingJobsForHyperParameterTuningJobOutput, SdkError<ListTrainingJobsForHyperParameterTuningJobError>>;
        async fn list_transform_jobs(&self, builder: ListTransformJobsInputBuilder) -> Result<ListTransformJobsOutput, SdkError<ListTransformJobsError>>;
        async fn list_trial_components(&self, builder: ListTrialComponentsInputBuilder) -> Result<ListTrialComponentsOutput, SdkError<ListTrialComponentsError>>;
        async fn list_trials(&self, builder: ListTrialsInputBuilder) -> Result<ListTrialsOutput, SdkError<ListTrialsError>>;
        async fn list_user_profiles(&self, builder: ListUserProfilesInputBuilder) -> Result<ListUserProfilesOutput, SdkError<ListUserProfilesError>>;
        async fn list_workforces(&self, builder: ListWorkforcesInputBuilder) -> Result<ListWorkforcesOutput, SdkError<ListWorkforcesError>>;
        async fn list_workteams(&self, builder: ListWorkteamsInputBuilder) -> Result<ListWorkteamsOutput, SdkError<ListWorkteamsError>>;
        async fn put_model_package_group_policy(&self, builder: PutModelPackageGroupPolicyInputBuilder) -> Result<PutModelPackageGroupPolicyOutput, SdkError<PutModelPackageGroupPolicyError>>;
        async fn query_lineage(&self, builder: QueryLineageInputBuilder) -> Result<QueryLineageOutput, SdkError<QueryLineageError>>;
        async fn register_devices(&self, builder: RegisterDevicesInputBuilder) -> Result<RegisterDevicesOutput, SdkError<RegisterDevicesError>>;
        async fn render_ui_template(&self, builder: RenderUiTemplateInputBuilder) -> Result<RenderUiTemplateOutput, SdkError<RenderUiTemplateError>>;
        async fn retry_pipeline_execution(&self, builder: RetryPipelineExecutionInputBuilder) -> Result<RetryPipelineExecutionOutput, SdkError<RetryPipelineExecutionError>>;
        async fn search(&self, builder: SearchInputBuilder) -> Result<SearchOutput, SdkError<SearchError>>;
        async fn send_pipeline_execution_step_failure(&self, builder: SendPipelineExecutionStepFailureInputBuilder) -> Result<SendPipelineExecutionStepFailureOutput, SdkError<SendPipelineExecutionStepFailureError>>;
        async fn send_pipeline_execution_step_success(&self, builder: SendPipelineExecutionStepSuccessInputBuilder) -> Result<SendPipelineExecutionStepSuccessOutput, SdkError<SendPipelineExecutionStepSuccessError>>;
        async fn start_edge_deployment_stage(&self, builder: StartEdgeDeploymentStageInputBuilder) -> Result<StartEdgeDeploymentStageOutput, SdkError<StartEdgeDeploymentStageError>>;
        async fn start_inference_experiment(&self, builder: StartInferenceExperimentInputBuilder) -> Result<StartInferenceExperimentOutput, SdkError<StartInferenceExperimentError>>;
        async fn start_mlflow_tracking_server(&self, builder: StartMlflowTrackingServerInputBuilder) -> Result<StartMlflowTrackingServerOutput, SdkError<StartMlflowTrackingServerError>>;
        async fn start_monitoring_schedule(&self, builder: StartMonitoringScheduleInputBuilder) -> Result<StartMonitoringScheduleOutput, SdkError<StartMonitoringScheduleError>>;
        async fn start_notebook_instance(&self, builder: StartNotebookInstanceInputBuilder) -> Result<StartNotebookInstanceOutput, SdkError<StartNotebookInstanceError>>;
        async fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>;
        async fn stop_auto_ml_job(&self, builder: StopAutoMlJobInputBuilder) -> Result<StopAutoMlJobOutput, SdkError<StopAutoMLJobError>>;
        async fn stop_compilation_job(&self, builder: StopCompilationJobInputBuilder) -> Result<StopCompilationJobOutput, SdkError<StopCompilationJobError>>;
        async fn stop_edge_deployment_stage(&self, builder: StopEdgeDeploymentStageInputBuilder) -> Result<StopEdgeDeploymentStageOutput, SdkError<StopEdgeDeploymentStageError>>;
        async fn stop_edge_packaging_job(&self, builder: StopEdgePackagingJobInputBuilder) -> Result<StopEdgePackagingJobOutput, SdkError<StopEdgePackagingJobError>>;
        async fn stop_hyper_parameter_tuning_job(&self, builder: StopHyperParameterTuningJobInputBuilder) -> Result<StopHyperParameterTuningJobOutput, SdkError<StopHyperParameterTuningJobError>>;
        async fn stop_inference_experiment(&self, builder: StopInferenceExperimentInputBuilder) -> Result<StopInferenceExperimentOutput, SdkError<StopInferenceExperimentError>>;
        async fn stop_inference_recommendations_job(&self, builder: StopInferenceRecommendationsJobInputBuilder) -> Result<StopInferenceRecommendationsJobOutput, SdkError<StopInferenceRecommendationsJobError>>;
        async fn stop_labeling_job(&self, builder: StopLabelingJobInputBuilder) -> Result<StopLabelingJobOutput, SdkError<StopLabelingJobError>>;
        async fn stop_mlflow_tracking_server(&self, builder: StopMlflowTrackingServerInputBuilder) -> Result<StopMlflowTrackingServerOutput, SdkError<StopMlflowTrackingServerError>>;
        async fn stop_monitoring_schedule(&self, builder: StopMonitoringScheduleInputBuilder) -> Result<StopMonitoringScheduleOutput, SdkError<StopMonitoringScheduleError>>;
        async fn stop_notebook_instance(&self, builder: StopNotebookInstanceInputBuilder) -> Result<StopNotebookInstanceOutput, SdkError<StopNotebookInstanceError>>;
        async fn stop_optimization_job(&self, builder: StopOptimizationJobInputBuilder) -> Result<StopOptimizationJobOutput, SdkError<StopOptimizationJobError>>;
        async fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>;
        async fn stop_processing_job(&self, builder: StopProcessingJobInputBuilder) -> Result<StopProcessingJobOutput, SdkError<StopProcessingJobError>>;
        async fn stop_training_job(&self, builder: StopTrainingJobInputBuilder) -> Result<StopTrainingJobOutput, SdkError<StopTrainingJobError>>;
        async fn stop_transform_job(&self, builder: StopTransformJobInputBuilder) -> Result<StopTransformJobOutput, SdkError<StopTransformJobError>>;
        async fn update_action(&self, builder: UpdateActionInputBuilder) -> Result<UpdateActionOutput, SdkError<UpdateActionError>>;
        async fn update_app_image_config(&self, builder: UpdateAppImageConfigInputBuilder) -> Result<UpdateAppImageConfigOutput, SdkError<UpdateAppImageConfigError>>;
        async fn update_artifact(&self, builder: UpdateArtifactInputBuilder) -> Result<UpdateArtifactOutput, SdkError<UpdateArtifactError>>;
        async fn update_cluster(&self, builder: UpdateClusterInputBuilder) -> Result<UpdateClusterOutput, SdkError<UpdateClusterError>>;
        async fn update_cluster_software(&self, builder: UpdateClusterSoftwareInputBuilder) -> Result<UpdateClusterSoftwareOutput, SdkError<UpdateClusterSoftwareError>>;
        async fn update_code_repository(&self, builder: UpdateCodeRepositoryInputBuilder) -> Result<UpdateCodeRepositoryOutput, SdkError<UpdateCodeRepositoryError>>;
        async fn update_context(&self, builder: UpdateContextInputBuilder) -> Result<UpdateContextOutput, SdkError<UpdateContextError>>;
        async fn update_device_fleet(&self, builder: UpdateDeviceFleetInputBuilder) -> Result<UpdateDeviceFleetOutput, SdkError<UpdateDeviceFleetError>>;
        async fn update_devices(&self, builder: UpdateDevicesInputBuilder) -> Result<UpdateDevicesOutput, SdkError<UpdateDevicesError>>;
        async fn update_domain(&self, builder: UpdateDomainInputBuilder) -> Result<UpdateDomainOutput, SdkError<UpdateDomainError>>;
        async fn update_endpoint(&self, builder: UpdateEndpointInputBuilder) -> Result<UpdateEndpointOutput, SdkError<UpdateEndpointError>>;
        async fn update_endpoint_weights_and_capacities(&self, builder: UpdateEndpointWeightsAndCapacitiesInputBuilder) -> Result<UpdateEndpointWeightsAndCapacitiesOutput, SdkError<UpdateEndpointWeightsAndCapacitiesError>>;
        async fn update_experiment(&self, builder: UpdateExperimentInputBuilder) -> Result<UpdateExperimentOutput, SdkError<UpdateExperimentError>>;
        async fn update_feature_group(&self, builder: UpdateFeatureGroupInputBuilder) -> Result<UpdateFeatureGroupOutput, SdkError<UpdateFeatureGroupError>>;
        async fn update_feature_metadata(&self, builder: UpdateFeatureMetadataInputBuilder) -> Result<UpdateFeatureMetadataOutput, SdkError<UpdateFeatureMetadataError>>;
        async fn update_hub(&self, builder: UpdateHubInputBuilder) -> Result<UpdateHubOutput, SdkError<UpdateHubError>>;
        async fn update_image(&self, builder: UpdateImageInputBuilder) -> Result<UpdateImageOutput, SdkError<UpdateImageError>>;
        async fn update_image_version(&self, builder: UpdateImageVersionInputBuilder) -> Result<UpdateImageVersionOutput, SdkError<UpdateImageVersionError>>;
        async fn update_inference_component(&self, builder: UpdateInferenceComponentInputBuilder) -> Result<UpdateInferenceComponentOutput, SdkError<UpdateInferenceComponentError>>;
        async fn update_inference_component_runtime_config(&self, builder: UpdateInferenceComponentRuntimeConfigInputBuilder) -> Result<UpdateInferenceComponentRuntimeConfigOutput, SdkError<UpdateInferenceComponentRuntimeConfigError>>;
        async fn update_inference_experiment(&self, builder: UpdateInferenceExperimentInputBuilder) -> Result<UpdateInferenceExperimentOutput, SdkError<UpdateInferenceExperimentError>>;
        async fn update_mlflow_tracking_server(&self, builder: UpdateMlflowTrackingServerInputBuilder) -> Result<UpdateMlflowTrackingServerOutput, SdkError<UpdateMlflowTrackingServerError>>;
        async fn update_model_card(&self, builder: UpdateModelCardInputBuilder) -> Result<UpdateModelCardOutput, SdkError<UpdateModelCardError>>;
        async fn update_model_package(&self, builder: UpdateModelPackageInputBuilder) -> Result<UpdateModelPackageOutput, SdkError<UpdateModelPackageError>>;
        async fn update_monitoring_alert(&self, builder: UpdateMonitoringAlertInputBuilder) -> Result<UpdateMonitoringAlertOutput, SdkError<UpdateMonitoringAlertError>>;
        async fn update_monitoring_schedule(&self, builder: UpdateMonitoringScheduleInputBuilder) -> Result<UpdateMonitoringScheduleOutput, SdkError<UpdateMonitoringScheduleError>>;
        async fn update_notebook_instance(&self, builder: UpdateNotebookInstanceInputBuilder) -> Result<UpdateNotebookInstanceOutput, SdkError<UpdateNotebookInstanceError>>;
        async fn update_notebook_instance_lifecycle_config(&self, builder: UpdateNotebookInstanceLifecycleConfigInputBuilder) -> Result<UpdateNotebookInstanceLifecycleConfigOutput, SdkError<UpdateNotebookInstanceLifecycleConfigError>>;
        async fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>;
        async fn update_pipeline_execution(&self, builder: UpdatePipelineExecutionInputBuilder) -> Result<UpdatePipelineExecutionOutput, SdkError<UpdatePipelineExecutionError>>;
        async fn update_project(&self, builder: UpdateProjectInputBuilder) -> Result<UpdateProjectOutput, SdkError<UpdateProjectError>>;
        async fn update_space(&self, builder: UpdateSpaceInputBuilder) -> Result<UpdateSpaceOutput, SdkError<UpdateSpaceError>>;
        async fn update_training_job(&self, builder: UpdateTrainingJobInputBuilder) -> Result<UpdateTrainingJobOutput, SdkError<UpdateTrainingJobError>>;
        async fn update_trial(&self, builder: UpdateTrialInputBuilder) -> Result<UpdateTrialOutput, SdkError<UpdateTrialError>>;
        async fn update_trial_component(&self, builder: UpdateTrialComponentInputBuilder) -> Result<UpdateTrialComponentOutput, SdkError<UpdateTrialComponentError>>;
        async fn update_user_profile(&self, builder: UpdateUserProfileInputBuilder) -> Result<UpdateUserProfileOutput, SdkError<UpdateUserProfileError>>;
        async fn update_workforce(&self, builder: UpdateWorkforceInputBuilder) -> Result<UpdateWorkforceOutput, SdkError<UpdateWorkforceError>>;
        async fn update_workteam(&self, builder: UpdateWorkteamInputBuilder) -> Result<UpdateWorkteamOutput, SdkError<UpdateWorkteamError>>;
    }
}
