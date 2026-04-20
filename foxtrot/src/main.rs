// Disable console on Windows for non-dev builds.
#![cfg_attr(feature = "release", windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod props;
mod screens;
mod theme;

use bevy::app::HierarchyPropagatePlugin;
use bevy::ecs::error::error;
use bevy::gltf::GltfPlugin;
use bevy::gltf::convert_coordinates::GltfConvertCoordinates;
use bevy::light::NotShadowCaster;
use bevy::log::LogPlugin;
use bevy::log::tracing_subscriber::field::MakeExt;
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy_seedling::SeedlingPlugin;
use enemies;
use gameplay_input;
use states::world::PausableSystems;
use states::world::Pause;
use states::world::PostPhysicsAppSystems;
use third_party;

use bevy::{asset::AssetMetaCheck, prelude::*};
use utils::asset_processing::default_image_sampler_descriptor;
use utils::post_process;
use utils::shader_compilation;

#[cfg(all(feature = "native", feature = "web"))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently disabled."
);
#[cfg(all(feature = "dev", feature = "release"))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "dev", feature = "release")))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently disabled."
);

fn main() -> AppExit {
    let mut app = App::new();
    // Don't panic on Bevy system errors, just log them.
    app.set_error_handler(error);

    // Add Bevy plugins.
    app.insert_resource(DefaultOpaqueRendererMethod::deferred());
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Foxtrot".to_string(),
                    fit_canvas_to_parent: true,
                    #[cfg(feature = "web")]
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: default_image_sampler_descriptor(),
            })
            .set(GltfPlugin {
                convert_coordinates: GltfConvertCoordinates {
                    rotate_scene_entity: true,
                    rotate_meshes: true,
                },
                ..default()
            })
            .set(LogPlugin {
                filter: format!(
                    concat!(
                        "{default},",
                        "symphonia_bundle_mp3::demuxer=warn,",
                        "symphonia_format_caf::demuxer=warn,",
                        "symphonia_format_isompf4::demuxer=warn,",
                        "symphonia_format_mkv::demuxer=warn,",
                        "symphonia_format_ogg::demuxer=warn,",
                        "symphonia_format_riff::demuxer=warn,",
                        "symphonia_format_wav::demuxer=warn,",
                        "calloop::loop_logic=error,",
                    ),
                    default = bevy::log::DEFAULT_FILTER
                ),
                fmt_layer: |_| {
                    Some(Box::new(
                        bevy::log::tracing_subscriber::fmt::Layer::default()
                            .without_time()
                            .map_fmt_fields(MakeExt::debug_alt)
                            .with_writer(std::io::stderr),
                    ))
                },
                ..default()
            }),
        #[cfg(feature = "native")]
        SeedlingPlugin::default(),
        #[cfg(feature = "web")]
        SeedlingPlugin::new_web_audio(),
        HierarchyPropagatePlugin::<NotShadowCaster>::new(Update),
        post_process::PostProcessPlugin,
    ));

    app.insert_resource(GlobalAmbientLight::NONE);

    // Order new `AppSet` variants by adding them here:
    app.configure_sets(
        Update,
        (
            PostPhysicsAppSystems::TickTimers,
            PostPhysicsAppSystems::ChangeUi,
            PostPhysicsAppSystems::PlaySounds,
            PostPhysicsAppSystems::PlayAnimations,
            PostPhysicsAppSystems::Update,
        )
            .chain(),
    );
    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    #[cfg(feature = "dev_native")]
    // Adding these here so that third party plugins can register their BRP methods.
    app.add_plugins((
        bevy::remote::RemotePlugin::default(),
        bevy::remote::http::RemoteHttpPlugin::default(),
    ));

    // Add third-party plugins.
    app.add_plugins(third_party::plugin);

    // Add other plugins.
    app.add_plugins((
        #[cfg(feature = "dev")]
        dev_tools::plugin,
        utils::plugin,
        screens::plugin,
        menus::plugin,
        props::plugin,
        theme::plugin,
        audio::plugin,
        gun::plugin,
        interaction_gateway::plugin,
        gameplay_input::plugin,
        enemies::plugin,
    ));

    // Add plugins that proload levels. These have to come later than the other plugins
    // because the objects they reference need to have been registered first.
    app.add_plugins((core_gameplay::plugin, shader_compilation::plugin));
    app.run()
}
