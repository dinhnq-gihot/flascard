use {
    crate::entities::sea_orm_active_enums::{PermissionEnum, QuestionTypeEnum, RoleEnum},
    std::fmt,
};

impl fmt::Display for PermissionEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::View => write!(f, "View"),
            Self::Comment => write!(f, "Comment"),
            Self::Edit => write!(f, "Edit"),
        }
    }
}

impl fmt::Display for QuestionTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultipleChoice => write!(f, "MultipleChoice"),
            Self::CheckBoxes => write!(f, "CheckBoxes"),
            Self::TextFill => write!(f, "TextFill"),
        }
    }
}

impl fmt::Display for RoleEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Staff => write!(f, "Staff"),
            Self::User => write!(f, "User"),
        }
    }
}
