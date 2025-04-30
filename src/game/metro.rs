use std::f32::INFINITY;

use rand::{Rng, rng};

use bevy::{
    color::Srgba,
    math::Vec2,
    platform::collections::HashSet,
    prelude::{Deref, Resource},
};

pub const MAP_SIZE: Vec2 = Vec2::new(200., 200.);
pub const LINE_COLORS: [Srgba; 10] = [
    Srgba::new(0.4, 0.8, 0.9, 1.0), // soft cyan
    Srgba::new(0.9, 0.6, 0.4, 1.0), // warm peach
    Srgba::new(0.5, 0.4, 0.8, 1.0), // lavender
    Srgba::new(0.7, 0.9, 0.4, 1.0), // limey green
    Srgba::new(0.9, 0.4, 0.7, 1.0), // rose pink
    Srgba::new(0.4, 0.9, 0.6, 1.0), // mint green
    Srgba::new(0.8, 0.5, 0.4, 1.0), // muted coral
    Srgba::new(0.4, 0.6, 0.9, 1.0), // soft blue
    Srgba::new(0.9, 0.8, 0.4, 1.0), // mellow yellow
    Srgba::new(0.6, 0.4, 0.9, 1.0), // soft violet
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum StationKind {
    Square,
    Triangle,
    Circle,
}

#[derive(Clone, Copy, Deref)]
pub struct Passenger {
    pub target: StationKind,
}

pub struct Station {
    pub passengers: Vec<Passenger>,
    pub kind: StationKind,

    /// How often new people spawn at the station. From 0.0 to 1.0.
    pub intensity: f32,

    pub position: Vec2,
}

impl Station {
    pub fn new(kind: StationKind, position: Vec2) -> Self {
        Station {
            kind,
            passengers: Vec::new(),
            intensity: 0.5,
            position,
        }
    }
}

pub type StationId = usize;
pub type LineId = usize;

pub struct Train {
    pub passengers: Vec<Passenger>,
    pub locomotive_count: usize,
    pub last_station: StationId,
    pub next_station: StationId,
    pub traveled_distance: f32,
    pub stopped: bool,
}

impl Train {
    pub fn new(last_station: StationId, next_station: StationId) -> Self {
        return Train {
            passengers: Vec::new(),
            locomotive_count: 1,
            last_station,
            next_station,
            traveled_distance: 0.0,
            stopped: true,
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Connection {
    line_id: LineId,
    target: StationId,
}

// TODO: maybe split this into multiple resources, so that bevy can parellelize access to them?
#[derive(Resource)]
pub struct Metro {
    pub stations: Vec<Station>,
    /// [Starting station id] = The ids of all the stations it has a direct connection to + id of the line
    /// which connects to it.
    pub connections: Vec<Vec<Connection>>,
    pub trains: Vec<Train>,

    pub distances: Vec<Vec<f32>>,
}

impl Metro {
    pub fn new() -> Self {
        Metro {
            stations: vec![
                Station::new(StationKind::Square, Vec2::new(-30., -20.)),
                Station::new(StationKind::Triangle, Vec2::new(20., -20.)),
                Station::new(StationKind::Circle, Vec2::new(-20., 40.)),
            ],
            connections: vec![Vec::new(); 3],
            trains: Vec::new(),
            distances: Vec::new(),
        }
    }

    pub fn get_active_lines(&self) -> HashSet<LineId> {
        let mut lines = HashSet::<LineId>::new();

        for station in &self.connections {
            for line in station {
                lines.insert(line.line_id);
            }
        }

        // lines.insert(0);
        lines
    }

    pub fn add_connection(&mut self, a: StationId, b: StationId, line_id: LineId) -> bool {
        let connection_a = Connection { line_id, target: b };
        let connection_b = Connection { line_id, target: a };

        if self.connections[a].contains(&connection_a)
            || self.connections[b].contains(&connection_b)
        {
            return false;
        }

        self.connections[a].push(connection_a);
        self.connections[b].push(connection_b);

        // TODO: not sure if this should be here?
        self.calculate_distances();

        return true;
    }

    pub fn spawn_random_station(&mut self) {
        let mut rng = rng();

        let position = Vec2::new(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0),
        );

        let kind = match rng.random() {
            0.0..0.2 => StationKind::Square,
            ..0.5 => StationKind::Triangle,
            _ => StationKind::Circle,
        };

        let intensity = rng.random_range(0.1..=0.2);
        let mut station = Station::new(kind, position);
        station.intensity = intensity;

        self.stations.push(station);
    }

    pub fn spawn_random_passengers(&mut self) {
        let mut rng = rng();

        for station in &mut self.stations {
            if rng.random::<f32>() < station.intensity {
                station.passengers.push(Passenger {
                    target: match rng.random_range(0..3) {
                        0 => StationKind::Circle,
                        1 => StationKind::Triangle,
                        _ => StationKind::Square,
                    },
                });
            }
        }
    }

    fn calculate_distances(&mut self) {
        // Floyd-Warshal algorithm
        let station_count = self.stations.len();
        self.distances = vec![vec![INFINITY; station_count]; station_count];

        for (i, connections) in self.connections.iter().enumerate() {
            for connection in connections.iter() {
                let from_station = &self.stations[i];
                let to_station = &self.stations[connection.target];

                self.distances[i][connection.target] =
                    from_station.position.distance(to_station.position);
            }
        }

        for k in 0..station_count {
            for i in 0..station_count {
                for j in 0..station_count {
                    if self.distances[k][i] != INFINITY && self.distances[k][j] != INFINITY {
                        self.distances[i][j] = f32::min(
                            self.distances[i][j],
                            self.distances[k][i] + self.distances[k][j],
                        );
                    }
                }
            }
        }
    }

    pub fn move_trains(&mut self, delta_distance: f32) {
        for train in &mut self.trains {
            if train.stopped {
                continue;
            }

            train.traveled_distance += delta_distance;

            let last_station = &self.stations[train.last_station];
            let next_station = &self.stations[train.next_station];
            let total_distance = last_station.position.distance(next_station.position);

            if train.traveled_distance >= total_distance {
                train.traveled_distance = total_distance;
                train.stopped = true;
            }
        }
    }

    // Onboards *one* passenger on each stopped train. The idea is to run this multiple times with
    // a delay (For gameplay purposes) until all the necessary passengers have settled.
    // If there are no passengers left to onboard, updates the train to indicate that it should
    // start moving.
    pub fn onboard_passengers(&mut self) {
        for train in &mut self.trains {
            if train.stopped {
                let curr_station_id = train.next_station;

                let next_station_candidates = self.connections[train.next_station]
                    .iter()
                    .filter(|connection| connection.line_id == curr_station_id)
                    .collect::<Vec<&Connection>>();

                let next_station = match next_station_candidates[..] {
                    // --(a)----(curr)-|
                    // The train arrived at the end of the line and should turn around.
                    [a] => a,
                    // --(a)----(curr)----(b)--
                    // There's another station ahead of the train.
                    [a, b] => {
                        if a.target == train.last_station {
                            b
                        } else {
                            a
                        }
                    }
                    _ => panic!("A single line shouldn't branch into multiple directinos."),
                };

                let next_passenger_to_board = self.stations[curr_station_id]
                    .passengers
                    .iter()
                    .position(|passenger| {
                        (0..self.distances[curr_station_id].len())
                            .find(|final_station_id_candidate| {
                                self.stations[*final_station_id_candidate].kind == passenger.target
                            })
                            .map(|final_station_id| final_station_id as StationId)
                            .filter(|final_station_id| {
                                self.distances[curr_station_id][*final_station_id]
                                    > self.distances[next_station.target][*final_station_id]
                            })
                            .is_some()
                    });

                if let Some(i) = next_passenger_to_board {
                    let passenger = self.stations[curr_station_id].passengers.remove(i);

                    train.passengers.push(passenger);
                } else {
                    train.stopped = false;
                    train.next_station = next_station.target;
                    train.last_station = curr_station_id;
                }

                return;
            }
        }
    }
}

#[derive(Resource)]
pub struct MetroResources {
    pub total_trains: usize,
    pub lines: usize,
    pub max_lines: usize,
}

impl MetroResources {
    pub fn new() -> Self {
        Self {
            total_trains: 3,
            lines: 3,
            max_lines: 9,
        }
    }
}
