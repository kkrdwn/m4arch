pub enum IpcCommand {
    SetBrightness(u8),
    SetRgb(u8, u8, u8),
    Power(bool),
}
