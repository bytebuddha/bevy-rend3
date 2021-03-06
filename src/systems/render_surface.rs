use bevy_math::UVec2;
use bevy_ecs::prelude::*;
use bevy_window::Windows;
use rend3_routine::base::BaseRenderGraph;
use rend3_routine::pbr::PbrRoutine;

use crate::{Renderer, Rend3Surfaces, Rend3Surface, AmbientLight, SkyBoxes};

pub fn render_surface(
    renderer: Res<Renderer>,
    surfaces: Res<Rend3Surfaces>,
    base_render_graph: Res<BaseRenderGraph>,
    pbr_routine: Res<PbrRoutine>,
    windows: Res<Windows>,
    mut skyboxes: ResMut<SkyBoxes>,
    ambient: Res<AmbientLight>
) {
    for (id, Rend3Surface { surface, tone_mapping, .. }) in surfaces.surfaces.iter() {
        let window = windows.get(*id).unwrap();
        let frame = rend3::util::output::OutputFrame::Surface {
            surface: surface.clone(),
        };
        // Ready up the renderer
        let (cmd_bufs, ready) = renderer.0.ready();

        // Build a rendergraph
        let mut graph = rend3::graph::RenderGraph::new();

        let skybox = if let Some(skybox) = skyboxes.sky_boxes.get_mut(id) {
            skybox.routine.ready(&renderer.0);
            Some(&skybox.routine)
        } else {
            None
        };

        base_render_graph.add_to_graph(
            &mut graph,
            &ready,
            &pbr_routine,
            skybox,
            tone_mapping,
            UVec2::new(window.width() as u32, window.height() as u32),
            rend3::types::SampleCount::One,
            ambient.0,
        );

        // Dispatch a render using the built up rendergraph!
        graph.execute(&renderer.0, frame, cmd_bufs, &ready);
    }
}