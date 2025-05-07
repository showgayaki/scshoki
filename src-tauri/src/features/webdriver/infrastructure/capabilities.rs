pub(in crate::features::webdriver) mod android;
pub(in crate::features::webdriver) mod ios;

// capabilitiesディレクトリの中から相対参照するとsuperがたくさんになるので、ここで再公開
pub(super) use super::super::constants::CHROME_DRIVER_PATH;
