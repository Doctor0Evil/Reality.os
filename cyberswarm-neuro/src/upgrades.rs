use bioscale_upgrade_macros::bioscale_upgrade;
use bioscale_upgrade_store::DEFAULT_BIOPHYS_EVIDENCE;

#[bioscale_upgrade(
    id       = "telepathic_motor_v2",
    evidence = "DEFAULT_BIOPHYS_EVIDENCE"
)]
#[downgradeon(
    inflammation    = 2.0,
    pain            = 3.0,
    performancedelta= 10,
    contract        = "eth0x519fC0eB4111323Cac44b70e1aE31c30e405802D:downgradeGuardV1",
    consentledger   = "neural-consentv1:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"
)]
pub struct TelepathicMotorV2;
