use anyhow::Result;

pub trait ResultExtend {
    fn sikp_fail(self);
}

impl<T> ResultExtend for Result<T> {
    fn sikp_fail(self) {
        if let Err(e) = self {
            println!("{}", e);
        }
    }
}
