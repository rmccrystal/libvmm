use crate::x86_64::instructions::msr::*;
use crate::x86_64::instructions::vmcs::*;
use crate::{AlignedAddress, SHIFT_4K};

#[derive(Debug)]
pub enum VMCSValidationFailure {
    PinBasedVmExecContrlFail,
    PrimaryProcBasedVmExecControlFail,
    SecondaryProcBasedVmExecContorlFail,
    VmExitControlFail,
    VmEntryControlFail,
    Cr3TargetCountFail,
    IoBitmapAAligned,
    IoBitmapBAligned,
    MsrBitmapAligned,
    VirtualApicPageAligned,
    TprThresholdA,
    TprThresholdB,
    Nmi1,
    Nmi2,
    TprShadowNotSet,
    X2Apic,
    VirtualInterruptDelivery1,
    PostedInterrupt1,
    PostedInterrupt2,
    PostedInterrupt3,
    PostedInterrupt4,
    PostedInterrupt5,
    VPID,
    EptMemTypeUC,
    EptMemTypeWB,
    EptIncorrectPageWalkLength,
    EptAccessedDirty,
    EptReservedBitsLow,
    PmlWithNoEpt,
    PmlAddressAligned,
    UnrestrictedGuestWithNoEpt,
    SubPageWritePermWithNoEpt,
    SPPTPointerAligned,
    VmReadBitmapAligned,
    VmWriteBitmapAligned,
    VirtualizationExceptionInfromationAddressAligned,
    PtUseGpa,
    TscMultiplier,
}

/// todo@
fn max_address() -> u64 {
    return 0;
}

