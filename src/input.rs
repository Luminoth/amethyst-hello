use std::fmt;

use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    LeftPaddle,
    RightPaddle,
}

impl fmt::Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {}

impl fmt::Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InputBindingTypes;

impl BindingTypes for InputBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
