use std::io::{Result, Write};
use std::fs::{File, read_dir};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";
static C_TARGET_PATH: &str = "../user/ctarget/";
fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    let c_elfs: Vec<_>=read_dir("../user/ctarget")
    .unwrap()
    .into_iter()
    .map(|dir_entry| {
        let name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
        name_with_ext
    })
    .collect();

    writeln!(f, r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#, apps.len()+c_elfs.len())?;

    for i in 0..apps.len()+c_elfs.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() +c_elfs.len()- 1)?;

    writeln!(f, r#"
    .global _app_names
_app_names:"#)?;
    for app in apps.iter() {
        writeln!(f, r#"    .string "{}""#, app)?;
    }
    for c_elf in c_elfs.iter(){
        writeln!(f, r#"    .string "{}""#, c_elf)?;
    }

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(f, r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
    .align 3
app_{0}_start:
    .incbin "{2}{1}"
app_{0}_end:"#, idx, app, TARGET_PATH)?;
    }
    for (idx, c_elf) in c_elfs.iter().enumerate() {
        println!("app_{}: {}", idx+apps.len(), c_elf);
        writeln!(f, r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
    .align 3
app_{0}_start:
    .incbin "{2}{1}"
app_{0}_end:"#, idx+apps.len(), c_elf, C_TARGET_PATH)?;
    }
    Ok(())
}