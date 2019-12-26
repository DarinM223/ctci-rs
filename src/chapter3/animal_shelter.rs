use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Animal {
    Dog { name: String, id: u32 },
    Cat { name: String, id: u32 },
}

impl Animal {
    pub fn id(&self) -> u32 {
        match *self {
            Animal::Dog { id, .. } => id,
            Animal::Cat { id, .. } => id,
        }
    }
}

pub struct AnimalShelter {
    dogs: VecDeque<Animal>,
    cats: VecDeque<Animal>,
    id: u32,
}

impl AnimalShelter {
    pub fn new() -> AnimalShelter {
        AnimalShelter {
            dogs: VecDeque::new(),
            cats: VecDeque::new(),
            id: 0,
        }
    }

    pub fn enqueue(&mut self, animal: Animal) {
        let id = self.id;
        match animal {
            Animal::Dog { name, .. } => self.dogs.push_front(Animal::Dog { name: name, id: id }),
            Animal::Cat { name, .. } => self.cats.push_front(Animal::Cat { name: name, id: id }),
        }

        self.id += 1;
    }

    pub fn dequeue_all(&mut self) -> Option<Animal> {
        let dog_id = self.dogs.iter().last().map(|d| d.id());
        let cat_id = self.cats.iter().last().map(|c| c.id());
        match (dog_id, cat_id) {
            (Some(dog), Some(cat)) => {
                if dog < cat {
                    self.dogs.pop_back()
                } else {
                    self.cats.pop_back()
                }
            }
            (Some(_), _) => self.dogs.pop_back(),
            (_, Some(_)) => self.cats.pop_back(),
            _ => None,
        }
    }

    pub fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dogs.pop_back()
    }

    pub fn dequeue_cat(&mut self) -> Option<Animal> {
        self.cats.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_shelter() {
        let mut animal_shelter = AnimalShelter::new();
        animal_shelter.enqueue(Animal::Dog {
            name: "Fido".to_string(),
            id: 0,
        });
        animal_shelter.enqueue(Animal::Cat {
            name: "Neko".to_string(),
            id: 0,
        });
        animal_shelter.enqueue(Animal::Cat {
            name: "Nyaa".to_string(),
            id: 0,
        });
        animal_shelter.enqueue(Animal::Dog {
            name: "Woof".to_string(),
            id: 0,
        });

        assert_eq!(
            animal_shelter.dequeue_all(),
            Some(Animal::Dog {
                name: "Fido".to_string(),
                id: 0,
            })
        );
        assert_eq!(
            animal_shelter.dequeue_all(),
            Some(Animal::Cat {
                name: "Neko".to_string(),
                id: 1,
            })
        );
        assert_eq!(
            animal_shelter.dequeue_all(),
            Some(Animal::Cat {
                name: "Nyaa".to_string(),
                id: 2,
            })
        );
        assert_eq!(
            animal_shelter.dequeue_all(),
            Some(Animal::Dog {
                name: "Woof".to_string(),
                id: 3,
            })
        );
    }
}
