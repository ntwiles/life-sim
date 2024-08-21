use super::neural_network::brain::Brain;
use super::neural_network::output_neuron::OutputNeuronKind;

pub struct Entity {
    x: u32,
    y: u32,
    brain: Brain,
    is_alive: bool,
}

impl Entity {
    pub fn new(x: u32, y: u32, brain: Brain) -> Self {
        Self {
            x,
            y,
            brain,
            is_alive: true,
        }
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

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn brain(&self) -> &Brain {
        &self.brain
    }
}
