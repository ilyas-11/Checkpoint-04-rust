#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name : String,
    pub age : u32,
    pub role : WorkerRole,

}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin, 
    User ,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let res: Vec<_>=s.split(",").collect();
        Self{
            name:  res[0].to_string(),
            age:  res[1].parse::<u32>().unwrap(),
            role:  WorkerRole::from(res[2]),

        }
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {
        match s{
            "admin"=>WorkerRole::Admin,
            "user"=>WorkerRole::User,
            "guest"=>WorkerRole::Guest,
            &_ => todo!(),

        }
    }
}
