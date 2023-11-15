/**
 * Author: V. Ganesh 
 * License: MIT 
 */

use crate::point::Point;

/** Atom is the primary struct used to define an AtomGroup or a Molecule */
#[allow(dead_code)]
#[derive(Default, PartialEq, Debug)]
pub struct Atom {
    pub center: Point,
    pub charge: f32,
    pub symbol: String,
    pub remark: String,
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        Atom {
            center: self.center,
            charge: self.charge,
            symbol: self.symbol.to_string(),
            remark: self.remark.to_string(),
        }
    }
}

#[allow(dead_code)]
impl Atom {
    pub fn distance_from(&mut self, at: Atom) -> f32 {
        return self.center.distance_from(at.center);
    }
}

/** AtomGroup is a collection of atoms with specific name */
#[allow(dead_code)]
#[derive(Default, PartialEq, Debug)]
pub struct AtomGroup {
    pub name: String,
    pub remark: String,

    atom_list: Vec<Atom>,
}

#[allow(dead_code)]
impl AtomGroup {
    pub fn new(name: String, remark: String) -> AtomGroup {
        AtomGroup {
            name: name,
            remark: remark,
            atom_list: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atom_list.push(atom)
    }

    pub fn get_number_of_atoms(&mut self) -> usize {
        return self.atom_list.len();
    }

    pub fn get_atom(&mut self, index: usize) -> Atom {
        return Atom {
            center: self.atom_list[index].center,
            symbol: self.atom_list[index].symbol.to_string(),
            charge: self.atom_list[index].charge,
            remark: self.atom_list[index].remark.to_string(),
        };
    }

    pub fn remove_atom(&mut self, index: usize) -> Atom {
        let removed_atom = self.atom_list.remove(index);

        return Atom {
            center: removed_atom.center,
            symbol: removed_atom.symbol.to_string(),
            charge: removed_atom.charge,
            remark: removed_atom.remark.to_string()
        };
    }

    pub fn index_of(&mut self, atom: &mut Atom) -> usize {
        return self
            .atom_list
            .iter()
            .position(|at| {
                at.center.x == atom.center.x
                    && at.center.y == atom.center.y
                    && at.center.z == atom.center.y
                    && at.symbol == atom.symbol
                    && at.charge == atom.charge
            })
            .unwrap();
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn atom_init() {
        let atom = super::Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "A Hydrogen Atom".to_string(),
        };

        assert_eq!(atom.center.x, 0.0);
        assert_eq!(atom.center.y, 0.0);
        assert_eq!(atom.center.z, 0.0);
        assert_eq!(atom.symbol, "H".to_string());
        assert_eq!(atom.remark, "A Hydrogen Atom".to_string());
    }

    #[test]
    fn atom_fn() {
        let mut at1 = super::Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "A Hydrogen Atom".to_string(),
        };
        let at2 = super::Atom {
            center: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "A Hydrogen Atom".to_string(),
        };  

        let dist = at1.distance_from(at2);
        assert_eq!(dist, 1.0);        
    }


    #[test]
    fn atom_group_init() {
        let atom_group: super::AtomGroup =
            super::AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        assert_eq!(atom_group.name, "OH".to_string());
        assert_eq!(atom_group.remark, "Alcohol".to_string());
    }

    #[test]
    fn atom_group_fn() {
        let mut atom_group: super::AtomGroup =
            super::AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        atom_group.add_atom(super::Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        atom_group.add_atom(super::Atom {
            center: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });

        let mut atom_1 = atom_group.get_atom(0);
        assert_eq!(atom_1.center.x, 0.0);
        assert_eq!(atom_1.center.y, 0.0);
        assert_eq!(atom_1.center.z, 0.0);
        assert_eq!(atom_1.charge, 0.0);
        assert_eq!(atom_1.symbol, "H".to_string());
        assert_eq!(atom_1.remark, "Hydrogen Atom".to_string());

        let mut atom_2 = atom_group.get_atom(1);
        assert_eq!(atom_2.center.x, 1.0);
        assert_eq!(atom_2.center.y, 0.0);
        assert_eq!(atom_2.center.z, 0.0);
        assert_eq!(atom_2.charge, 0.0);
        assert_eq!(atom_2.symbol, "O".to_string());
        assert_eq!(atom_2.remark, "Oxygen Atom".to_string());

        assert_eq!(atom_group.index_of(&mut atom_1), 0);
        assert_eq!(atom_group.index_of(&mut atom_2), 1);

        assert_eq!(atom_group.get_number_of_atoms(), 2);

        let removed_atom = atom_group.remove_atom(0);
        assert_eq!(removed_atom.center.x, atom_1.center.x);
        assert_eq!(removed_atom.center.y, atom_1.center.y);
        assert_eq!(removed_atom.center.z, atom_1.center.z);
        assert_eq!(removed_atom.charge, atom_1.charge);
        assert_eq!(removed_atom.symbol, atom_1.symbol);

        assert_eq!(atom_group.get_number_of_atoms(), 1);
    }
}
