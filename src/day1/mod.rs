use std::num::ParseIntError;

#[derive(Debug, Default)]
struct Inventory {
    calories: Vec<u32>,
}

impl TryFrom<Vec<String>> for Inventory {
    type Error = ParseIntError;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let calories = value
            .iter()
            .map(|cal| cal.parse::<u32>().unwrap())
            .collect();
        Ok(Inventory { calories })
    }
}

impl Inventory {
    pub fn total(&self) -> u32 {
        self.calories.iter().sum()
    }
}

#[derive(Debug, Default)]
pub struct Expedition {
    inventories: Vec<Inventory>,
}

impl TryFrom<Vec<String>> for Expedition {
    type Error = &'static String;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let inventories = value
            .into_iter()
            .fold(Vec::new(), |mut inventory, line| {
                if line.is_empty() || inventory.is_empty() {
                    inventory.push(Vec::new())
                } else {
                    inventory.last_mut().unwrap().push(line)
                }
                inventory
            })
            .into_iter()
            .map(|x| Inventory::try_from(x).unwrap())
            .collect();
        Ok(Expedition { inventories })
    }
}

impl Expedition {
    pub fn highest_calories(&self) -> u32 {
        self.inventories.iter().fold(0, |acc, inventory| {
            let total = inventory.total();
            if total > acc {
                total
            } else {
                acc
            }
        })
    }

    pub fn highest_3_calories(&self) -> u32 {
        let top3: [u32; 3] = self.inventories.iter().fold([0; 3], |mut acc, inventory| {
            let total = inventory.total();
            if total > acc[0] {
                acc.rotate_right(1);
                acc[0] = total;
            } else if total > acc[1] {
                acc[2] = acc[1];
                acc[1] = total;
            } else if total > acc[2] {
                acc[2] = total;
            }
            acc
        });
        top3.iter().sum()
    }
}
