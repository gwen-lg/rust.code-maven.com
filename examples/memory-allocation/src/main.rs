
use sysinfo::System;
use thousands::Separable;

fn main() {
    let filler = "0123456789".repeat(10);
    let size = 200_000_000;

    println!(
        "Size: {} filler: {} total: {}\n",
        size.separate_with_commas(),
        filler.len(),
        (size * filler.len()).separate_with_commas()
    );

    println!("Total memory:                        {:>15}",  total_memory().separate_with_commas());
    let used_before = used_memory();
    println!("Used memory before allocation:       {:>15}", used_before.separate_with_commas());

    let used_allocated = allocate(size, &filler);
    println!("Used memory after allocation:        {:>15}", used_allocated.separate_with_commas());

    let used_after = used_memory();
    println!("Used memory after deallocation:      {:>15}", used_after.separate_with_commas());

    println!("Memory used by allocation (diff):    {:>15}", (used_allocated - used_before).separate_with_commas()
    );
    println!("Memory freed by deallocation (diff): {:>15}", (used_allocated - used_after).separate_with_commas()
    );

}

fn allocate(size: usize, filler: &str) -> u64 {

    let mut text = String::with_capacity(size * filler.len());
    for _ in 0..size {
        text.push_str(filler);
    }

    used_memory()
}


fn total_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.total_memory()
}

fn used_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.used_memory()
}



