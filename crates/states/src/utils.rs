pub enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}
