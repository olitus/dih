pub enum ToolType {
    Compiler,
    Runtime,
    Other,
}
pub struct Tool {
    name: String,
    cmd: String,
    typ: String,
    default_version_option: String,
}
