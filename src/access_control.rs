//access management using RBAC model 
use std::collections::HashSet;

// lists os all permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    // Admin Permissions only
    CreateClinicianAccount,
    ViewClinicianAccount,
    RemoveClinicianAccount,
    // Clinician Permissions only
    CreatePatientAccount,
    EditPatientData,
    ViewPatient,
    SetDosageLimits,
    AdjustInsulinParameters,
    // Patient Permissions only
    CreateCaretakerLink,
    // Patient and Caretaker Permissions
    ViewGlucose,
    ViewInsulinRates,
    RequestBolusDose,
    EditBasalDose,
    ReviewHistoricalData,
    ViewAlerts,
}

// struct to represent roles and their associated permissions
pub struct Role{
    pub name: String,
    pub id: String,
    pub permissions: HashSet<Permission>,
}

// impl methods for Role struct and permission checking
impl Role{
    pub fn new(name: &str, id:&str) -> Self {
        // get default permissions using role
        let permissions = Self::default_permissions(name);
        // create new role with given name and permissions
        Self {
            name: name.to_string(),
            id:id.to_string(),
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
                perms.insert(Permission::ViewClinicianAccount);
                perms.insert(Permission::RemoveClinicianAccount);
            }
            "clinician" => {
                perms.insert(Permission::CreatePatientAccount);
                perms.insert(Permission::EditPatientData);
                perms.insert(Permission::ViewPatient);
                perms.insert(Permission::SetDosageLimits);
                perms.insert(Permission::AdjustInsulinParameters);
            }
            "patient" => {
                perms.insert(Permission::CreateCaretakerLink);
                perms.insert(Permission::ViewGlucose);
                perms.insert(Permission::ViewInsulinRates);
                perms.insert(Permission::RequestBolusDose);
                perms.insert(Permission::EditBasalDose);
                perms.insert(Permission::ReviewHistoricalData);
                perms.insert(Permission::ViewAlerts);
            }
            "caretaker" => {
                perms.insert(Permission::ViewGlucose);
                perms.insert(Permission::ViewInsulinRates);
                perms.insert(Permission::RequestBolusDose);
                perms.insert(Permission::EditBasalDose);
                perms.insert(Permission::ReviewHistoricalData);
                perms.insert(Permission::ViewAlerts);
            }
            _ => {
                eprintln!("Warning: Unknown role '{}', no permissions assigned.", role_name);
            }
        }
        perms
    }
}
