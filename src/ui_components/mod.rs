use bevy::app::plugin_group;

pub mod button;

plugin_group! {
  pub struct UIPlugins {
    button:::ButtonPlugin,
  }
}
