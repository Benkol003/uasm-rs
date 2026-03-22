use std::path::{Path, PathBuf};

const SOURCES: &[&str] = &[
	"apiemu.c",
	"assemble.c",
	"assume.c",
	"atofloat.c",
	"backptch.c",
	"bin.c",
	"branch.c",
	"cmdline.c",
	"codegen.c",
	"codegenv2.c",
	"coff.c",
	"condasm.c",
	"context.c",
	"cpumodel.c",
	"data.c",
	"dbgcv.c",
	"directiv.c",
	"elf.c",
	"end.c",
	"equate.c",
	"errmsg.c",
	"expans.c",
	"expreval.c",
	"extern.c",
	"fastpass.c",
	"fixup.c",
	"fpfixup.c",
	"hll.c",
	"input.c",
	"invoke.c",
	"label.c",
	"linnum.c",
	"listing.c",
	"loop.c",
	"ltype.c",
	"lqueue.c",
	"macho64.c",
	"macro.c",
	"macrolib.c",
	"main.c",
	"mangle.c",
	"memalloc.c",
	"msgtext.c",
	"omf.c",
	"omffixup.c",
	"omfint.c",
	"option.c",
	"orgfixup.c",
	"parser.c",
	"posndir.c",
	"preproc.c",
	"proc.c",
	"pseudoFilter.c",
	"queue.c",
	"reswords.c",
	"safeseh.c",
	"segment.c",
	"simd.c",
	"simsegm.c",
	"string.c",
	"symbols.c",
	"tbyte.c",
	"tokenize.c",
    "types.c"
];

#[cfg(any(target_os="linux", target_os = "macos"))]
fn build<P>(inc_dir: PathBuf, sources: P, out_dir: PathBuf) -> PathBuf
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
{
	let objs = cc::Build::new()
	.include(inc_dir)
	.files(sources)
	.opt_level(3)
	.std("c99")
	.define("__UNIX__", None)
    .flag("-Wno-implicit-function-declaration")
    .flag("-Wno-incompatible-pointer-types")
    .flag("-Wno-int-conversion")
	.flag("-w")
	.compile_intermediates();

	let status = cc::Build::new().get_compiler().to_command()
	.arg("-o").arg(out_dir.join("uasm"))
	.args(&objs)
	.status().unwrap();

	if !status.success() {
		panic!("cc: exit with code {}",status);
	}

	out_dir.join("uasm")
}

#[cfg(target_os="windows")]
fn cl_arch_arg() -> &'static str {
		match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
		Ok("x86_64") => "X64",
		Ok("x86")    => "X86",
		Ok("aarch64")=> "ARM64",
		Ok("arm")    => "ARM",
		Ok(a)  => panic!("unsupported target arch: {}",a),
		Err(e) => panic!("{}",e)
	}
}

#[cfg(target_os="windows")]
fn build<P>(inc_dir: PathBuf, sources: P, out_dir: PathBuf) -> PathBuf
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
{
	let objs = cc::Build::new()
	.include(inc_dir)
	.files(sources)
	.opt_level(3)
	.flag("/Ox")
	.flag("/GS-")
	.flag("/D__NT__")
	.flag("/DNDEBUG")
	.static_crt(true)
	.compile_intermediates();

	let status = cc::Build::new().get_compiler().to_command()
		.current_dir(&out_dir)
		.args(&objs)
		.arg("/Feuasm.exe")
		.arg("/MT")
		.arg("/link")
		.arg(format!("/MACHINE:{}",cl_arch_arg()))
		.arg("/SUBSYSTEM:CONSOLE")
		.status().unwrap();
	if !status.success() {
		panic!("cc: exit with code {}",status);
	}

	out_dir.join("uasm.exe")
}


fn main() -> Result<(),Box<dyn std::error::Error>>{

    let cwd = std::env::current_dir().unwrap();
	let uasm_dir =cwd.join("UASM");
	let inc_dir  =uasm_dir.join("H");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	let sources = SOURCES.iter().map(|s| uasm_dir.join(s));
	let uasm = build(inc_dir,sources,out_dir);

	if !uasm.exists() {
		panic!("expected uasm to be built but did not find {}",uasm.display());
	}

	println!("cargo::metadata=PATH={}",uasm.display());
	println!("cargo::rustc-env=UASM_PATH={}",uasm.display());
    Ok(())
}

