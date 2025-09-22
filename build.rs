

#[cfg(any(target_os="linux", target_os = "macos"))]
const CC_FLAGS: &[&'static str] = &[
    "-O3",
    "-D__UNIX__",
    "-std=c99", 
    "-Wno-implicit-function-declaration", 
    "-Wno-incompatible-pointer-types",
    "-Wno-int-conversion",
	"-Wno-enum-conversion",
	"-ouasm"
	];

#[cfg(target_os="windows")]
const CC_FLAGS: &[&'static str] = &[
    "/nologo",
    "/Ox",
    "/GS-",
    "/D__NT__",
    "/DNDEBUG",
    "/MT",
    "/Feuasm"
];

#[cfg(target_os="windows")]
const LINK_FLAGS: &[&'static str] = &[
	"/link",
	"/SUSBSYSTEM:CONSOLE",
];

#[cfg(any(target_os="linux", target_os = "macos"))]
const LINK_FLAGS: &[&'static str] = &[
];

const SOURCES: &[&'static str] = &[
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

fn main() -> Result<(),Box<dyn std::error::Error>>{

    let cwd = std::env::current_dir().unwrap();
	let uasm_dir =cwd.join("UASM");
	let inc_dir  =uasm_dir.join("H");

	#[cfg(any(target_os="linux", target_os = "macos"))]
	let inc_flag = format!("-I{}",inc_dir.display());

	#[cfg(target_os="windows")]
	let inc_flag = format!("/I{}",inc_dir.display());

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let uasm_exe = out_dir.join("uasm");

	let status  = cc::Build::new().get_compiler().to_command()
	.current_dir(out_dir).args(CC_FLAGS).args(SOURCES.iter().map(|s| uasm_dir.join(s)))
	.arg(inc_flag).args(LINK_FLAGS).status()?;

	if !status.success() {
		return Err(format!("cc compiler failed with exit code {}",status.code().unwrap()).into());
	}

	println!("cargo::rustc-env=UASM_PATH={}",uasm_exe.display());
    Ok(())
}

