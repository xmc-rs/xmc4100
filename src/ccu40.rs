#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gctrl: Gctrl,
    gstat: Gstat,
    gidls: Gidls,
    gidlc: Gidlc,
    gcss: Gcss,
    gcsc: Gcsc,
    gcst: Gcst,
    _reserved7: [u8; 0x34],
    ecrd: Ecrd,
    _reserved8: [u8; 0x2c],
    midr: Midr,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    #[inline(always)]
    pub const fn gctrl(&self) -> &Gctrl {
        &self.gctrl
    }
    #[doc = "0x04 - Global Status Register"]
    #[inline(always)]
    pub const fn gstat(&self) -> &Gstat {
        &self.gstat
    }
    #[doc = "0x08 - Global Idle Set"]
    #[inline(always)]
    pub const fn gidls(&self) -> &Gidls {
        &self.gidls
    }
    #[doc = "0x0c - Global Idle Clear"]
    #[inline(always)]
    pub const fn gidlc(&self) -> &Gidlc {
        &self.gidlc
    }
    #[doc = "0x10 - Global Channel Set"]
    #[inline(always)]
    pub const fn gcss(&self) -> &Gcss {
        &self.gcss
    }
    #[doc = "0x14 - Global Channel Clear"]
    #[inline(always)]
    pub const fn gcsc(&self) -> &Gcsc {
        &self.gcsc
    }
    #[doc = "0x18 - Global Channel Status"]
    #[inline(always)]
    pub const fn gcst(&self) -> &Gcst {
        &self.gcst
    }
    #[doc = "0x50 - Extended Capture Mode Read"]
    #[inline(always)]
    pub const fn ecrd(&self) -> &Ecrd {
        &self.ecrd
    }
    #[doc = "0x80 - Module Identification"]
    #[inline(always)]
    pub const fn midr(&self) -> &Midr {
        &self.midr
    }
}
#[doc = "GCTRL (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctrl`]
module"]
#[doc(alias = "GCTRL")]
pub type Gctrl = crate::Reg<gctrl::GctrlSpec>;
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "GSTAT (r) register accessor: Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gstat`]
module"]
#[doc(alias = "GSTAT")]
pub type Gstat = crate::Reg<gstat::GstatSpec>;
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "GIDLS (w) register accessor: Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gidls`]
module"]
#[doc(alias = "GIDLS")]
pub type Gidls = crate::Reg<gidls::GidlsSpec>;
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "GIDLC (w) register accessor: Global Idle Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidlc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gidlc`]
module"]
#[doc(alias = "GIDLC")]
pub type Gidlc = crate::Reg<gidlc::GidlcSpec>;
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "GCSS (w) register accessor: Global Channel Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcss`]
module"]
#[doc(alias = "GCSS")]
pub type Gcss = crate::Reg<gcss::GcssSpec>;
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "GCSC (w) register accessor: Global Channel Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsc`]
module"]
#[doc(alias = "GCSC")]
pub type Gcsc = crate::Reg<gcsc::GcscSpec>;
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "GCST (r) register accessor: Global Channel Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcst`]
module"]
#[doc(alias = "GCST")]
pub type Gcst = crate::Reg<gcst::GcstSpec>;
#[doc = "Global Channel Status"]
pub mod gcst;
#[doc = "ECRD (r) register accessor: Extended Capture Mode Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecrd::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecrd`]
module"]
#[doc(alias = "ECRD")]
pub type Ecrd = crate::Reg<ecrd::EcrdSpec>;
#[doc = "Extended Capture Mode Read"]
pub mod ecrd;
#[doc = "MIDR (r) register accessor: Module Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
#[doc(alias = "MIDR")]
pub type Midr = crate::Reg<midr::MidrSpec>;
#[doc = "Module Identification"]
pub mod midr;
