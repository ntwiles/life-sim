use super::network::NeuralNetwork;
use super::output_neuron::OutputNeuronKind;

pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub brain: NeuralNetwork,
}

impl Entity {
    pub fn new(x: u32, y: u32, brain: NeuralNetwork) -> Self {
        Self { x, y, brain }
    }

    pub fn update(&mut self, grid_width: u32, grid_height: u32, generation_time: f32) {
        // TODO: Disallow multiple entities from occupying the same cell.
        for decision in self.brain.decide(generation_time) {
            match decision {
                OutputNeuronKind::Stay => {}
                OutputNeuronKind::MoveLeft => {
                    if self.x > 0 {
                        self.x -= 1;
                    }
                }
                OutputNeuronKind::MoveRight => {
                    if self.x < grid_width - 1 {
                        self.x += 1;
                    }
                }
                OutputNeuronKind::MoveUp => {
                    if self.y > 0 {
                        self.y -= 1;
                    }
                }
                OutputNeuronKind::MoveDown => {
                    if self.y < grid_height - 1 {
                        self.y += 1;
                    }
                }
                OutputNeuronKind::MoveRandom => {
                    let direction = rand::random::<u8>() % 4;

                    match direction {
                        0 => {
                            if self.x > 0 {
                                self.x -= 1;
                            }
                        }
                        1 => {
                            if self.x < grid_width - 1 {
                                self.x += 1;
                            }
                        }
                        2 => {
                            if self.y > 0 {
                                self.y -= 1;
                            }
                        }
                        3 => {
                            if self.y < grid_height - 1 {
                                self.y += 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
