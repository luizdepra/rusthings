mod resource;
mod state;

use crate::resource::Resource;

fn main() {
    // First Resource
    let pending = Resource::new(1, &"My Resource");
    pending.log();

    let creating = pending.start_creation();
    creating.log();

    // Won't work!
    // let deleting = pending.start_deletion();

    let available = creating.finish_creation();
    available.log();

    let deleting = available.start_deletion();
    deleting.log();

    let failed = deleting.deletion_failed(123, &"oh shit");
    failed.log();

    // Second Resource
    let resource = Resource::new(2, &"Another one")
        .start_creation()
        .finish_creation();
    resource.log();
}
