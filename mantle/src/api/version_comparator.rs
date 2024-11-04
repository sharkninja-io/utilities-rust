use std::cmp::Ordering;

pub trait VersionComparator {
    //const CURRENT_VERSION: &'static str;

    fn is_equal_to(target_version: &str) -> bool {
        Self::compare_operation(target_version) == std::cmp::Ordering::Equal
    }

    fn is_greater_than(target_version: &str) -> bool {
        Self::compare_operation(target_version) == std::cmp::Ordering::Greater
    }

    fn is_greater_than_or_equal_to(target_version: &str) -> bool {
        Self::compare_operation(target_version) != std::cmp::Ordering::Less
    }

    fn is_less_than(target_version: &str) -> bool {
        Self::compare_operation(target_version) == std::cmp::Ordering::Less
    }

    fn is_less_than_or_equal_to(target_version: &str) -> bool {
        Self::compare_operation(target_version) != std::cmp::Ordering::Greater
    }

    fn is_minimum_required(target_version: &str) -> bool {
        Self::is_greater_than_or_equal_to(target_version)
    }

    fn is_maximum_required(target_version: &str) -> bool {
        Self::is_less_than_or_equal_to(target_version)
    }

    fn compare_operation(target_version: &str) -> std::cmp::Ordering;
}

pub struct VoicePackageVersion;

impl VersionComparator for VoicePackageVersion {
    //const CURRENT_VERSION: &'static str = "";
    
    fn compare_operation(target_version: &str) -> Ordering {
        match VoicePackageVersion::compare_operation(target_version) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

pub struct AppOSVersion;

impl VersionComparator for AppOSVersion {
    //const CURRENT_VERSION: &'static str = AppConstants::OS_VERSION;
    
    fn compare_operation(target_version: &str) -> Ordering {
        match AppOSVersion::compare_operation(target_version) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

pub struct AppMarketVersion;

impl VersionComparator for AppMarketVersion {
    //const CURRENT_VERSION: &'static str = AppConstants::APP_VERSION;
    
    fn compare_operation(target_version: &str) -> Ordering {
        match AppMarketVersion::compare_operation(target_version) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

pub struct AppBuildNumberVersion;

impl VersionComparator for AppBuildNumberVersion {
    //const CURRENT_VERSION: &'static str = AppConstants::BUILD_NUMBER;
    
    fn compare_operation(target_version: &str) -> Ordering {
        match AppBuildNumberVersion::compare_operation(target_version) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}