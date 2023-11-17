pub enum AtmosphericModel {
    /// Constant air density (kg/m^3)
    Constant(f32),
    // TODO: draw the rest of the atmospheric model
}

impl Default for AtmosphericModel {
    fn default() -> Self {
        Self::Constant(1.2) // TODO
    }
}

pub struct Environment {
    pub atmospheric_model: AtmosphericModel,
    pub launch_location: (f32, f32),
    pub launch_altitude: f32,
    pub launch_rail_length: f32,
    pub azimuth: f32,
    pub elevation: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            atmospheric_model: AtmosphericModel::default(),
            launch_location: (0.0, 0.0),
            launch_altitude: 2.0,
            launch_rail_length: 6.0,
            azimuth: 0.0,
            elevation: 0.0,
        }
    }
}
