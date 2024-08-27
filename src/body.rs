use crate::grid_config::GridConfig;

use super::neural_network::output_neuron::OutputNeuron;

pub struct Body {
    pub x: u32,
    pub y: u32,
    pub is_alive: bool,
    pub color_gradient_index: f64,
}

impl Body {
    pub fn new(x: u32, y: u32, color_gradient_index: f64) -> Self {
        Self {
            x,
            y,
            is_alive: true,
            color_gradient_index,
        }
    }

    pub fn update(&mut self, decision: OutputNeuron, grid_config: &GridConfig) {
        match decision {
            OutputNeuron::Stay => {}
            OutputNeuron::MoveLeft => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            OutputNeuron::MoveRight => {
                if self.x < grid_config.width - 1 {
                    self.x += 1;
                }
            }
            OutputNeuron::MoveUp => {
                if self.y > 0 {
                    self.y -= 1;
                }
            }
            OutputNeuron::MoveDown => {
                if self.y < grid_config.height - 1 {
                    self.y += 1;
                }
            }
            OutputNeuron::MoveRandom => {
                let direction = rand::random::<u8>() % 4;

                match direction {
                    0 => {
                        if self.x > 0 {
                            self.x -= 1;
                        }
                    }
                    1 => {
                        if self.x < grid_config.width - 1 {
                            self.x += 1;
                        }
                    }
                    2 => {
                        if self.y > 0 {
                            self.y -= 1;
                        }
                    }
                    3 => {
                        if self.y < grid_config.height - 1 {
                            self.y += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn mutate_color(&mut self, mutation_amount: f64) {
        self.color_gradient_index += mutation_amount;
    }
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            is_alive: self.is_alive,
            color_gradient_index: self.color_gradient_index,
        }
    }
}
