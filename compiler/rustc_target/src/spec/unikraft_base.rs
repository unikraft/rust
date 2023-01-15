use crate::spec::{cvs, Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    let linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::No);
    let pre_link_args = TargetOptions::link_args(
        linker_flavor,
        &[
            // Linker script
            "-Wl,-T,unikraft_linker_script.ld",
            // Default linker script
            "-Wl,-dT,default_unikraft_linker_script.ld",
        ],
    );
    let post_link_args = TargetOptions::link_args(
        linker_flavor,
        &[
            // Require unikraft crate for binaries
            "-Wl,--require-defined=_unikraft_rs_start",
            // Entry point
            "-Wl,-e,_unikraft_rs_start",
        ],
    );

    TargetOptions {
        os: "linux".into(),
        families: cvs!["unix"],
        env: "musl".into(),
        linker_flavor,
        relocation_model: RelocModel::Static,
        tls_model: TlsModel::InitialExec,
        pre_link_args,
        post_link_args,
        has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        entry_name: "main".into(),
        ..Default::default()
    }
}
