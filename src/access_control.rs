//access management using RBAC model 
use std::collections::HashSet;

// lists os all permissions 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    ViewPatient,
    CreateClinicianAccount,
    RemoveClinicianAccount,
    CreatePatientAccount,
    CreateCaretakerLink,
    EditPatientData,
    ViewGlucose,
    AddGlucose,
    ViewAlerts,
}

// struct to represent roles and their associated permissions
pub struct Role{
    pub name: String,
    pub permissions: HashSet<Permission>,
}

// impl methods for Role struct and permission checking
impl Role{
    pub fn new(name: &str) -> Self {
        // get default permissions using role
        let permissions = Self::default_permissions(name);
        // create new role with given name and permissions
        Self {
            name: name.to_string(),
            permissions,
        }
    }

    // method to check if role has specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    fn default_permissions(role_name: &str) -> HashSet<Permission> {
        let mut perms = HashSet::new();
        match role_name{
            "admin" => {
                perms.insert(Permission::CreateClinicianAccount);
                perms.insert(Permission::RemoveClinicianAccount);
            }
            "clinician" => {
                perms.insert(Permission::CreatePatientAccount);
                perms.insert(Permission::EditPatientData);
                perms.insert(Permission::ViewGlucose);
                perms.insert(Permission::ViewAlerts);
                perms.insert(Permission::ViewPatient);
            }
            "patient" => {
                perms.insert(Permission::ViewPatient);
                perms.insert(Permission::ViewGlucose);
                perms.insert(Permission::AddGlucose);
                perms.insert(Permission::CreateCaretakerLink);
            }
            "caretaker" => {
                perms.insert(Permission::ViewPatient);
                perms.insert(Permission::ViewGlucose);

            }
            _ => {
                eprintln!("Warning: Unknown role '{}', no permissions assigned.", role_name);
            }
        }
        perms
    }
}
