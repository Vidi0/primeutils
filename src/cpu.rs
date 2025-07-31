// Get the L1 cache size of the processor in bytes
pub fn get_cache_size() -> usize {

  #[cfg(feature = "get_cpu_info")]
  {
    let cpuid: raw_cpuid::CpuId<raw_cpuid::CpuIdReaderNative> = raw_cpuid::CpuId::new();
  
    if let Some(cparams) = cpuid.get_cache_parameters() {
      for cache in cparams {
        if cache.cache_type() == raw_cpuid::CacheType::Data {
          if cache.level() == 1 {
            return cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
          }
        }
      }
    }
  }
  
  32768
}

// Get the number of cores in the processor
pub fn get_cores() -> usize {
  #[cfg(feature = "get_cpu_info")]
  {
    let cores: usize = num_cpus::get();
    if cores > 0 {
      return cores;
    }
  }

  1
}
