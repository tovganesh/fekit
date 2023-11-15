/**
 * Author: V. Ganesh
 * License: MIT
 */

/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */
use crate::atom::Atom;
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
    pub fn new(name: String, remark: String) -> Molecule {
        Molecule {
            name: name,
            remark: remark,
            atom_list: Vec::new(),
            bond_list: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atom_list.push(atom);
    }

    pub fn add_bond(&mut self, atom_1_idx: usize, atom_2_idx: usize, bond_type: BondType) {
        self.bond_list.push(BondIndex {
            atom_1_idx: atom_1_idx,
            atom_2_idx: atom_2_idx,
            bond_type: bond_type,
        });
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
            remark: removed_atom.remark.to_string(),
        };
    }

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
    use crate::atom::Atom;
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
}
