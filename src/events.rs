pub struct SpeedControlEvent {
    pub combine_id: i32,
    pub action: SpeedControlAction,
}

#[derive(Clone, PartialEq)]
pub enum SpeedControlAction {
    Forward,
    Back,
    NoPower,
    Brake,
}

pub struct SteerControlEvent {
    pub combine_id: i32,
    pub action: SteerControlAction,
}

#[derive(Clone)]
pub enum SteerControlAction {
    Left,
    NoSteer,
    Right,
}

#[derive(Clone)]
pub enum SoundSampleEvent {
    HayBale,
    Cow,
    EnginePower,
    NoEnginePower,
}
