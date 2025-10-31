//access management using RBAC model 
use std::collections::HashSet;

// lists os all permissions 
#[derive(Debug)]
pub enum Permission {
    ViewPatient,
    CreateClinicianAccount,
    CreatePatientAccount,
    CreateCaretakerLink,
    UpdatePatient,
    ViewGlucose,
    AddGlucose
}

// struct to represent roles and their associated permissions
pub struct Role{
    pub name: String,
    pub permissions: HashSet<Permission>,
}

// impl methods for Role struct and permission checking
impl Role{
    pub fn new(name: &str, permissions: HashSet<Permission>) -> Self {
        // create new role with given name and permissions
        Self {
            name: name.to_string(),
            permissions: permissions.iter().cloned().collect(),
        }
    }
    // method to check if role has specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
}


// function to create clinician role with specific permissions
pub fn clinician_role() -> Role {
    Role::new(
        "clinician",
        &[
            Permission::ViewPatient,
            Permission::CreateClinicianAccount,
            Permission::CreatePatientAccount,
            Permission::CreateCaretakerLink,
            Permission::UpdatePatient,
            Permission::ViewGlucose,
            Permission::AddGlucose,
        ]
    )
}

// function to create patient role with specific permissions
pub fn patient_role() -> Role {
    Role::new(
        "patient",
        &[
            Permission::ViewPatient,
            Permission::ViewGlucose,
            Permission::AddGlucose,
            Permission::CreateCaretakerLink,
        ]
    )
}

// function to create caretaker role with specific permissions
pub fn caretaker_role() -> Role {
    Role::new(
        "caretaker",
        &[
            Permission::ViewPatient,
            Permission::ViewGlucose,
        ]
    )
}

pub fn admin_role() -> Role {
    Role::new(
        "admin",
        &[
            Permission::CreateClinicianAccount,
        ]
    )
}