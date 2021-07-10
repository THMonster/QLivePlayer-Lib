use crate::interface::*;

pub struct QLivePlayerLib {
    emit: QLivePlayerLibEmitter,
}

impl QLivePlayerLibTrait for QLivePlayerLib {
    fn new(emit: QLivePlayerLibEmitter) -> QLivePlayerLib {
        QLivePlayerLib { emit }
    }

    fn emit(&mut self) -> &mut QLivePlayerLibEmitter {
        &mut self.emit
    }

    fn run_danmaku_client(&mut self, unix_socket: String) -> () {
        todo!()
    }

    fn get_url(&mut self, room_url: String, extras: String) -> String {
        crate::get_url(&room_url, &extras)
    }
}
