use core::arch::asm;
use core::ffi::{c_ulong, c_void};

use windows::Win32::Foundation::UNICODE_STRING;
use windows::Win32::System::Kernel::LIST_ENTRY;

type HANDLE = *const c_void;
type PVOID = *const c_void;

#[repr(C)]
pub struct SectionPointer {
	pub section_pointer: PVOID,
	pub check_sum: c_ulong,
}

#[repr(C)]
pub union HashLinksOrSectionPointer {
	pub hash_links: LIST_ENTRY,
	pub section_pointer: SectionPointer,
}

#[repr(C)]
pub union TimeDateStampOrLoadedImports {
	pub time_date_stamp: c_ulong,
	pub loaded_imports: PVOID,
}

#[repr(C)]
pub struct LoaderDataTableEntry {
	pub in_load_order_links: LIST_ENTRY,
	pub in_memory_order_links: LIST_ENTRY,
	pub in_initialization_order_links: LIST_ENTRY,
	pub dll_base: PVOID,
	pub entry_point: PVOID,
	pub size_of_image: c_ulong,
	pub full_dll_name: UNICODE_STRING,
	pub base_dll_name: UNICODE_STRING,
	pub flags: c_ulong,
	pub load_count: i16,
	pub tls_index: i16,
	pub hash_links_or_section_pointer: HashLinksOrSectionPointer,
	pub time_date_stamp_or_loaded_imports: TimeDateStampOrLoadedImports,
	pub entry_point_activation_context: PVOID,
	pub patch_information: PVOID,
	pub forwarder_links: LIST_ENTRY,
	pub service_tag_links: LIST_ENTRY,
	pub static_links: LIST_ENTRY,
}

#[repr(C)]
pub struct PebLoaderData {
	pub length: c_ulong,
	pub initialized: c_ulong,
	pub ss_handle: PVOID,
	pub in_load_order_module_list: LIST_ENTRY,
	pub in_memory_order_module_list: LIST_ENTRY,
	pub in_initialization_order_module_list: LIST_ENTRY,
}

#[repr(C)]
pub struct PEB {
	pub inherited_address_space: bool,
	pub read_image_file_exec_options: bool,
	pub being_debugged: bool,
	pub spare: bool,
	pub mutant: HANDLE,
	pub image_base: PVOID,
	pub loader_data: *const PebLoaderData,
	pub process_parameters: PVOID,
	pub sub_system_data: PVOID,
	pub process_heap: PVOID,
	pub fast_peb_lock: PVOID,
	pub fast_peb_lock_routine: PVOID,
	pub fast_peb_unlock_routine: PVOID,
	pub environment_update_count: c_ulong,
	pub kernel_callback_table: *const PVOID,
	pub event_log_section: PVOID,
	pub event_log: PVOID,
	pub free_list: PVOID,
	pub tls_expansion_counter: c_ulong,
	pub tls_bitmap: PVOID,
	pub tls_bitmap_bits: [c_ulong; 2],
	pub read_only_shared_memory_base: PVOID,
	pub read_only_shared_memory_heap: PVOID,
	pub read_only_static_server_data: *const PVOID,
	pub ansi_code_page_data: PVOID,
	pub oem_code_page_data: PVOID,
	pub unicode_case_table_data: PVOID,
	pub number_of_processors: c_ulong,
	pub nt_global_flag: c_ulong,
	pub spare_2: [u8; 4],
	pub critical_section_timeout: i64,
	pub heap_segment_reserve: c_ulong,
	pub heap_segment_commit: c_ulong,
	pub heap_de_commit_total_free_threshold: c_ulong,
	pub heap_de_commit_free_block_threshold: c_ulong,
	pub number_of_heaps: c_ulong,
	pub maximum_number_of_heaps: c_ulong,
	pub process_heaps: *const *const PVOID,
	pub gdi_shared_handle_table: PVOID,
	pub process_starter_helper: PVOID,
	pub gdi_dc_attribute_list: PVOID,
	pub loader_lock: PVOID,
	pub os_major_version: c_ulong,
	pub os_minor_version: c_ulong,
	pub os_build_number: c_ulong,
	pub os_platform_id: c_ulong,
	pub image_sub_system: c_ulong,
	pub image_sub_system_major_version: c_ulong,
	pub image_sub_system_minor_version: c_ulong,
	pub gdi_handle_buffer: [c_ulong; 22],
	pub post_process_init_routine: c_ulong,
	pub tls_expansion_bitmap: c_ulong,
	pub tls_expansion_bitmap_bits: [u8; 80],
	pub session_id: c_ulong,
}

/// Find the Process Environment Block (PEB) of the current process on x86_64
#[cfg(target_arch = "x86_64")]
pub fn find_peb() -> *mut PEB {
	let peb_ptr: *mut PEB;
	unsafe {
		asm!(
		"mov {}, gs:[0x60]",
		out(reg) peb_ptr
		);
	}
	peb_ptr
}

/// Find the Process Environment Block (PEB) of the current process on x86
#[cfg(target_arch = "x86")]
pub fn find_peb() -> *const PEB {
	let peb_ptr: *const PEB;
	unsafe {
		asm!(
		"mov {}, gs:[0x30]",
		out(reg) peb_ptr
		);
	}
	peb_ptr
}