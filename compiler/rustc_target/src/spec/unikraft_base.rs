use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "unikraft".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        relocation_model: RelocModel::Static,
        tls_model: TlsModel::InitialExec,
        has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        entry_name: "main".into(),
        ..Default::default()
    }
}
