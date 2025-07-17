use bevy::app::plugin_group;

pub mod bounding_box_visualization;

plugin_group! {
  pub struct DebugPlugins {
    bounding_box_visualization:::BoundingBoxVisualizationPlugin,
  }
}