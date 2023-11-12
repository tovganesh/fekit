/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */
use crate::atom::Atom;
use crate::bond::Bond;
use crate::bond::BondType;

#[allow(dead_code)]
pub struct Molecule {
    pub name: String,
    pub remark: String,

    atom_list: Vec<Atom>,
    bond_list: Vec<Bond>,
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

    pub fn add_bond(&mut self, atom_1: Atom, atom_2: Atom, bond_type: BondType) {
        self.bond_list.push(Bond {
            atom_a: atom_1,
            atom_b: atom_2,
            bond_type: bond_type,
        });
    }

    pub fn get_number_of_atoms(self) -> usize {
        return self.atom_list.len();
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use crate::atom::Atom;
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
    }
}
