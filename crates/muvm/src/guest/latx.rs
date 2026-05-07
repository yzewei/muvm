use std::fs::File;
use std::io::Write;

use anyhow::{anyhow, Context, Result};

use crate::utils::env::find_in_path;

const LATX_X86_BINFMT_MISC_RULE: &str = ":LATX-x86:M:0:\\x7fELF\\x01\\x01\\x01\\x00\\x00\\x00\\x00\\\
                                        x00\\x00\\x00\\x00\\x00\\x02\\x00\\x03\\x00:\\xff\\xff\\\
                                        xff\\xff\\xff\\xfe\\xfe\\x00\\x00\\x00\\x00\\xff\\xff\\\
                                        xff\\xff\\xff\\xfe\\xff\\xff\\xff:${LATX_I386}:POCF";
const LATX_X86_64_BINFMT_MISC_RULE: &str =
    ":LATX-x86_64:M:0:\\x7fELF\\x02\\x01\\x01\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x02\\\
     x00\\x3e\\x00:\\xff\\xff\\xff\\xff\\xff\\xfe\\xfe\\x00\\x00\\x00\\x00\\xff\\xff\\xff\\xff\\\
     xff\\xfe\\xff\\xff\\xff:${LATX_X86_64}:POCF";

pub fn setup_latx() -> Result<()> {
    let latx_i386_path =
        find_in_path("latx-i386").context("Failed to check existence of `latx-i386`")?;
    let Some(latx_i386_path) = latx_i386_path else {
        return Err(anyhow!("Failed to find `latx-i386` in PATH"));
    };
    let latx_i386_path = latx_i386_path
        .to_str()
        .context("Failed to process `latx-i386` path as it contains invalid UTF-8")?;

    let latx_x86_64_path =
        find_in_path("latx-x86_64").context("Failed to check existence of `latx-x86_64`")?;
    let Some(latx_x86_64_path) = latx_x86_64_path else {
        return Err(anyhow!("Failed to find `latx-x86_64` in PATH"));
    };
    let latx_x86_64_path = latx_x86_64_path
        .to_str()
        .context("Failed to process `latx-x86_64` path as it contains invalid UTF-8")?;

    let mut file = File::options()
        .write(true)
        .open("/proc/sys/fs/binfmt_misc/register")
        .context("Failed to open binfmt_misc/register for writing")?;

    {
        let rule = LATX_X86_BINFMT_MISC_RULE.replace("${LATX_I386}", latx_i386_path);
        file.write_all(rule.as_bytes())
            .context("Failed to register `LATX-x86` binfmt_misc rule")?;
    }
    {
        let rule = LATX_X86_64_BINFMT_MISC_RULE.replace("${LATX_X86_64}", latx_x86_64_path);
        file.write_all(rule.as_bytes())
            .context("Failed to register `LATX-x86_64` binfmt_misc rule")?;
    }

    Ok(())
}
