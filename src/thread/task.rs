

pub trait Task {
    fn do_work(&mut self);
    fn finish_work(&mut self);
}