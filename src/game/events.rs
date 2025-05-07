use bevy::prelude::*;

use super::metro::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ActiveLinesChanged>()
        .add_event::<LinePathChanged>()
        .add_event::<LineDragHoversStation>();
}

#[derive(Event)]
pub struct ActiveLinesChanged;

#[derive(Event)]
pub struct LineDragHoversStation {
    pub station_id: StationId,
}

#[derive(Event)]
pub struct LinePathChanged {
    pub line_id: LineId,
    pub new_path: Vec<StationId>,
}
