pub trait ResourceState {
    fn log(&self) -> String;
}

#[derive(Debug)]
pub struct PendingState;

impl ResourceState for PendingState {
    fn log(&self) -> String {
        "current state is PendingState".to_string()
    }
}

#[derive(Debug)]
pub struct CreatingState;

impl ResourceState for CreatingState {
    fn log(&self) -> String {
        "current state is CreatingState".to_string()
    }
}

#[derive(Debug)]
pub struct CreationErrorState {
    pub code: u32,
    pub message: String,
}

impl ResourceState for CreationErrorState {
    fn log(&self) -> String {
        "current state is CreationErrorState".to_string()
    }
}

#[derive(Debug)]
pub struct AvailableState;

impl ResourceState for AvailableState {
    fn log(&self) -> String {
        "current state is AvailableState".to_string()
    }
}

#[derive(Debug)]
pub struct DeletingState;

impl ResourceState for DeletingState {
    fn log(&self) -> String {
        "current state is DeletingState".to_string()
    }
}

#[derive(Debug)]
pub struct DeletionErrorState {
    pub code: u32,
    pub message: String,
}

impl ResourceState for DeletionErrorState {
    fn log(&self) -> String {
        "current state is DeletionErrorState".to_string()
    }
}

#[derive(Debug)]
pub struct DeletedState;

impl ResourceState for DeletedState {
    fn log(&self) -> String {
        "current state is DeletedState".to_string()
    }
}
