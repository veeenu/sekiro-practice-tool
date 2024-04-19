mod aob_scans;

pub(crate) fn codegen() {
    aob_scans::get_base_addresses();
}
