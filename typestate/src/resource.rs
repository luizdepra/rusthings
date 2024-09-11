use crate::state::*;

#[derive(Debug)]
pub struct Resource<S: ResourceState> {
    id: u64,
    name: String,
    state: Box<S>,
}

impl<S: ResourceState> Resource<S> {
    fn next<N: ResourceState>(self, state: N) -> Resource<N> {
        Resource {
            id: self.id,
            name: self.name.to_owned(),
            state: Box::new(state),
        }
    }

    pub fn log(&self) {
        println!("{}: {}", self.name, self.state.log());
    }
}

impl Resource<PendingState> {
    pub fn new(id: u64, name: &str) -> Self {
        Resource {
            id: id,
            name: name.to_owned(),
            state: Box::new(PendingState),
        }
    }

    pub fn start_creation(self) -> Resource<CreatingState> {
        self.next(CreatingState)
    }
}

impl Resource<CreatingState> {
    pub fn finish_creation(self) -> Resource<AvailableState> {
        self.next(AvailableState)
    }

    pub fn creation_failed(self, code: u32, message: &str) -> Resource<CreationErrorState> {
        self.next(CreationErrorState {
            code: code,
            message: message.to_owned(),
        })
    }
}

impl Resource<AvailableState> {
    pub fn start_deletion(self) -> Resource<DeletingState> {
        self.next(DeletingState)
    }
}

impl Resource<DeletingState> {
    pub fn finish_deletion(self) -> Resource<DeletedState> {
        self.next(DeletedState)
    }

    pub fn deletion_failed(self, code: u32, message: &str) -> Resource<DeletionErrorState> {
        self.next(DeletionErrorState {
            code: code,
            message: message.to_owned(),
        })
    }
}
