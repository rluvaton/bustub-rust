
#[cfg(all(test, not(feature = "test_concurrency")))]
mod tests {
    use super::super::*;


    // ########################
    //     Unlimited Memory
    // ########################

    #[test]
    fn multi_threaded_memory_disk_manager() {
        let options = OptionsBuilder::default()
            .disk_manager_specific(
                DiskManagerImplementationOptions::UnlimitedMemory(
                    UnlimitedMemoryDiskManagerOptions {
                        enable_latency: false
                    }
                )
            )
            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_5s() {
        let options = OptionsBuilder::default()
            // 5s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .disk_manager_specific(
                DiskManagerImplementationOptions::UnlimitedMemory(
                    UnlimitedMemoryDiskManagerOptions {
                        enable_latency: false
                    }
                )
            )
            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_with_latency() {
        let options = OptionsBuilder::default()
            // 4s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(4000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(4000))

            .disk_manager_specific(
                DiskManagerImplementationOptions::UnlimitedMemory(
                    UnlimitedMemoryDiskManagerOptions {
                        enable_latency: true
                    }
                )
            )
            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_0_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(0)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_0_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(0)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_2_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_10_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_2_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_10_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    // ########################
    //         Default
    // ########################

    #[test]
    fn multi_threaded_default_disk_manager() {
        let options = OptionsBuilder::default()
            .disk_manager_specific(
                DiskManagerImplementationOptions::get_default()
            )
            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_5s() {
        let options = OptionsBuilder::default()

            // 5s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(5000))

            .disk_manager_specific(
                DiskManagerImplementationOptions::get_default()
            )
            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_0_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(0)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_0_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(0)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_2_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_10_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_2_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_10_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threads_tests(options)
    }
}

