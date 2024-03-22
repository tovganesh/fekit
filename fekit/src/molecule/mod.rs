/**
 * Author: V. Ganesh
 * License: MIT
 */

/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */
use crate::atom::Atom;
use crate::atom::AtomOperations;
use crate::bond::Bond;
use crate::bond::BondIndex;
use crate::bond::BondType;

#[allow(dead_code)]
pub struct Molecule {
    pub name: String,
    pub remark: String,

    atom_list: Vec<Atom>,
    bond_list: Vec<BondIndex>,
}

#[allow(dead_code)]
impl Molecule {
    /** new() creates a new molecule */
    pub fn new(name: String, remark: String) -> Molecule {
        Molecule {
            name: name,
            remark: remark,
            atom_list: Vec::new(),
            bond_list: Vec::new(),
        }
    }

    /** add_bond() adds an bond to the molecule */
    pub fn add_bond(&mut self, atom_1_idx: usize, atom_2_idx: usize, bond_type: BondType) {
        self.bond_list.push(BondIndex {
            atom_1_idx: atom_1_idx,
            atom_2_idx: atom_2_idx,
            bond_type: bond_type,
        });
    }

    /** get_bond() returns the bond between two atom indices */
    pub fn get_bond(&mut self, atom_1_idx: usize, atom_2_idx: usize) -> Bond {
        // find the bond position
        let bond_idx = self
            .bond_list
            .iter()
            .position(|bnd| bnd.atom_1_idx == atom_1_idx && bnd.atom_2_idx == atom_2_idx)
            .unwrap();

        // return the bond object
        return Bond {
            atom_a: self.get_atom(atom_1_idx),
            atom_b: self.get_atom(atom_2_idx),
            bond_type: self.bond_list[bond_idx].bond_type,
        };
    }

    /** compute_simple_bonds() computes bonds based on distance between atoms */
    pub fn compute_simple_bonds(&mut self) {
        const SINGLE_BOND_DIST_THRESHOLD: f32 = 1.0;

        // iterate through all the atoms and compute bonds based on distance between atoms
        for i in 0..self.atom_list.len() {
            for j in i + 1..self.atom_list.len() {
                let at_j_center = self.atom_list[j].center;
                let dist = self.atom_list[i].center.distance_from(&at_j_center);

                if dist < SINGLE_BOND_DIST_THRESHOLD {
                    self.add_bond(i, j, BondType::SINGLE);
                    println!("Added bond between {} and {}", i, j);
                }
            }
        }
    }

    /** get_number_of_bonds() returns the number of bonds in the molecule */
    pub fn get_number_of_bonds(&mut self) -> usize {
        return self.bond_list.len();
    }

    /** get_bond_index() returns the index of the bond in the bond list */
    pub fn get_bond_index(&mut self, atom_1_idx: usize, atom_2_idx: usize) -> usize {
        return self
            .bond_list
            .iter()
            .position(|bnd| bnd.atom_1_idx == atom_1_idx && bnd.atom_2_idx == atom_2_idx)
            .unwrap();
    }

    /** get_bond_type() returns the bond type of the bond between the two atoms */
    pub fn get_bond_type(&mut self, atom_1_idx: usize, atom_2_idx: usize) -> BondType {
        let bond_idx = self.get_bond_index(atom_1_idx, atom_2_idx);
        return self.bond_list[bond_idx].bond_type;
    }

    /** set_bond_type() sets the bond type of the bond between the two atoms */
    pub fn set_bond_type(&mut self, atom_1_idx: usize, atom_2_idx: usize, bond_type: BondType) {
        let bond_idx = self.get_bond_index(atom_1_idx, atom_2_idx);
        self.bond_list[bond_idx].bond_type = bond_type;
    }

    /** remove_bond() removes the bond between the two atoms */
    pub fn remove_bond(&mut self, atom_1_idx: usize, atom_2_idx: usize) {
        let bond_idx = self.get_bond_index(atom_1_idx, atom_2_idx);
        self.bond_list.remove(bond_idx);
    }

    /** compute bond order of the bond between the two atoms */
    pub fn compute_bond_order(&mut self, atom_1_idx: usize, atom_2_idx: usize) -> f32 {
        let bond_idx = self.get_bond_index(atom_1_idx, atom_2_idx);
        let bond_type = self.bond_list[bond_idx].bond_type;

        match bond_type {
            BondType::SINGLE => {
                return 1.0;
            }
            BondType::DOUBLE => {
                return 2.0;
            }
            BondType::AROMATIC => {
                return 1.5;
            }
            BondType::TRIPLE => {
                return 3.0;
            }
            BondType::WEAK => {
                return 0.5;
            }
            BondType::COORDINATE => {
                return 1.0;
            }
        }
    }
}

#[allow(dead_code)]
impl AtomOperations for Molecule {
    fn add_atom(&mut self, atom: Atom) {
        self.atom_list.push(atom);
    }

    fn get_number_of_atoms(&mut self) -> usize {
        return self.atom_list.len();
    }

    fn get_atom(&mut self, index: usize) -> Atom {
        return Atom {
            center: self.atom_list[index].center,
            symbol: self.atom_list[index].symbol.to_string(),
            charge: self.atom_list[index].charge,
            remark: self.atom_list[index].remark.to_string(),
        };
    }

    fn remove_atom(&mut self, index: usize) -> Atom {
        let removed_atom = self.atom_list.remove(index);

        return Atom {
            center: removed_atom.center,
            symbol: removed_atom.symbol.to_string(),
            charge: removed_atom.charge,
            remark: removed_atom.remark.to_string(),
        };
    }

    fn index_of(&mut self, atom: &mut Atom) -> usize {
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
    use crate::atom::Atom;
    use crate::atom::AtomOperations;
    use crate::bond::BondType;
    use crate::point::Point;

    #[test]
    fn molecule_init() {
        let mol = super::Molecule::new("H2O".to_string(), "Water Molecule".to_string());

        assert_eq!(mol.name, "H2O".to_string());
        assert_eq!(mol.remark, "Water Molecule".to_string());
    }

    #[test]
    fn molecule_add_atm_bnd() {
        let mut mol = super::Molecule::new("H2O".to_string(), "Water Molecule".to_string());

        mol.add_atom(Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: 0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: -0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });

        assert_eq!(mol.get_number_of_atoms(), 3);

        mol.add_bond(0, 1, BondType::SINGLE);
        mol.add_bond(0, 2, BondType::SINGLE);

        let bond1 = mol.get_bond(0, 1);
        assert_eq!(bond1.bond_type, BondType::SINGLE);

        let bond2 = mol.get_bond(0, 2);
        assert_eq!(bond2.bond_type, BondType::SINGLE);
    }

    #[test]
    fn molecule_compute_simple_bonds() {
        let mut mol = super::Molecule::new("H2O".to_string(), "Water Molecule".to_string());

        mol.add_atom(Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: 0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: -0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });

        mol.compute_simple_bonds();

        let bond1 = mol.get_bond(0, 1);
        assert_eq!(bond1.bond_type, BondType::SINGLE);

        let bond2 = mol.get_bond(0, 2);
        assert_eq!(bond2.bond_type, BondType::SINGLE);
    }

    #[test]
    fn molecule_get_number_of_bonds() {
        let mut mol = super::Molecule::new("H2O".to_string(), "Water Molecule".to_string());

        mol.add_atom(Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: 0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        mol.add_atom(Atom {
            center: Point {
                x: 0.758602,
                y: 0.0,
                z: -0.504284,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });

        mol.compute_simple_bonds();

        assert_eq!(mol.get_number_of_bonds(), 2);
    }
}
