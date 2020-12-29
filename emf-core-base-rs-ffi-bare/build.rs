use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    pub const VERSION_MAJOR: i32 = 0;
    pub const VERSION_MINOR: i32 = 1;
    pub const VERSION_PATCH: i32 = 0;
    pub const VERSION_TYPE: i8 = 0;
    pub const VERSION_RELEASE_NUMBER: i8 = 0;

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let version_type_str = match VERSION_TYPE {
        0 => "gold",
        1 => "pre-alpha",
        2 => "alpha",
        3 => "beta",
        4 => "rc",
        _ => panic!(),
    };

    let version_type_int = match VERSION_TYPE {
        0 => "ReleaseType::Gold",
        1 => "ReleaseType::PreAlpha",
        2 => "ReleaseType::Alpha",
        3 => "ReleaseType::Beta",
        4 => "ReleaseType::RC",
        _ => panic!(),
    };

    let version_build = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let version_short = format!("{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);

    let version_long = if VERSION_TYPE == 0 {
        version_short.clone()
    } else {
        format!(
            "{}-{}.{}",
            version_short, version_type_str, VERSION_RELEASE_NUMBER
        )
    };

    let version_full = format!("{}+{}", version_long, version_build);

    let mut versions_file = File::create(out_path.join("versions.rs")).unwrap();

    versions_file
        .write_all("use crate::version::ReleaseType;\n\n".as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Major version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_MAJOR: i32 = {};\n\n", VERSION_MAJOR).as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Minor version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_MINOR: i32 = {};\n\n", VERSION_MINOR).as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Patch version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_PATCH: i32 = {};\n\n", VERSION_PATCH).as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Release type of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(
            format!(
                "pub const VERSION_RELEASE_TYPE: ReleaseType = {};\n\n",
                version_type_int
            )
            .as_bytes(),
        )
        .unwrap();

    versions_file
        .write_all("/// Release number of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(
            format!(
                "pub const VERSION_RELEASE_NUMBER: i8 = {};\n\n",
                VERSION_RELEASE_NUMBER
            )
            .as_bytes(),
        )
        .unwrap();

    versions_file
        .write_all("/// Build number of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(
            format!(
                "pub const VERSION_BUILD_NUMBER: i64 = {};\n\n",
                version_build
            )
            .as_bytes(),
        )
        .unwrap();

    versions_file
        .write_all("/// Short version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_SHORT: &str = \"{}\";\n\n", version_short).as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Long version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_LONG: &str = \"{}\";\n\n", version_long).as_bytes())
        .unwrap();

    versions_file
        .write_all("/// Full version of the `emf-core-base` interface\n".as_bytes())
        .unwrap();
    versions_file
        .write_all(format!("pub const VERSION_FULL: &str = \"{}\";\n\n", version_full).as_bytes())
        .unwrap();
}
