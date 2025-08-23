use crate::builders::fields::StaffField;

#[derive(Copy, Clone)]
pub enum StaffRoleTypeField<'a> {
    VoiceActor(&'a [StaffField<'a>]),
    RoleNotes,
    DubGroup,
}

impl StaffRoleTypeField<'_> {
    pub const fn all() -> &'static [StaffRoleTypeField<'static>] {
        const ALL: &[StaffRoleTypeField<'static>] = &[
            StaffRoleTypeField::VoiceActor(StaffField::all()),
            StaffRoleTypeField::RoleNotes,
            StaffRoleTypeField::DubGroup,
        ];

        ALL
    }
}

impl From<StaffRoleTypeField<'_>> for String {
    fn from(value: StaffRoleTypeField) -> Self {
        match value {
            StaffRoleTypeField::VoiceActor(fields) => format!(
                "voiceActor {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            StaffRoleTypeField::RoleNotes => "roleNotes".to_owned(),
            StaffRoleTypeField::DubGroup => "dubGroup".to_owned(),
        }
    }
}
