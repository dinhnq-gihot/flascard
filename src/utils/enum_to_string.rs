use crate::entities::sea_orm_active_enums::{PermissionEnum, QuestionTypeEnum, RoleEnum};

impl ToString for PermissionEnum {
    fn to_string(&self) -> String {
        match self {
            Self::View => "View".to_string(),
            Self::Comment => "Comment".to_string(),
            Self::Edit => "Edit".to_string(),
        }
    }
}

impl ToString for QuestionTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::MultipleChoice => "MultipleChoice".to_string(),
            Self::CheckBoxes => "CheckBoxes".to_string(),
            Self::TextFill => "TextFill".to_string(),
        }
    }
}

impl ToString for RoleEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Staff => "Staff".to_string(),
            Self::User => "User".to_string(),
        }
    }
}
