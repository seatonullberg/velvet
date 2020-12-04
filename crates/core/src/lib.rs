pub mod potential;
pub mod system;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

use potential::pair::PairArgs;
use potential::Potential;

// TODO: add other register functions
pub trait PluginRegistrar {
    fn register_pair_potential(
        &mut self,
        name: &str,
        potential: Box<dyn Potential<Args = PairArgs>>,
    );
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PLUGIN_DECLARATION: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}
