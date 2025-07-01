use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};

fn main() {
    let uefi_path = env!("UEFI_PATH");
    let prebuilt = Prebuilt::fetch(Source::LATEST, "target/ovmf")
        .expect("Failed to update prebuilt");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("if=pflash,format=raw,unit=0,file={},readonly=on",
                                  prebuilt.get_file(Arch::X64, FileType::Code).display()));
    cmd.arg("-drive").arg(format!("if=pflash,format=raw,unit=1,file={},readonly=on",
                                  prebuilt.get_file(Arch::X64, FileType::Vars).display()));
    cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    cmd.arg("-m").arg("256M");
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-no-reboot");
    cmd.arg("-no-shutdown");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}