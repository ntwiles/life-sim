use super::neural_network::output_neuron::OutputNeuron;

pub struct Body {
    x: u32,
    y: u32,
    is_alive: bool,
    color: [u8; 4],
}

impl Body {
    pub fn new(x: u32, y: u32, color: [u8; 4]) -> Self {
        Self {
            x,
            y,
            is_alive: true,
            color,
        }
    }

    pub fn update(&mut self, decisions: Vec<OutputNeuron>, grid_size: (u32, u32)) {
        // TODO: Disallow multiple entities from occupying the same cell.

        for decision in decisions {
            match decision {
                OutputNeuron::Stay => {}
                OutputNeuron::MoveLeft => {
                    if self.x > 0 {
                        self.x -= 1;
                    }
                }
                OutputNeuron::MoveRight => {
                    if self.x < grid_size.0 - 1 {
                        self.x += 1;
                    }
                }
                OutputNeuron::MoveUp => {
                    if self.y > 0 {
                        self.y -= 1;
                    }
                }
                OutputNeuron::MoveDown => {
                    if self.y < grid_size.1 - 1 {
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
                            if self.x < grid_size.0 - 1 {
                                self.x += 1;
                            }
                        }
                        2 => {
                            if self.y > 0 {
                                self.y -= 1;
                            }
                        }
                        3 => {
                            if self.y < grid_size.1 - 1 {
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

    pub fn color(&self) -> [u8; 4] {
        self.color
    }
}
