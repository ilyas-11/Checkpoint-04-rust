#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        if self.rh_factor==RhFactor::Negative&&other.rh_factor==RhFactor::Positive{
            return false
        }
        match self.antigen{
            Antigen::O=>other.antigen==Antigen::O,
            Antigen::A=>{other.antigen==Antigen::O||other.antigen==Antigen::A},
            Antigen::B=>{other.antigen==Antigen::O||other.antigen==Antigen::B}
            Antigen::AB=>true,
        }
    }

    pub fn donors(self) -> Vec<Self> {
        let arr = vec![
            BloodType{antigen: Antigen::A,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::A,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::B,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::B,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::O,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::O,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::AB,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::AB,rh_factor:RhFactor::Negative},
        ];
        let mut res=Vec::new();
        for b in arr{
            if self.can_receive_from(b){
                res.push(b);
            }
        }
        res
    }

    pub fn recipients(self) -> Vec<Self> {
        let arr = vec![
            BloodType{antigen: Antigen::A,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::A,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::B,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::B,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::O,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::O,rh_factor:RhFactor::Negative},
            BloodType{antigen: Antigen::AB,rh_factor:RhFactor::Positive},
            BloodType{antigen: Antigen::AB,rh_factor:RhFactor::Negative},
        ];
        let mut res=Vec::new();
        for b in arr{
            if b.can_receive_from(self){
                res.push(b);
            }
        }
        res
    }
}
