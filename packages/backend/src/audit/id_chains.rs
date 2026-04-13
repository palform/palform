use palform_tsid::{
    resources::{IDForm, IDTeam},
    tsid::PalformDatabaseID,
};

pub trait AuditLogIDChain {
    fn to_vec(self) -> Vec<String>;
}

pub struct IDChainEmpty {}
impl AuditLogIDChain for IDChainEmpty {
    fn to_vec(self) -> Vec<String> {
        Vec::default()
    }
}

pub struct IDChainBranding {
    team_id: PalformDatabaseID<IDTeam>,
}
impl IDChainBranding {
    pub fn new(team_id: PalformDatabaseID<IDTeam>) -> Self {
        Self { team_id }
    }
}
impl AuditLogIDChain for IDChainBranding {
    fn to_vec(self) -> Vec<String> {
        vec![self.team_id.to_string()]
    }
}

pub struct IDChainTeamMember {
    team_id: PalformDatabaseID<IDTeam>,
}
impl IDChainTeamMember {
    pub fn new(team_id: PalformDatabaseID<IDTeam>) -> Self {
        Self { team_id }
    }
}
impl AuditLogIDChain for IDChainTeamMember {
    fn to_vec(self) -> Vec<String> {
        vec![self.team_id.to_string()]
    }
}

pub struct IDChainSubmission {
    form_id: PalformDatabaseID<IDForm>,
}
impl IDChainSubmission {
    pub fn new(form_id: PalformDatabaseID<IDForm>) -> Self {
        Self { form_id }
    }
}
impl AuditLogIDChain for IDChainSubmission {
    fn to_vec(self) -> Vec<String> {
        vec![self.form_id.to_string()]
    }
}
